//! proposed shared XCM setup parameters and impl

#[cfg(test)]
mod tests;

use frame_support::{dispatch::Weight, parameter_types, log};
parameter_types! {
	pub const BaseXcmWeight: Weight = 100_000_000;
}
