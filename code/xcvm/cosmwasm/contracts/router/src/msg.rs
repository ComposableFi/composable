use crate::state::UserId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{Displayed, Funds, MessageOrigin, NetworkId};
use xcvm_interpreter::msg::ExecuteMsg as InterpreterExecuteMsg;

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
		message_origin: MessageOrigin,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
