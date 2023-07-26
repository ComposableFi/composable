use crate::{cosmos::addess_hash, prelude::*};

use ibc_rs_scale::core::ics24_host::identifier::ChannelId;

pub const SENDER_PREFIX: &str = "ibc-wasm-hook-intermediary";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum IBCLifecycleComplete {
	#[serde(rename = "ibc_ack")]
	IBCAck {
		/// The source channel (osmosis side) of the IBC packet
		channel: ChannelId,
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
		channel: ChannelId,
		/// The sequence number that the packet was sent with
		sequence: u64,
	},
}

/// from Go code to make compliant wasm hook
pub fn derive_intermediate_sender(
	channel: &ChannelId,
	original_sender: &str,
	bech32_prefix: &str,
) -> Result<String, bech32_no_std::Error> {
	use bech32_no_std::ToBase32;
	let sender_str = alloc::format!("{channel}/{original_sender}");
	let sender_hash_32 = addess_hash(SENDER_PREFIX, sender_str.as_bytes());
	let sender = sender_hash_32.to_base32();
	bech32_no_std::encode(bech32_prefix, sender)
}

/// see https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks
#[derive(
	Serialize, Deserialize, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode,
)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct WasmMemo {
	pub contract: String,
	pub msg: Vec<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ibc_callback: Option<String>,
}
