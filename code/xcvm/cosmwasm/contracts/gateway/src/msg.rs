use crate::state::Config;
use cosmwasm_std::CosmosMsg;
use cw_xcvm_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, Displayed, Funds, NetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	pub config: Config,
}

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
	Batch {
		msgs: Vec<CosmosMsg>,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
