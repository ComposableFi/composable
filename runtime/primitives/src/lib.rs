#![cfg_attr(not(test), warn(clippy::disallowed_method))] // allow in tests
#![cfg_attr(not(feature = "std"), no_std)]

pub mod currency;
