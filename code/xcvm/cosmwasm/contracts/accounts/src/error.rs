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

	#[error("IBC channel version mismatch {0}.")]
	InvalidIbcVersion(String),
	#[error("Unexpected IBC channel ordering {0:?}.")]
	InvalidIbcOrdering(IbcOrder),
	#[error("An invalid XCVM packet has been received.")]
	InvalidPacket,
	#[error("No IBC channel is opened to the target network.")]
	UnsupportedNetwork,
	#[error("Could not serialize to JSON")]
	FailedToSerialize,
	#[error("The asset is not yet supported.")]
	UnsupportedAsset,
	#[error("The contract must be initialized first.")]
	NotInitialized,
	#[error("An overflow occurred.")]
	ArithmeticOverflow,
	#[error("Not enough funds to cover the operation.")]
	InsufficientFunds,
	#[error("An invalid ACK was provided, this MUST be impossible.")]
	InvalidAck,
	#[error("An unknown reply ID was provided, this MUST be impossible.")]
	UnknownReply,
	#[error("The provided channel has not been previously opened.")]
	UnknownChannel,
	#[error("Entity is already registered.")]
	AlreadyRegistered,
	#[error("Unknown account.")]
	UnknownAccount,
	#[error("Account has locked asset {0}.")]
	HasLockedBalance(AssetId),
}
