pub mod assets;
pub mod exchange;
pub mod interpreter;
pub mod tracking;
pub mod xcvm;
use crate::prelude::*;

use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Item;

use xc_core::NetworkId;

const CONFIG: Item<HereItem> = Item::new("this");

pub(crate) fn load(storage: &dyn Storage) -> StdResult<HereItem> {
	CONFIG.load(storage)
}

pub(crate) fn save(storage: &mut dyn Storage, value: &HereItem) -> StdResult<()> {
	CONFIG.save(storage, value)
}

/// the connection description from first network to second
pub(crate) const NETWORK_TO_NETWORK: Map<(NetworkId, NetworkId), OtherNetworkItem> =
	Map::new("network_to_network");

/// network state shared among all networks about it
pub(crate) const NETWORK: Map<NetworkId, NetworkItem> = Map::new("network");
