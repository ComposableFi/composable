use serde_json::Value;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub enum IBCLifecycleComplete {
	#[serde(rename = "ibc_ack")]
	IBCAck {
		/// The source channel (osmosis side) of the IBC packet
		channel: String,
		/// The sequence number that the packet was sent with
		sequence: u64,
		/// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
		ack: String,
		/// Weather an ack is a success of failure according to the transfer spec
		success: bool,
	},
	#[serde(rename = "ibc_timeout")]
	IBCTimeout {
		/// The source channel (osmosis side) of the IBC packet
		channel: String,
		/// The sequence number that the packet was sent with
		sequence: u64,
	},
}

/// This message should be send as part of wasm termination memo.
/// So that can match it to sender hash and know what channel and origin was used to send message.
/// All information here is not secured until compared with existing secured data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct VerifiableWasmMsg {
	pub bech32_prefix: String,
	pub channel: String,
	pub original_sender: String,
	pub asset: Coin,
}

/// Message type for `sudo` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub enum SudoMsg {
	#[serde(rename = "ibc_lifecycle_complete")]
	IBCLifecycleComplete(IBCLifecycleComplete),
}

pub const SENDER_PREFIX: &str = "ibc-wasm-hook-intermediary";

/// from Go code to make compliant wasm hook
pub fn derive_intermediate_sender(
	channel: &str,
	original_sender: &str,
	bech32_prefix: &str,
) -> Result<String, ()> {
	use bech32_no_std::ToBase32;
	let sender_str = format!("{channel}/{original_sender}");
	let sender_hash_32 = crate::cosmos::addess_hash(SENDER_PREFIX, sender_str.as_bytes());
	let sender = sender_hash_32.to_base32();
	bech32_no_std::encode(&bech32_prefix, sender).map_err(|_| ())
}

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
	Wasm(Wasm),
}

/// see https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Wasm {
	contract: String,
	msg: Value,
	#[serde(skip_serializing_if = "Option::is_none")]
	ibc_callback: Option<String>,
}

impl MemoData {
	pub fn forward(forward: Forward) -> Self {
		Self::Forward(forward)
	}

	pub fn wasm(wasm: Wasm) -> Self {
		Self::Wasm(wasm)
	}
}
