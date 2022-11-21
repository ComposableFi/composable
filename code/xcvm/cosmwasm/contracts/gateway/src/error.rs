use cosmwasm_std::StdError;
use thiserror::Error;
use xcvm_core::BridgeSecurity;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),
	#[error("Caller is not authorized to take this action")]
	NotAuthorized,
}
