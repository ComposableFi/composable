use cosmwasm_std::StdError;
use thiserror::Error;
use xcvm_core::BridgeSecurity;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Expected at least {0:?}, got {1:?}")]
	InsufficientBridgeSecurity(BridgeSecurity, BridgeSecurity),
}
