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
#![allow(incomplete_features)]
#![feature(associated_type_defaults)] // https://github.com/Rust-for-Linux/linux/issues/2
#![feature(trait_alias)] // complete
#![feature(const_trait_impl)] // https://github.com/Rust-for-Linux/linux/issues/2
#![feature(const_convert)] // that is just const fn for into/from - easy
#![feature(adt_const_params)] // avoids write own serde and bit shifts for Rational64

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
