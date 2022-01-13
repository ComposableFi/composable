#![allow(unused_parens)]
#![allow(unused_imports)]

use sp_std::marker::PhantomData;
use support::{traits::Get, weights::Weight};

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32) -> Weight {
		(14_618_000 as Weight)
			// Standard Error: 0
			.saturating_add((610_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(3_175_000 as Weight)
	}
	fn batch_all(c: u32) -> Weight {
		(14_561_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_013_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(10_000_000 as Weight)
	}
}
