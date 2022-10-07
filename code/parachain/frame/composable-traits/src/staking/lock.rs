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
	/// The possible locking duration.
	pub duration_presets:
		BoundedBTreeMap<DurationSeconds, Validated<FixedU64, GeOne>, MaxDurationPresets>,
	/// The penalty applied if a staker unstake before the end date.
	/// In case of zero penalty, you cannot unlock before it duration ends.
	pub unlock_penalty: Perbill,
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
