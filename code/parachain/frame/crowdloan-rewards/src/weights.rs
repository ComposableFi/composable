#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(trivial_numeric_casts)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn populate(x: u32) -> Weight;
	fn initialize(x: u32) -> Weight;
	fn associate(x: u32) -> Weight;
	fn claim(x: u32) -> Weight;
	fn unlock_rewards_for(x: u32) -> Weight;
}

impl WeightInfo for () {
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:0)
	// Storage: CrowdloanRewards Rewards (r:1001 w:1000)
	// Storage: CrowdloanRewards TotalContributors (r:0 w:1)
	// Storage: CrowdloanRewards TotalRewards (r:0 w:1)
	fn populate(x: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 109_000
			.saturating_add(Weight::from_ref_time(6_792_000 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(x as u64)))
	}
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:1)
	fn initialize(x: u32) -> Weight {
		Weight::from_ref_time(33_355_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:0)
	// Storage: CrowdloanRewards Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CrowdloanRewards ClaimedRewards (r:1 w:1)
	// Storage: CrowdloanRewards Associations (r:0 w:1)
	fn associate(x: u32) -> Weight {
		Weight::from_ref_time(169_323_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(8_000 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: CrowdloanRewards Associations (r:1 w:0)
	// Storage: CrowdloanRewards VestingBlockStart (r:1 w:0)
	// Storage: CrowdloanRewards Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CrowdloanRewards ClaimedRewards (r:1 w:1)
	fn claim(x: u32) -> Weight {
		Weight::from_ref_time(94_034_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(31_000 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}

	fn unlock_rewards_for(x: u32) -> Weight {
		x as _
	}
}
