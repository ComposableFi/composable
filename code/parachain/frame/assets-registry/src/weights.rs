#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

// The weight info trait for `pallet_assets_registry`.
pub trait WeightInfo {
	fn register_asset() -> Weight;
	fn update_asset() -> Weight;
	fn set_min_fee() -> Weight;
	fn update_asset_location() -> Weight;
}

impl WeightInfo for () {
	fn register_asset() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn update_asset() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn set_min_fee() -> Weight {
		Weight::from_parts(100_000, 0)
	}

	fn update_asset_location() -> Weight {
		Weight::from_parts(100_000, 0)
	}
}

/// Weights for pallet_assets_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn register_asset() -> Weight {
		Weight::from_parts(9_958_000_u64, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn update_asset() -> Weight {
		Weight::from_parts(9_958_000_u64, 0)
	}

	fn set_min_fee() -> Weight {
		Weight::from_parts(9_958_000_u64, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn update_asset_location() -> Weight {
		Weight::from_parts(9_958_000_u64, 0).saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
