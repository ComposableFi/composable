use mantis_node::mantis::{args::*, cosmos::*};
use cosmos_sdk_proto::cosmwasm::wasm::v1::QuerySmartContractStateRequest;
use cosmrs::cosmwasm::*;
use cosmrs::rpc::{Client, HttpClient, HttpClientUrl};

#[tokio::main]
async fn main() {
    let args = MantisArgs::parsed();
    let mut client = create_wasm_query_client(&args.centauri).await;

    while (true) {
        let query = cw_mantis_order::QueryMsg::GetAllOrders {};
        let orders_request = QuerySmartContractStateRequest {
            address: args.order_contract.clone(),
            query_data:  serde_json_wasm::to_vec(&query).expect("json"),
        };
        let orders = client
            .smart_contract_state(orders_request)
            .await
            .expect("orders obtained")
            .into_inner();
        cw_mantis_order::c

        
        // just print them for now
        println!("orders: {:?}", orders);
    }
}


    // ================= work in progress =================
    //     // connect to orders node

    //     // request all orders
    //     let orders_request = QuerySmartContractStateRequest {
    //         address: args.order_contract,
    //         query_data: r###"{ "get_all_orders": {} }"###.as_bytes().to_vec(),
    //     };
    //     let orders_response = client
    //         .smart_contract_state(orders_request)
    //         .await
    //         .expect("orders obtained");
    
    //     // just print them for now
    //     println!("orders: {:?}", orders_response);
        
    // while (true) {
    //     if let Some(simulate) = args.simulate  {
    //         if tick(1,10) {
    //             random_orders(orders_client, args.order_contract, args.wallet, simulate).await;
    //         }
    //     }

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

// /// `simulate` - is comma separate list. each entry is amount u64 glued with alphanumeric denomination 
// /// that is splitted into array of CosmWasm coins.
// /// one coin is chose as given,
// /// from remaining 2 other coins one is chosen as wanted
// /// amount of count is randomized around 30% of given amount
// /// 
// /// `wallet` - is cosmos mnemonic, key to be created of it
// /// 
// /// `order_contract` - orders are formed for give and want, and send to orders contract.
// /// timeout is also randomized starting from 10 to 100 blocks
// #[async]
// fn random_orders(orders_client: Client, order_contract: String, wallet: String, simulate: String) {

// }