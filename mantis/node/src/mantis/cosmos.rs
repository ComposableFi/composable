use crate::prelude::*;

use cosmos_sdk_proto::cosmwasm::wasm::v1::QuerySmartContractStateRequest;
use cosmrs::cosmwasm::*;
use cosmrs::rpc::{Client, HttpClient, HttpClientUrl};
use cosmos_sdk_proto::cosmwasm::wasm::v1::query_client::QueryClient;
use tonic::transport::Channel;


pub  async fn  create_wasm_query_client(rpc: &str) -> QueryClient<Channel> {
    let url = tonic::transport::Endpoint::from_str(rpc).expect("url");
    QueryClient::connect(url)
        .await
        .expect("connected")
}