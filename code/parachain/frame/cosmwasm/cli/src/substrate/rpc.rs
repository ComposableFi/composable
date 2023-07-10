use crate::{
	args::{QueryCommand, QuerySubcommands, WasmInstantiate, WasmRpcQuery},
	error::Error,
};

use super::{cosmwasm, OutputType};
use clap::{Args, Subcommand};
use cosmwasm_std::{Binary, QueryRequest, WasmQuery};
use jsonrpc::{Request, Response};
use serde::de::DeserializeOwned;
use serde_json::{value::RawValue, Value};
use sp_core::crypto::AccountId32;

macro_rules! rpc_params {
    ( $( $x:expr ),* ) => {
        [
            $(jsonrpc::arg($x), )*
        ]
    }
}

pub struct QueryCommandRunner;

impl QueryCommandRunner {
	pub async fn run(
		command: QueryCommand,
		chain_endpoint: String,
		output: OutputType,
	) -> Result<(), Error> {
		match command.subcommands {
			QuerySubcommands::Wasm(WasmRpcQuery { contract, gas, query }) => {
				let query = QueryRequest::<()>::Wasm(WasmQuery::Smart {
					contract_addr: contract.to_string(),
					msg: Binary(query.into()),
				});
				let params = rpc_params!(contract.to_string(), gas, serde_json::to_vec(&query)?);
				let resp: Vec<u8> = rpc_call("cosmwasm_query", &params, chain_endpoint).await?;
				match output {
					OutputType::Text =>
						println!("[ + ] Query response: {}", String::from_utf8_lossy(&resp)),
					OutputType::Json => println!("{}", String::from_utf8_lossy(&resp)),
				}
				Ok(())
			},
		}
	}
}

async fn rpc_call<Res: DeserializeOwned>(
	method: &str,
	params: &[Box<RawValue>],
	endpoint: String,
) -> Result<Res, Error> {
	let client = reqwest::Client::new();
	let request = Request { method, params, id: Value::Number(1.into()), jsonrpc: Some("2.0") };
	let text = client
		.post(&endpoint)
		.header(reqwest::header::CONTENT_TYPE, "application/json")
		.body(serde_json::to_string(&request)?)
		.send()
		.await?
		.text()
		.await?;
	let response: Response = serde_json::from_str(&text)?;
	let result: Result<Res, Error> = response.result().map_err(Into::into);
	result
}
