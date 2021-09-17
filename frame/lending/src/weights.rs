#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(trivial_numeric_casts)]

use frame_support::{traits::Get, weights::{constants::RocksDbWeight, Weight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn deposit_collateral() -> Weight;
	fn withdraw_collateral() -> Weight;
}

/// Weight functions for lending.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn deposit_collateral() -> Weight {
		(123_789_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn withdraw_collateral() -> Weight {
		(138_802_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

impl WeightInfo for () {
	fn deposit_collateral() -> Weight {
		(123_789_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn withdraw_collateral() -> Weight {
		(138_802_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
