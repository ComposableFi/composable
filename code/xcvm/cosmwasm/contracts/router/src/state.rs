use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::NetworkId;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
	pub registry_address: Addr,
	pub interpreter_code_id: u64,
	pub network_id: NetworkId,
}

pub type UserId = Vec<u8>;

pub const INTERPRETERS: Map<(u8, UserId), Addr> = Map::new("interpreters");
pub const CONFIG: Item<Config> = Item::new("config");
