use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Error during parsing asset id")]
	CannotParseAssetId,

	#[error("Caller is not authenticated to take the action")]
	NotAuthorized,

	#[error("The asset is already registered. Please unregister it first")]
  AlreadyRegistered,

	#[error("The asset is not registered")]
  NotRegistered,
}
