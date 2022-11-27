use cw_xcvm_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, Displayed, Funds, NetworkId, UserOrigin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BridgeMsg {
	pub user_origin: UserOrigin,
	pub network_id: NetworkId,
	pub security: BridgeSecurity,
	pub salt: Vec<u8>,
	pub program: DefaultXCVMProgram,
	pub assets: Funds<Displayed<u128>>,
}
