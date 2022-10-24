use cosmwasm_std::StdError;
use thiserror::Error;
use xcvm_core::BridgeSecurity;

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

	#[error("Expected bridge security to be at least {0:?}, got {1:?}")]
	InsufficientBridgeSecurity(BridgeSecurity, BridgeSecurity),

	#[error("Caller is not authenticated to take the action")]
	NotAuthorized,

	#[error("Instruction {0} is not supported")]
	InstructionNotSupported(String),
}
