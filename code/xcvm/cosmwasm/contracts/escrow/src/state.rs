use cosmwasm_std::Storage;
use xc_core::NetworkId;

use crate::error::{ContractError, Result};

const CONFIG_NS: &str = "config";
pub(crate) const ADMINS_NS: &str = "admins";
pub(crate) const PENDING_DEPOSITS_NS: &str = "deposits";
pub(crate) const LAST_DEPOSIT_ID_NS: &str = "deposit-last-id";
pub(crate) const BREAK_GLASS_NS: &str = "break-glass";

const CONFIG: cw_storage_plus::Item<Config> = cw_storage_plus::Item::new(CONFIG_NS);

/// Configuration of the contract.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Config {
	/// Network id of the chain this contract is running on.
	pub network_id: NetworkId,
	/// The XCVM gateway contract.
	pub gateway: xc_core::gateway::Gateway,
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
