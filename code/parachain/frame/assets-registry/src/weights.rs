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
}

impl WeightInfo for () {
	fn register_asset() -> Weight {
		0
	}

	fn update_asset() -> Weight {
		0
	}

	fn set_min_fee() -> Weight {
		0
	}
}

/// Weights for pallet_assets_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn register_asset() -> Weight {
		(9_958_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn update_asset() -> Weight {
		(9_958_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn set_min_fee() -> Weight {
		(9_958_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
