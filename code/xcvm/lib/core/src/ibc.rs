
use crate::prelude::*;
use cosmwasm_std::IbcTimeout;
use serde_json::Value;

use crate::cosmos::addess_hash;


/// These are messages in the IBC lifecycle. Only usable by IBC-enabled contracts
/// (contracts that directly speak the IBC protocol via 6 entry points)
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
//#[cfg_attr(feature = "std", derive(JsonSchema))]
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


/// see https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Wasm {
	contract: String,
	msg: Value,
	#[serde(skip_serializing_if = "Option::is_none")]
	ibc_callback: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
//#[cfg_attr(feature = "std", derive(JsonSchema))]
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
//#[cfg_attr(feature = "std", derive(JsonSchema))]
pub struct VerifiableWasmMsg {
	pub bech32_prefix: String,
	pub channel: String,
	pub original_sender: String,
	pub asset: Coin,
}

/// Message type for `sudo` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
//#[cfg_attr(feature = "std", derive(JsonSchema))]
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
	let sender_hash_32 = addess_hash(SENDER_PREFIX, sender_str.as_bytes());
	let sender = sender_hash_32.to_base32();
	bech32_no_std::encode(bech32_prefix, sender)
}