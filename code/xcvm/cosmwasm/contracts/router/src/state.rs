use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use cw_xcvm_utils::UserId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, NetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
  pub gateway_address: Addr,
	pub registry_address: Addr,
	pub interpreter_code_id: u64,
	pub network_id: NetworkId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Interpreter {
	pub address: Option<Addr>,
	pub security: BridgeSecurity,
}

pub const INTERPRETERS: Map<(u32, UserId), Interpreter> = Map::new("interpreters");
pub const CONFIG: Item<Config> = Item::new("config");
pub const ADMIN: Item<Addr> = Item::new("admin");
