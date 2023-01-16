use codec::{Decode, Encode};
use composable_support::validation::{validators::GeOne, Validated};
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, BoundedBTreeMap};
use scale_info::TypeInfo;
use sp_arithmetic::fixed_point::FixedU64;
use sp_runtime::Perbill;

use core::fmt::Debug;

use crate::time::{DurationSeconds, Timestamp};

/// defines staking duration, rewards and early unstake penalty
#[derive(
	DebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, MaxEncodedLen, Encode, Decode, TypeInfo,
)]
#[scale_info(skip_type_params(MaxDurationPresets))]
pub struct LockConfig<MaxDurationPresets: Get<u32>> {
	/// The possible locking durations.
	pub duration_multipliers: DurationMultipliers<MaxDurationPresets>,
	/// The penalty applied when unstaking before the lock period is completed. The amount recieved
	/// is `1 - unlock_penalty`.
	pub unlock_penalty: Perbill,
}

/// Represents both valid staking lock durations and ther associated share multiplier bonus.
#[derive(
	DebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, MaxEncodedLen, Encode, Decode, TypeInfo,
)]
#[scale_info(skip_type_params(MaxDurationPresets))]
pub enum DurationMultipliers<MaxDurationPresets: Get<u32>> {
	/// Fixed duration multipliers mapped to their respective multipliers.
	// TODO(benluelo): Wrap this in `Validated` to ensure that at least one preset is provided?
	Presets(BoundedBTreeMap<DurationSeconds, Validated<FixedU64, GeOne>, MaxDurationPresets>),
	// eventually add linear, curve, etc
}

impl<MaxDurationPresets: Get<u32>>
	From<BoundedBTreeMap<DurationSeconds, Validated<FixedU64, GeOne>, MaxDurationPresets>>
	for DurationMultipliers<MaxDurationPresets>
{
	fn from(
		presets: BoundedBTreeMap<DurationSeconds, Validated<FixedU64, GeOne>, MaxDurationPresets>,
	) -> Self {
		Self::Presets(presets)
	}
}

impl<MaxDurationPresets: Get<u32>> DurationMultipliers<MaxDurationPresets> {
	/// Get the multiplier for the given duration, if it's valid for this type of
	/// [`DurationMultiplier`].
	pub fn multiplier(&self, duration: DurationSeconds) -> Option<&Validated<FixedU64, GeOne>> {
		match self {
			DurationMultipliers::Presets(presets) => presets.get(&duration),
		}
	}

	/// Checks that there is at least one valid lock duration for this [`DurationMultiplier`].
	pub fn has_at_least_one_valid_duration(&self) -> bool {
		match self {
			DurationMultipliers::Presets(presets) => presets.len() > 0,
		}
	}
}

/// staking typed fNFT, usually can be mapped to raw fNFT storage type
#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub struct Lock {
	/// The date at which this NFT was minted or to which lock was extended too.
	pub started_at: Timestamp,
	/// The duration for which this NFT stake was locked.
	pub duration: DurationSeconds,

	pub unlock_penalty: Perbill,
}

pub trait Locking {
	type AccountId;
	type InstanceId;
	fn extend_duration(
		who: &Self::AccountId,
		instance_id: &Self::InstanceId,
		duration: Option<DurationSeconds>,
	) -> DispatchResult;
}
