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
    10_000 as Weight
	}
	fn bond() -> Weight {
    10_000 as Weight
	}
	fn cancel() -> Weight {
    10_000 as Weight
	}
}
