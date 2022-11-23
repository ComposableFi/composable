use cw_xcvm_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, Displayed, Funds, NetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	IbcSetNetworkChannel {
		network_id: NetworkId,
		channel_id: String,
	},
	Bridge {
		network_id: NetworkId,
		security: BridgeSecurity,
		salt: Vec<u8>,
		program: DefaultXCVMProgram,
		assets: Funds<Displayed<u128>>,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
