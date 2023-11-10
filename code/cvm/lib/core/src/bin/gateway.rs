#[cfg(feature = "std")]
use cosmwasm_schema::write_api;

#[cfg(feature = "std")]
use xc_core::gateway;

#[cfg(feature = "std")]
#[allow(clippy::disallowed_methods)]
fn main() {
	write_api! {
		instantiate: gateway::InstantiateMsg,
		query: gateway::QueryMsg,
		execute: gateway::ExecuteMsg,
	}
}

#[cfg(not(feature = "std"))]
fn main() {}
