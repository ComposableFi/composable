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

/// Weights for pallet_assets_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn register_asset() -> Weight {
		(9_958_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn update_asset() -> Weight {
		(9_958_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_min_fee() -> Weight {
		(9_958_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
