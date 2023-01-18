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
	fn adjust_rewards() -> Weight;
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
		Weight::from_ref_time(33_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn set_signer() -> Weight {
		Weight::from_ref_time(134_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn adjust_rewards() -> Weight {
		Weight::from_ref_time(134_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn add_stake() -> Weight {
		Weight::from_ref_time(219_457_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn remove_stake() -> Weight {
		Weight::from_ref_time(42_512_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn reclaim_stake() -> Weight {
		Weight::from_ref_time(51_245_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn submit_price(p: u32) -> Weight {
		Weight::from_ref_time(85_274_000_u64)
			// Standard Error: 148_000
			.saturating_add(Weight::from_ref_time(254_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn update_pre_prices(p: u32) -> Weight {
		Weight::from_ref_time(11_336_000_u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(238_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn update_price(p: u32) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 2_426_000
			.saturating_add(Weight::from_ref_time(22_017_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn add_asset_and_info() -> Weight {
		Weight::from_ref_time(33_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn set_signer() -> Weight {
		Weight::from_ref_time(134_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	fn adjust_rewards() -> Weight {
		Weight::from_ref_time(134_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	fn add_stake() -> Weight {
		Weight::from_ref_time(219_457_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn remove_stake() -> Weight {
		Weight::from_ref_time(42_512_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn reclaim_stake() -> Weight {
		Weight::from_ref_time(51_245_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn submit_price(p: u32) -> Weight {
		Weight::from_ref_time(85_274_000_u64)
			// Standard Error: 148_000
			.saturating_add(Weight::from_ref_time(254_000_u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn update_pre_prices(p: u32) -> Weight {
		Weight::from_ref_time(11_336_000_u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(238_000_u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn update_price(p: u32) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 2_426_000
			.saturating_add(Weight::from_ref_time(22_017_000_u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
