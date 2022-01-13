use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn add_liquidation_strategy() -> Weight {
		10000
	}
}

impl WeightInfo for () {}
