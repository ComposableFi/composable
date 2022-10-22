use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::UserId;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use xcvm_core::NetworkId;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
	pub registry_address: Addr,
	pub network_id: NetworkId,
	pub user_id: UserId,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const OWNERS: Map<Addr, ()> = Map::new("owners");
