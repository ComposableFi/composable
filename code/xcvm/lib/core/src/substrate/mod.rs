//! so we need just some code which helps to convert substrate and CW primitives back and forth
use crate::prelude::*;

/// Errors from integration of CosmwWasm <-> Substrate (types, conversions, encoding, host
/// functions, etc)
#[derive(thiserror::Error, Debug)]
pub enum CosmwasmSubstrateError {
	#[error("")]
	AssetConversion,
	#[error("")]
	AccountConvert,
	#[error("")]
	DispatchError,
	#[error("")]
	QuerySerialize,
	#[error("")]
	ExecuteSerialize,
	#[error("")]
	Ibc,
}

/// makes it easier to convert CW types to underlying IBC types without dependency on gazillion of
/// crates from centauri
pub trait CosmwasmIbc {
	fn transfer(
		from: cosmwasm_std::Addr,
		channel_id: String,
		to_address: String,
		amount: cosmwasm_std::Coin,
		timeout: cosmwasm_std::IbcTimeout,
	) -> Result<(), CosmwasmSubstrateError>;
}
