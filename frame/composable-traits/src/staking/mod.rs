use crate::staking::{
	lock::{Lock, LockConfig},
	rewards::RewardConfig,
};
use codec::{Decode, Encode};

use core::fmt::Debug;
use frame_support::dispatch::DispatchResult;
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, Perbill, Permill};

pub mod lock;
pub mod math;
pub mod nft;
pub mod rewards;

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct StakeConfig<DurationPresets, RewardsRate> {
	pub lock: Option<LockConfig<DurationPresets>>,
	pub reward: Option<RewardConfig<RewardsRate>>,
}

/// staking typed fNFT, usually can be mapped to raw fNFT storage type
#[derive(Debug, PartialEq, Eq, Copy, Clone, Encode, Decode, TypeInfo)]
pub struct Stake<Balance, Rewards> {
	/// The original stake this NFT was minted for or updated NFT with increased stake amount.
	pub real_stake: Balance,
	/// List of reward asset/pending rewards.
	pub rewards: Rewards,
	pub lock: Option<Lock>,
	/// The reward multiplier.
	pub reward_multiplier: Perbill,
}

/// implemented by instances which know their share of something bigger
pub trait Shares {
	type Balance;
	fn shares(&self) -> Self::Balance;
}

/// is unaware of concrete positions
pub trait ProtocolStaking {
	type AccountId;
	type AssetId;
	type Balance;
	type PoolId;

	/// Adds reward to common pool share.
	/// Does not actually transfers real assets.
	fn accumulate_reward(
		pool: &Self::PoolId,
		reward_currency: Self::AssetId,
		reward_increment: Self::Balance,
	) -> DispatchResult;

	/// Transfers rewards `from` to pool.
	/// If may be bigger than total shares.
	fn transfer_reward(
		from: &Self::AccountId,
		pool: &Self::PoolId,
		reward_currency: Self::AssetId,
		reward_increment: Self::Balance,
	) -> DispatchResult;
}

/// Interface for protocol staking.
pub trait Staking {
	type AccountId;
	type PoolId;
	type Balance;
	type PositionId;

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
	/// * `config_index` - reference to one of  staking configs for `pool_id`
	fn stake(
		who: &Self::AccountId,
		pool_id: &Self::PoolId,
		config_index: u8,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::PositionId, DispatchError>;

	fn add_share(
		who: &Self::AccountId,
		position: Self::PositionId,
		amount: Self::Balance,
		keep_alive: bool,
	);

	/// Unstake an actual staked position, represented by a NFT.
	///
	/// Arguments
	///
	/// * `instance_id` the ID uniquely identifying the NFT from which we will compute the available
	///   rewards.
	/// * `to` the account to transfer the final claimed rewards to.
	fn unstake(
		who: &Self::AccountId,
		instance_id: &Self::PositionId,
		remove_amount: Self::Balance,
	) -> DispatchResult;

	/// `ratio` - how much of share to retain in original position.
	fn split(
		who: &Self::AccountId,
		instance_id: &Self::PositionId,
		ratio: Permill,
	) -> [Self::PositionId; 2];
}

pub trait StakingReward {
	type AccountId;
	type AssetId;
	type Balance;
	type PositionId;

	/// Claim the current rewards.
	///
	/// Arguments
	///
	/// * `who` the actual account triggering this claim.
	/// * `instance_id` the ID uniquely identifying the NFT from which we will compute the available
	///   rewards.
	/// * `to` the account to transfer the rewards to.
	/// Return amount if reward asset which was staked asset claimed.
	fn claim_rewards(
		who: &Self::AccountId,
		instance_id: &Self::PositionId,
	) -> Result<(Self::AssetId, Self::Balance), DispatchError>;

	/// Transfer a reward to the staking rewards protocol.
	///
	/// Arguments
	///
	/// * `asset` the protocol asset to reward.
	/// * `reward_asset` the reward asset to transfer.
	/// * `from` the account to transfer the reward from.
	/// * `amount` the amount of reward to transfer.
	/// * `keep_alive` whether to keep alive or not the `from` account while transferring the
	///   reward.
	fn claim_reward(
		who: &Self::AccountId,
		instance_id: &Self::PositionId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> DispatchResult;
}
