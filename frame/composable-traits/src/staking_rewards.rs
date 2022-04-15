use crate::{
	financial_nft::{NFTClass, NFTVersion},
	math::SafeSub,
	time::{DurationSeconds, Timestamp},
};
use codec::{Decode, Encode};
use core::fmt::Debug;
use frame_support::{dispatch::DispatchResult, traits::Get};
use scale_info::TypeInfo;
use sp_runtime::{traits::AtLeast32BitUnsigned, DispatchError, Perbill, SaturatedConversion};

pub enum PositionState {
	/// The position is not being rewarded yet and waiting for the next epoch.
	Pending,
	/// The position is currently locked and being rewarded.
	LockedRewarding,
	/// The position expired but still being rewarded.
	Expired,
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub enum ClaimStrategy {
	/// Basic, canonical claiming.
	Canonical,
	/// Force claiming the reward by restaking if the position expired.
	RestakeOnExpiry,
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct StakingConfig<AccountId, DurationPresets, Rewards> {
	/// The possible locking duration.
	pub duration_presets: DurationPresets,
	/// The assets we can reward stakers with.
	pub rewards: Rewards,
	/// The penalty applied if a staker unstake before the end date.
	pub early_unstake_penalty: Perbill,
	/// The beneficiary of the penalty.
	pub penalty_beneficiary: AccountId,
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct StakingTag<AccountId, CollectedRewards> {
	/// The account that actually tagged the nft.
	pub tagger: AccountId,
	/// The beneficiary of the rewards distributed after tagging the position.
	pub beneficiary: AccountId,
	/// The rewards collected so far, rewards collected after this point will be distributed to the
	/// `beneficiary`.
	pub collected_rewards: CollectedRewards,
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct StakingNFT<AccountId, AssetId, Balance, CollectedRewards> {
	/// The staked asset.
	pub asset: AssetId,
	/// The stake this NFT was minted for.
	pub stake: Balance,
	/// The date at which this NFT was minted.
	pub lock_date: Timestamp,
	/// The duration for which this NFT stake was locked.
	pub lock_duration: DurationSeconds,
	/// The collected rewards counters at which this NFT was minted, used to compute the rewards.
	pub collected_rewards: CollectedRewards,
	/// The reward multiplier.
	pub reward_multiplier: Perbill,
	/// A liquidator tagged the staking position.
	pub tag: Option<StakingTag<AccountId, CollectedRewards>>,
}

impl<AccountId, AssetId, Balance: AtLeast32BitUnsigned + Copy, CollectedRewards>
	StakingNFT<AccountId, AssetId, Balance, CollectedRewards>
{
	pub fn penalize(&self, penalty: Perbill) -> Result<(Balance, Balance), DispatchError> {
		let penalty_amount = penalty.mul_floor(self.stake);
		let penalized_amount = self.stake.safe_sub(&penalty_amount)?;
		Ok((penalized_amount, penalty_amount))
	}

	pub fn shares(&self) -> u128 {
		self.reward_multiplier.mul_floor(self.stake.saturated_into::<u128>())
	}

	pub fn state(&self, now: Timestamp, epoch_start: Timestamp) -> PositionState {
		if self.lock_date.saturating_add(self.lock_duration) < now {
			PositionState::Expired
		} else if self.lock_date < epoch_start {
			PositionState::LockedRewarding
		} else {
			PositionState::Pending
		}
	}
}

impl<AccountId, AssetId, Balance, CollectedRewards> Get<NFTClass>
	for StakingNFT<AccountId, AssetId, Balance, CollectedRewards>
{
	fn get() -> NFTClass {
		NFTClass::STAKING
	}
}

impl<AccountId, AssetId, Balance, CollectedRewards> Get<NFTVersion>
	for StakingNFT<AccountId, AssetId, Balance, CollectedRewards>
{
	fn get() -> NFTVersion {
		NFTVersion::VERSION_1
	}
}

/// Interface for protocol staking.
pub trait Staking {
	type AccountId;
	type AssetId;
	type Balance;
	type InstanceId;

	/// Stake an amount of protocol asset. A new NFT representing the user position will be
	/// minted.
	///
	/// Arguments
	///
	/// * `asset` the protocol asset to stake.
	/// * `from` the account to transfer the stake from.
	/// * `amount` the amount to stake.
	/// * `duration` the staking duration (must be one of the predefined presets). Unstaking before
	///   the end trigger the unstake penalty.
	/// * `keep_alive` whether to keep the `from` account alive or not while transferring the stake.
	fn stake(
		asset: &Self::AssetId,
		from: &Self::AccountId,
		amount: Self::Balance,
		duration: DurationSeconds,
		keep_alive: bool,
	) -> Result<Self::InstanceId, DispatchError>;

	/// Unstake an actual staked position, represented by a NFT.
	///
	/// Arguments
	///
	/// * `instance_id` the ID uniquely identifiying the NFT from which we will compute the
	///   available rewards.
	/// * `to` the account to transfer the final claimed rewards to.
	fn unstake(instance_id: &Self::InstanceId, to: &Self::AccountId) -> DispatchResult;

	/// Claim the current rewards.
	///
	/// Arguments
	///
	/// * `who` the actual account triggering this claim.
	/// * `instance_id` the ID uniquely identifiying the NFT from which we will compute the
	///   available rewards.
	/// * `to` the account to transfer the rewards to.
	/// * `strategy` the strategy used to claim the rewards.
	fn claim(
		instance_id: &Self::InstanceId,
		to: &Self::AccountId,
		strategy: ClaimStrategy,
	) -> DispatchResult;
}

pub trait StakingReward {
	type AccountId;
	type AssetId;
	type Balance;

	/// Transfer a reward to the Chaos protocol.
	///
	/// Arguments
	///
	/// * `asset` the protocol asset to reward.
	/// * `reward_asset` the reward asset to transfer.
	/// * `from` the account to transfer the reward from.
	/// * `amount` the amount of reward to transfer.
	/// * `keep_alive` whether to keep alive or not the `from` account while transferring the
	///   reward.
	fn transfer_reward(
		asset: &Self::AssetId,
		reward_asset: &Self::AssetId,
		from: &Self::AccountId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> DispatchResult;
}
