#[cfg(feature = "std")]
use cosmwasm_schema::write_api;

#[cfg(feature = "std")]
use composable_traits::{dex, prelude::*};

#[cfg(feature = "std")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
enum InstantiateMsg {}

#[cfg(feature = "std")]
#[allow(clippy::disallowed_methods)]
fn main() {
	write_api! {
		instantiate: InstantiateMsg,
		query: dex::QueryMsg,
		execute: dex::ExecuteMsg,
	}
}

#[cfg(not(feature = "std"))]

fn main() {}
