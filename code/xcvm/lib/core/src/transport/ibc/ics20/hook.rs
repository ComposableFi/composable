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
	bech32_no_std::encode(bech32_prefix, sender, bech32_no_std::Variant::Bech32)
}

/// see https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks
/// Information about which contract to call when the crosschain CW spawn finishes
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
// Encode, Decode, scale_info::TypeInfo, to be manually implemented for subset of know messages
pub struct Callback {
	// really Addr, but it does not have scale, I guess we need to impl `type XcAddr = SS58 |
	// Bech32` with signer inside for serde
	pub contract: Addr,
	/// Is a valid JSON object. The contract will be called with this as the message.
	pub msg: serde_cw_value::Value,
}
