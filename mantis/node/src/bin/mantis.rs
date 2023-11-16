use cosmos_sdk_proto::cosmwasm::wasm::v1::QuerySmartContractStateRequest;
use cosmrs::cosmwasm::*;
use cosmrs::rpc::{Client, HttpClient, HttpClientUrl};
use cw_mantis_order::OrderItem;
use mantis_node::{
    mantis::{args::*, cosmos::*},
    prelude::*,
};
use std::fmt::Write;

#[tokio::main]
async fn main() {
    let args = MantisArgs::parsed();
    let mut client = create_wasm_query_client(&args.centauri).await;

    while (true) {
        if let Some(assets) = args.simulate {
            simulate_order(&mut write_client, args.order_contract, assets).await;
        };
    }
}

/// `assets` - is comma separate list. each entry is amount u64 glued with alphanumeric denomination
/// that is splitted into array of CosmWasm coins.
/// one coin is chosen as given,
/// from remaining 2 other coins one is chosen as wanted
/// amount of count is randomized around values
///
/// `write_client`
/// `order_contract` - orders are formed for give and want, and send to orders contract.
/// timeout is also randomized starting from 10 to 100 blocks
///
/// Also calls `timeout` so old orders are cleaned.
async fn simulate_order(write_client: WriteClient, order_contract: String, assets: String) {
    if std::time::Instant::now().elapsed().as_millis() % 10 == 0 {}
}

/// gets orders, groups by pairs
/// solves them using algorithm
/// if any volume solved, posts solution
///
/// gets data from chain pools/fees on osmosis and neutron
/// gets CVM routing data
/// uses cfmm algorithm
async fn solve(read: ReadClient, write: WriteClient, order_contract: String, cvm_contract: String) {
    let query = cw_mantis_order::QueryMsg::GetAllOrders {};
    let orders_request = QuerySmartContractStateRequest {
        address: args.order_contract.clone(),
        query_data: serde_json_wasm::to_vec(&query).expect("json"),
    };
    let orders = read
        .smart_contract_state(orders_request)
        .await
        .expect("orders obtained")
        .into_inner()
        .data;
    let orders: Vec<OrderItem> = serde_json_wasm::from_slice(&orders).expect("orders");

    let orders = orders
        .into_iter()
        .group_by(|x| (x.given.denom, x.msg.wants.denom).into_iter().sorted());
    for pair in orders {
        // solve here !
        // post solution
        // just print them for now
        println!("orders: {:?}", orders);
    }
}
