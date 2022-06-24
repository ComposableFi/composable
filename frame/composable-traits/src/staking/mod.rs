use crate::staking::lock::{Lock, LockConfig};
use codec::{Decode, Encode};

use crate::time::DurationSeconds;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, BoundedBTreeMap};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Zero;
use sp_runtime::{DispatchError, Perbill, Permill};

pub mod lock;
pub mod math;
pub mod nft;

pub type StakingDurationToRewardsMultiplierConfig<Limit> =
	BoundedBTreeMap<DurationSeconds, Perbill, Limit>;

/// Defines staking duration, rewards and early unstake penalty for a given asset type.
/// TODO refer to the relevant section in the design doc.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct Reward<AssetId, Balance> {
	/// asset id of the reward
	pub asset_id: AssetId,

	/// Total rewards including inflation for adjusting for new stakers joining the pool. All
	/// stakers in a pool are eligible to receive a part of this value based on their share of the
	/// pool.
	pub total_rewards: Balance,

	/// A book keeping field to track the actual total reward without the
	/// reward dilution adjustment caused by new stakers joining the pool.
	pub total_dilution_adjustment: Balance,

	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: Perbill,
}

impl<AssetId, Balance: Zero> Reward<AssetId, Balance> {
	pub fn from(reward_config: RewardConfig<AssetId, Balance>) -> Reward<AssetId, Balance> {
		Reward {
			asset_id: reward_config.asset_id,
			total_rewards: Zero::zero(),
			total_dilution_adjustment: Zero::zero(),
			max_rewards: reward_config.max_rewards,
			reward_rate: reward_config.reward_rate,
		}
	}
}

/// A reward pool is a collection of rewards that are allocated to stakers to incentivize a
/// particular purpose. Eg: a pool of rewards for incentivizing adding liquidity to a pablo swap
/// pool. TODO refer to the relevant section in the design doc.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct RewardPool<
	AccountId,
	AssetId,
	Balance,
	BlockNumber,
	DurationPresets,
	RewardsLength: Get<u32>,
> {
	pub owner: AccountId,

	/// The staked asset id of the reward pool.
	pub asset_id: AssetId,

	/// rewards accumulated
	pub rewards: BoundedBTreeMap<AssetId, Reward<AssetId, Balance>, RewardsLength>,

	/// Total shares distributed among stakers
	pub total_shares: Balance,

	/// Pool would stop adding rewards to pool at this block number.
	pub end_block: BlockNumber,

	// possible lock config for this pool
	pub lock: LockConfig<DurationPresets>,
}

/// Reward configurations for a given asset type.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RewardConfig<AssetId, Balance> {
	/// asset id of the reward
	pub asset_id: AssetId,

	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward` (and `actual_total_reward`)
	/// at a given time.
	pub reward_rate: Perbill,
}

/// Categorize the reward pool by it's incentive characteristics and expose
/// initial configuration parameters.
/// TODO refer to the relevant section in the design doc.
#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
pub enum RewardPoolConfiguration<AccountId, AssetId, Balance, BlockNumber, DurationPresets> {
	/// A pool with an adjustable reward rate to be used as incentive.
	RewardRateBasedIncentive {
		/// Protocol or the user account that owns this pool
		owner: AccountId,
		/// The staked asset id of the reward pool.
		asset_id: AssetId,
		/// Pool would stop adding rewards to pool at this block number.
		end_block: BlockNumber,
		/// initial reward configuration for this pool
		reward_config: RewardConfig<AssetId, Balance>,
		// possible lock config for this reward
		lock: LockConfig<DurationPresets>,
	},
}

/// Staking typed fNFT, usually can be mapped to raw fNFT storage type. A position identifier
/// should exist for each position when stored in the runtime storage.
/// TODO refer to the relevant section in the design doc.
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct Stake<RewardPoolId, AssetId, Balance, RewardsLength: Get<u32>> {
	/// Reward Pool ID from which pool to allocate rewards for this
	pub reward_pool_id: RewardPoolId,

	/// The original stake this NFT was minted for or updated NFT with increased stake amount.
	pub stake: Balance,

	/// Pool share received for this position
	pub share: Balance,

	/// Reduced rewards by asset for the position (d_n)
	reductions: BoundedBTreeMap<AssetId, Balance, RewardsLength>,

	/// The lock period for the stake.
	pub lock: Lock,
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
	type RewardPoolId;

	/// Adds reward to common pool share.
	/// Does not actually transfers real assets.
	fn accumulate_reward(
		pool: &Self::RewardPoolId,
		reward_currency: Self::AssetId,
		reward_increment: Self::Balance,
	) -> DispatchResult;

	/// Transfers rewards `from` to pool.
	/// If may be bigger than total shares.
	fn transfer_reward(
		from: &Self::AccountId,
		pool: &Self::RewardPoolId,
		reward_currency: Self::AssetId,
		reward_increment: Self::Balance,
	) -> DispatchResult;
}

/// Interface for protocol staking.
pub trait Staking {
	type AccountId;
	type RewardPoolId;
	type Balance;
	type PositionId;

	/// Stake an amount of protocol asset.
	///
	/// Arguments
	///
	/// * `who` the account to transfer the stake from.
	/// * `amount` the amount to stake. the end trigger the unstake penalty.
	/// * `keep_alive` whether to keep the `from` account alive or not while transferring the stake.
	fn stake(
		who: &Self::AccountId,
		pool_id: &Self::RewardPoolId,
		amount: Self::Balance,
		duration_preset: DurationSeconds,
		keep_alive: bool,
	) -> Result<Self::PositionId, DispatchError>;

	/// Extend the stake of an existing position.
	fn extend(
		who: &Self::AccountId,
		position: Self::PositionId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::PositionId, DispatchError>;

	/// Unstake an actual staked position, represented by a NFT.
	///
	/// Arguments
	///
	/// * `instance_id` the ID uniquely identifying the NFT from which we will compute the available
	///   rewards.
	/// * `to` the account to transfer the final claimed rewards to.
	fn unstake(
		who: &Self::AccountId,
		position: &Self::PositionId,
		remove_amount: Self::Balance,
	) -> DispatchResult;

	/// `ratio` - how much of share to retain in the original position.
	fn split(
		who: &Self::AccountId,
		position: &Self::PositionId,
		ratio: Permill,
	) -> Result<[Self::PositionId; 2], DispatchError>;
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
