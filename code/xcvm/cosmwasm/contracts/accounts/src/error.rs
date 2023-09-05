use cosmwasm_std::{IbcOrder, Response, StdError};
use thiserror::Error;

use xc_core::AssetId;

pub type Result<T = Response, E = ContractError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),
	#[error("Caller is not authorised to take this action.")]
	NotAuthorized,
	#[error("Contract is in broken glass state and doesn’t allow non-admin actions.")]
	BrokenGlass,

	#[error("Invalid IBC channel version ‘{0}’.")]
	InvalidIbcVersion(String),
	#[error("Invalid IBC channel ordering ‘{0:?}’.")]
	InvalidIbcOrdering(IbcOrder),
	#[error("An invalid XCVM packet has been received: {0}")]
	InvalidPacket(xc_core::proto::DecodeError),

	#[error("Account has been already registered.")]
	AlreadyRegistered,
	#[error("Unknown account.")]
	UnknownAccount,
	#[error("Account has locked asset {0}.")]
	HasLockedBalance(AssetId),

	#[error("Overflow during arithmetic operation.")]
	ArithmeticOverflow,
}

impl From<xc_core::proto::DecodeError> for ContractError {
	fn from(err: xc_core::proto::DecodeError) -> Self {
		Self::InvalidPacket(err)
	}
}
