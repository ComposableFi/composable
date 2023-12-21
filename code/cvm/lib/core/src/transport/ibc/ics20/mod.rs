#![allow(clippy::disallowed_types)] // tells f32, weird, but works

pub mod hook;
pub mod pfm;

use crate::prelude::*;

use self::{hook::Callback, pfm::ForwardingMemo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
// Encode, Decode, scale_info::TypeInfo, to be manually implemented for subset of know messages
pub struct Memo {
	/// memo has at least one key, with value "wasm", than wasm hooks will try to execute it
	#[serde(skip_serializing_if = "Option::is_none")]
	pub wasm: Option<Callback>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub forward: Option<ForwardingMemo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
// Encode, Decode, scale_info::TypeInfo, to be manually implemented for subset of know messages
pub struct SendMemo {
	#[serde(flatten)]
	pub inner: Memo,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ibc_callback: Option<Addr>,
}

impl Memo {
	pub fn forward(forward: ForwardingMemo) -> Self {
		Self { forward: Some(forward), wasm: None }
	}
}
// We define the response as a prost message to be able to decode the protobuf data.
#[derive(Clone, PartialEq, Eq, prost::Message)]
pub struct MsgTransferResponse {
	#[prost(uint64, tag = "1")]
	pub sequence: u64,
}
