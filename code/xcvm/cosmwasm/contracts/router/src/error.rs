use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Caller is not authorized to take this action")]
	NotAuthorized,

	#[error("The user did not provide enough fund to cover the execution.")]
	InsufficientFunds,
}
