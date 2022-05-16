use crate::{
	financial_nft::{NftClass, NftVersion},
	time::{DurationSeconds, Timestamp},
};
use codec::{Decode, Encode};
use composable_support::math::safe::SafeSub;
use core::fmt::Debug;
use frame_support::{dispatch::DispatchResult, traits::Get};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Zero},
	DispatchError, Perbill, SaturatedConversion,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub enum PositionState {
	/// The position is not being rewarded yet and waiting for the next epoch.
	Pending,
	/// The position is currently locked and being rewarded.
	LockedRewarding,
	/// The position expired but still being rewarded.
	Expired,
}

/// The outcome of a penalty applied/notapplied to an amount.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub enum PenaltyOutcome<AccountId, Balance> {
	/// The penalty has been actually applied.
	Applied {
		/// The amount remaining after having subtracted the penalty.
		amount_remaining: Balance,
		/// The penalty amount, a fraction of the amount we penalized (i.e. amount_remaining +
		/// amount_penalty = amount_to_penalize).
		amount_penalty: Balance,
		/// The beneficiary of the applied penalty.
		penalty_beneficiary: AccountId,
	},
	/// The penalty has not beend applied, i.e. identity => f x = x.
	NotApplied { amount: Balance },
}

impl<AccountId, Balance: Zero + Copy> PenaltyOutcome<AccountId, Balance> {
	pub fn penalty_amount(&self) -> Option<Balance> {
		match self {
			PenaltyOutcome::Applied { amount_penalty, .. } => Some(*amount_penalty),
			PenaltyOutcome::NotApplied { .. } => None,
		}
	}

	// NOTE(hussein-aitlahcen): sadly, Zero is asking for Add<Output = Self> for no particular
	// reason?
	pub fn is_zero(&self) -> bool {
		match self {
			PenaltyOutcome::Applied { amount_remaining, amount_penalty, .. } =>
				amount_remaining.is_zero() && amount_penalty.is_zero(),
			PenaltyOutcome::NotApplied { amount } => amount.is_zero(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub struct Penalty<AccountId> {
	/// The penalty.
	pub value: Perbill,
	/// The beneficiary of the penalty.
	pub beneficiary: AccountId,
}

impl<AccountId: Clone> Penalty<AccountId> {
	pub fn penalize<Balance>(
		&self,
		amount: Balance,
	) -> Result<PenaltyOutcome<AccountId, Balance>, DispatchError>
	where
		Balance: AtLeast32BitUnsigned + Copy,
	{
		if self.value.is_zero() {
			Ok(PenaltyOutcome::NotApplied { amount })
		} else {
			let amount_penalty = self.value.mul_floor(amount);
			let amount_remaining = amount.safe_sub(&amount_penalty)?;
			Ok(PenaltyOutcome::Applied {
				amount_penalty,
				amount_remaining,
				penalty_beneficiary: self.beneficiary.clone(),
			})
		}
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub struct StakingConfig<AccountId, DurationPresets, RewardAssets> {
	/// The possible locking duration.
	pub duration_presets: DurationPresets,
	/// The assets we can reward stakers with.
	pub reward_assets: RewardAssets,
	/// The penalty applied if a staker unstake before the end date.
	pub early_unstake_penalty: Penalty<AccountId>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub struct StakingNFT<AccountId, AssetId, Balance, Epoch, Rewards> {
	/// The staked asset.
	pub asset: AssetId,
	/// The stake this NFT was minted for.
	pub stake: Balance,
	/// The reward epoch at which this NFT will start yielding rewards.
	pub reward_epoch_start: Epoch,
	/// List of reward asset/pending rewards.
	pub pending_rewards: Rewards,
	/// The date at which this NFT was minted.
	pub lock_date: Timestamp,
	/// The duration for which this NFT stake was locked.
	pub lock_duration: DurationSeconds,
	/// The penalty applied if a staker unstake before the end date.
	pub early_unstake_penalty: Penalty<AccountId>,
	/// The reward multiplier.
	pub reward_multiplier: Perbill,
}

impl<AccountId, AssetId, Balance: AtLeast32BitUnsigned + Copy, Epoch: Ord, Rewards>
	StakingNFT<AccountId, AssetId, Balance, Epoch, Rewards>
{
	pub fn shares(&self) -> u128 {
		self.reward_multiplier.mul_floor(self.stake.saturated_into::<u128>())
	}

	pub fn state(&self, epoch: &Epoch, epoch_start: Timestamp) -> PositionState {
		if self.lock_date.saturating_add(self.lock_duration) < epoch_start {
			PositionState::Expired
		} else if self.reward_epoch_start > *epoch {
			PositionState::Pending
		} else {
			PositionState::LockedRewarding
		}
	}
}

impl<AccountId, AssetId, Balance, Epoch, Rewards> Get<NftClass>
	for StakingNFT<AccountId, AssetId, Balance, Epoch, Rewards>
{
	fn get() -> NftClass {
		NftClass::STAKING
	}
}

impl<AccountId, AssetId, Balance, Epoch, Rewards> Get<NftVersion>
	for StakingNFT<AccountId, AssetId, Balance, Epoch, Rewards>
{
	fn get() -> NftVersion {
		NftVersion::VERSION_1
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
	fn claim(instance_id: &Self::InstanceId, to: &Self::AccountId) -> DispatchResult;
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
