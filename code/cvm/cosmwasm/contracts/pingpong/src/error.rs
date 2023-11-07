use cosmwasm_std::{Response, StdError};
use thiserror::Error;

pub type Result<T = Response, E = ContractError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),
}
