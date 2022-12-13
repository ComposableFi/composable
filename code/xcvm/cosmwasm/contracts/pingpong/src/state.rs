extern crate alloc;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use xcvm_core::NetworkId;

pub const ROUTER: Item<Addr> = Item::new("router");
pub const NETWORK: Item<NetworkId> = Item::new("network");
