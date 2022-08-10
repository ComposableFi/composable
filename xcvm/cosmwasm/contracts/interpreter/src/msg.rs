use std::collections::VecDeque;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{
    Instruction, Program,
    Funds, NetworkID
};

pub type XCVMInstruction =
    Instruction<NetworkID, Vec<u8>, String, Funds>;

pub type XCVMProgram = Program<VecDeque<XCVMInstruction>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub registry_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Execute { program: XCVMProgram },
}
