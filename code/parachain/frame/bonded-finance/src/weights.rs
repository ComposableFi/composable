#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::weights::Weight;

pub trait WeightInfo {
	fn offer() -> Weight;
	fn bond() -> Weight;
	fn cancel() -> Weight;
}

impl WeightInfo for () {
	fn offer() -> Weight {
    Weight::from_ref_time(10_000)
	}
	fn bond() -> Weight {
    Weight::from_ref_time(10_000)
	}
	fn cancel() -> Weight {
    Weight::from_ref_time(10_000)
	}
}
