use cosmwasm_std::{IbcOrder, StdError};
use thiserror::Error;

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
}
