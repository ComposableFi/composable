//! Implements staking rewards protocol.
#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)]
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
mod prelude;
#[cfg(test)]
mod test;
mod validation;
pub mod weights;

use sp_std::ops::{Add, Div, Mul};

use crate::prelude::*;
use composable_support::math::safe::SafeSub;
use composable_traits::staking::{Reward, RewardUpdate};
use frame_support::{
	traits::{
		fungibles::{Inspect, InspectHold, MutateHold, Transfer},
		UnixTime,
	},
	transactional, BoundedBTreeMap,
};
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use composable_support::{
		abstractions::{
			nonce::Nonce,
			utils::{
				increment::{Increment, SafeIncrement},
				start_at::ZeroInit,
			},
		},
		math::safe::{SafeAdd, SafeArithmetic, SafeDiv, SafeMul, SafeSub},
		validation::Validated,
	};
	use composable_traits::{
		currency::{BalanceLike, CurrencyFactory},
		fnft::{FinancialNft, FinancialNftProtocol},
		staking::{
			RewardPoolConfiguration::RewardRateBasedIncentive, RewardRatePeriod,
			DEFAULT_MAX_REWARDS,
		},
		time::DurationSeconds,
	};
	use frame_support::{
		traits::{
			fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer},
			tokens::{nonfungibles, WithdrawConsequence},
			TryCollect, UnixTime,
		},
		transactional, BoundedBTreeMap, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{traits::One, Permill};
	use sp_runtime::{
		traits::{AccountIdConversion, BlockNumberProvider},
		PerThing, Perbill,
	};
	use sp_std::{cmp::max, fmt::Debug, vec, vec::Vec};

	use crate::{
		prelude::*, reward_accumulation_calculation, update_rewards_pool,
		validation::ValidSplitRatio, RewardAccumulationCalculationError,
	};

	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::RewardPoolId` was created successfully by `T::AccountId`.
		RewardPoolCreated {
			/// Id of newly created pool.
			pool_id: T::RewardPoolId,
			/// Owner of the pool.
			owner: T::AccountId,
			/// End block
			end_block: T::BlockNumber,
		},
		Staked {
			/// Id of newly created stake.
			pool_id: T::RewardPoolId,
			/// Owner of the stake.
			owner: T::AccountId,
			amount: T::Balance,
			/// Duration of stake.
			duration_preset: DurationSeconds,
			/// Position Id of newly created stake.
			position_id: T::PositionId,
			keep_alive: bool,
		},
		StakeAmountExtended {
			position_id: T::PositionId,
			/// Extended amount
			amount: T::Balance,
		},
		Unstaked {
			/// Owner of the stake.
			owner: T::AccountId,
			/// Position Id of newly created stake.
			position_id: T::PositionId,
		},
		/// Split stake position into two positions
		SplitPosition {
			positions: Vec<T::PositionId>,
		},
		/// Reward transfer event.
		RewardTransferred {
			from: T::AccountId,
			pool: T::RewardPoolId,
			reward_currency: T::AssetId,
			/// amount of reward currency transferred.
			reward_increment: T::Balance,
		},
		RewardAccumulationHookError {
			pool_id: T::RewardPoolId,
			asset_id: T::AssetId,
			error: RewardAccumulationHookError,
		},
		MaxRewardsAccumulated {
			pool_id: T::RewardPoolId,
			asset_id: T::AssetId,
		},
		RewardPoolUpdated {
			pool_id: T::RewardPoolId,
		},
	}

	#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
	pub enum RewardAccumulationHookError {
		BackToTheFuture,
		RewardsPotEmpty,
		RewardsAccountFull,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error when creating reward configs.
		RewardConfigProblem,
		/// No duration presets configured.
		NoDurationPresetsConfigured,
		/// Too many rewarded asset types per pool violating the storage allowed.
		TooManyRewardAssetTypes,
		/// Invalid end block number provided for creating a pool.
		EndBlockMustBeInTheFuture,
		/// Unimplemented reward pool type.
		UnimplementedRewardPoolConfiguration,
		/// Rewards pool not found.
		RewardsPoolNotFound,
		/// Error when creating reduction configs.
		ReductionConfigProblem,
		/// Not enough assets for a stake.
		NotEnoughAssets,
		/// No stake found for given id.
		StakeNotFound,
		/// Reward's max limit reached.
		MaxRewardLimitReached,
		/// Only pool owner can add new reward asset.
		OnlyPoolOwnerCanAddNewReward,
		/// only the owner of stake can unstake it
		OnlyStakeOwnerCanUnstake,
		/// Reward asset not found in reward pool.
		RewardAssetNotFound,
		BackToTheFuture,
		/// The rewards pot (cold wallet) for this pool is empty.
		RewardsPotEmpty,
		/// The rewards account (hot wallet) for this pool is full.
		RewardsAccuntFull,
	}

	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The reward balance type.
		type Balance: Parameter
			+ Member
			+ BalanceLike
			+ FixedPointOperand
			+ From<u128>
			+ Into<u128>
			+ Zero;

		/// The reward pool ID type.
		/// Type representing the unique ID of a pool.
		type RewardPoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ Debug
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Zero
			+ One
			+ SafeArithmetic;

		/// The position id type.
		type PositionId: Parameter + Member + Clone + FullCodec + Copy + Zero + One + SafeArithmetic;

		type AssetId: Parameter
			+ Member
			+ AssetIdLike
			+ MaybeSerializeDeserialize
			+ Ord
			+ From<u128>
			+ Into<u128>;

		type FinancialNftInstanceId: FullCodec
			+ Debug
			+ SafeAdd
			+ MaxEncodedLen
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Zero
			+ One;

		type FinancialNft: nonfungibles::Mutate<AccountIdOf<Self>>
			+ nonfungibles::Create<AccountIdOf<Self>>
			+ FinancialNft<AccountIdOf<Self>>;

		/// Is used to create staked asset per `Self::RewardPoolId`
		type CurrencyFactory: CurrencyFactory<Self::AssetId, Self::Balance>;

		/// Dependency allowing this pallet to transfer funds from one account to another.
		type Assets: Transfer<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Mutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ MutateHold<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ InspectHold<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

		/// is used for rate based rewarding and position lock timing
		type UnixTime: UnixTime;

		/// the size of batch to take each time trying to release rewards
		#[pallet::constant]
		type ReleaseRewardsPoolsBatchSize: Get<u8>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Maximum number of staking duration presets allowed.
		#[pallet::constant]
		type MaxStakingDurationPresets: Get<u32>;

		/// Maximum number of reward configurations per pool.
		#[pallet::constant]
		type MaxRewardConfigsPerPool: Get<u32>;

		/// Required origin for reward pool creation.
		type RewardPoolCreationOrigin: EnsureOrigin<Self::Origin>;

		/// Required origin for reward pool creation.
		type RewardPoolUpdateOrigin: EnsureOrigin<Self::Origin>;

		type WeightInfo: WeightInfo;
	}

	/// Abstraction over RewardPoolConfiguration type
	type RewardPoolConfigurationOf<T> = RewardPoolConfiguration<
		<T as frame_system::Config>::AccountId,
		<T as Config>::AssetId,
		<T as frame_system::Config>::BlockNumber,
		RewardConfigs<
			<T as Config>::AssetId,
			<T as Config>::Balance,
			<T as Config>::MaxRewardConfigsPerPool,
		>,
		StakingDurationToRewardsMultiplierConfig<<T as Config>::MaxStakingDurationPresets>,
	>;

	/// Abstraction over RewardPool type
	type RewardPoolOf<T> = RewardPool<
		<T as frame_system::Config>::AccountId,
		<T as Config>::AssetId,
		<T as Config>::Balance,
		<T as frame_system::Config>::BlockNumber,
		StakingDurationToRewardsMultiplierConfig<<T as Config>::MaxStakingDurationPresets>,
		Rewards<
			<T as Config>::AssetId,
			<T as Config>::Balance,
			<T as Config>::MaxRewardConfigsPerPool,
		>,
	>;

	/// Abstraction over Stake type
	type StakeOf<T> = Stake<
		<T as frame_system::Config>::AccountId,
		<T as Config>::RewardPoolId,
		<T as Config>::Balance,
		Reductions<
			<T as Config>::AssetId,
			<T as Config>::Balance,
			<T as Config>::MaxRewardConfigsPerPool,
		>,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	#[allow(clippy::disallowed_types)]
	pub type RewardPoolCount<T: Config> =
		StorageValue<_, T::RewardPoolId, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type RewardPools<T: Config> =
		StorageMap<_, Blake2_128Concat, T::RewardPoolId, RewardPoolOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn stake_count)]
	#[allow(clippy::disallowed_types)]
	pub type StakeCount<T: Config> =
		StorageValue<_, T::PositionId, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	#[pallet::storage]
	#[pallet::getter(fn stakes)]
	pub type Stakes<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PositionId, StakeOf<T>, OptionQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Weight: see `begin_block`
		fn on_initialize(_: T::BlockNumber) -> Weight {
			Self::acumulate_rewards_hook()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new reward pool based on the config.
		///
		/// Emits `RewardPoolCreated` event when successful.
		#[pallet::weight(T::WeightInfo::create_reward_pool(T::MaxRewardConfigsPerPool::get()))]
		#[transactional]
		pub fn create_reward_pool(
			origin: OriginFor<T>,
			pool_config: RewardPoolConfigurationOf<T>,
		) -> DispatchResult {
			T::RewardPoolCreationOrigin::ensure_origin(origin)?;
			let _ = <Self as ManageStaking>::create_staking_pool(pool_config)?;
			Ok(())
		}

		/// Create a new stake.
		///
		/// Emits `Staked` event when successful.
		#[pallet::weight(T::WeightInfo::stake(T::MaxRewardConfigsPerPool::get()))]
		pub fn stake(
			origin: OriginFor<T>,
			pool_id: T::RewardPoolId,
			amount: T::Balance,
			duration_preset: DurationSeconds,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let keep_alive = true;
			let _position_id =
				<Self as Staking>::stake(&owner, &pool_id, amount, duration_preset, keep_alive)?;

			Ok(())
		}

		/// Extend an existing stake.
		///
		/// Emits `StakeExtended` event when successful.
		#[pallet::weight(T::WeightInfo::extend(T::MaxRewardConfigsPerPool::get()))]
		pub fn extend(
			origin: OriginFor<T>,
			position: T::PositionId,
			amount: T::Balance,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let keep_alive = true;
			let _position_id = <Self as Staking>::extend(&owner, position, amount, keep_alive)?;

			Ok(())
		}

		/// Remove a stake.
		///
		/// Emits `Unstaked` event when successful.
		#[pallet::weight(T::WeightInfo::unstake(T::MaxRewardConfigsPerPool::get()))]
		pub fn unstake(origin: OriginFor<T>, position_id: T::PositionId) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			<Self as Staking>::unstake(&owner, &position_id)?;

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::split(T::MaxRewardConfigsPerPool::get()))]
		pub fn split(
			origin: OriginFor<T>,
			position: T::PositionId,
			ratio: Validated<Permill, ValidSplitRatio>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Staking>::split(&who, &position, ratio.value())?;
			Ok(())
		}

		/// Updates the reward pool configuration.
		///
		/// Emits `RewardPoolUpdated` when successful.
		#[pallet::weight(T::WeightInfo::update_rewards_pool(reward_updates.len() as u32))]
		pub fn update_rewards_pool(
			origin: OriginFor<T>,
			pool_id: T::RewardPoolId,
			reward_updates: BoundedBTreeMap<
				AssetIdOf<T>,
				RewardUpdate<BalanceOf<T>>,
				T::MaxRewardConfigsPerPool,
			>,
		) -> DispatchResult {
			T::RewardPoolUpdateOrigin::ensure_origin(origin)?;
			update_rewards_pool::<T>(pool_id, reward_updates)
		}
	}

	impl<T: Config> ManageStaking for Pallet<T> {
		type AssetId = T::AssetId;
		type AccountId = T::AccountId;
		type BlockNumber = <T as frame_system::Config>::BlockNumber;
		type Balance = T::Balance;
		type RewardConfigsLimit = T::MaxRewardConfigsPerPool;
		type StakingDurationPresetsLimit = T::MaxStakingDurationPresets;
		type RewardPoolId = T::RewardPoolId;

		#[transactional]
		fn create_staking_pool(
			pool_config: RewardPoolConfiguration<
				Self::AccountId,
				Self::AssetId,
				Self::BlockNumber,
				RewardConfigs<Self::AssetId, Self::Balance, Self::RewardConfigsLimit>,
				StakingDurationToRewardsMultiplierConfig<Self::StakingDurationPresetsLimit>,
			>,
		) -> Result<Self::RewardPoolId, DispatchError> {
			let (owner, pool_id, end_block) = match pool_config {
				RewardRateBasedIncentive {
					owner,
					asset_id,
					reward_configs: initial_reward_config,
					end_block,
					lock,
				} => {
					ensure!(
						end_block > frame_system::Pallet::<T>::current_block_number(),
						Error::<T>::EndBlockMustBeInTheFuture
					);

					let pool_id = RewardPoolCount::<T>::increment()?;

					let now_seconds = T::UnixTime::now().as_secs();

					let rewards = initial_reward_config
						.into_iter()
						.map(|(asset_id, amount)| {
							(asset_id, Reward::from_config(amount, now_seconds))
						})
						.try_collect()
						.expect("No items were added; qed;");

					RewardPools::<T>::insert(
						pool_id,
						RewardPool {
							owner: owner.clone(),
							asset_id,
							rewards,
							total_shares: T::Balance::zero(),
							claimed_shares: T::Balance::zero(),
							end_block,
							lock,
						},
					);
					// TODO (vim): Create the financial NFT collection for the rewards pool
					Ok((owner, pool_id, end_block))
				},
				_ => Err(Error::<T>::UnimplementedRewardPoolConfiguration),
			}?;
			Self::deposit_event(Event::<T>::RewardPoolCreated { pool_id, owner, end_block });
			Ok(pool_id)
		}
	}

	impl<T: Config> FinancialNftProtocol for Pallet<T> {
		type ItemId = T::FinancialNftInstanceId;
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;

		fn collection_asset_ids() -> Vec<Self::AssetId> {
			// TODO (vim): Following is a dummy value. Store and retrieve from storage
			[Self::AssetId::from(1000_u128)].into()
		}

		fn value_of(
			_collection: &Self::AssetId,
			_instance: &Self::ItemId,
		) -> Vec<(Self::AssetId, Self::Balance)> {
			// TODO (vim): Following is a dummy value. Store and retrieve from storage
			[(Self::AssetId::from(1001_u128), Self::Balance::zero())].into()
		}
	}

	impl<T: Config> Staking for Pallet<T> {
		type AccountId = T::AccountId;
		type RewardPoolId = T::RewardPoolId;
		type Balance = T::Balance;
		type PositionId = T::PositionId;

		#[transactional]
		fn stake(
			who: &Self::AccountId,
			pool_id: &Self::RewardPoolId,
			amount: Self::Balance,
			duration_preset: DurationSeconds,
			keep_alive: bool,
		) -> Result<Self::PositionId, DispatchError> {
			let mut rewards_pool =
				RewardPools::<T>::try_get(pool_id).map_err(|_| Error::<T>::RewardsPoolNotFound)?;

			let reward_multiplier = Self::reward_multiplier(&rewards_pool, duration_preset)
				.ok_or(Error::<T>::NoDurationPresetsConfigured)?;

			ensure!(
				matches!(
					T::Assets::can_withdraw(rewards_pool.asset_id, who, amount),
					WithdrawConsequence::Success
				),
				Error::<T>::NotEnoughAssets
			);

			let boosted_amount = Self::boosted_amount(reward_multiplier, amount);
			let (rewards, reductions) =
				Self::compute_rewards_and_reductions(boosted_amount, &rewards_pool)?;

			let new_position = Stake {
				owner: who.clone(),
				reward_pool_id: *pool_id,
				stake: amount,
				share: boosted_amount,
				reductions,
				lock: lock::Lock {
					started_at: T::UnixTime::now().as_secs(),
					duration: duration_preset,
					unlock_penalty: rewards_pool.lock.unlock_penalty,
				},
			};
			// TODO (vim): Transfer the shares with share asset ID to the Financial NFT account and
			// lock it. T::Assets::mint_into(rewards_pool.share_asset_id fnft_account, amount)?;

			rewards_pool.total_shares = rewards_pool.total_shares.safe_add(&boosted_amount)?;
			rewards_pool.rewards = rewards;

			let position_id = StakeCount::<T>::increment()?;
			// TODO (vim):
			// 	1. Mint the NFT into the relevant NFT collection mapped to the reward pool
			//  2. Map and store the nft_id -> position_id
			// let next_nft_id = T::FinancialNFT::get_next_nft_id(reward_pool.fnft_collection_id)?;
			// T::FinancialNFT::mint_into(reward_pool.fnft_collection_id, next_nft_id)?;
			// FnftToPositionId<T>::insert(next_nft_id, position_id);
			// let fnft_account = T::FinancialNFT::asset_account(reward_pool.fnft_collection_id,
			// next_nft_id); TODO (vim): transfer the staked amount to the NFT account and lock it
			// T::Assets::transfer(rewards_pool.asset_id, who, fnft_account, amount)?;
			T::Assets::transfer(
				rewards_pool.asset_id,
				who,
				&Self::hot_pool_account_id(pool_id),
				amount,
				keep_alive,
			)?;
			RewardPools::<T>::insert(pool_id, rewards_pool);
			Stakes::<T>::insert(position_id, new_position);

			Self::deposit_event(Event::<T>::Staked {
				pool_id: *pool_id,
				owner: who.clone(),
				amount,
				duration_preset,
				position_id,
				keep_alive,
			});

			Ok(position_id)
		}

		#[transactional]
		fn extend(
			who: &Self::AccountId,
			position: Self::PositionId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::PositionId, DispatchError> {
			let mut stake = Stakes::<T>::get(position).ok_or(Error::<T>::StakeNotFound)?;
			let mut rewards_pool = RewardPools::<T>::try_get(stake.reward_pool_id)
				.map_err(|_| Error::<T>::RewardsPoolNotFound)?;
			let reward_multiplier = Perbill::one();

			ensure!(
				matches!(
					T::Assets::can_withdraw(rewards_pool.asset_id, who, amount),
					WithdrawConsequence::Success
				),
				Error::<T>::NotEnoughAssets
			);

			let boosted_amount = Self::boosted_amount(reward_multiplier, amount);

			let (rewards, reductions) =
				Self::compute_rewards_and_reductions(boosted_amount, &rewards_pool)?;
			rewards_pool.total_shares = rewards_pool.total_shares.safe_add(&boosted_amount)?;
			rewards_pool.rewards = rewards;
			stake.stake = stake.stake.safe_add(&boosted_amount)?;
			stake.share = stake.share.safe_add(&boosted_amount)?;
			for (asset, additional_inflation) in reductions.iter() {
				let inflation =
					stake.reductions.get_mut(asset).ok_or(Error::<T>::ReductionConfigProblem)?;
				*inflation = inflation.safe_add(additional_inflation)?;
			}

			// TODO (vim): transfer the staked amount to the NFT account and lock it
			// TODO (vim): Transfer the shares with share asset ID to the Financial NFT account and
			// lock it.
			T::Assets::transfer(
				rewards_pool.asset_id,
				who,
				&Self::hot_pool_account_id(&stake.reward_pool_id),
				amount,
				keep_alive,
			)?;
			RewardPools::<T>::insert(stake.reward_pool_id, rewards_pool);
			Stakes::<T>::insert(position, stake);
			Self::deposit_event(Event::<T>::StakeAmountExtended { position_id: position, amount });
			Ok(position)
		}

		#[transactional]
		fn unstake(who: &Self::AccountId, position_id: &Self::PositionId) -> DispatchResult {
			let keep_alive = false;
			let stake = Stakes::<T>::try_get(position_id).map_err(|_| Error::<T>::StakeNotFound)?;
			ensure!(who == &stake.owner, Error::<T>::OnlyStakeOwnerCanUnstake);
			let early_unlock = stake.lock.started_at.safe_add(&stake.lock.duration)? >=
				T::UnixTime::now().as_secs();
			let pool_id = stake.reward_pool_id;
			let mut rewards_pool =
				RewardPools::<T>::try_get(pool_id).map_err(|_| Error::<T>::RewardsPoolNotFound)?;

			let mut inner_rewards = rewards_pool.rewards.into_inner();
			for (asset_id, reward) in inner_rewards.iter_mut() {
				let inflation = stake.reductions.get(asset_id).cloned().unwrap_or_else(Zero::zero);
				let claim = if rewards_pool.total_shares == Zero::zero() {
					Zero::zero()
				} else {
					reward
						.total_rewards
						.safe_mul(&stake.share)?
						.safe_div(&rewards_pool.total_shares)?
						.safe_sub(&inflation)?
				};
				let claim = if early_unlock {
					(Perbill::one() - stake.lock.unlock_penalty).mul_ceil(claim)
				} else {
					claim
				};
				let claim = sp_std::cmp::min(
					claim,
					reward.total_rewards.safe_sub(&reward.claimed_rewards)?,
				);
				reward.claimed_rewards = reward.claimed_rewards.safe_add(&claim)?;
				T::Assets::transfer(
					reward.asset_id,
					&Self::hot_pool_account_id(&pool_id),
					&stake.owner,
					claim,
					keep_alive,
				)?;
			}
			rewards_pool.rewards = Rewards::try_from(inner_rewards)
				.expect("Conversion must work as it's the same data structure; qed;");
			rewards_pool.claimed_shares = rewards_pool.claimed_shares.safe_add(&stake.share)?;

			let stake_with_penalty = if early_unlock {
				(Perbill::one() - stake.lock.unlock_penalty).mul_ceil(stake.stake)
			} else {
				stake.stake
			};

			// TODO (vim): Unlock staked amount on financial NFT account and transfer from that
			// account to the owner of the NFT
			T::Assets::transfer(
				rewards_pool.asset_id,
				&Self::hot_pool_account_id(&pool_id),
				&stake.owner,
				stake_with_penalty,
				keep_alive,
			)?;

			RewardPools::<T>::insert(pool_id, rewards_pool);
			Stakes::<T>::remove(position_id);
			// TODO (vim): burn the financial NFT and the shares it holds

			Self::deposit_event(Event::<T>::Unstaked {
				owner: who.clone(),
				position_id: *position_id,
			});

			Ok(())
		}

		#[transactional]
		fn split(
			_who: &Self::AccountId,
			position: &Self::PositionId,
			ratio: Permill,
		) -> Result<[Self::PositionId; 2], DispatchError> {
			let mut old_position =
				Stakes::<T>::try_mutate(position, |old_stake| match old_stake {
					Some(stake) => {
						let old_value = stake.clone();
						stake.stake = ratio.mul_floor(stake.stake);
						stake.share = ratio.mul_floor(stake.share);
						let assets: Vec<T::AssetId> = stake.reductions.keys().cloned().collect();
						for asset in assets {
							let reduction = stake.reductions.get_mut(&asset);
							if let Some(value) = reduction {
								*value = ratio.mul_floor(*value);
							}
						}
						Ok(old_value)
					},
					None => Err(Error::<T>::StakeNotFound),
				})?;
			let left_from_one_ratio = ratio.left_from_one();
			let assets: Vec<T::AssetId> = old_position.reductions.keys().cloned().collect();
			for asset in assets {
				let reduction = old_position.reductions.get_mut(&asset);
				if let Some(value) = reduction {
					*value = left_from_one_ratio.mul_floor(*value);
				}
			}

			let new_stake = StakeOf::<T> {
				stake: left_from_one_ratio.mul_floor(old_position.stake),
				share: left_from_one_ratio.mul_floor(old_position.share),
				..old_position
			};
			let new_position = StakeCount::<T>::increment()?;
			Stakes::<T>::insert(new_position, new_stake);
			// TODO (vim):
			// 	1. Create the new financial NFT for the new position
			//	2. transfer the split staked amount to the NFT account and lock it
			//	3. transfer the split share amount to the NFT account and lock it
			Self::deposit_event(Event::<T>::SplitPosition {
				positions: vec![*position, new_position],
			});
			Ok([*position, new_position])
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn hot_pool_account_id(pool_id: &T::RewardPoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account_truncating((b"hot", pool_id))
		}
		pub(crate) fn cold_pool_account_id(pool_id: &T::RewardPoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account_truncating((b"cold", pool_id))
		}

		pub(crate) fn reward_multiplier(
			rewards_pool: &RewardPoolOf<T>,
			duration_preset: DurationSeconds,
		) -> Option<Perbill> {
			rewards_pool.lock.duration_presets.get(&duration_preset).cloned()
		}

		pub(crate) fn boosted_amount(reward_multiplier: Perbill, amount: T::Balance) -> T::Balance {
			reward_multiplier.mul_ceil(amount)
		}

		fn compute_rewards_and_reductions(
			boosted_amount: T::Balance,
			rewards_pool: &RewardPoolOf<T>,
		) -> Result<
			(
				Rewards<T::AssetId, T::Balance, T::MaxRewardConfigsPerPool>,
				Reductions<T::AssetId, T::Balance, T::MaxRewardConfigsPerPool>,
			),
			DispatchError,
		> {
			let mut reductions = Reductions::new();
			let mut rewards_btree_map = Rewards::new();

			for (asset_id, reward) in rewards_pool.rewards.iter() {
				let reward = reward.clone();
				let inflation = if rewards_pool.total_shares == T::Balance::zero() {
					T::Balance::zero()
				} else {
					reward
						.total_rewards
						.safe_mul(&boosted_amount)?
						.safe_div(&rewards_pool.total_shares)?
				};

				let total_rewards = reward.total_rewards.safe_add(&inflation)?;
				let total_dilution_adjustment =
					reward.total_dilution_adjustment.safe_add(&inflation)?;
				let updated_reward = Reward { total_rewards, total_dilution_adjustment, ..reward };
				rewards_btree_map
					.try_insert(*asset_id, updated_reward)
					.map_err(|_| Error::<T>::ReductionConfigProblem)?;

				reductions
					.try_insert(*asset_id, inflation)
					.map_err(|_| Error::<T>::ReductionConfigProblem)?;
			}

			Ok((rewards_btree_map, reductions))
		}

		pub(crate) fn reward_accumulation_hook_reward_update_calculation(
			pool_id: T::RewardPoolId,
			reward: Reward<T::AssetId, T::Balance>,
			now_seconds: u64,
		) -> Reward<T::AssetId, T::Balance> {
			use RewardAccumulationCalculationError::*;

			let cold_wallet = Self::cold_pool_account_id(&pool_id);
			let hot_wallet = Self::hot_pool_account_id(&pool_id);

			match reward_accumulation_calculation::<T>(
				reward,
				&cold_wallet,
				&hot_wallet,
				now_seconds,
			) {
				Ok(reward) => reward,
				Err(BackToTheFuture(reward)) => {
					Self::deposit_event(Event::<T>::RewardAccumulationHookError {
						pool_id,
						asset_id: reward.asset_id,
						error: RewardAccumulationHookError::BackToTheFuture,
					});
					reward
				},
				Err(RewardsPotEmpty(reward)) => {
					Self::deposit_event(Event::<T>::RewardAccumulationHookError {
						pool_id,
						asset_id: reward.asset_id,
						error: RewardAccumulationHookError::RewardsPotEmpty,
					});
					reward
				},
				Err(RewardsAccountFull(reward)) => {
					Self::deposit_event(Event::<T>::RewardAccumulationHookError {
						pool_id,
						asset_id: reward.asset_id,
						error: RewardAccumulationHookError::RewardsAccountFull,
					});
					reward
				},
				Err(MaxRewardsAccumulated(reward)) => {
					Self::deposit_event(Event::<T>::MaxRewardsAccumulated {
						pool_id,
						asset_id: reward.asset_id,
					});
					reward
				},
				Err(MaxRewardsAccumulatedPreviously(reward)) => reward,
			}
		}

		pub(crate) fn acumulate_rewards_hook() -> Weight {
			let now_seconds = T::UnixTime::now().as_secs();
			let unix_time_now_weight = T::WeightInfo::unix_time_now();

			let updated_pools = RewardPools::<T>::iter()
				.into_iter()
				.map(|(pool_id, reward_pool)| {
					let updated_rewards = reward_pool
						.rewards
						.into_iter()
						.map(|(asset_id, reward)| {
							(
								asset_id,
								Self::reward_accumulation_hook_reward_update_calculation(
									pool_id,
									reward,
									now_seconds,
								),
							)
						})
						.try_collect()
						// SAFETY(benluelo): No elements were added to the BTreeMap; the only
						// operation was `.map()`. This expect call will be unnecessary once this PR
						// is merged and we update to whatever version it's included in:
						// https://github.com/paritytech/substrate/pull/11869
						.expect("no elements were added; qed;");

					(pool_id, RewardPool { rewards: updated_rewards, ..reward_pool })
				})
				// NOTE(benluelo): As per these docs on `StorageMap::iter`:
				// https://github.com/paritytech/substrate/blob/cac91f59b9e3fb8fd59842c023f87b4206931993/frame/support/src/storage/types/map.rs,
				// "If you alter the map while doing this, you'll get undefined results."
				// hence the double iteration.
				.collect::<Vec<_>>();

			let mut total_weight = unix_time_now_weight;

			for (pool_id, reward_pool) in updated_pools {
				// 128 bit platforms don't exist as of writing this so this usize -> u64 cast should
				// be ok
				let number_of_rewards_in_pool = reward_pool.rewards.len() as u64;

				RewardPools::<T>::insert(pool_id, reward_pool);

				total_weight += (number_of_rewards_in_pool * T::WeightInfo::reward_accumulation_hook_reward_update_calculation()) +
						// NOTE: `StorageMap::iter` does one read per item
						T::DbWeight::get().reads(1) +
						T::DbWeight::get().writes(1)
			}

			total_weight
		}
	}

	impl<T: Config> ProtocolStaking for Pallet<T> {
		type AssetId = T::AssetId;
		type AccountId = T::AccountId;
		type RewardPoolId = T::RewardPoolId;
		type Balance = T::Balance;

		fn accumulate_reward(
			_pool: &Self::RewardPoolId,
			_reward_currency: Self::AssetId,
			_reward_increment: Self::Balance,
		) -> DispatchResult {
			Ok(())
		}

		#[transactional]
		fn transfer_reward(
			from: &Self::AccountId,
			pool: &Self::RewardPoolId,
			reward_currency: Self::AssetId,
			reward_increment: Self::Balance,
		) -> DispatchResult {
			RewardPools::<T>::try_mutate(pool, |reward_pool| {
				match reward_pool {
					Some(reward_pool) => {
						match reward_pool.rewards.get_mut(&reward_currency) {
							Some(mut reward) => {
								let new_total_reward =
									reward.total_rewards.safe_add(&reward_increment)?;
								ensure!(
									(new_total_reward
										.safe_sub(&reward.total_dilution_adjustment)?) <=
										reward.max_rewards,
									Error::<T>::MaxRewardLimitReached
								);
								reward.total_rewards = new_total_reward;
								let pool_account = Self::hot_pool_account_id(pool);
								T::Assets::transfer(
									reward_currency,
									from,
									&pool_account,
									reward_increment,
									false,
								)?;
							},
							None => {
								// new reward asset so only pool owner is allowed to add.
								ensure!(
									*from == reward_pool.owner,
									Error::<T>::OnlyPoolOwnerCanAddNewReward
								);
								let reward = Reward {
									asset_id: reward_currency,
									total_rewards: reward_increment,
									claimed_rewards: Zero::zero(),
									total_dilution_adjustment: T::Balance::zero(),
									max_rewards: max(reward_increment, DEFAULT_MAX_REWARDS.into()),
									reward_rate: RewardRate {
										amount: T::Balance::zero(),
										period: RewardRatePeriod::PerSecond,
									},
									last_updated_timestamp: 0,
								};
								reward_pool
									.rewards
									.try_insert(reward_currency, reward)
									.map_err(|_| Error::<T>::TooManyRewardAssetTypes)?;
								let pool_account = Self::hot_pool_account_id(pool);
								T::Assets::transfer(
									reward_currency,
									from,
									&pool_account,
									reward_increment,
									false,
								)?;
							},
						}
						Self::deposit_event(Event::RewardTransferred {
							from: from.clone(),
							pool: *pool,
							reward_currency,
							reward_increment,
						});
						Ok(())
					},
					None => Err(Error::<T>::UnimplementedRewardPoolConfiguration.into()),
				}
			})
		}
	}
}

#[transactional]
fn update_rewards_pool<T: Config>(
	pool_id: T::RewardPoolId,
	reward_updates: BoundedBTreeMap<
		AssetIdOf<T>,
		RewardUpdate<BalanceOf<T>>,
		T::MaxRewardConfigsPerPool,
	>,
) -> DispatchResult {
	RewardPools::<T>::try_mutate(pool_id, |pool| {
		let pool = pool.as_mut().ok_or(Error::<T>::RewardsPoolNotFound)?;

		let cold_wallet = Pallet::<T>::cold_pool_account_id(&pool_id);
		let hot_wallet = Pallet::<T>::hot_pool_account_id(&pool_id);

		let now_seconds = T::UnixTime::now().as_secs();

		for (asset_id, update) in reward_updates {
			let reward = pool.rewards.get_mut(&asset_id).ok_or(Error::<T>::RewardAssetNotFound)?;
			let new_reward = match reward_accumulation_calculation::<T>(
				reward.clone(),
				&cold_wallet,
				&hot_wallet,
				now_seconds,
			) {
				Ok(reward) => reward,
				Err(RewardAccumulationCalculationError::BackToTheFuture(_)) =>
					return Err(Error::<T>::BackToTheFuture.into()),
				Err(RewardAccumulationCalculationError::MaxRewardsAccumulated(reward)) => {
					Pallet::<T>::deposit_event(Event::<T>::MaxRewardsAccumulated {
						pool_id,
						asset_id: reward.asset_id,
					});
					continue
				},
				Err(RewardAccumulationCalculationError::MaxRewardsAccumulatedPreviously(_)) =>
					continue,
				Err(RewardAccumulationCalculationError::RewardsPotEmpty(_)) =>
					return Err(Error::<T>::RewardsPotEmpty.into()),
				Err(RewardAccumulationCalculationError::RewardsAccountFull(_)) =>
					return Err(Error::<T>::RewardsAccuntFull.into()),
			};

			*reward = Reward { reward_rate: update.reward_rate, ..new_reward };
		}

		Pallet::<T>::deposit_event(Event::<T>::RewardPoolUpdated { pool_id });

		Ok(())
	})
}

pub(crate) fn reward_accumulation_calculation<T: Config>(
	reward: Reward<T::AssetId, T::Balance>,
	cold_wallet: &T::AccountId,
	hot_wallet: &T::AccountId,
	now_seconds: u64,
) -> Result<Reward<T::AssetId, T::Balance>, RewardAccumulationCalculationError<T>> {
	match now_seconds.safe_sub(&reward.last_updated_timestamp) {
		Ok(elapsed_time) => {
			let reward_rate_period_seconds = reward.reward_rate.period.as_secs();

			// SAFETY(benluelo): Usage of Div::div:
			//
			// Integer division can only fail if rhs == 0, and
			// reward_rate_period_seconds is a NonZeroU64 here.
			let periods_surpassed = elapsed_time.div(reward_rate_period_seconds.get());

			if periods_surpassed.is_zero() {
				Ok(reward)
			} else {
				let new_rewards =
					u128::from(periods_surpassed).saturating_mul(reward.reward_rate.amount.into());

				let new_total_rewards = new_rewards.saturating_add(reward.total_rewards.into());

				let last_updated_timestamp = reward
					.last_updated_timestamp
					.add(periods_surpassed.mul(reward_rate_period_seconds.get()));

				if new_total_rewards <= reward.max_rewards.into() {
					if <T::Assets as InspectHold<AccountIdOf<T>>>::balance_on_hold(
						reward.asset_id,
						&cold_wallet,
					) >= new_rewards.into()
					{
						// release funds from cold wallet
						<T::Assets as MutateHold<AccountIdOf<T>>>::release(
							reward.asset_id,
							&cold_wallet,
							new_rewards.into(),
							false, // not best effort, entire amount must be released
						)
						.expect(
							"funds should be avaliable to release based on previous check; qed;",
						);

						// ensure the transfer will succeed
						if <T::Assets as Inspect<AccountIdOf<T>>>::can_deposit(
							reward.asset_id,
							&hot_wallet,
							new_rewards.into(),
							false, // amount already exists, will not be minted
						)
						.into_result()
						.is_ok()
						{
							// transfer released funds from cold wallet hot wallet
							<T::Assets as Transfer<AccountIdOf<T>>>::transfer(
								reward.asset_id,
								&cold_wallet,
								&hot_wallet,
								new_rewards.into(),
								false, // doesn't need to be kept alive since it's a pallet sub account
							)
							.expect(
								"funds should be avaliable to transfer based on previous release and deposit check; qed;",
							);

							Ok(Reward {
								total_rewards: new_total_rewards.into(),
								last_updated_timestamp,
								..reward
							})
						} else {
							Err(RewardAccumulationCalculationError::RewardsPotEmpty(reward))
						}
					} else {
						Err(RewardAccumulationCalculationError::RewardsPotEmpty(reward))
					}
				} else if reward.total_rewards < reward.max_rewards {
					Err(RewardAccumulationCalculationError::MaxRewardsAccumulated(Reward {
						total_rewards: reward.max_rewards,
						last_updated_timestamp,
						..reward
					}))
				} else {
					Err(RewardAccumulationCalculationError::MaxRewardsAccumulatedPreviously(reward))
				}
			}
		},
		Err(_) => Err(RewardAccumulationCalculationError::BackToTheFuture(reward)),
	}
}

pub(crate) enum RewardAccumulationCalculationError<T: Config> {
	/// T::UnixTime::now() returned a value in the past.
	BackToTheFuture(Reward<T::AssetId, T::Balance>),
	/// The `max_rewards` for this reward was hit during this calculation.
	MaxRewardsAccumulated(Reward<T::AssetId, T::Balance>),
	/// The `max_rewards` were hit previously; i.e. `total_rewards == max_rewards` at the start of
	/// this calculation.
	MaxRewardsAccumulatedPreviously(Reward<T::AssetId, T::Balance>),
	/// The rewards pot (cold wallet) for this pool is empty.
	RewardsPotEmpty(Reward<T::AssetId, T::Balance>),
	/// The rewards account (hot wallet) for this pool is full.
	RewardsAccountFull(Reward<T::AssetId, T::Balance>),
	// /// There was an error transferring or releasing funds.
	// FundsError(Reward<T::AssetId, T::Balance>, DispatchError),
}
