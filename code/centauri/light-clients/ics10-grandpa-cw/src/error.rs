use cosmwasm_std::StdError;
use ics10_grandpa::error::Error as GrandpaError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Unauthorized")]
	Unauthorized {},
	// Add any other custom errors you like here.
	// Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
	#[error("Storage error")]
	StorageError,

	#[error("Grandpa error: {0}")]
	Grandpa(String),
}

impl From<GrandpaError> for ContractError {
	fn from(e: GrandpaError) -> Self {
		ContractError::Grandpa(e.to_string())
	}
}
