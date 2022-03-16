use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_margin() -> Weight;
}

/// Weights for pallet_clearing_house using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn add_margin() -> Weight {
		1_000u32.into()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn add_margin() -> Weight {
		1_000u32.into()
	}
}
