extern crate alloc;

use alloc::string::String;
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

pub const CONFIG: Item<Config> = Item::new("config");
pub const OWNERS: Map<Addr, ()> = Map::new("owners");

// Registers
pub const IP_REGISTER: Item<u32> = Item::new("ip_register");
pub const RESULT_REGISTER: Item<Result<SubMsgResponse, String>> = Item::new("result_register");
pub const RELAYER_REGISTER: Item<Addr> = Item::new("relayer_register");
