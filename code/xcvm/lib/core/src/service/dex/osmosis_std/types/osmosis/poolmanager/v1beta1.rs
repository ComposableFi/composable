use crate::prelude::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, prost::Message, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MsgSwapExactAmountIn {
	#[prost(string, tag = "1")]
	pub sender: String,
	#[prost(message, repeated, tag = "2")]
	pub routes: Vec<SwapAmountInRoute>,
	#[prost(message, optional, tag = "3")]
	pub token_in: ::core::option::Option<crate::cosmos::Coin>,
	#[prost(string, tag = "4")]
	pub token_out_min_amount: String,
}

impl MsgSwapExactAmountIn {
	pub const PROTO_MESSAGE_URL: &str = "/osmosis.poolmanager.v1beta1.MsgSwapExactAmountIn";
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, prost::Message, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct SwapAmountInRoute {
	#[prost(uint64, tag = "1")]
	#[serde(alias = "poolID")]
	pub pool_id: u64,
	#[prost(string, tag = "2")]
	pub token_out_denom: String,
}

impl SwapAmountInRoute {
	pub const PROTO_MESSAGE_URL: &str = "/osmosis.poolmanager.v1beta1.SwapAmountInRoute";
}
