#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `mosaic`.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> liquidation::weights::WeightInfo for WeightInfo<T> {
	fn add_liquidation_strategy() -> Weight {
		10000
	}
}