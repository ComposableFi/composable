use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use cw_xc_common::gateway::{ExecuteMsg, InstantiateMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
