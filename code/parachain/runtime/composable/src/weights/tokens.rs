#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Default weights.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_tokens::WeightInfo for WeightInfo<T> {
	fn transfer() -> Weight {
		Weight::from_ref_time(69_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn transfer_all() -> Weight {
		Weight::from_ref_time(69_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn transfer_keep_alive() -> Weight {
		Weight::from_ref_time(38_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn force_transfer() -> Weight {
		Weight::from_ref_time(45_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn set_balance() -> Weight {
		Weight::from_ref_time(34_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
