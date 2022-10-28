extern crate alloc;

use alloc::{string::String, vec::Vec};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{NetworkId, Register};

pub type UserId = Vec<u8>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	pub registry_address: String,
	pub relayer_address: String,
	pub network_id: NetworkId,
	pub user_id: UserId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	Execute { program: Vec<u8> },
	// Only meant for to be used by the interpreter itself
	_SelfExecute { program: Vec<u8> },
	AddOwners { owners: Vec<Addr> },
	RemoveOwners { owners: Vec<Addr> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
	Register(Register),
}
