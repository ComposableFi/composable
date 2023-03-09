mod input;
mod output;
mod prelude;

use clap::Parser;
use input::*;
use output::*;
use prelude::*;
use sp_core::Pair;
use std::str::FromStr;
use subxt::{dynamic::Value, tx::PairSigner, utils::AccountId32, OnlineClient, SubstrateConfig};
use tracing::info;

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
			let all: Vec<_> = rdr.deserialize::<Record>().map(|x| x.expect("record")).collect();

			let key = sp_core::sr25519::Pair::from_string(&subargs.key, None).expect("secret");

			let signer = PairSigner::new(key.clone());

			let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;

			let mut out = csv::Writer::from_writer(vec![]);
			for record in all {
				println!("{:?}", &record);

				let address = AccountId32::from_str(&record.address).expect("address");

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
				let tx_value = subxt::dynamic::tx("Vesting", "vested_transfer", data.clone());
				let signed = api
					.tx()
					.create_signed(&tx_value, &signer, <_>::default())
					.await
					.expect("offline");
				let result = signed.dry_run(None).await;
				info!("dry_run {:?}", result);

				let tx = api.tx().create_unsigned(&tx_value).expect("offline");
				{
					let tx = "0x".to_string() + &hex::encode(signed.into_encoded());
					info!("Signed Vesting::vested_transfer {:?}", &tx);
				}

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
				info!("dry_run {:?}", result);

				let tx = api.tx().create_unsigned(&tx).expect("offline");
				let tx = "0x".to_string() + &hex::encode(signed.into_encoded());
				info!("Signed Sudo::sudoUncheckedWeight(Vesting::vested_transfer) {:?}", &tx);
				out.serialize(OutputRecord {
					to: record.address,
					vesting_schedule_added: tx,
					window_start: format!(
						"{}",
						OffsetDateTime::from_unix_timestamp(record.window_moment_start as i64)
							.unwrap()
					),
					window_period: format!(
						"{}",
						Duration::seconds(record.window_moment_period as i64)
					),
					total: record.per_period * record.period_count as u128,
				})
				.expect("out");
			}
			out.flush()?;
			let out = out.into_inner().expect("table");
			let data = String::from_utf8(out).unwrap();
			println!("-=================================-");
			println!("{}", data);
		},
		Action::List => {
			let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;
			let storage_address = subxt::dynamic::storage_root("Vesting", "VestingSchedules");
			let mut iter = api.storage().at(None).await?.iter(storage_address, 10).await?;
			while let Some((key, vestingSchedule)) = iter.next().await? {
				println!("{}: {}", hex::encode(key), vestingSchedule.to_value()?);
			}
		},
	}

	Ok(())
}
