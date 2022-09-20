use core::fmt::Debug;

use crate::{staking::lock::LockConfig, time::DurationSeconds};

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, BoundedBTreeMap};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, Permill};

use reward::{Reward, RewardConfig};

pub mod lock;
pub mod math;
pub mod reward;
pub mod stake;

/// A reward pool is a collection of rewards that are allocated to stakers to incentivize a
/// particular purpose. Eg: a pool of rewards for incentivizing adding liquidity to a pablo swap
/// pool. TODO refer to the relevant section in the design doc.
#[derive(
	RuntimeDebugNoBound, PartialEqNoBound, EqNoBound, CloneNoBound, Encode, Decode, TypeInfo,
)]
#[scale_info(skip_type_params(MaxDurationPresets, MaxRewards))]
pub struct RewardPool<
	AccountId: Debug + PartialEq + Eq + Clone,
	AssetId: Debug + PartialEq + Eq + Clone,
	Balance: Debug + PartialEq + Eq + Clone,
	BlockNumber: Debug + PartialEq + Eq + Clone,
	MaxDurationPresets: Get<u32>,
	MaxRewards: Get<u32>,
> {
	pub owner: AccountId,

	/// rewards accumulated
	pub rewards: BoundedBTreeMap<AssetId, Reward<Balance>, MaxRewards>,

	/// Total shares distributed among stakers
	pub total_shares: Balance,

	/// Already claimed shares by stakers by unstaking
	pub claimed_shares: Balance,

	/// Pool would stop adding rewards to pool at this block number.
	pub end_block: BlockNumber,

	// possible lock config for this pool
	pub lock: LockConfig<MaxDurationPresets>,

	// Asset ID issued as shares for staking in the pool. Eg: for PBLO -> xPBLO
	pub share_asset_id: AssetId,

	// Asset ID (collection ID) of the financial NFTs issued for staking positions of this pool
	pub fnft_collection_id: AssetId,
}

/// Categorize the reward pool by it's incentive characteristics and expose
/// initial configuration parameters.
/// TODO refer to the relevant section in the design doc.
#[derive(
	RuntimeDebugNoBound,
	Encode,
	Decode,
	MaxEncodedLen,
	CloneNoBound,
	PartialEqNoBound,
	EqNoBound,
	TypeInfo,
)]
#[scale_info(skip_type_params(MaxRewardConfigs, MaxDurationPresets))]
pub struct RewardPoolConfig<
	AccountId: Eq + PartialEq + Clone + Debug,
	AssetId: Eq + PartialEq + Clone + Debug,
	Balance: Eq + PartialEq + Clone + Debug,
	BlockNumber: Eq + PartialEq + Clone + Debug,
	MaxRewardConfigs: Get<u32>,
	MaxDurationPresets: Get<u32>,
> {
	/// Protocol or the user account that owns this pool
	pub owner: AccountId,

	/// The staked asset id of the reward pool.
	pub asset_id: AssetId,

	/// Pool would stop adding rewards to pool at this block number.
	pub end_block: BlockNumber,

	/// initial reward configuration for this pool
	pub reward_configs: BoundedBTreeMap<AssetId, RewardConfig<Balance>, MaxRewardConfigs>,

	/// possible lock config for this reward
	pub lock: LockConfig<MaxDurationPresets>,

	/// Asset ID issued as shares for staking in the pool. Eg: for PBLO -> xPBLO
	pub share_asset_id: AssetId,

	/// Asset ID (collection ID) of the financial NFTs issued for staking positions of this pool
	pub financial_nft_asset_id: AssetId,
}

/// Trait to provide interface to manage staking reward pool.
pub trait ManageStaking {
	type AccountId: Eq + Clone + PartialEq + Debug;
	type AssetId: Eq + Clone + PartialEq + Debug;
	type BlockNumber: Eq + Clone + PartialEq + Debug;
	type Balance: Eq + Clone + PartialEq + Debug;
	type RewardPoolId: Eq + Clone + PartialEq + Debug;

	type RewardConfigsLimit: Get<u32>;
	type StakingDurationPresetsLimit: Get<u32>;

	/// Create a staking reward pool from configurations passed as inputs.
	fn create_staking_pool(
		pool_config: RewardPoolConfig<
			Self::AccountId,
			Self::AssetId,
			Self::Balance,
			Self::BlockNumber,
			Self::RewardConfigsLimit,
			Self::StakingDurationPresetsLimit,
		>,
	) -> Result<Self::RewardPoolId, DispatchError>;
}

/// is unaware of concrete positions
pub trait ProtocolStaking {
	type AccountId;
	type AssetId;
	type Balance;
	type RewardPoolId;

	/// Transfers rewards `from` to pool.
	/// If may be bigger than total shares.
	fn transfer_earnings(
		from: &Self::AccountId,
		pool: &Self::RewardPoolId,
		reward_currency: Self::AssetId,
		reward_increment: Self::Balance,
		keep_alive: bool,
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

	/// Unstake an actual staked position, represented by an NFT.
	///
	/// Arguments
	///
	/// * `who` the account to transfer the final claimed rewards to.
	/// * `position` the ID uniquely identifying the NFT to unstake.
	fn unstake(who: &Self::AccountId, position: &Self::PositionId) -> DispatchResult;

	/// `ratio` - how much of share to retain in the original position.
	fn split(
		who: &Self::AccountId,
		position: &Self::PositionId,
		ratio: Permill,
	) -> Result<Self::PositionId, DispatchError>;

	/// Claim remaining reward earned up to this point in time.
	///
	/// Arguments
	/// * `who` - the account to transfer the final claimed rewards to.
	/// * `position` - The uniquely identifying NFT from which we will compute the rewards.
	fn claim(who: &Self::AccountId, position: &Self::PositionId) -> DispatchResult;
}

/// Interface for managing staking through financial NFTs.
pub trait StakingFinancialNft {
	type AccountId;
	type CollectionId;
	type InstanceId;
	type Balance;

	/// Extend the stake of an existing position represented by a financial NFT.
	fn extend(
		who: &Self::AccountId,
		collection: Self::CollectionId,
		instance: Self::InstanceId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::InstanceId, DispatchError>;

	/// Unstake an actual staked position, represented by a financial NFT.
	fn burn(
		who: &Self::AccountId,
		collection: Self::CollectionId,
		instance: Self::InstanceId,
		remove_amount: Self::Balance,
	) -> DispatchResult;

	/// `ratio` - how much of share to retain in the original position.
	fn split(
		who: &Self::AccountId,
		collection: Self::CollectionId,
		instance: Self::InstanceId,
		ratio: Permill,
	) -> Result<[Self::InstanceId; 2], DispatchError>;
}
