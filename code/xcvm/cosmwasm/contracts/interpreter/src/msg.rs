use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use xcvm_core::{BridgeSecurity, Funds, Instruction, MessageOrigin, NetworkId, Program};

pub type XCVMInstruction = Instruction<NetworkId, Vec<u8>, String, Funds>;
pub type XCVMProgram = Program<VecDeque<XCVMInstruction>>;
pub type UserId = Vec<u8>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	pub registry_address: String,
	pub network_id: NetworkId,
	pub user_id: UserId,
	pub message_origin: MessageOrigin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	Execute { program: XCVMProgram, message_origin: MessageOrigin },
	SetMessageFilter { bridge_security: BridgeSecurity },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
