//! proposed shared XCM setup parameters and impl

use frame_support::{dispatch::Weight, parameter_types};
parameter_types! {
	pub const BaseXcmWeight: Weight = 100_000_000;
}
