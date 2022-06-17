use crate::time::{DurationSeconds, Timestamp};
use codec::{Decode, Encode};
use composable_support::collections::vec::bounded::BiBoundedVec;

use core::fmt::Debug;
use frame_support::{storage::bounded_btree_map::BoundedBTreeMap, RuntimeDebug};
use scale_info::TypeInfo;
use sp_runtime::Perbill;

pub type DurationMultiplierRewardsConfig<Limit> = BoundedBTreeMap<DurationSeconds, Perbill, Limit>;

#[derive(RuntimeDebug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct RewardConfig<RewardsRate> {
	/// if not update, stops add rewards to pool
	pub end_block: Timestamp,
	/// for example, asset id, amount, and release frequency   
	pub reward_rates: BiBoundedVec<RewardsRate, 1, 16>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub struct Rewards<RewardsUpdates> {
	/// List of reward asset/pending rewards.
	pub rewards: RewardsUpdates,
	/// The reward multiplier. Captured from configuration on creation.
	pub reward_multiplier: Perbill,
}
