use std::env;
use std::fmt::Write;

use cosmos_sdk_proto::cosmwasm::wasm::v1::QuerySmartContractStateRequest;
use cosmrs::cosmwasm::*;
use cosmrs::rpc::{Client, HttpClient, HttpClientUrl};
use cw_mantis_order::OrderItem;
use mantis_node::mantis::{args::*, cosmos::*};

#[tokio::main]
async fn main() {
    let args = MantisArgs::parsed();
    let mut client = create_wasm_query_client(&args.centauri).await;

    while (true) {
        if let Some(assets) = args.simulate {
            simulate_order(&mut write_client, args.order_contract, assets).await;
        };
        let query = cw_mantis_order::QueryMsg::GetAllOrders {};
        let orders_request = QuerySmartContractStateRequest {
            address: args.order_contract.clone(),
            query_data: serde_json_wasm::to_vec(&query).expect("json"),
        };
        let orders = client
            .smart_contract_state(orders_request)
            .await
            .expect("orders obtained")
            .into_inner()
            .data;
        let orders: Vec<OrderItem> = serde_json_wasm::from_slice(&orders).expect("orders");

        // just print them for now
        println!("orders: {:?}", orders);
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
async fn simulate_order(write_client: WriteClient, order_contract: String, assets : String)  {
    if std::time::Instant::now().elapsed().as_millis() % 10 == 0 {
        
    }
}

//     if tick(1, 50) {
//         solve(orders_client, args.wallet, args.order_contract).await;
//     }
// }
// fn solve(orders_client: Client, wallet: String, order_contract: String) {
//     // get orders
//     // call bruno solver with formed data
//     // if any solved, post it

//     // for cffm next:
//     // 1. connect to osmosis node and get PICA<-> ATOM pool data
//     // 2. shape data and call algorithm

// }

