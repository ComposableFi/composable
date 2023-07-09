use crate::{cosmwasm::CosmwasmSubstrateError, prelude::*};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Forward {
	pub receiver: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub port: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub retries: Option<u64>,

	/// since other parachain does not support ibc memo
	/// there is only two option: send to parachain or send to relay-chain
	// #[serde(skip_serializing_if = "Option::is_none")]
	/// we do not need parrent id. if para id is none, it means send to relay-chain
	// pub parent: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub para_id: Option<u32>, //if para id is none, it means send to relay-chain
	#[serde(skip_serializing_if = "Option::is_none")]
	pub substrate: Option<bool>,
	///
	#[serde(skip_serializing_if = "Option::is_none")]
	pub next: Option<sp_std::boxed::Box<MemoData>>,
}

impl Forward {
	pub fn new_ibc_memo(
		receiver: String,
		port: String,
		channel: String,
		timeout: String,
		retries: u64,
	) -> Self {
		Self {
			receiver,
			port: Some(port),
			channel: Some(channel),
			timeout: Some(timeout),
			retries: Some(retries),
			para_id: None,
			substrate: None,
			next: None,
		}
	}

	pub fn new_xcm_memo(receiver: String, para_id: u32, substrate: bool) -> Self {
		Self {
			receiver,
			port: None,
			channel: None,
			timeout: None,
			retries: None,
			para_id: Some(para_id),
			substrate: Some(substrate),
			next: None,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MemoData {
	Forward(Forward),
	Wasm(xc_core::ibc::WasmMemo),
}

/// makes it easier to convert CW types to underlying IBC types without dependency on gazillion of
/// crates from centauri
pub trait CosmwasmIbc {
	fn transfer(
		from: cosmwasm_std::Addr,
		channel_id: String,
		to_address: String,
		amount: cosmwasm_std::Coin,
		timeout: cosmwasm_std::IbcTimeout,
	) -> Result<(), CosmwasmSubstrateError>;
}
