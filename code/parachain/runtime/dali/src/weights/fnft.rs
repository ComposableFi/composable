#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `fnft`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fnft::WeightInfo for WeightInfo<T> {
	fn transfer() -> Weight {
		(10_000 as Weight)
	}
}
