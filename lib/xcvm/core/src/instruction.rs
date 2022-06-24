use crate::XCVMProgram;
use alloc::{collections::VecDeque, vec::Vec};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum XCVMInstruction<Network, Payload, Account, Assets> {
	#[serde(rename_all = "snake_case")]
	Transfer { to: Account, assets: Assets },
	#[serde(rename_all = "snake_case")]
	Call { encoded: Payload },
	#[serde(rename_all = "snake_case")]
	Spawn { network: Network, salt: Vec<u8>, assets: Assets, program: XCVMProgram<VecDeque<Self>> },
}
