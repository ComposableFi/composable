mod client;
mod input;
mod output;
mod prelude;

use clap::Parser;
use client::VestingSchedule;
use codec::Output;
use input::*;
use output::*;
use prelude::*;
use sp_core::{hexdisplay, Blake2Hasher, Pair};
use sp_runtime::print;
use std::{
	alloc::System,
	collections::{HashMap, HashSet},
	str::FromStr,
	time::SystemTime,
};
use subxt::{dynamic::Value, tx::PairSigner, utils::AccountId32, OnlineClient, SubstrateConfig};
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::client::{VestingScheduleKeyT, VestingScheduleT};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	let args = Args::parse();
	println!("{:?}", &args);
	match args.action {
		Action::Add(subargs) => {
			let csv_file: String =
				String::from_utf8(std::fs::read(subargs.schedule).expect("file")).expect("string");
			let mut rdr = csv::Reader::from_reader(csv_file.as_bytes());
			let all: Vec<_> = rdr.deserialize::<AddRecord>().map(|x| x.expect("record")).collect();
			let key = sp_core::sr25519::Pair::from_string(&subargs.key, None).expect("secret");
			let signer = PairSigner::new(key.clone());
			let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;

			let calls = all.clone().into_iter().map(|record| {
				let address = AccountId32::from_str(&record.account).expect("address");
				let data = vec![
					("from", Value::unnamed_variant("Id", vec![Value::from_bytes(key.public().0)])),
					(
						"beneficiary",
						Value::unnamed_variant("Id", vec![Value::from_bytes(address.0)]),
					),
					("asset", Value::u128(1)),
					(
						"schedule_info",
						Value::named_composite(vec![
							(
								"window",
								Value::named_variant(
									"MomentBased",
									vec![
										("start", Value::u128(record.window_moment_start as u128)),
										(
											"period",
											Value::u128(record.window_moment_period as u128),
										),
									],
								),
							),
							("period_count", Value::u128(record.period_count as u128)),
							("per_period", Value::u128(record.per_period)),
						]),
					),
				];
				let out = OutputRecord {
					to: record.account,
					// in substrate unix time is in milliseconds, while in unix it is in seconds
					window_start: format!(
						"{}",
						OffsetDateTime::from_unix_timestamp(
							(record.window_moment_start / 1000) as i64
						)
						.unwrap()
					),
					window_period: format!(
						"{}",
						Duration::milliseconds(record.window_moment_period as i64)
					),
					total: record.per_period * record.period_count as u128,
				};
				if std::time::SystemTime::UNIX_EPOCH
					.checked_add(std::time::Duration::from_secs(record.window_moment_start / 1000))
					.unwrap() <= std::time::SystemTime::now()
				{
					warn!("start earlier then now")
				}
				if std::time::Duration::from_secs(
					(record.window_moment_period / 1000) * record.period_count as u64,
				) < std::time::Duration::from_secs(7 * 86000)
				{
					warn!("vesting time  is less earlier then week")
				}
				(data, out)
			});

			let mut out = csv::Writer::from_writer(vec![]);

			if let Some(batch) = subargs.batch {
				let batch: Vec<_> = calls
					.into_iter()
					.map(|(data, record)| data)
					.map(|data| subxt::dynamic::tx("Vesting", "vested_transfer", data))
					.map(|x| x.into_value())
					.collect();

				let data = vec![("calls", batch)];
				let tx_value = subxt::dynamic::tx("Utility", "batch", data);
				let data = vec![
					("call", tx_value.into_value()),
					(
						"weight",
						Value::named_composite(vec![
							("ref_time", Value::u128(0)),
							("proof_size", Value::u128(0)),
						]),
					),
				];
				let tx = subxt::dynamic::tx("Sudo", "sudo_unchecked_weight", data);
				let signed =
					api.tx().create_signed(&tx, &signer, <_>::default()).await.expect("offline");
				let result = signed.dry_run(None).await;
				println!("dry_run {:?}", result);
	
				let tx = "0x".to_string() + &hex::encode(signed.into_encoded());
				println!(
					"Signed Sudo::sudoUncheckedWeight(Vesting::vested_transfer) \n {:}",
					&tx
				);
			} else {
				for (data, record) in calls.into_iter() {
					let tx_value = subxt::dynamic::tx("Vesting", "vested_transfer", data.clone());
					let signed = api
						.tx()
						.create_signed(&tx_value, &signer, <_>::default())
						.await
						.expect("offline");
					let result = signed.dry_run(None).await;
					info!("dry_run {:?}", result);

					let tx = "0x".to_string() + &hex::encode(signed.into_encoded());
					info!("Signed Vesting::vested_transfer {:?}", &tx);

					let data = vec![
						("call", tx_value.into_value()),
						(
							"weight",
							Value::named_composite(vec![
								("ref_time", Value::u128(0)),
								("proof_size", Value::u128(0)),
							]),
						),
					];
					let tx = subxt::dynamic::tx("Sudo", "sudo_unchecked_weight", data);
					let signed = api
						.tx()
						.create_signed(&tx, &signer, <_>::default())
						.await
						.expect("offline");
					let result = signed.dry_run(None).await;
					info!("dry_run {:?}", result);

					let tx = "0x".to_string() + &hex::encode(signed.into_encoded());
					info!("Signed Sudo::sudoUncheckedWeight(Vesting::vested_transfer) {:}", &tx);
					out.serialize(OutputRecordOne {
						vesting_schedule_added: tx,
						to: record.to,
						total: record.total,
						window_period: record.window_period,
						window_start: record.window_start,
					})
					.expect("serialize");
				}
			}
		},
		Action::Unlock(subargs) => {
			let csv_file: String =
				String::from_utf8(std::fs::read(subargs.schedule).expect("file")).expect("string");
			let mut rdr = csv::Reader::from_reader(csv_file.as_bytes());
			let all: Vec<_> =
				rdr.deserialize::<UnlockRecord>().map(|x| x.expect("record")).collect();
			let key = sp_core::sr25519::Pair::from_string(&subargs.key, None).expect("secret");
			let signer = PairSigner::new(key.clone());
			let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;
			let mut out = csv::Writer::from_writer(vec![]);
			let all: HashSet<_> = all.into_iter().map(|x| x.account).collect();

			let clean: Vec<_> = all
				.into_iter()
				.map(|record| {
					let address = AccountId32::from_str(&record).expect("address");
					vec![
						("who", Value::unnamed_variant("Id", vec![Value::from_bytes(address.0)])),
						("asset", Value::u128(1)),
						("vesting_schedules", Value::unnamed_composite(vec![])),
					]
				})
				.map(|data| subxt::dynamic::tx("Vesting", "update_vesting_schedules", data))
				.map(|x| x.into_value())
				.collect();

			let data = vec![("calls", clean)];
			let tx_value = subxt::dynamic::tx("Utility", "batch_all", data);
			let data = vec![
				("call", tx_value.into_value()),
				(
					"weight",
					Value::named_composite(vec![
						("ref_time", Value::u128(0)),
						("proof_size", Value::u128(0)),
					]),
				),
			];
			let tx = subxt::dynamic::tx("Sudo", "sudo_unchecked_weight", data);
			let signed =
				api.tx().create_signed(&tx, &signer, <_>::default()).await.expect("offline");
			let result = signed.dry_run(None).await;
			println!("dry_run {:?}", result);

			let tx = "0x".to_string() + &hex::encode(signed.into_encoded());
			println!(
				"Signed Sudo::sudoUncheckedWeight(Vesting::update_vesting_schedules) {:?}",
				&tx
			);
		},
		Action::Delete(subargs) => {
			let csv_file: String =
				String::from_utf8(std::fs::read(subargs.schedule).expect("file")).expect("string");
			let mut rdr = csv::Reader::from_reader(csv_file.as_bytes());
			let all: Vec<_> =
				rdr.deserialize::<DeleteRecord>().map(|x| x.expect("record")).collect();
			let key = sp_core::sr25519::Pair::from_string(&subargs.key, None).expect("secret");
			let signer = PairSigner::new(key.clone());
			let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;
			let mut out = csv::Writer::from_writer(vec![]);
			let mut deletes: HashMap<&str, u128> = HashMap::new();
			for record in all.iter() {
				let mut entry = deletes.entry(&record.account);
				let mut value = entry.or_default();
				*value += record.total;
			}

			let mut clean: Vec<_> = deletes
				.clone()
				.into_iter()
				.map(|record| {
					let address = AccountId32::from_str(&record.0).expect("address");
					vec![
						("who", Value::unnamed_variant("Id", vec![Value::from_bytes(address.0)])),
						("asset", Value::u128(1)),
						("vesting_schedules", Value::unnamed_composite(vec![])),
					]
				})
				.map(|data| subxt::dynamic::tx("Vesting", "update_vesting_schedules", data))
				.map(|x| x.into_value())
				.collect();

			let dest = AccountId32::from_str(&subargs.to).expect("address");

			let mut force: Vec<_> = deletes
				.into_iter()
				.map(|record| {
					let address = AccountId32::from_str(&record.0).expect("address");
					vec![
						(
							"source",
							Value::unnamed_variant("Id", vec![Value::from_bytes(address.0)]),
						),
						("dest", Value::unnamed_variant("Id", vec![Value::from_bytes(dest.0)])),
						("value", Value::u128(record.1)),
					]
				})
				.map(|data| subxt::dynamic::tx("Balances", "force_transfer", data))
				.map(|x| x.into_value())
				.collect();

			clean.append(&mut force);

			let data = vec![("calls", clean)];
			let tx_value = subxt::dynamic::tx("Utility", "batch_all", data);
			let data = vec![
				("call", tx_value.into_value()),
				(
					"weight",
					Value::named_composite(vec![
						("ref_time", Value::u128(0)),
						("proof_size", Value::u128(0)),
					]),
				),
			];
			let tx = subxt::dynamic::tx("Sudo", "sudo_unchecked_weight", data);
			let signed =
				api.tx().create_signed(&tx, &signer, <_>::default()).await.expect("offline");
			let result = signed.dry_run(None).await;
			match result {
				Ok(_) => info!("dry runned well",),
				Err(_) => println!("dry_run {:?}", result),
			}

			let tx = "0x".to_string() + &hex::encode(signed.into_encoded());
			println!(
				"Signed Sudo::sudoUncheckedWeight(Vesting::update_vesting_schedules+Balances::force_transfer):\n {:}",
				&tx
			);
		},
		Action::List(subargs) => {
			let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;
			let storage_address = subxt::dynamic::storage_root("Vesting", "VestingSchedules");
			let mut iter = api.storage().at(None).await?.iter(storage_address, 200).await?;

			let mut out = csv::Writer::from_writer(vec![]);
			while let Some((key, vesting_schedule)) = iter.next().await? {
				let vesting_schedule = VestingScheduleT::decode(&mut vesting_schedule.encoded())
					.expect("scale decoded");
				let key = VestingScheduleKeyT::decode(&mut key.0.as_ref()).expect("key decoded");

				for (id, record) in vesting_schedule.iter() {
					let (window_moment_start, window_moment_period) = match record.window {
						client::VestingWindow::MomentBased { start, period } => (start, period),
						_ => panic!("block to time"),
					};
					let window_start = match OffsetDateTime::from_unix_timestamp(
						(window_moment_start / 1000) as i64,
					)
					.map(|x| format!("{}", x))
					.map_err(|x| "#BAD_START_TME".to_string())
					{
						Err(x) => x,
						Ok(x) => x,
					};
					out.serialize(ListRecord {
						pubkey: hex::encode(&key.2),
						account: key.2.to_string(),

						window_start,
						window_period: format!(
							"{}",
							Duration::milliseconds(window_moment_period as i64)
						),
						total: record.per_period * record.period_count as u128,
						already_claimed: record.already_claimed,
						per_period: record.per_period,
						period_count: record.period_count,
						vesting_schedule_id: record.vesting_schedule_id,
					})
					.expect("out");
				}
			}

			out.flush()?;
			let out = out.into_inner().expect("table");

			if let Some(path) = subargs.out {
				let mut target = std::fs::File::create(path).expect("file");
				target.write(out.as_ref());
			} else {
				println!("All vestings:");
				let data = String::from_utf8(out).unwrap();
				println!("{}", data);
			}
		},
	}

	Ok(())
}
