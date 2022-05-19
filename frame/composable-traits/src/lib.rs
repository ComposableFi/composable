#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod assets;
pub mod bonded_finance;
pub mod bounded;
pub mod call_filter;
pub mod currency;
pub mod defi;
pub mod dex;
pub mod financial_nft;
pub mod governance;
pub mod lending;
pub mod liquidation;
pub mod mosaic;
pub mod oracle;
pub mod privilege;
pub mod staking_rewards;
pub mod time;
pub mod vault;
pub mod vesting;
pub mod xcm;
