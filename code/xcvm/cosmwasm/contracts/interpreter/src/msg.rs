extern crate alloc;

use alloc::{string::String, vec::Vec};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{NetworkId, Register};

pub type UserId = Vec<u8>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	/// Address of the gateway,
	pub gateway_address: String,
	/// Address of the XCVM asset registry
	pub registry_address: String,
	/// Address of the router
	pub router_address: String,
	/// Network ID of the origin network
	pub network_id: NetworkId,
	/// Id of the origin user
	pub user_id: UserId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	/// Execute an XCVM program
	Execute { relayer: Addr, program: Vec<u8> },
	/// This is only meant to be used by the interpreter itself, otherwise it will return an error
	_SelfExecute { relayer: Addr, program: Vec<u8> },
	/// Add owners of this contract
	AddOwners { owners: Vec<Addr> },
	/// Remove owners from the contract
	RemoveOwners { owners: Vec<Addr> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {
	/// Owners to be added to the list of owners which acts more like a recovery in case all of the
	/// owners are erased accidentally
	pub owners: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
	/// Get a specific register
	Register(Register),
}
