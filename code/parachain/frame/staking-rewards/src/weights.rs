use frame_support::dispatch::Weight;

pub trait WeightInfo {
	fn create_reward_pool(r: u32) -> Weight;
	fn stake(r: u32) -> Weight;
	fn extend(r: u32) -> Weight;
	fn unstake(r: u32) -> Weight;
	fn split(r: u32) -> Weight;
	fn reward_accumulation_hook_reward_update_calculation() -> Weight;
	fn unix_time_now() -> Weight;
	fn update_rewards_pool(r: u32) -> Weight;
	fn claim(r: u32) -> Weight;
	fn add_to_rewards_pot() -> Weight;
}

impl WeightInfo for () {
	fn create_reward_pool(_r: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn stake(_r: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn extend(_r: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn unstake(_r: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn split(_r: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn reward_accumulation_hook_reward_update_calculation() -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn unix_time_now() -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn update_rewards_pool(_r: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn claim(_r: u32) -> Weight {
		Weight::from_ref_time(10_000)
	}

	fn add_to_rewards_pot() -> Weight {
		Weight::from_ref_time(10_000)
	}
}
