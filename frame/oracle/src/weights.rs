#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

// The weight info trait for `pallet_oracle`.
pub trait WeightInfo {
	fn add_asset_and_info() -> Weight;
	fn set_signer() -> Weight;
	fn add_stake() -> Weight;
	fn remove_stake() -> Weight;
	fn reclaim_stake() -> Weight;
	fn submit_price(p: u32) -> Weight;
	fn update_pre_prices(p: u32) -> Weight;
	fn update_price(p: u32) -> Weight;
}

/// Weights for pallet_oracle using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn add_asset_and_info() -> Weight {
		(33_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn set_signer() -> Weight {
		(134_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn add_stake() -> Weight {
		(219_457_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_stake() -> Weight {
		(42_512_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn reclaim_stake() -> Weight {
		(51_245_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn submit_price(p: u32) -> Weight {
		(85_274_000 as Weight)
			// Standard Error: 148_000
			.saturating_add((254_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn update_pre_prices(p: u32) -> Weight {
		(11_336_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((238_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn update_price(p: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 2_426_000
			.saturating_add((22_017_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn add_asset_and_info() -> Weight {
		(33_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn set_signer() -> Weight {
		(134_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn add_stake() -> Weight {
		(219_457_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn remove_stake() -> Weight {
		(42_512_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn reclaim_stake() -> Weight {
		(51_245_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn submit_price(p: u32) -> Weight {
		(85_274_000 as Weight)
			// Standard Error: 148_000
			.saturating_add((254_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn update_pre_prices(p: u32) -> Weight {
		(11_336_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((238_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn update_price(p: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 2_426_000
			.saturating_add((22_017_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
