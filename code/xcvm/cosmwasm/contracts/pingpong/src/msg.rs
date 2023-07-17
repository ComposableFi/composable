extern crate alloc;

use serde::{Deserialize, Serialize};
use xc_core::{NetworkId, UserOrigin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct InstantiateMsg {
	pub gateway_address: String,
	pub network_id: NetworkId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	Ping { user_origin: UserOrigin, counter: u32 },
	Pong { user_origin: UserOrigin, counter: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum QueryMsg {}
