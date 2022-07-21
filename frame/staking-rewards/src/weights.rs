use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn create_reward_pool(r: u32) -> Weight;
	fn stake(r: u32) -> Weight;
	fn extend(r: u32) -> Weight;
	fn unstake(r: u32) -> Weight;
	fn split(r: u32) -> Weight;
	fn reward_acumulation_hook_reward_update_calculation() -> Weight;
}

impl WeightInfo for () {
	fn create_reward_pool(_r: u32) -> Weight {
		10_000
	}

	fn stake(_r: u32) -> Weight {
		10_000
	}

	fn extend(_r: u32) -> Weight {
		10_000
	}

	fn unstake(_r: u32) -> Weight {
		10_000
	}

	fn split(_r: u32) -> Weight {
		10_000
	}

	fn reward_acumulation_hook_reward_update_calculation() -> Weight {
		10_000
	}
}
