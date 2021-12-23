#![cfg_attr(not(test), warn(clippy::disallowed_method, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod assets;
pub mod auction;
pub mod bonded_finance;
pub mod call_filter;
pub mod currency;
pub mod dex;
pub mod governance;
pub mod lending;
pub mod liquidation;
pub mod loans;
pub mod math;
pub mod oracle;
pub mod privilege;
pub mod rate_model;
pub mod vault;
pub mod vesting;
