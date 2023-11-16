#[cfg(not(target_arch = "wasm32"))]
use cosmwasm_schema::write_api;

#[cfg(not(target_arch = "wasm32"))]
use cw_mantis_order::*;

#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::disallowed_methods)]
fn main() {
	use cw_mantis_order::sv::*;
	write_api! {
		instantiate: InstantiateMsg,
		query: QueryMsg,
		execute: ExecMsg,
	}
}

#[cfg(target_arch = "wasm32")]
fn main() {}
