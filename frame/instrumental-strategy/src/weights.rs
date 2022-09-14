#![allow(clippy::unnecessary_cast)]

use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn test() -> Weight;
}

/// Weights for pallet_instrumental_strategy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn test() -> Weight {
		10_000 as Weight
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn test() -> Weight {
		10_000 as Weight
	}
}