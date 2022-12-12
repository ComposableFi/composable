extern crate alloc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{NetworkId, UserOrigin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub router_address: String,
    pub network_id: NetworkId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Ping {
        user_origin: UserOrigin,
        counter: u32,
    },
    Pong {
        user_origin: UserOrigin,
        counter: u32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
