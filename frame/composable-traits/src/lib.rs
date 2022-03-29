#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod assets;
pub mod auction;
pub mod bonded_finance;
pub mod call_filter;
pub mod currency;
pub mod defi;
pub mod dex;
pub mod governance;
pub mod lending;
pub mod liquidation;
pub mod math;
pub mod oracle;
pub mod privilege;
pub mod time;
pub mod vault;
pub mod vesting;
pub mod xcm;
