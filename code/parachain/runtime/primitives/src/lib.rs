#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)]
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub mod currency;
mod prelude;
pub mod topology;
