// TODO: make `deny`
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
// TODO: make `deny`
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
// TODO: make `deny`
#![warn(
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
	unused_extern_crates
)]
#![feature(associated_type_defaults)]
#![feature(trait_alias)]
#![feature(const_trait_impl)]
#![feature(const_convert)]

pub mod account_proxy;
pub mod airdrop;
pub mod assets;
pub mod bonded_finance;
pub mod bounded;
pub mod call_filter;
pub mod currency;
pub mod defi;
pub mod dex;
pub mod fnft;
pub mod governance;
pub mod lending;
pub mod liquidation;
pub mod mosaic;
pub mod oracle;
pub mod privilege;
pub mod staking;
pub mod time;
pub mod vault;
pub mod vesting;
pub mod xcm;
