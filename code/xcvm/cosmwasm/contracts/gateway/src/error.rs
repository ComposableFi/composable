use cosmwasm_std::{IbcOrder, StdError};
use thiserror::Error;
use xc_core::proto::DecodingFailure;

pub type ContractResult<T, E = ContractError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),
	#[error("Caller is not authorised to take this action.")]
	NotAuthorized,
	#[error("IBC channel version mismatch {0}.")]
	InvalidIbcVersion(String),
	#[error("Unexpected IBC channel ordering {0:?}.")]
	InvalidIbcOrdering(IbcOrder),
	#[error("An invalid XCVM packet has been received.")]
	InvalidIbcXcvmPacket,
	#[error("No IBC channel is opened to the target network.")]
	UnsupportedNetwork,
	#[error("Could not serialize to JSON")]
	FailedToSerialize,
	#[error("The asset is not yet supported.")]
	UnsupportedAsset,
	#[error("The contract must be initialized first.")]
	NotInitialized,
	#[error("An overflow occurred.")]
	ArithmeticOverflow,
	#[error("Not enough funds to cover the operation.")]
	InsufficientFunds,
	#[error("{0:?}")]
	Protobuf(DecodingFailure),
	#[error("An invalid ACK was provided, this MUST be impossible.")]
	InvalidAck,
	#[error("An unknown reply ID was provided, this MUST be impossible.")]
	UnknownReply,
	#[error("The provided channel has not been previously opened.")]
	UnknownChannel,
	#[error("The asset is already registered.")]
	AlreadyRegistered,
	#[error("Route not found.")]
	RouteNotFound,
	#[error("{0}")]
	Bech32(bech32_no_std::Error),
	#[error("{0}")]
	Serde(#[from] serde_json_wasm::ser::Error),
}

impl From<bech32_no_std::Error> for ContractError {
	fn from(value: bech32_no_std::Error) -> Self {
		Self::Bech32(value)
	}
}
