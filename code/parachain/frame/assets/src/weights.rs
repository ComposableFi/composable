#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn transfer() -> Weight;
	fn transfer_native() -> Weight;
	fn force_transfer() -> Weight;
	fn force_transfer_native() -> Weight;
	fn transfer_all() -> Weight;
	fn transfer_all_native() -> Weight;
	fn mint_initialize() -> Weight;
	fn set_administrator() -> Weight;
	fn mint_into() -> Weight;
	fn burn_from() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn transfer_all() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn transfer_all_native() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn transfer() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn transfer_native() -> Weight {
		Weight::from_ref_time(70_665_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn force_transfer() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn force_transfer_native() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn mint_initialize() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn set_administrator() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn mint_into() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn burn_from() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn transfer_native() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn force_transfer_native() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn transfer_all() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn transfer_all_native() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn transfer() -> Weight {
		Weight::from_ref_time(83_205_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn force_transfer() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn mint_initialize() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn set_administrator() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn mint_into() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn burn_from() -> Weight {
		Weight::from_ref_time(81_458_000_u64)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
