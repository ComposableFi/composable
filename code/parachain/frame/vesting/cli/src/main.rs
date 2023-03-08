mod input;

use clap::Parser;
use csv::DeserializeRecordsIntoIter;
use input::*;
use sp_core::{hexdisplay::AsBytesRef, Pair};
use sp_keyring::AccountKeyring;
use std::io::Read;
use subxt::{dynamic::Value, tx::PairSigner, OnlineClient, SubstrateConfig};

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
		let data = vec![
			(
				"from",
				Value::unnamed_variant("Id", vec![Value::from_bytes(key.public().as_bytes_ref())]),
			),
			(
				"beneficiary",
				Value::unnamed_variant("Id", vec![Value::from_bytes(record.address.as_bytes())]),
			),
            (
				"asset",
				Value::u128(1),
			),
            (
				"schedule_info",
				Value::u128(1),
			),
		];
		let tx = subxt::dynamic::tx("Vesting", "vested_transfer", data);
		println!("submit the transaction with default params");
		println!("{:?}", &tx);
		let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;
		println!("Balance transfer extrinsic submitted: {hash}");
	}

	Ok(())
}
