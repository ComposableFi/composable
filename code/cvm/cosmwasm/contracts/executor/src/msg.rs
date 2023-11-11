use crate::{prelude::*, state, state::State};
use xc_core::{shared::*, InterpreterOrigin, Register};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct Step {
	/// Tip party facilitated bridging and execution.
	pub tip: Addr,
	/// The current instruction pointer in the program.
	/// Note that the [`Step::program`] instructions are poped when executed, we can't rely on this
	/// instruction pointer to index into the instructions. In fact, this pointer tells us how many
	/// instructions we already consumed.
	pub instruction_pointer: u16,
	/// The next instructions to execute (actual program).
	pub program: XcProgram,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
	/// Address of the gateway.
	pub gateway_address: String,
	/// The interpreter origin.
	pub interpreter_origin: InterpreterOrigin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	/// Execute an CVM program
	Execute { tip: Addr, program: XcProgram },
	/// This is only meant to be used by the interpreter itself, otherwise it will return an error
	/// The existence of this message is to allow the execution of the `Call` instruction. Once we
	/// hit a call, the program queue the call and queue itself after it to ensure that the side
	/// effect of the call has been executed.
	ExecuteStep { step: Step },
	/// Add owners of this contract
	AddOwners { owners: Vec<Addr> },
	/// Remove owners from the contract
	RemoveOwners { owners: Vec<Addr> },
	/// spawn is cross chain, so sometimes errors are came from other blocks
	/// so gateway can set that error on interpreter
	SetErr { reason: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
	/// Owners to be added to the list of owners which acts more like a recovery in case all of the
	/// owners are erased accidentally
	pub owners: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema, QueryResponses))]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
	/// Get a specific register
	#[cfg_attr(feature = "std", returns(QueryStateResponse))]
	Register(Register),
	/// dumps the whole state of interpreter
	#[cfg_attr(feature = "std", returns(QueryStateResponse))]
	State(),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct QueryStateResponse {
	pub state: state::State,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct QueryExchangeResponse {
	pub state: State,
}
