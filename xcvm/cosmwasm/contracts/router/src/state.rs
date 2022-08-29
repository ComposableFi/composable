use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::NetworkId;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
	pub registry_address: Addr,
}

pub type UserId = Vec<u8>;

pub const INTERPRETER_CODE_ID: Item<u64> = Item::new("interpreter_code_id");
pub const INTERPRETERS: Map<(u8, UserId), Addr> = Map::new("interpreters");
pub const CONFIG: Item<Config> = Item::new("config");
