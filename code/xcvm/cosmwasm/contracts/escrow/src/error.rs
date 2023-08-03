use cosmwasm_std::{Response, StdError};
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

	#[error("Invalid IBC packet.")]
	InvalidIbcPacket,

	#[error("Invalid CW20 message.")]
	InvalidCw20Packet,

	#[error("Internal contract error.")]
	InternalError,
}
