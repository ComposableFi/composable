use core::ops::Mul;

use crate::staking::lock::{Lock, LockConfig};
use codec::{Decode, Encode};
use composable_support::validation::{Validate, Validated};

use crate::time::DurationSeconds;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, BoundedBTreeMap};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Zero;
use sp_runtime::{DispatchError, Perbill, Permill};

pub mod lock;
pub mod math;
pub mod nft;

/// Abstraction over the map of possible lock durations and corresponding reward multipliers.
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

	/// Already claimed rewards by stakers by unstaking.
	pub claimed_rewards: Balance,

	/// A book keeping field to track the actual total reward without the
	/// reward dilution adjustment caused by new stakers joining the pool.
	pub total_dilution_adjustment: Balance,

	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,

	/// The last time the reward was updated, in seconds.
	pub last_updated_timestamp: u64,
}

/// Abstraction over the asset to reduction map stored for staking.
pub type Reductions<AssetId, Balance, Limit> = BoundedBTreeMap<AssetId, Balance, Limit>;

/// Abstraction over the asset to rewards map stored for staking.
pub type Rewards<AssetId, Balance, Limit> =
	BoundedBTreeMap<AssetId, Reward<AssetId, Balance>, Limit>;

impl<AssetId, Balance: Zero> Reward<AssetId, Balance> {
	pub fn from(reward_config: RewardConfig<AssetId, Balance>) -> Reward<AssetId, Balance> {
		Reward {
			asset_id: reward_config.asset_id,
			total_rewards: Zero::zero(),
			claimed_rewards: Zero::zero(),
			total_dilution_adjustment: Zero::zero(),
			max_rewards: reward_config.max_rewards,
			reward_rate: reward_config.reward_rate,
			last_updated_timestamp: 0,
		}
	}
}

/// A reward pool is a collection of rewards that are allocated to stakers to incentivize a
/// particular purpose. Eg: a pool of rewards for incentivizing adding liquidity to a pablo swap
/// pool. TODO refer to the relevant section in the design doc.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct RewardPool<AccountId, AssetId, Balance, BlockNumber, DurationPresets, Rewards> {
	pub owner: AccountId,

	/// The staked asset id of the reward pool.
	pub asset_id: AssetId,

	/// rewards accumulated
	pub rewards: Rewards,

	/// Total shares distributed among stakers
	pub total_shares: Balance,

	/// Already claimed shares by stakers by unstaking
	pub claimed_shares: Balance,

	/// Pool would stop adding rewards to pool at this block number.
	pub end_block: BlockNumber,

	// possible lock config for this pool
	pub lock: LockConfig<DurationPresets>,
}

/// Default transfer limit on new asset added as rewards.
pub const DEFAULT_MAX_REWARDS: u128 = 1_000_000_000_000_000_000_u128;

/// Reward configurations for a given asset type.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RewardConfig<AssetId, Balance> {
	/// asset id of the reward
	pub asset_id: AssetId,

	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RewardRate<Balance> {
	/// The period that the rewards are handed out in, in seconds.
	// REVIEW(benluelo): NonZeroU64?
	pub period: u64,
	/// The amount that is rewarded each period.
	pub amount: Balance,
}

pub struct BoundedU64<const MIN: u64 = 0, const MAX: u64 = { u64::MAX }> {
	// _marker: PhantomData<(MIN, MAX)>,
}

// TODO: TESTS!!!
impl<const MIN: u64, const MAX: u64> Validate<u64, BoundedU64<MIN, MAX>> for BoundedU64<MIN, MAX> {
	fn validate(input: u64) -> Result<u64, &'static str> {
		if input > MAX {
			Err("value too big")
		} else if input < MIN {
			Err("value too small")
		} else {
			Ok(input)
		}
	}
}

// TODO: TESTS!!!
// TODO(benluelo): uom?
impl<Balance> RewardRate<Balance> {
	pub fn per_second(period: u64, amount: Balance) -> Self {
		Self { period, amount }
	}

	pub fn per_minute(
		amount: Balance,
		minutes: Validated<u64, BoundedU64<0, { u64::MAX / 60 }>>,
	) -> Self {
		RewardRate { period: minutes.mul(60), amount }
	}

	pub fn per_hour(
		amount: Balance,
		hours: Validated<u64, BoundedU64<0, { u64::MAX / (60 * 60) }>>,
	) -> Self {
		RewardRate { period: hours.mul(60).mul(60), amount }
	}
}

pub type RewardConfigs<AssetId, Balance, Limit> =
	BoundedBTreeMap<AssetId, RewardConfig<AssetId, Balance>, Limit>;

/// Categorize the reward pool by it's incentive characteristics and expose
/// initial configuration parameters.
/// TODO refer to the relevant section in the design doc.
#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
#[non_exhaustive]
pub enum RewardPoolConfiguration<AccountId, AssetId, BlockNumber, RewardConfigs, DurationPresets> {
	/// A pool with an adjustable reward rate to be used as incentive.
	RewardRateBasedIncentive {
		/// Protocol or the user account that owns this pool
		owner: AccountId,
		/// The staked asset id of the reward pool.
		asset_id: AssetId,
		/// Pool would stop adding rewards to pool at this block number.
		end_block: BlockNumber,
		/// initial reward configuration for this pool
		reward_configs: RewardConfigs,
		// possible lock config for this reward
		lock: LockConfig<DurationPresets>,
	},
}

/// Staking typed fNFT, usually can be mapped to raw fNFT storage type. A position identifier
/// should exist for each position when stored in the runtime storage.
/// TODO refer to the relevant section in the design doc.
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct Stake<AccountId, RewardPoolId, Balance, Reductions> {
	/// Protocol or the user account that owns this stake
	pub owner: AccountId,

	/// Reward Pool ID from which pool to allocate rewards for this
	pub reward_pool_id: RewardPoolId,

	/// The original stake this NFT was minted for or updated NFT with increased stake amount.
	pub stake: Balance,

	/// Pool share received for this position
	pub share: Balance,

	/// Reduced rewards by asset for the position (d_n)
	pub reductions: Reductions,

	/// The lock period for the stake.
	pub lock: Lock,
}

/// Trait to provide interface to manage staking reward pool.
pub trait ManageStaking {
	type AccountId;
	type AssetId;
	type BlockNumber;
	type Balance;
	type RewardConfigsLimit;
	type StakingDurationPresetsLimit;
	type RewardPoolId;

	/// Create a staking reward pool from configurations passed as inputs.
	fn create_staking_pool(
		pool_config: RewardPoolConfiguration<
			Self::AccountId,
			Self::AssetId,
			Self::BlockNumber,
			RewardConfigs<Self::AssetId, Self::Balance, Self::RewardConfigsLimit>,
			StakingDurationToRewardsMultiplierConfig<Self::StakingDurationPresetsLimit>,
		>,
	) -> Result<Self::RewardPoolId, DispatchError>;
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
	fn unstake(who: &Self::AccountId, position: &Self::PositionId) -> DispatchResult;

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
