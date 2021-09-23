#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(trivial_numeric_casts)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create_new_market() -> Weight;
	fn deposit_collateral() -> Weight;
	fn withdraw_collateral() -> Weight;
	fn borrow() -> Weight;
	fn repay_borrow() -> Weight;
}

/// Weight functions for lending.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_new_market() -> Weight {
		(96_881_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn deposit_collateral() -> Weight {
		123_789_000_u64
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn withdraw_collateral() -> Weight {
		138_802_000_u64
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn borrow() -> Weight {
		(332_730_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(19 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	fn repay_borrow() -> Weight {
		(209_694_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}

impl WeightInfo for () {
	fn create_new_market() -> Weight {
		(96_881_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn deposit_collateral() -> Weight {
		123_789_000_u64
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn withdraw_collateral() -> Weight {
		138_802_000_u64
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn borrow() -> Weight {
		(332_730_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(19 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	fn repay_borrow() -> Weight {
		(209_694_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
}
