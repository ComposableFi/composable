use crate::{
	time::{DurationSeconds},
};
use codec::{Decode, Encode};

use core::fmt::Debug;
use frame_support::{
	storage::bounded_btree_map::BoundedBTreeMap, 
};
use scale_info::TypeInfo;
use sp_runtime::{
	Perbill,
};


pub type DurationMultiplierRewardsConfig<Limit> = BoundedBTreeMap<DurationSeconds, Perbill, Limit>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub struct Rewards<RewardsUpdates> {
	/// List of reward asset/pending rewards.
	pub rewards: RewardsUpdates,
	/// The reward multiplier. Captured from configuration  on creation.
	pub reward_multiplier: Perbill,
}
