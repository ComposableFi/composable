use super::cosmwasm;
use clap::{Args, Subcommand};
use cosmwasm_std::{Binary, QueryRequest, WasmQuery};
use hex::ToHex;
use jsonrpc::{Request, Response};
use serde::de::DeserializeOwned;
use serde_json::{value::RawValue, Value};
use sp_core::crypto::AccountId32;
use std::collections::BTreeMap;

#[derive(Args, Debug)]
pub struct Command {
	#[command(subcommand)]
	pub subcommands: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
	/// Query a CosmWasm contract
	Query(cosmwasm::Query),

	/// Dry-run an instantiate call
	#[group(skip)]
	Instantiate {
		/// Caller of the instantiate call
		#[arg(long)]
		sender: AccountId32,
		#[command(flatten)]
		instantiate: cosmwasm::Instantiate,
	},
}

#[derive(Args, Debug)]
pub struct Asd {
	#[arg(short, long)]
	world: String,
}

macro_rules! rpc_params {
    ( $( $x:expr ),* ) => {
        [
            $(jsonrpc::arg($x), )*
        ]
    }
}

impl Command {
	pub async fn run(self, chain_endpoint: String) -> anyhow::Result<()> {
		match self.subcommands {
			Subcommands::Query(cosmwasm::Query { contract, gas, query }) => {
				let query = QueryRequest::<()>::Wasm(WasmQuery::Smart {
					contract_addr: format!("0x{}", contract.clone().encode_hex::<String>()),
					msg: Binary(query.into()),
				});
				let params = rpc_params!(contract.to_string(), gas, serde_json::to_vec(&query)?);
				let resp: Vec<u8> = rpc_call("cosmwasm_query", &params, chain_endpoint).await?;
				println!("[ + ] Query response: {}", String::from_utf8_lossy(&resp));
				Ok(())
			},
			Subcommands::Instantiate {
				sender,
				instantiate:
					cosmwasm::Instantiate { code_id, salt, admin, label, funds, gas, message },
			} => {
				let params = rpc_params!(
					sender,
					code_id,
					Into::<Vec<u8>>::into(salt),
					admin,
					Into::<Vec<u8>>::into(label),
					funds
						.unwrap_or_default()
						.into_iter()
						.map(|(asset, amount)| (asset, (amount, true)))
						.collect::<BTreeMap<u128, (u128, bool)>>(),
					gas,
					Into::<Vec<u8>>::into(message)
				);
				let resp: AccountId32 =
					rpc_call("cosmwasm_instantiate", &params, chain_endpoint).await?;
				println!("[ + ] Contract address: {}", resp);
				Ok(())
			},
		}
	}
}

async fn rpc_call<Res: DeserializeOwned>(
	method: &str,
	params: &[Box<RawValue>],
	endpoint: String,
) -> anyhow::Result<Res> {
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
	let result: anyhow::Result<Res> = response.result().map_err(Into::into);
	result
}
