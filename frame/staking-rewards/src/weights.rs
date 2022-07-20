use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn create_reward_pool() -> Weight;
	fn stake() -> Weight;
	fn extend() -> Weight;
	fn split() -> Weight;
}

impl WeightInfo for () {
	fn create_reward_pool() -> Weight {
		10_000
	}

	fn stake() -> Weight {
		10_000
	}

	fn extend() -> Weight {
		10_000
	}

	fn split() -> Weight {
		10_000
	}
}
