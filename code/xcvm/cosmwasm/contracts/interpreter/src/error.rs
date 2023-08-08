use cosmwasm_std::{Response, StdError};
use thiserror::Error;
use xc_core::LateBindingError;

pub type Result<T = Response, E = ContractError> = core::result::Result<T, E>;

impl From<xc_core::ArithmeticError> for ContractError {
	fn from(_: xc_core::ArithmeticError) -> Self {
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

	#[error("Caller is not authorised to take the action")]
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

	#[error("An error occured while doing arithmetic operations.")]
	ArithmeticError,

	#[error("Not implemented")]
	NotImplemented,

	#[error("The asset is not yet supported.")]
	UnsupportedAsset,
}
