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
		math::safe::{SafeArithmetic, SafeDiv, SafeMul, SafeSub},
		validation::Validated,
	};
	use composable_traits::{
		currency::{BalanceLike, CurrencyFactory},
		staking::{
			RewardPoolConfiguration::RewardRateBasedIncentive, RewardUpdates, DEFAULT_MAX_REWARDS,
		},
		time::DurationSeconds,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::WithdrawConsequence,
			UnixTime,
		},
		transactional, BoundedBTreeMap, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{traits::One, Permill};
	use sp_runtime::{
		traits::{AccountIdConversion, BlockNumberProvider},
		PerThing, Perbill,
	};
	use sp_std::{cmp::max, collections::btree_map::BTreeMap, fmt::Debug, vec, vec::Vec};

	use crate::{prelude::*, validation::ValidSplitRatio};

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
		/// Pool with specified id `T::RewardPoolId` was updated successfully by `T::AccountId`.
		RewardPoolUpdated {
			/// Id of updated pool.
			pool_id: T::RewardPoolId,
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
		/// Split stake position into two positions
		SplitPosition { positions: Vec<T::PositionId> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error when creating reward configs.
		RewardConfigProblem,
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
		/// Max reward is exceeded
		MaxRewardExceeded,
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

		/// Is used to create staked asset per `Self::RewardPoolId`
		type CurrencyFactory: CurrencyFactory<Self::AssetId, Self::Balance>;

		/// Dependency allowing this pallet to transfer funds from one account to another.
		type Assets: Transfer<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Mutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

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

		/// Required origin for reward pool update.
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
		<T as Config>::RewardPoolId,
		<T as Config>::Balance,
		Reductions<
			<T as Config>::AssetId,
			<T as Config>::Balance,
			<T as Config>::MaxRewardConfigsPerPool,
		>,
	>;

	/// Abstraction over RewardUpdates
	type RewardUpdatesOf<T> = RewardUpdates<<T as Config>::AssetId, <T as Config>::Balance>;

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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new reward pool based on the config.
		///
		/// Emits `RewardPoolCreated` event when successful.
		#[pallet::weight(T::WeightInfo::create_reward_pool())]
		#[transactional]
		pub fn create_reward_pool(
			origin: OriginFor<T>,
			pool_config: RewardPoolConfigurationOf<T>,
		) -> DispatchResult {
			T::RewardPoolCreationOrigin::ensure_origin(origin)?;
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
					let mut rewards = BTreeMap::new();
					for (_, reward_config) in initial_reward_config.into_iter().enumerate() {
						rewards.insert(reward_config.0, Reward::from(reward_config.1));
					}
					RewardPools::<T>::insert(
						pool_id,
						RewardPool {
							owner: owner.clone(),
							asset_id,
							rewards: BoundedBTreeMap::<
								T::AssetId,
								Reward<T::AssetId, T::Balance>,
								T::MaxRewardConfigsPerPool,
							>::try_from(rewards)
							.map_err(|_| Error::<T>::RewardConfigProblem)?,
							total_shares: T::Balance::zero(),
							end_block,
							lock,
							last_updated: T::UnixTime::now().as_secs(),
						},
					);
					Ok((owner, pool_id, end_block))
				},
				_ => Err(Error::<T>::UnimplementedRewardPoolConfiguration),
			}?;
			Self::deposit_event(Event::<T>::RewardPoolCreated { pool_id, owner, end_block });
			Ok(())
		}

		/// Create a new stake.
		///
		/// Emits `Staked` event when successful.
		#[pallet::weight(T::WeightInfo::stake())]
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
		#[pallet::weight(T::WeightInfo::extend())]
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

		#[pallet::weight(T::WeightInfo::split())]
		pub fn split(
			origin: OriginFor<T>,
			position: T::PositionId,
			ratio: Validated<Permill, ValidSplitRatio>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Staking>::split(&who, &position, ratio.value())?;
			Ok(())
		}

		/// Update a new reward pool based on the provided amounts and rates.
		///
		/// Emits `RewardPoolUpdated` event when successful.
		#[pallet::weight(T::WeightInfo::update_reward_pool())]
		#[transactional]
		pub fn update_reward_pool(
			origin: OriginFor<T>,
			pool_id: T::RewardPoolId,
			reward_updates: RewardUpdatesOf<T>,
		) -> DispatchResult {
			T::RewardPoolUpdateOrigin::ensure_origin(origin)?;

			let current_time = T::UnixTime::now().as_secs();

			let mut rewards_pool =
				RewardPools::<T>::try_get(pool_id).map_err(|_| Error::<T>::RewardsPoolNotFound)?;

			let mut inner_rewards = rewards_pool.rewards.into_inner();

			let elapsed_time = current_time - rewards_pool.last_updated;

			for (asset_id, reward) in inner_rewards.iter_mut() {
				match reward_updates.get(asset_id) {
					Some(reward_update) => {
						ensure!(
							reward.max_rewards > reward.total_rewards + reward_update.amount,
							Error::<T>::MaxRewardExceeded
						);

						reward.total_rewards =
							reward.total_rewards.safe_add(&reward_update.amount).map_err(
								|_| Error::<T>::RewardConfigProblem, // TODO: use proper error
							)?;
						// reward.total_rewards = reward.total_rewards
						// 	.safe_add(elapsed_time * pool.reward_rate)
						// 	.map_err(|_| ArithmeticError::Overflow)?;
						reward.reward_rate = reward_update.reward_rate;
					},
					None => (),
				}
			}

			let rewards =
				Rewards::try_from(inner_rewards).map_err(|_| Error::<T>::RewardConfigProblem)?;

			rewards_pool.last_updated = current_time;
			rewards_pool.rewards = rewards;

			RewardPools::<T>::insert(pool_id, rewards_pool);

			Self::deposit_event(Event::<T>::RewardPoolUpdated { pool_id });

			Ok(())
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
				.ok_or(Error::<T>::RewardConfigProblem)?;

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

			rewards_pool.total_shares = rewards_pool.total_shares.safe_add(&boosted_amount)?;
			rewards_pool.rewards = rewards;

			let position_id = StakeCount::<T>::increment()?;
			T::Assets::transfer(
				rewards_pool.asset_id,
				who,
				&Self::pool_account_id(pool_id),
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

			T::Assets::transfer(
				rewards_pool.asset_id,
				who,
				&Self::pool_account_id(&stake.reward_pool_id),
				amount,
				keep_alive,
			)?;
			RewardPools::<T>::insert(stake.reward_pool_id, rewards_pool);
			Stakes::<T>::insert(position, stake);
			Self::deposit_event(Event::<T>::StakeAmountExtended { position_id: position, amount });
			Ok(position)
		}

		#[transactional]
		fn unstake(
			_who: &Self::AccountId,
			_position: &Self::PositionId,
			_remove_amount: Self::Balance,
		) -> DispatchResult {
			Err("Not implemented".into())
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
			Self::deposit_event(Event::<T>::SplitPosition {
				positions: vec![*position, new_position],
			});
			Ok([*position, new_position])
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn pool_account_id(pool_id: &T::RewardPoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account_truncating(pool_id)
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
								let pool_account = Self::pool_account_id(pool);
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
									total_dilution_adjustment: T::Balance::zero(),
									max_rewards: max(reward_increment, DEFAULT_MAX_REWARDS.into()),
									reward_rate: Perbill::zero(),
								};
								reward_pool
									.rewards
									.try_insert(reward_currency, reward)
									.map_err(|_| Error::<T>::RewardConfigProblem)?;
								let pool_account = Self::pool_account_id(pool);
								T::Assets::transfer(
									reward_currency,
									from,
									&pool_account,
									reward_increment,
									false,
								)?;
							},
						}
						Ok(())
					},
					None => Err(Error::<T>::UnimplementedRewardPoolConfiguration.into()),
				}
			})
		}
	}
}
