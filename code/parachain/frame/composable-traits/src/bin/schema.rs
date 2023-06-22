use cosmwasm_schema::write_api;

use composable_traits::{
	dex::{ExecuteMsg, QueryMsg},
	prelude::*,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(JsonSchema))]
enum InstantiateMsg {}

#[allow(clippy::disallowed_methods)]
fn main() {
	write_api! {
		instantiate: InstantiateMsg,
		query: QueryMsg,
		execute: ExecuteMsg,
	}
}
