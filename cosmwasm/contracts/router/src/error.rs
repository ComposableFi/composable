use cosmwasm_std::StdError;
use thiserror::Error;
use xcvm_core::BridgeSecurity;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Security requirement not meet, expected {0:?}")]
	ExpectedBridgeSecurity(BridgeSecurity),

	#[error("Caller is not authorized to take this action")]
	NotAuthorized,

	#[error("The user did not provide enough fund to cover the execution.")]
	InsufficientFunds,
}
