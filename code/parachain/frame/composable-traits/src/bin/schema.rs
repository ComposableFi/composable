use cosmwasm_schema::write_api;

use composable_traits::{dex::{ExecuteMsg, QueryMsg}, prelude::*};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub enum InstantiateMsg {
}


fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}