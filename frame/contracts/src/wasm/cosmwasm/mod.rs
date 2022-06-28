use serde::{Deserialize, Serialize};
use sp_std::{collections::vec_deque::VecDeque, vec::Vec};
use xcvm_core::*;

pub mod types;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComposableMsg {
	XCVM {
    salt: Vec<u8>,
		funds: XCVMTransfer<Displayed<u128>>,
		program:
			XCVMProgram<VecDeque<XCVMInstruction<XCVMNetwork, Vec<u8>, Vec<u8>, XCVMTransfer>>>,
	},
}
