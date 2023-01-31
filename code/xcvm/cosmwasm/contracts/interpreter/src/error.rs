use cosmwasm_std::StdError;
use thiserror::Error;
use xcvm_core::{BridgeSecurity, LateBindingError};

impl From<()> for ContractError {
	fn from(_: ()) -> Self {
		ContractError::InvalidProgram
	}
}

impl From<LateBindingError<ContractError>> for ContractError {
	fn from(e: LateBindingError<ContractError>) -> Self {
		match e {
			LateBindingError::InvalidBinding => ContractError::InvalidBindings,
			LateBindingError::App(e) => e,
		}
	}
}

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Invalid call payload")]
	InvalidCallPayload,

	#[error("Data cannot be serialized")]
	DataSerializationError,

	#[error("XCVM program is invalid")]
	InvalidProgram,

	#[error("A program tag must be a correct utf8 encoded string")]
	InvalidProgramTag,

	#[error("Bindings are invalid")]
	InvalidBindings,

	#[error("Expected bridge security to be at least {0:?}, got {1:?}")]
	InsufficientBridgeSecurity(BridgeSecurity, BridgeSecurity),

	#[error("Caller is not authenticated to take the action")]
	NotAuthorized,

	#[error("Only the contract is authorized for this action")]
	NotSelf,

	#[error("Instruction {0} is not supported")]
	InstructionNotSupported(String),

	#[error("Address is invalid")]
	InvalidAddress,

	#[error("Native token doesn't support `decimals`")]
	DecimalsInNativeToken,

	#[error("Unsupported")]
	Unsupported,
}
