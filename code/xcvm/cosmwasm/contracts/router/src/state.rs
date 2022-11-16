use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, NetworkId, UserOrigin};

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

pub const INTERPRETERS: Map<UserOrigin, Interpreter> = Map::new("interpreters");
pub const CONFIG: Item<Config> = Item::new("config");
