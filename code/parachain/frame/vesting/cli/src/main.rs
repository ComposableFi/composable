mod args;

use args::*;
use clap::Parser;
use subxt::SubstrateConfig;
use sp_keyring::AccountKeyring;
use subxt::{
    dynamic::Value,
    tx::PairSigner,
    OnlineClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
	let args = Args::parse();
    println!("{:?}", &args);
	let api = OnlineClient::<SubstrateConfig>::from_url(args.client).await?;
    // 1. Dynamic Balance Transfer (the dynamic equivalent to the balance_transfer example).
    //sp_core::{sr25519::Pair
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        
        let dest = AccountKeyring::Bob.to_account_id();
        let tx = subxt::dynamic::tx(
        "System",
        "remark",
        vec![
            Value::from_bytes([0,1]),
        ],
    );
    
    println!("submit the transaction with default params");
    println!("{:?}", &tx);
    let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;
    println!("Balance transfer extrinsic submitted: {hash}");
    Ok(())
}
