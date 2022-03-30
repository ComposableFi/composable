use crate::{
	financial_nft::{NFTClass, NFTVersion},
	math::{SafeAdd, SafeSub},
	time::{DurationSeconds, Timestamp},
};
use codec::{Decode, Encode};
use core::fmt::Debug;
use frame_support::{dispatch::DispatchResult, traits::Get};
use scale_info::TypeInfo;
use sp_runtime::{traits::AtLeast32BitUnsigned, DispatchError, Perbill};

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct ProtocolStakingConfig<AccountId, DurationPresets, Rewards> {
	pub duration_presets: DurationPresets,
	pub rewards: Rewards,
	pub early_unstake_penalty: Perbill,
	pub penalty_beneficiary: AccountId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct ProtocolStakingNFT<AssetId, Balance, Indexes> {
	/// The staked asset.
	pub asset: AssetId,
	/// The stake this NFT was minted for.
	pub stake: Balance,
	/// The indexes at which this NFT was minted, used to compute the rewards.
	pub reward_indexes: Indexes,
	/// The date at which this NFT was minted.
	pub lock_date: Timestamp,
	/// The duration for which this NFT stake was locked.
	pub lock_duration: DurationSeconds,
}

impl<AssetId, Balance: AtLeast32BitUnsigned + Copy, Indexes>
	ProtocolStakingNFT<AssetId, Balance, Indexes>
{
	pub fn penalize_early_unstake_amount(
		&self,
		now: Timestamp,
		penalty: Perbill,
	) -> Result<(Balance, Balance), DispatchError> {
		if self.lock_date.safe_add(&self.lock_duration)? <= now {
			Ok((self.stake, Balance::zero()))
		} else {
			let penalty_amount = penalty.mul_floor(self.stake);
			let penalized_amount = self.stake.safe_sub(&penalty_amount)?;
			Ok((penalized_amount, penalty_amount))
		}
	}
}

impl<AssetId, Balance, Indexes> Get<NFTClass> for ProtocolStakingNFT<AssetId, Balance, Indexes> {
	fn get() -> NFTClass {
		NFTClass::PROTOCOL_STAKING
	}
}

impl<AssetId, Balance, Indexes> Get<NFTVersion> for ProtocolStakingNFT<AssetId, Balance, Indexes> {
	fn get() -> NFTVersion {
		NFTVersion::VERSION_1
	}
}

/// Interface for protocol staking.
pub trait ProtocolStaking {
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
	/// * `instance_id` the ID uniquely identifiying the NFT from which we will compute the
	///   available rewards.
	/// * `to` the account to transfer the rewards to.
	fn claim(instance_id: &Self::InstanceId, to: &Self::AccountId) -> DispatchResult;
}

pub trait ProtocolReward {
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
