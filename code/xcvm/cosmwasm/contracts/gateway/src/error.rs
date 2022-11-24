use cosmwasm_std::{IbcOrder, StdError};
use thiserror::Error;
use xcvm_proto::DecodingFailure;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),
	#[error("Caller is not authorized to take this action.")]
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
	#[error("The required BridgeSecurity is not yet supported.")]
	UnsupportedBridgeSecurity,
	#[error("The asset is not yet supported.")]
	UnsupportedAsset,
	#[error("The contract must be initialized first.")]
	NotInitialized,
	#[error("An overflow occured.")]
	ArithmeticOverflow,
	#[error("Not enough funds to cover the operation.")]
	InsufficientFunds,
	#[error("{0:?}")]
	Protobuf(DecodingFailure),
	#[error("The function is not yet implemented.")]
	Unimplemented,
	#[error("An invalid ACK was provided, this MUST be impossible.")]
	InvalidAck,
	#[error("An unknown reply ID was provided, this MUST be impossible.")]
	UnknownReply,
}
