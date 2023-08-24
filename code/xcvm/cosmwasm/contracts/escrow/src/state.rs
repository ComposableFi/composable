use cosmwasm_std::{Api, Storage};
use xc_core::NetworkId;

use crate::{
	error::{ContractError, Result},
	msg,
};

const CONFIG_NS: &str = "config";
pub(crate) const ADMINS_NS: &str = "admins";
pub(crate) const PENDING_DEPOSITS_NS: &str = "deposits";
pub(crate) const LAST_DEPOSIT_ID_NS: &str = "deposit-last-id";
pub(crate) const BREAK_GLASS_NS: &str = "break-glass";

const CONFIG: cw_storage_plus::Item<Config> = cw_storage_plus::Item::new(CONFIG_NS);

/// Configuration of the contract.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Config {
	/// Network id of the chain this contract is running on.
	pub network_id: NetworkId,

	/// Location of the accounts contract.
	pub accounts_contract: AccountsContract,

	/// The XCVM gateway contract.
	pub gateway: xc_core::gateway::Gateway,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AccountsContract {
	Local(cosmwasm_std::Addr),
	Remote(String),
	None,
}

impl AccountsContract {
	pub fn from_msg(api: &dyn Api, ac: msg::AccountsContract) -> Result<Self> {
		Ok(match ac {
			msg::AccountsContract::Local(addr) => Self::Local(api.addr_validate(addr.as_str())?),
			msg::AccountsContract::Remote(channel_id) => Self::Remote(channel_id),
			msg::AccountsContract::None => Self::None,
		})
	}
}

impl Config {
	/// Loads configuration from the persistent storage.
	pub(crate) fn load(storage: &dyn Storage) -> Result<Self> {
		CONFIG.load(storage).map_err(ContractError::from)
	}

	/// Saves configuration to the persistent storage.
	pub(crate) fn save(&self, storage: &mut dyn Storage) -> Result<()> {
		CONFIG.save(storage, self).map_err(ContractError::from)
	}
}
