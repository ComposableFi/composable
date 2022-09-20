use crate::staking::lock::Lock;
use codec::{Decode, Encode};
use core::fmt::Debug;
use frame_support::{
	traits::Get, BoundedBTreeMap, CloneNoBound, DebugNoBound, EqNoBound, PartialEqNoBound,
};
use scale_info::TypeInfo;

/// Staking typed fNFT, usually can be mapped to raw fNFT storage type. A position identifier
/// should exist for each position when stored in the runtime storage.
/// TODO refer to the relevant section in the design doc.
#[derive(DebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(MaxReductions))]
pub struct Stake<
	AssetId: Debug + PartialEq + Eq + Clone,
	// REVIEW(benluelo): Remove this type parameter and use AssetId instead?
	RewardPoolId: Debug + PartialEq + Eq + Clone,
	Balance: Debug + PartialEq + Eq + Clone,
	MaxReductions: Get<u32>,
> {
	/// Reward Pool ID from which pool to allocate rewards for this
	pub staked_asset_id: RewardPoolId,

	/// The original stake this position was created for or updated position with any extended
	/// stake amount.
	pub amount: Balance,

	/// Pool share received for this position
	pub share: Balance,

	/// Reduced rewards by asset for the position (d_n)
	// REVIEW(benluelo): Consider moving the reductions out of the Stake struct.
	//
	// Options:
	//
	// - Separate storage item just for reductions
	//   - Advantages:
	//     - No longer need RateBasedReward.total_dilution_adjustment, as that field is the same as
	//       the sum of all of the reductions of all of the stakes in the pool. Having to keep both
	//       of those in sync feels like unnecessary complexity. This may also apply to other
	//       fields as well.
	//   - Disadvantages:
	//     - In order to get the reductions for a stake, another storage read is required.
	//
	// - Move reductions into the Reward itself
	//   - Advantages:
	//     - Same as above.
	//   - Disadvantages:
	//     - In order to get the reductions for a stake, the RewardPool associated with it must be
	//       read. This isn't too much of an issue currently as we almost always read the pool
	//       whenever we read the stake.
	//     - The size of the pool struct would increase as the amount of stakers increases, causing
	//       storage reads to become more expensive as more stakers enter the pool.
	pub reductions: BoundedBTreeMap<AssetId, Balance, MaxReductions>,

	/// The lock period for the stake.
	pub lock: Lock,
}
