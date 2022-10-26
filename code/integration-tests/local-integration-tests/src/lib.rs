#![cfg_attr(
	not(any(test, feature = "runtime-benchmarks")),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	unused_imports,
	clippy::useless_conversion,
	bad_style,
	bare_trait_objects,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates,
	dead_code,
	unused_must_use
)]

#[cfg(test)]
pub fn env_logger_init() {
	use std::sync::Once;
	static START: Once = Once::new();
	START.call_once(|| {
		env_logger::builder()
			.is_test(true)
			.try_init()
			.expect("test log runs in env it can run");
	});
}

#[cfg(test)]
mod kusama_test_net;

#[cfg(test)]
mod low_level_xcm_orml_tests;

#[cfg(test)]
mod cross_chain_transfer;

#[cfg(test)]
mod runtime_tests;

#[cfg(test)]
mod assets_integration;
#[cfg(test)]
mod helpers;
pub mod prelude;
mod relaychain;
#[cfg(feature = "dali")]
#[cfg(test)]
mod transact_calls;

#[cfg(test)]
mod common_goods_assets;
#[cfg(test)]
mod errors;
pub mod testing;
#[cfg(test)]
mod relay_transfer;
