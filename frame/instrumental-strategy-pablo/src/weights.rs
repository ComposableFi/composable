#![allow(clippy::unnecessary_cast)]

use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn test() -> Weight;
	fn set_pool_id_for_asset() -> Weight;
	fn liquidity_rebalance() -> Weight;
	fn associate_vault() -> Weight;
	fn transferring_funds() -> Weight;
}

/// Weights for instrumental_strategy_pablo using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn test() -> Weight {
		10_000 as Weight
	}

	fn set_pool_id_for_asset() -> Weight {
		10_000 as Weight
	}

	fn liquidity_rebalance() -> Weight {
		10_000 as Weight
	}

	fn associate_vault() -> Weight {
		10_000 as Weight
	}

	fn transferring_funds() -> Weight {
		10_000 as Weight
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn test() -> Weight {
		10_000 as Weight
	}

	fn set_pool_id_for_asset() -> Weight {
		10_000 as Weight
	}

	fn liquidity_rebalance() -> Weight {
		10_000 as Weight
	}

	fn associate_vault() -> Weight {
		10_000 as Weight
	}

	fn transferring_funds() -> Weight {
		10_000 as Weight
	}
}
