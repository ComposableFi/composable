use cosmwasm_std::{Addr, SubMsgResponse};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::InterpreterOrigin;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
	pub gateway_address: Addr,
	pub registry_address: Addr,
	pub router_address: Addr,
	pub interpreter_origin: InterpreterOrigin,
}

/// The interpreter configuration.
pub const CONFIG: Item<Config> = Item::new("config");

/// List of owners able to execute programs on our behalf. Be aware that only `trusted` address must
/// be added.
pub const OWNERS: Map<Addr, ()> = Map::new("owners");

/// This register hold the latest program instruction (index) executed.
pub const IP_REGISTER: Item<u16> = Item::new("ip_register");

/// This register contains the latest executed program result.
pub const RESULT_REGISTER: Item<Result<SubMsgResponse, String>> = Item::new("result_register");

/// This register contains the latest relayer that executed a program on our behalf.
pub const RELAYER_REGISTER: Item<Addr> = Item::new("relayer_register");
