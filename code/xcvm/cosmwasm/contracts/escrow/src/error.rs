use cosmwasm_std::{IbcOrder, Response, StdError};
use thiserror::Error;

pub type Result<T = Response, E = ContractError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),

	#[error("Caller is not authorised to take this action.")]
	NotAuthorized,

	#[error("Contract is in broken glass state and doesn’t allow non-admin actions.")]
	BrokenGlass,

	#[error("Unrecognised asset ‘{0}’.")]
	UnknownAsset(String),

	#[error("Invalid IBC channel version ‘{0}’.")]
	InvalidIbcVersion(String),
	#[error("Invalid IBC channel ordering ‘{0:?}’.")]
	InvalidIbcOrdering(IbcOrder),
	#[error("Invalid packet.")]
	InvalidPacket,
	#[error("Remote contract error: {0}")]
	RemoteContractError(String),

	#[error("Accounts contract not found")]
	NoAccountsContract,

	#[error("Internal contract error.")]
	InternalError,
}
