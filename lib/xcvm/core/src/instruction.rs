use crate::XCVMProgram;
use alloc::collections::VecDeque;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum XCVMInstruction<Network, Payload, Account, Assets> {
	#[serde(rename_all = "camelCase")]
	Transfer { to: Account, assets: Assets },
	#[serde(rename_all = "camelCase")]
	Call { encoded: Payload },
	#[serde(rename_all = "camelCase")]
	Spawn { network: Network, assets: Assets, program: XCVMProgram<VecDeque<Self>> },
}
