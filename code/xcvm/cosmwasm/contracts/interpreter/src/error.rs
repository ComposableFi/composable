use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Invalid call payload")]
	InvalidCallPayload,

	#[error("Data cannot be serialized")]
	DataSerializationError,

	#[error("A program tag must be a correct utf8 encoded string")]
	InvalidProgramTag,

	#[error("Bindings are invalid")]
	InvalidBindings,
}
