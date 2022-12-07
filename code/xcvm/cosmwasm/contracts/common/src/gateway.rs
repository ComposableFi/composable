use crate::shared::BridgeMsg;
use cosmwasm_std::{Addr, CosmosMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::NetworkId;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	IbcSetNetworkChannel { network_id: NetworkId, channel_id: String },
	Bridge { interpreter: Addr, msg: BridgeMsg },
	Batch { msgs: Vec<CosmosMsg> },
}
