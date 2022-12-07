extern crate alloc;

use alloc::{string::String, vec::Vec};
use cosmwasm_std::Addr;
use cw_xcvm_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{InterpreterOrigin, Register};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	/// Address of the gateway.
	pub gateway_address: String,
	/// Address of the XCVM asset registry.
	pub registry_address: String,
	/// Address of the router.
	pub router_address: String,
	/// The interpreter origin.
	pub interpreter_origin: InterpreterOrigin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	/// Execute an XCVM program
	Execute { relayer: Addr, program: DefaultXCVMProgram },
	/// This is only meant to be used by the interpreter itself, otherwise it will return an error
	/// The existence of this message is to allow the execution of the `Call` instruction. Once we
	/// hit a call, the program queue the call and queue itself after it to ensure that the side
	/// effect of the call has been executed.
	ExecuteStep { relayer: Addr, program: DefaultXCVMProgram },
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
