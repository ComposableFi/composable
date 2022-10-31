use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{Bridge, BridgeSecurity, NetworkId as XCVMNetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
	pub registry_address: Addr,
	pub relayer_address: Addr,
	pub interpreter_code_id: u64,
	pub network_id: XCVMNetworkId,
}

pub type UserId = Vec<u8>;
type NetworkId = u8;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Interpreter {
	pub address: Option<Addr>,
	pub security: BridgeSecurity,
}

pub const INTERPRETERS: Map<(NetworkId, UserId), Interpreter> = Map::new("interpreters");
pub const CONFIG: Item<Config> = Item::new("config");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const BRIDGES: Map<Bridge, ()> = Map::new("bridges");
