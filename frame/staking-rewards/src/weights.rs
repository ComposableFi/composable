use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn create_reward_pool() -> Weight;
}

impl WeightInfo for () {
	fn create_reward_pool() -> Weight {
		10_000
	}
}
