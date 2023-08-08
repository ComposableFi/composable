use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Item;
use xc_core::NetworkId;

use crate::{error, msg};

pub(crate) const ADMINS_NS: &str = "admins";
pub(crate) const CONFIG_NS: &str = "config";
pub(crate) const ACCOUNTS_NS: &str = "accounts";
pub(crate) const RECOVERY_ADDRESSES_NS: &str = "recovery-addrs";
pub(crate) const BREAK_GLASS_NS: &str = "break-glass";
pub(crate) const IBC_CHANNEL_INFO_NS: &str = "channels";
pub(crate) const IBC_NETWORK_CHANNEL_NS: &str = "networks";

/// Configuration of the contract.
#[cw_serde]
pub(crate) struct Config {
	/// Identifier of the network this contract is running on.
	pub network_id: NetworkId,

	/// Address of an escrow account running locally.
	///
	/// If specified, the contract with this address may execute
	/// [`ExecuteMsg::LocalPacket`] messages on the accounts contract and they
	/// will be interpreted like cross-chain messages from `network_id`.
	pub local_escrow: Option<cosmwasm_std::Addr>,
}

impl Config {
	/// Storage for the [`Config`].
	const CONFIG: Item<'_, Config> = Item::new(CONFIG_NS);

	pub fn try_instantiate(
		api: &dyn cosmwasm_std::Api,
		msg: &msg::InstantiateMsg,
	) -> error::Result<Self> {
		let network_id = msg.network_id;
		let local_escrow =
			msg.local_escrow.as_ref().map(|addr| api.addr_validate(addr)).transpose()?;
		Ok(Self { network_id, local_escrow })
	}

	pub fn load(storage: &dyn Storage) -> StdResult<Self> {
		Self::CONFIG.load(storage)
	}

	pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
		Self::CONFIG.save(storage, self)
	}
}
