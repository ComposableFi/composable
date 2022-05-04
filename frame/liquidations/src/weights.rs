use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn liquidate() -> Weight {
		10_000
	}

	fn add_liquidation_strategy() -> Weight {
		10_000
	}

	fn sell() -> Weight {
		10_000
	}
}

impl WeightInfo for () {}
