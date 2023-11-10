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

	#[error("Program is invalid")]
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

	#[error("Unsupported")]
	Unsupported,

	#[error("An error occured while doing arithmetic operations")]
	ArithmeticError,

	#[error("Not implemented")]
	NotImplemented,

	#[error("The asset is not yet supported")]
	UnsupportedAsset,

	#[error("Only single asset exchange is supported by pool")]
	OnlySingleAssetExchangeIsSupportedByPool,

	/// for the case when specific pool does not supports slippage
	#[error("Exchange does not support slippage")]
	ExchangeDoesNotSupportSlippage,

	#[error("Cannot define both slippage and limit at same time")]
	CannotDefineBothSlippageAndLimitAtSameTime,

	#[error("Asset not found: {0}")]
	AssetNotFound(StdError),
	#[error("Exchange not found: {0}")]
	ExchangeNotFound(StdError),

	#[error("Asset unsupported on this network")]
	AssetUnsupportedOnThisNetwork,
}
