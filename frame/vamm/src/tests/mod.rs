// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use, dead_code)]

mod close;
mod compute_invariant;
mod constants;
mod create_vamm;
mod get_price;
mod get_twap;
mod helpers;
mod helpers_propcompose;
mod move_price;
mod swap;
mod swap_simulation;
mod types;
mod update_twap;
