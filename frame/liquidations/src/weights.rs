use crate::Config;
use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn liquidate() -> Weight {
		10_000
	}

	fn add_liquidation_strategy() -> Weight {
		10_000
	}

	fn sell<T: Config>(confs: &[T::LiquidationStrategyId]) -> Weight {
		use frame_support::traits::Get as _;
		let len = if confs.is_empty() {
			1 // because if configurations is empty we add default Configuration
		} else {
			confs.len() as Weight
		};
		let reads = T::DbWeight::get().reads(5);
		let writes = T::DbWeight::get().writes(3);
		len.saturating_mul(reads.saturating_add(writes)).saturating_add(10_000)
	}
}

impl WeightInfo for () {}
