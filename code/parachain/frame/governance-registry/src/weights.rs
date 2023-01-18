#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn set() -> Weight;
	fn grant_root() -> Weight;
	fn remove() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn set() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn grant_root() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn remove() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// TODO: remove, really it is not needed
impl WeightInfo for () {
	fn set() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn grant_root() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn remove() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
