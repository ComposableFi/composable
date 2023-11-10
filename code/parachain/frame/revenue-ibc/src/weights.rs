#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

// The weight info trait for `pallet_oracle`.
pub trait WeightInfo {
	fn on_initialize(c: usize) -> Weight;
	fn set_period() -> Weight;
	fn set_memo() -> Weight;
	fn trigger_transfer() -> Weight;
	fn set_allowed() -> Weight;
	fn add_allowed() -> Weight;
	fn remove_allowed() -> Weight;
	fn set_disallowed() -> Weight;
	fn add_disallowed() -> Weight;
	fn remove_disallowed() -> Weight;
	fn set_channel() -> Weight;
	fn set_address() -> Weight;
    fn set_cvm_osmo_address() -> Weight;
    fn set_cvm_centauri_address() -> Weight;
}

/// Weights for pallet_oracle using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn on_initialize(c: usize) -> Weight {
		Weight::from_parts(100_000, 0).saturating_mul(c as u64)
	}
	fn set_period() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_memo() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn trigger_transfer() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_allowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn add_allowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn remove_allowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_disallowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn add_disallowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn remove_disallowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_channel() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_address() -> Weight {
		Weight::from_parts(100_000, 0)
	}
    fn set_cvm_osmo_address() -> Weight {
		Weight::from_parts(100_000, 0)
	}
    fn set_cvm_centauri_address() -> Weight {
		Weight::from_parts(100_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize(c: usize) -> Weight {
		Weight::from_parts(100_000, 0).saturating_mul(c as u64)
	}
	fn set_period() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_memo() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn trigger_transfer() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_allowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn add_allowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn remove_allowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_disallowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn add_disallowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn remove_disallowed() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_channel() -> Weight {
		Weight::from_parts(100_000, 0)
	}
    fn set_address() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	fn set_cvm_osmo_address() -> Weight {
		Weight::from_parts(100_000, 0)
	}
    fn set_cvm_centauri_address() -> Weight {
		Weight::from_parts(100_000, 0)
	}
}
