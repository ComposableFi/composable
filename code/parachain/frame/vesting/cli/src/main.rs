mod input;

use clap::Parser;
use csv::DeserializeRecordsIntoIter;
use input::*;
use sp_core::{hexdisplay::AsBytesRef, Pair};
use sp_keyring::AccountKeyring;
use tracing::info;
use std::{io::Read, str::FromStr};
use subxt::{dynamic::Value, tx::PairSigner, utils::AccountId32, OnlineClient, SubstrateConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	let args = Args::parse();
	println!("{:?}", &args);
	let csv_file: String =
		String::from_utf8(std::fs::read(args.schedule).expect("file")).expect("string");
	let mut rdr = csv::Reader::from_reader(csv_file.as_bytes());
	let all: Vec<_> = rdr.deserialize::<Record>().map(|x| x.expect("record")).collect();
	

	let key = sp_core::sr25519::Pair::from_string(&args.key, None).expect("secret");
	
	let signer = PairSigner::new(key.clone());

	let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;

	for record in all {
		println!("{:?}", &record);

		let address = AccountId32::from_str(&record.address).expect("address");

		let data = vec![
			("from", Value::unnamed_variant("Id", vec![Value::from_bytes(key.public().0)])),
			("beneficiary", Value::unnamed_variant("Id", vec![Value::from_bytes(address.0)])),
			("asset", Value::u128(1)),
			(
				"schedule_info",
				Value::named_composite(vec![
					(
						"window",
						Value::named_variant(
							"MomentBased",
							vec![("start", Value::u128(1)), ("period", Value::u128(1))],
						),
					),
					("period_count", Value::u128(1)),
					("per_period", Value::u128(1)),
				]),
			),
		];
		let tx = subxt::dynamic::tx("Vesting", "vested_transfer", data);
		let signed = api.tx().create_signed(&tx, &signer, <_>::default()).await.expect("offline");
		signed.dry_run(None).await.expect("dry_run");
		let signed = hex::encode(signed.into_encoded());
		info!("Signed Vesting::vested_transfer `0x{:?}`", signed);

		// just for testing, do not really submit
		// let hash = api.tx().sign_and_submit_then_watch_default(&tx, &signer).await?;
		// let result = hash.wait_for_finalized_success().await.expect("block");		
		// info!("Vesting schedule_info submitted {:?}", result);
	}

	Ok(())
}
