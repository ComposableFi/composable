use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XCVMProgram<Instructions> {
	pub instructions: Instructions,
	pub nonce: u32,
}
