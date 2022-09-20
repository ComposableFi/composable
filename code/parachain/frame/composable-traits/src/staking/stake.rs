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
	pub reductions: BoundedBTreeMap<AssetId, Balance, MaxReductions>,

	/// The lock period for the stake.
	pub lock: Lock,
}
