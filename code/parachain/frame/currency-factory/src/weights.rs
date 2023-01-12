#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast, trivial_numeric_casts)]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_range() -> Weight;
	fn set_metadata() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn add_range() -> Weight {
		Weight::from_ref_time(83_205_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}

	fn set_metadata() -> Weight {
		Weight::from_ref_time(10_0000)
	}
}

impl WeightInfo for () {
	fn add_range() -> Weight {
		Weight::from_ref_time(10_0000)
	}

	fn set_metadata() -> Weight {
		Weight::from_ref_time(10_0000)
	}
}
