use crate::state::UserId;
use cw_xcvm_interpreter::msg::ExecuteMsg as InterpreterExecuteMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{Bridge, Displayed, Funds, NetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	pub registry_address: String,
	pub interpreter_code_id: u64,
	pub network_id: NetworkId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	Run {
		network_id: NetworkId,
		user_id: UserId,
		interpreter_execute_msg: InterpreterExecuteMsg,
		funds: Funds<Displayed<u128>>,
		bridge: Bridge,
	},
	RegisterBridge {
		bridge: Bridge,
	},
	UnregisterBridge {
		bridge: Bridge,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
