mod args;
use args::*;
use clap::Parser;
use cosmos_sdk_proto::cosmwasm::wasm::v1::QuerySmartContractStateRequest;
use cosmrs::cosmwasm::*;
use cosmrs::rpc::{Client, HttpClient, HttpClientUrl};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let args = MantisArgs::parse();

    // connect to orders node
    let url = tonic::transport::Endpoint::from_str(&args.order_rpc).expect("url");
    let mut client = cosmos_sdk_proto::cosmwasm::wasm::v1::query_client::QueryClient::connect(url)
        .await
        .expect("connected");

    // request all orders
    let orders_request = QuerySmartContractStateRequest {
        address: args.order_contract,
        query_data: r###"{ "get_all_orders": {} }"###.as_bytes().to_vec(),
    };
    let orders_response = client
        .smart_contract_state(orders_request)
        .await
        .expect("orders obtained");

    // just print them for now
    println!("orders: {:?}", orders_response);

    // here are next steps are,
    // 1. connect to osmosis node and get PICA<-> ATOM pool data
    // 2. shape data and call algorithm
}
