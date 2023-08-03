use cosmwasm_std::{Addr, ContractInfo, CosmosMsg, Storage};

use xc_core::AssetId;

use crate::error::{ContractError, Result};

/// Maps native coin denom to an asset identifier.
///
/// Returns an error if the denom is unrecognised (or if any other error while
/// trying to map the denom).
pub fn resolve_denom(_storage: &dyn Storage, _denom: String) -> Result<(AssetId, Native)> {
	todo!()
}

/// Wrapper for a denomination of a local asset.
#[derive(Clone, serde::Serialize, serde::Deserialize, derive_more::From, derive_more::Into)]
pub struct Native(String);

/// Representation of a local asset.  Either a native coin or CW20 token.
#[derive(Clone, serde::Serialize, serde::Deserialize, derive_more::From)]
pub enum Local {
	Native(Native),
	Cw20(cw20::Cw20Contract),
}

/// Constructs a Bank message which transfers all specified `coins` to
/// `recipient`.
///
/// Since `BankMsg::Send` takes vector of coins transferring all the funds can
/// be done in a single message.
pub fn make_bank_transfer_msg(recipient: Addr, coins: Vec<cosmwasm_std::Coin>) -> CosmosMsg {
	CosmosMsg::from(cosmwasm_std::BankMsg::Send { to_address: recipient.into(), amount: coins })
}

/// Maps local CW20 contract address to an asset identifier.
///
/// Returns an error if the address is unrecognised (or if any other error while
/// trying to map it to an asset).
pub fn resolve_cw20(
	_storage: &dyn Storage,
	_address: Addr,
) -> Result<(xc_core::AssetId, cw20::Cw20Contract)> {
	todo!()
}

pub trait Cw20Ext {
	/// Constructs a message to transfer `amount` CW20 tokens from sender to
	/// given `recipient`.
	fn make_transfer_msg(&self, recipient: Addr, amount: u128) -> Result<CosmosMsg>;

	/// Constructs a message to transfer `amount` CW20 tokens from given `owner`
	/// to `contract`.
	///
	/// The operation relies on `owner` pre-approving given `amount` to the
	/// `contract`.  I.e. it takes advantage of the allowance CW20 interface.
	fn make_take_msg(
		&self,
		contract: &ContractInfo,
		owner: Addr,
		amount: u128,
	) -> Result<CosmosMsg>;
}

impl Cw20Ext for cw20::Cw20Contract {
	fn make_transfer_msg(&self, recipient: Addr, amount: u128) -> Result<CosmosMsg> {
		self.call(cw20::Cw20ExecuteMsg::Transfer {
			recipient: recipient.into(),
			amount: amount.into(),
		})
		.map_err(ContractError::from)
	}

	fn make_take_msg(
		&self,
		contract: &ContractInfo,
		owner: Addr,
		amount: u128,
	) -> Result<CosmosMsg> {
		self.call(cw20::Cw20ExecuteMsg::TransferFrom {
			owner: owner.into(),
			recipient: contract.address.clone().into(),
			amount: amount.into(),
		})
		.map_err(ContractError::from)
	}
}
