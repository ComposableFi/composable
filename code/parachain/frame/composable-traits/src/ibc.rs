use crate::{cosmwasm::CosmwasmSubstrateError, prelude::*};
use cosmwasm_std::IbcTimeout;
use serde_json::Value;
use std::fs::rename;

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
) -> Result<String, bech32_no_std::Error> {
	use bech32_no_std::ToBase32;
	let sender_str = alloc::format!("{channel}/{original_sender}");
	let sender_hash_32 = crate::cosmos::addess_hash(SENDER_PREFIX, sender_str.as_bytes());
	let sender = sender_hash_32.to_base32();
	bech32_no_std::encode(bech32_prefix, sender)
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

// impl From<pallet_ibc::ics20::MemoData> for MemoData {
// 	fn from(value: pallet_ibc::ics20::MemoData) -> Self {
// 		MemoData::Forward(value)
// 	}
// }
//
// impl From<pallet_ibc::ics20::Forward> for Forward {
// 	fn from(value: pallet_ibc::ics20::Forward) -> Self {
// 		Forward {
// 			receiver: value.receiver,
// 			port,
// 			channel: None,
// 			timeout: None,
// 			retries: None,
// 			para_id: None,
// 			substrate: None,
// 			next: value.next.map(Into::into),
// 		}
// 	}
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemoData {
	Forward(Forward),
	Wasm(Wasm),
}

trait Memo {
	fn receiver(&self) -> &str;
}

#[test]
fn tst() {
	let data = MemoData::Forward(Forward {
		receiver: "".to_string(),
		port: None,
		channel: None,
		timeout: None,
		retries: None,
		para_id: None,
		substrate: None,
		next: Some(Box::new(MemoData::Forward(Forward {
			receiver: "".to_string(),
			port: None,
			channel: None,
			timeout: None,
			retries: None,
			para_id: None,
			substrate: None,
			next: None,
		}))),
	});
	println!("{}", serde_json::to_string(&data).unwrap());
	let s = serde_json::to_string(&data).unwrap();
	let data2: MemoData = serde_json::from_str(&s).unwrap();
	assert_eq!(data, data2);
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

/// These are messages in the IBC lifecycle. Only usable by IBC-enabled contracts
/// (contracts that directly speak the IBC protocol via 6 entry points)
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum IbcMsg {
	/// Sends bank tokens owned by the contract to the given address on another chain.
	/// The channel must already be established between the ibctransfer module on this chain
	/// and a matching module on the remote chain.
	/// We cannot select the port_id, this is whatever the local chain has bound the ibctransfer
	/// module to.
	Transfer {
		/// exisiting channel to send the tokens over
		channel_id: String,
		/// address on the remote chain to receive these tokens
		to_address: String,
		/// packet data only supports one coin
		/// https://github.com/cosmos/cosmos-sdk/blob/v0.40.0/proto/ibc/applications/transfer/v1/transfer.proto#L11-L20
		amount: Coin,
		/// when packet times out, measured on remote chain
		timeout: IbcTimeout,
		memo: Option<String>,
	},
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
