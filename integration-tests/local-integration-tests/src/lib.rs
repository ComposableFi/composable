#![deny(
	unused_imports,
	clippy::useless_conversion,
	bad_style,
	bare_trait_objects,
	const_err,
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
	unused_extern_crates
)]

/// this must be singleton
#[cfg(test)]
pub fn env_logger_init() {
	let _ = env_logger::builder().is_test(true).try_init();
}

#[cfg(test)]
mod kusama_test_net;

#[cfg(test)]
mod xcm_tests;

#[cfg(test)]
mod cross_chain_transfer;

#[cfg(test)]
mod runtime_tests;
