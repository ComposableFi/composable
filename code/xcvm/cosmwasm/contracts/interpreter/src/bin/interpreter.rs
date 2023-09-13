#[cfg(feature = "std")]
use cosmwasm_schema::write_api;

#[cfg(feature = "std")]
use cw_xc_interpreter::msg::*;

#[cfg(feature = "std")]
#[allow(clippy::disallowed_methods)]
fn main() {
	write_api! {
		instantiate: InstantiateMsg,
		query: QueryMsg,
		execute: ExecuteMsg,
	}
}

#[cfg(not(feature = "std"))]
fn main() {}
