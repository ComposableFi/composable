use clap::Parser;
use cosmrs::rpc::{Client, HttpClient, HttpClientUrl};
use std::str::FromStr;

#[derive(clap::Parser)]
struct MantisArgs {
    /// the node hosting order contract
    #[arg(long)]
    order_rpc: String,
    /// address of the order contract on `order_rpc` chain
    #[arg(long)]
    order_contract: String,
}

#[tokio::main]
async fn main() {
    let args: MantisArgs = MantisArgs::parse();
    let url = HttpClientUrl::from_str(&args.order_rpc).expect("url");

    // just gets orders from storage and show
    let cosmwasm: HttpClient = cosmrs::rpc::HttpClient::builder(url)
        .build()
        .expect("client");
    let orders = cosmwasm
        .abci_query(None, "", None, false)
        .await
        .expect("order");
    println!("orders: {:?}", orders);
}
