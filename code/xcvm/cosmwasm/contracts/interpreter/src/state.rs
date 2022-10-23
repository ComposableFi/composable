extern crate alloc;

use crate::msg::UserId;
use alloc::string::String;
use cosmwasm_std::{Addr, SubMsgResponse};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::NetworkId;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
	pub registry_address: Addr,
	pub network_id: NetworkId,
	pub user_id: UserId,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const OWNERS: Map<Addr, ()> = Map::new("owners");
// Registers
pub const IP_REGISTER: Item<u32> = Item::new("ip_register");
pub const RESULT_REGISTER: Item<Result<SubMsgResponse, String>> = Item::new("result_register");
