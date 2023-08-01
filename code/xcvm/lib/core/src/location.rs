use crate::prelude::*;
#[cfg(feature="ibc")]
use ibc_rs_scale::applications::transfer::{BaseDenom, TracePath};
use thiserror::Error;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub enum ForeignAssetId {
	#[cfg(feature="ibc")]
	IbcIcs20(PrefixedDenom),
}

#[cfg(feature="ibc")]
impl From<PrefixedDenom> for ForeignAssetId {
	fn from(this: PrefixedDenom) -> Self {
		Self::IbcIcs20(this)
	}
}
#[cfg(feature="ibc")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct PrefixedDenom {
	/// A series of `{port-id}/{channel-id}`s for tracing the source of the token.
	pub trace_path: TracePath,
	/// Base denomination of the relayed fungible token.
	pub base_denom: BaseDenom,
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("TokenTransferError")]
	TokenTransferError,
}
