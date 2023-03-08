mod input;

use clap::Parser;
use csv::DeserializeRecordsIntoIter;
use input::*;
use sp_core::Pair;
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
	let mut iter = rdr.deserialize::<Record>();

    let key = sp_core::sr25519::Pair::from_string(&args.key, None).expect("secret");
	//let signer = PairSigner::new( AccountKeyring::Alice.pair());
	let signer = PairSigner::new(key);


	let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;


	let dest = AccountKeyring::Bob.to_account_id();
	let tx = subxt::dynamic::tx("System", "remark_with_event", vec![Value::from_bytes([0, 1])]);

	println!("submit the transaction with default params");
	println!("{:?}", &tx);
	let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;
	println!("Balance transfer extrinsic submitted: {hash}");
	Ok(())
}
