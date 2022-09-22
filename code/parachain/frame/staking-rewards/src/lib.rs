//! Implements staking rewards protocol.
#![cfg_attr(not(feature = "std"), no_std)]
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
	unused_extern_crates,
	clippy::unseparated_literal_suffix,
	clippy::disallowed_types
)]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
mod prelude;
#[cfg(test)]
mod test;
mod validation;
pub mod weights;

use sp_runtime::{traits::Saturating, SaturatedConversion};
use sp_std::{
	cmp,
	ops::{Div, Sub},
};

use crate::prelude::*;
use composable_support::math::safe::SafeSub;
use composable_traits::staking::{RateBasedReward, Reward, RewardUpdate};
use frame_support::{
	traits::{
		fungibles::{InspectHold, MutateHold, Transfer},
		UnixTime,
	},
	transactional, BoundedBTreeMap,
};
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use composable_support::{
		math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub},
		validation::Validated,
	};
	use composable_traits::{
		currency::{BalanceLike, CurrencyFactory},
		fnft::{FinancialNft, FinancialNftProtocol},
		staking::{
			lock::LockConfig, RateBasedReward, Reward, RewardPoolConfig, RewardUpdate, Stake,
		},
		time::{DurationSeconds, ONE_MONTH, ONE_WEEK},
	};
	use frame_support::{
		traits::{
			fungibles::{
				Inspect as FungiblesInspect, InspectHold as FungiblesInspectHold,
				Mutate as FungiblesMutate, MutateHold as FungiblesMutateHold,
				Transfer as FungiblesTransfer,
			},
			tokens::{
				nonfungibles,
				nonfungibles::{
					Create as NonFungiblesCreate, Inspect as NonFungiblesInspect,
					Mutate as NonFungiblesMutate,
				},
				WithdrawConsequence,
			},
			Defensive, DefensiveSaturating, TryCollect, UnixTime,
		},
		transactional, BoundedBTreeMap, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use orml_traits::{LockIdentifier, MultiLockableCurrency};
	use sp_arithmetic::Permill;
	use sp_runtime::{
		traits::{AccountIdConversion, BlockNumberProvider},
		PerThing, Perbill,
	};
	use sp_std::{collections::btree_map::BTreeMap, fmt::Debug, vec::Vec};

	use crate::{
		add_to_rewards_pot, do_rate_based_reward_accumulation, prelude::*, update_rewards_pool,
		validation::ValidSplitRatio, RewardAccumulationCalculationError,
	};

	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::AssetId` was created successfully by `T::AccountId`.
		RewardPoolCreated {
			/// The staked asset of the pool, also used as the pool's id.
			// TODO(benluelo): Rename to 'staked asset id'
			pool_id: T::AssetId,
			/// Owner of the pool.
			owner: T::AccountId,
			/// End block
			end_block: T::BlockNumber,
		},
		Staked {
			/// Id of the pool that was staked in.
			pool_id: T::AssetId,
			/// Owner of the stake.
			owner: T::AccountId,
			/// The amount that was staked.
			amount: T::Balance,
			/// Duration of stake.
			duration_preset: DurationSeconds,
			/// FNFT Collection Id
			fnft_collection_id: T::AssetId,
			/// FNFT Instance Id
			fnft_instance_id: T::FinancialNftInstanceId,
			// REVIEW(benluelo) is this required to be in the event?
			keep_alive: bool,
		},
		Claimed {
			/// Owner of the stake.
			owner: T::AccountId,
			/// FNFT Collection Id
			fnft_collection_id: T::AssetId,
			/// FNFT Instance Id
			fnft_instance_id: T::FinancialNftInstanceId,
		},
		StakeAmountExtended {
			/// FNFT Collection Id
			fnft_collection_id: T::AssetId,
			/// FNFT Instance Id
			fnft_instance_id: T::FinancialNftInstanceId,
			/// Extended amount
			amount: T::Balance,
		},
		Unstaked {
			/// Owner of the stake.
			owner: T::AccountId,
			/// FNFT Collection Id
			fnft_collection_id: T::AssetId,
			/// FNFT Instance Id
			fnft_instance_id: T::FinancialNftInstanceId,
		},
		/// Split stake position into two positions
		SplitPosition {
			// The amount staked in the new position.
			stake: BalanceOf<T>,
			/// FNFT Collection Id
			fnft_collection_id: T::AssetId,
			/// FNFT Instance Id
			fnft_instance_id: T::FinancialNftInstanceId,
		},
		/// Reward transfer event.
		RewardTransferred {
			from: T::AccountId,
			pool_id: T::AssetId,
			reward_currency: T::AssetId,
			/// amount of reward currency transferred.
			reward_increment: T::Balance,
		},
		RewardAccumulationHookError {
			pool_id: T::AssetId,
			asset_id: T::AssetId,
			error: RewardAccumulationHookError,
		},
		MaxRewardsAccumulated {
			pool_id: T::AssetId,
			asset_id: T::AssetId,
		},
		RewardPoolUpdated {
			pool_id: T::AssetId,
		},
		RewardsPotIncreased {
			pool_id: T::AssetId,
			asset_id: T::AssetId,
			amount: T::Balance,
		},
	}

	#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
	pub enum RewardAccumulationHookError {
		BackToTheFuture,
		RewardsPotEmpty,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error when creating reward configs.
		RewardConfigProblem,
		/// Reward pool already exists
		RewardsPoolAlreadyExists,
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
		/// The rewards pot for this pool is empty.
		RewardsPotEmpty,
		FnftNotFound,
	}

	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type FinancialNftInstanceIdOf<T> =
		<<T as Config>::FinancialNft as nonfungibles::Inspect<
			<T as frame_system::Config>::AccountId,
		>>::ItemId;

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

		type AssetId: Parameter
			+ Member
			+ AssetIdLike
			+ MaybeSerializeDeserialize
			+ Ord
			+ From<u128>
			+ Into<u128>
			+ Copy;

		// REVIEW(benluelo): Mutate::CollectionId type?
		type FinancialNft: NonFungiblesMutate<AccountIdOf<Self>>
			+ NonFungiblesCreate<
				AccountIdOf<Self>,
				CollectionId = Self::AssetId,
				ItemId = Self::FinancialNftInstanceId,
			> + FinancialNft<
				AccountIdOf<Self>,
				CollectionId = Self::AssetId,
				ItemId = Self::FinancialNftInstanceId,
			>;

		// https://github.com/rust-lang/rust/issues/52662
		type FinancialNftInstanceId: Parameter + Member + Copy + From<u64> + Into<u64>;

		/// Is used to create staked asset per reward pool
		// REVIEW(benluelo): This is currently unused; remove?
		type CurrencyFactory: CurrencyFactory<Self::AssetId, Self::Balance>;

		/// Dependency allowing this pallet to transfer funds from one account to another.
		type Assets: FungiblesTransfer<
				AccountIdOf<Self>,
				Balance = BalanceOf<Self>,
				AssetId = AssetIdOf<Self>,
			> + FungiblesMutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ FungiblesMutateHold<
				AccountIdOf<Self>,
				Balance = BalanceOf<Self>,
				AssetId = AssetIdOf<Self>,
			> + FungiblesInspectHold<
				AccountIdOf<Self>,
				Balance = BalanceOf<Self>,
				AssetId = AssetIdOf<Self>,
			> + MultiLockableCurrency<
				AccountIdOf<Self>,
				Balance = BalanceOf<Self>,
				CurrencyId = AssetIdOf<Self>,
			>;

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

		#[pallet::constant]
		type PicaAssetId: Get<Self::AssetId>;

		#[pallet::constant]
		type XPicaAssetId: Get<Self::AssetId>;

		#[pallet::constant]
		type PbloAssetId: Get<Self::AssetId>;

		#[pallet::constant]
		type XPbloAssetId: Get<Self::AssetId>;

		#[pallet::constant]
		type PicaStakeFinancialNftCollectionId: Get<Self::AssetId>;

		#[pallet::constant]
		type PbloStakeFinancialNftCollectionId: Get<Self::AssetId>;

		#[pallet::constant]
		type LockId: Get<LockIdentifier>;

		// The account to send the slashed stakes to.
		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;

		type WeightInfo: WeightInfo;
	}

	/// Abstraction over RewardPoolConfiguration type
	pub(crate) type RewardPoolConfigurationOf<T> = RewardPoolConfig<
		AccountIdOf<T>,
		AssetIdOf<T>,
		BalanceOf<T>,
		<T as frame_system::Config>::BlockNumber,
		<T as Config>::MaxRewardConfigsPerPool,
		<T as Config>::MaxStakingDurationPresets,
	>;

	/// Abstraction over RewardPool type
	pub(crate) type RewardPoolOf<T> = RewardPool<
		AccountIdOf<T>,
		AssetIdOf<T>,
		BalanceOf<T>,
		<T as frame_system::Config>::BlockNumber,
		<T as Config>::MaxStakingDurationPresets,
		<T as Config>::MaxRewardConfigsPerPool,
	>;

	/// Abstraction over Stake type
	pub(crate) type StakeOf<T> = Stake<
		AssetIdOf<T>,
		AssetIdOf<T>, // we use AssetId as the reward pool id
		BalanceOf<T>,
		<T as Config>::MaxRewardConfigsPerPool,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type RewardPools<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, RewardPoolOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn stakes)]
	// REVIEW(benluelo): Twox128 for the hasher?
	pub type Stakes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AssetId, // collection id
		Blake2_128Concat,
		FinancialNftInstanceIdOf<T>,
		StakeOf<T>,
	>;

	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub(super) type RewardsPotIsEmpty<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::AssetId, Blake2_128Concat, T::AssetId, ()>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		_phantom: sp_std::marker::PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig { _phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let owner: T::AccountId = T::PalletId::get().into_account_truncating();
			create_default_pool::<T>(
				&owner,
				T::PicaAssetId::get(),
				T::XPicaAssetId::get(),
				T::PicaStakeFinancialNftCollectionId::get(),
			);
			create_default_pool::<T>(
				&owner,
				T::PbloAssetId::get(),
				T::XPbloAssetId::get(),
				T::PbloStakeFinancialNftCollectionId::get(),
			);
		}
	}

	fn create_default_pool<T: Config>(
		owner: &T::AccountId,
		staked_asset_id: T::AssetId,
		share_asset_id: T::AssetId,
		financial_nft_asset_id: T::AssetId,
	) {
		// TODO (vim): Review these with product
		let staking_pool: RewardPoolOf<T> = RewardPool {
			owner: owner.clone(),
			rewards: Default::default(),
			total_shares: T::Balance::zero(),
			claimed_shares: T::Balance::zero(),
			end_block: T::BlockNumber::zero(),
			lock: LockConfig {
				duration_presets: [
					(ONE_WEEK, Perbill::from_percent(1)),
					(ONE_MONTH, Perbill::from_percent(10)),
				]
				.into_iter()
				.try_collect()
				.expect("Genesis config must be correct; qed"),
				unlock_penalty: Default::default(),
			},
			share_asset_id,
			fnft_asset_id: financial_nft_asset_id,
		};
		RewardPools::<T>::insert(staked_asset_id, staking_pool);
		T::FinancialNft::create_collection(&financial_nft_asset_id, owner, owner)
			.expect("Genesis config must be correct; qed");
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Weight: see `begin_block`
		fn on_initialize(_: T::BlockNumber) -> Weight {
			Self::accumulate_rewards_hook()
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
			pool_id: T::AssetId,
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
			fnft_collection_id: T::AssetId,
			fnft_instance_id: T::FinancialNftInstanceId,
			amount: T::Balance,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let keep_alive = true;
			let _position_id = <Self as Staking>::extend(
				&owner,
				(fnft_collection_id, fnft_instance_id),
				amount,
				keep_alive,
			)?;

			Ok(())
		}

		/// Remove a stake.
		///
		/// Emits `Unstaked` event when successful.
		#[pallet::weight(T::WeightInfo::unstake(T::MaxRewardConfigsPerPool::get()))]
		pub fn unstake(
			origin: OriginFor<T>,
			fnft_collection_id: T::AssetId,
			fnft_instance_id: T::FinancialNftInstanceId,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			<Self as Staking>::unstake(&owner, &(fnft_collection_id, fnft_instance_id))?;

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::split(T::MaxRewardConfigsPerPool::get()))]
		pub fn split(
			origin: OriginFor<T>,
			fnft_collection_id: T::AssetId,
			fnft_instance_id: T::FinancialNftInstanceId,
			ratio: Validated<Permill, ValidSplitRatio>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Staking>::split(&who, &(fnft_collection_id, fnft_instance_id), ratio.value())?;
			Ok(())
		}

		/// Updates the reward pool configuration.
		///
		/// Emits `RewardPoolUpdated` when successful.
		#[pallet::weight(T::WeightInfo::update_rewards_pool(reward_updates.len() as u32))]
		pub fn update_rewards_pool(
			origin: OriginFor<T>,
			pool_id: T::AssetId,
			reward_updates: BoundedBTreeMap<
				AssetIdOf<T>,
				RewardUpdate<BalanceOf<T>>,
				T::MaxRewardConfigsPerPool,
			>,
		) -> DispatchResult {
			T::RewardPoolUpdateOrigin::ensure_origin(origin)?;
			update_rewards_pool::<T>(pool_id, reward_updates)
		}

		/// Claim a current reward for some position.
		///
		/// Emits `Claimed` event when successful.
		#[pallet::weight(T::WeightInfo::claim(T::MaxRewardConfigsPerPool::get()))]
		pub fn claim(
			origin: OriginFor<T>,
			fnft_collection_id: T::AssetId,
			fnft_instance_id: T::FinancialNftInstanceId,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			<Self as Staking>::claim(&owner, &(fnft_collection_id, fnft_instance_id))?;

			Ok(())
		}

		/// Add funds to the reward pool's rewards pot for the specified asset.
		///
		/// Emits `RewardsPotIncreased` when successful.
		#[pallet::weight(T::WeightInfo::add_to_rewards_pot())]
		pub fn add_to_rewards_pot(
			origin: OriginFor<T>,
			pool_id: T::AssetId,
			asset_id: T::AssetId,
			amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			add_to_rewards_pot::<T>(who, pool_id, asset_id, amount, keep_alive)
		}
	}

	impl<T: Config> ManageStaking for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type BlockNumber = <T as frame_system::Config>::BlockNumber;
		type Balance = T::Balance;
		type RewardConfigsLimit = T::MaxRewardConfigsPerPool;
		type StakingDurationPresetsLimit = T::MaxStakingDurationPresets;
		type RewardPoolId = T::AssetId;

		#[transactional]
		fn create_staking_pool(
			pool_config: RewardPoolConfigurationOf<T>,
		) -> Result<Self::RewardPoolId, DispatchError> {
			let RewardPoolConfig {
				owner,
				asset_id: pool_asset,
				reward_configs: initial_reward_config,
				end_block,
				lock,
				share_asset_id,
				fnft_asset_id,
			} = pool_config;

			ensure!(
				end_block > frame_system::Pallet::<T>::current_block_number(),
				Error::<T>::EndBlockMustBeInTheFuture
			);

			ensure!(
				!RewardPools::<T>::contains_key(pool_asset),
				Error::<T>::RewardsPoolAlreadyExists
			);

			let now_seconds = T::UnixTime::now().as_secs();

			let rewards = initial_reward_config
				.into_iter()
				.map(|(asset_id, reward_config)| {
					(asset_id, Reward::from_config(reward_config, now_seconds))
				})
				.try_collect()
				.expect("No items were added; qed;");

			RewardPools::<T>::insert(
				pool_asset,
				RewardPool {
					owner: owner.clone(),
					rewards,
					total_shares: T::Balance::zero(),
					claimed_shares: T::Balance::zero(),
					end_block,
					lock,
					share_asset_id,
					fnft_asset_id,
				},
			);

			T::FinancialNft::create_collection(&fnft_asset_id, &owner, &owner)?;

			Self::deposit_event(Event::<T>::RewardPoolCreated {
				pool_id: pool_asset,
				owner,
				end_block,
			});

			Ok(pool_asset)
		}
	}

	impl<T: Config> FinancialNftProtocol for Pallet<T> {
		type ItemId = FinancialNftInstanceIdOf<T>;
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
		type RewardPoolId = T::AssetId;
		type Balance = T::Balance;
		type PositionId = (T::AssetId, T::FinancialNftInstanceId);

		#[transactional]
		fn stake(
			who: &Self::AccountId,
			pool_id: &Self::RewardPoolId,
			amount: Self::Balance,
			duration_preset: DurationSeconds,
			keep_alive: bool,
		) -> Result<Self::PositionId, DispatchError> {
			RewardPools::<T>::try_mutate(pool_id, |maybe_rewards_pool| {
				let rewards_pool =
					maybe_rewards_pool.as_mut().ok_or(Error::<T>::RewardsPoolNotFound)?;

				// REVIEW(benluelo): Is this check necessary/ worth it?
				ensure!(
					matches!(
						T::Assets::can_withdraw(*pool_id, who, amount),
						WithdrawConsequence::Success
					),
					Error::<T>::NotEnoughAssets
				);

				// REVIEW(benluelo): Make this a method on `Lock`
				let boosted_amount = rewards_pool
					.lock
					.duration_presets
					.get(&duration_preset)
					.cloned()
					// TODO(benluelo): Rename this error to something like "DurationPresetNotFound"
					.ok_or(Error::<T>::NoDurationPresetsConfigured)?
					.mul_ceil(amount);

				let reductions = {
					let mut reductions = BTreeMap::new();

					for (asset_id, reward) in &mut rewards_pool.rewards {
						let rate_based_reward = match reward {
							Reward::ProtocolDistribution() => continue,
							Reward::RateBased(ref mut rate_based_reward) => rate_based_reward,
						};

						let inflation = if rewards_pool.total_shares == T::Balance::zero() {
							T::Balance::zero()
						} else {
							rate_based_reward
								.total_rewards
								.safe_mul(&boosted_amount)?
								.safe_div(&rewards_pool.total_shares)?
						};

						rate_based_reward.total_rewards =
							rate_based_reward.total_rewards.safe_add(&inflation)?;
						rate_based_reward.total_dilution_adjustment =
							rate_based_reward.total_dilution_adjustment.safe_add(&inflation)?;

						reductions.insert(*asset_id, inflation);
					}

					reductions
				};

				let fnft_instance_id =
					T::FinancialNft::get_next_nft_id(&rewards_pool.fnft_asset_id)?;
				let fnft_account =
					T::FinancialNft::asset_account(&rewards_pool.fnft_asset_id, &fnft_instance_id);

				rewards_pool.total_shares = rewards_pool.total_shares.safe_add(&boosted_amount)?;

				let new_position = StakeOf::<T> {
					staked_asset_id: *pool_id,
					amount,
					share: boosted_amount,
					reductions: reductions.try_into().expect("created from rewards_pool.rewards which is bounded by the same maximum; qed;"),
					lock: lock::Lock {
						started_at: T::UnixTime::now().as_secs(),
						duration: duration_preset,
						unlock_penalty: rewards_pool.lock.unlock_penalty,
					},
				};

				// Move staked funds into fNFT asset account & lock the assets
				T::Assets::transfer(*pool_id, who, &fnft_account, amount, keep_alive)?;
				T::Assets::set_lock(T::LockId::get(), *pool_id, &fnft_account, amount)?;

				// Mint share tokens into fNFT asset account & lock the assets
				T::Assets::mint_into(rewards_pool.share_asset_id, &fnft_account, amount)?;
				T::Assets::set_lock(
					T::LockId::get(),
					rewards_pool.share_asset_id,
					&fnft_account,
					amount,
				)?;

				// Mint the fNFT
				T::FinancialNft::mint_into(&rewards_pool.fnft_asset_id, &fnft_instance_id, who)?;

				Stakes::<T>::insert(rewards_pool.fnft_asset_id, fnft_instance_id, new_position);

				Self::deposit_event(Event::<T>::Staked {
					pool_id: *pool_id,
					owner: who.clone(),
					amount,
					duration_preset,
					fnft_instance_id,
					fnft_collection_id: rewards_pool.fnft_asset_id,
					keep_alive,
				});

				Ok((rewards_pool.fnft_asset_id, fnft_instance_id))
			})
		}

		#[transactional]
		fn extend(
			who: &Self::AccountId,
			(fnft_collection_id, fnft_instance_id): Self::PositionId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::PositionId, DispatchError> {
			Stakes::<T>::try_mutate(fnft_collection_id, fnft_instance_id, |maybe_stake| {
				let stake = maybe_stake.as_mut().ok_or(Error::<T>::StakeNotFound)?;

				RewardPools::<T>::try_mutate(stake.staked_asset_id, |maybe_rewards_pool| {
					let rewards_pool =
						maybe_rewards_pool.as_mut().ok_or(Error::<T>::RewardsPoolNotFound)?;

					let reward_multiplier = Perbill::one();

					ensure!(
						matches!(
							T::Assets::can_withdraw(stake.staked_asset_id, who, amount),
							WithdrawConsequence::Success
						),
						Error::<T>::NotEnoughAssets
					);

					let boosted_amount = Self::boosted_amount(reward_multiplier, amount);

					for (asset_id, reward) in &mut rewards_pool.rewards {
						let rate_based_reward = match reward {
							Reward::ProtocolDistribution() => continue,
							Reward::RateBased(ref mut rate_based_reward) => rate_based_reward,
						};

						let additional_inflation =
							if rewards_pool.total_shares == T::Balance::zero() {
								T::Balance::zero()
							} else {
								rate_based_reward
									.total_rewards
									.safe_mul(&boosted_amount)?
									.safe_div(&rewards_pool.total_shares)?
							};

						// mutate reward
						rate_based_reward.total_rewards =
							rate_based_reward.total_rewards.safe_add(&additional_inflation)?;
						rate_based_reward.total_dilution_adjustment = rate_based_reward
							.total_dilution_adjustment
							.safe_add(&additional_inflation)?;

						// REVIEW(benluelo): Review the expected behaviour of stake.reductions with
						// the new rewards types
						if let Some(existing_inflation) = stake.reductions.get_mut(asset_id) {
							*existing_inflation =
								existing_inflation.safe_add(&additional_inflation)?;
						} else {
							let _ = stake
								.reductions
								.try_insert(*asset_id, additional_inflation)
								.defensive();
						}
					}

					rewards_pool.total_shares =
						rewards_pool.total_shares.safe_add(&boosted_amount)?;

					stake.amount = stake.amount.safe_add(&amount)?;
					stake.share = stake.share.safe_add(&boosted_amount)?;

					let fnft_asset_account =
						T::FinancialNft::asset_account(&fnft_collection_id, &fnft_instance_id);

					T::Assets::transfer(
						stake.staked_asset_id,
						who,
						&fnft_asset_account,
						amount,
						keep_alive,
					)?;

					T::Assets::mint_into(rewards_pool.share_asset_id, &fnft_asset_account, amount)?;

					Self::deposit_event(Event::<T>::StakeAmountExtended {
						amount,
						fnft_collection_id,
						fnft_instance_id,
					});

					Ok((fnft_collection_id, fnft_instance_id))
				})
			})
		}

		#[transactional]
		fn unstake(
			who: &Self::AccountId,
			(fnft_collection_id, fnft_instance_id): &Self::PositionId,
		) -> DispatchResult {
			let keep_alive = false;

			let mut stake = Stakes::<T>::take(fnft_collection_id, fnft_instance_id)
				.ok_or(Error::<T>::StakeNotFound)?;

			let owner = T::FinancialNft::owner(fnft_collection_id, fnft_instance_id)
				.ok_or(Error::<T>::FnftNotFound)?;

			ensure!(who == &owner, Error::<T>::OnlyStakeOwnerCanUnstake);

			let is_early_unlock = stake.lock.started_at.safe_add(&stake.lock.duration)? >=
				T::UnixTime::now().as_secs();

			let share_asset_id =
				RewardPools::<T>::try_mutate(stake.staked_asset_id, |rewards_pool| {
					let rewards_pool =
						rewards_pool.as_mut().ok_or(Error::<T>::RewardsPoolNotFound)?;

					Self::collect_rewards(
						rewards_pool,
						&mut stake,
						&owner,
						is_early_unlock,
						keep_alive,
					)?;

					rewards_pool.claimed_shares =
						rewards_pool.claimed_shares.safe_add(&stake.share)?;

					Ok::<_, DispatchError>(rewards_pool.share_asset_id)
				})?;

			// REVIEW(benluelo): Make this logic a method on Stake
			let staked_amount_returned_to_staker = if is_early_unlock {
				stake.lock.unlock_penalty.left_from_one().mul_ceil(stake.amount)
			} else {
				stake.amount
			};

			let fnft_asset_account =
				T::FinancialNft::asset_account(fnft_collection_id, fnft_instance_id);

			T::Assets::remove_lock(T::LockId::get(), stake.staked_asset_id, &fnft_asset_account)?;
			T::Assets::remove_lock(T::LockId::get(), share_asset_id, &fnft_asset_account)?;
			T::Assets::transfer(
				stake.staked_asset_id,
				&fnft_asset_account,
				&owner,
				staked_amount_returned_to_staker,
				keep_alive,
			)?;

			// transfer slashed stake to the treasury
			if is_early_unlock {
				// If there is no penalty then there is nothing to burn as it will all have been
				// transferred back to the staker. burn_from isn't a noop if the amount to burn
				// is 0, hence the check
				T::Assets::transfer(
					stake.staked_asset_id,
					&fnft_asset_account,
					&T::TreasuryAccount::get(),
					// staked_amount_returned_to_staker should always be <= stake.amount as per
					// the formula used to calculate it, so this should never fail.
					// defensive_saturating_sub uses saturating_sub as a fallback if the
					// operation *were* to fail, resulting in no transfer happening (the
					// transferred amount would be 0) and an error being logged.
					stake.amount.defensive_saturating_sub(staked_amount_returned_to_staker),
					false, // pallet account, doesn't need to be kept alive
				)?;
			}
			// burn the shares
			T::Assets::burn_from(share_asset_id, &fnft_asset_account, stake.amount)?;
			// burn the nft itself
			T::FinancialNft::burn(fnft_collection_id, fnft_instance_id, Some(who))?;

			Self::deposit_event(Event::<T>::Unstaked {
				owner: who.clone(),
				fnft_collection_id: *fnft_collection_id,
				fnft_instance_id: *fnft_instance_id,
			});

			Ok(())
		}

		#[transactional]
		fn split(
			who: &Self::AccountId,
			(fnft_collection_id, old_fnft_instance_id): &Self::PositionId,
			ratio: Permill,
		) -> Result<Self::PositionId, DispatchError> {
			let (new_fnft_instance_id, new_position) = Stakes::<T>::try_mutate(
				fnft_collection_id,
				old_fnft_instance_id,
				|maybe_old_position| {
					let old_position =
						maybe_old_position.as_mut().ok_or(Error::<T>::StakeNotFound)?;

					let left_from_one_ratio = ratio.left_from_one();

					let new_stake = left_from_one_ratio.mul_ceil(old_position.amount);
					let new_share = left_from_one_ratio.mul_ceil(old_position.share);
					let new_reductions = {
						let mut r = old_position.reductions.clone();
						for (_, reduction) in &mut r {
							*reduction = left_from_one_ratio.mul_floor(*reduction);
						}
						r
					};

					old_position.amount = ratio.mul_floor(old_position.amount);
					old_position.share = ratio.mul_floor(old_position.share);
					for (_, reduction) in &mut old_position.reductions {
						*reduction = ratio.mul_floor(*reduction);
					}

					let rewards_pool = RewardPools::<T>::get(old_position.staked_asset_id)
						.ok_or(Error::<T>::RewardsPoolNotFound)?;

					// REVIEW(benluelo): Make this one operation? (Would require a change in the
					// fNFT interface)
					let new_fnft_instance_id =
						T::FinancialNft::get_next_nft_id(fnft_collection_id)?;
					T::FinancialNft::mint_into(
						&rewards_pool.fnft_asset_id,
						&new_fnft_instance_id,
						who,
					)?;

					let old_fnft_asset_account =
						T::FinancialNft::asset_account(fnft_collection_id, &old_fnft_instance_id);
					let new_fnft_asset_account =
						T::FinancialNft::asset_account(fnft_collection_id, &new_fnft_instance_id);

					// staked asset
					Self::split_lock(
						old_position.staked_asset_id,
						&old_fnft_asset_account,
						&new_fnft_asset_account,
						old_position.amount,
						new_stake,
					)?;

					// share asset (x-token)
					Self::split_lock(
						rewards_pool.share_asset_id,
						&old_fnft_asset_account,
						&new_fnft_asset_account,
						old_position.amount,
						new_stake,
					)?;

					Self::deposit_event(Event::<T>::SplitPosition {
						stake: new_stake,
						fnft_collection_id: rewards_pool.fnft_asset_id,
						fnft_instance_id: new_fnft_instance_id,
					});

					Ok::<_, DispatchError>((
						new_fnft_instance_id,
						Stake {
							amount: new_stake,
							share: new_share,
							reductions: new_reductions,
							staked_asset_id: old_position.staked_asset_id,
							lock: old_position.lock.clone(),
						},
					))
				},
			)?;

			Stakes::<T>::insert(&fnft_collection_id, &new_fnft_instance_id, &new_position);

			Ok((*fnft_collection_id, new_fnft_instance_id))
		}

		#[transactional]
		fn claim(
			who: &Self::AccountId,
			(fnft_collection_id, fnft_instance_id): &Self::PositionId,
		) -> DispatchResult {
			Stakes::<T>::try_mutate(fnft_collection_id, fnft_instance_id, |stake| {
				let stake = stake.as_mut().ok_or(Error::<T>::StakeNotFound)?;
				RewardPools::<T>::try_mutate(stake.staked_asset_id, |rewards_pool| {
					let rewards_pool =
						rewards_pool.as_mut().ok_or(Error::<T>::RewardsPoolNotFound)?;

					let owner = T::FinancialNft::owner(fnft_collection_id, fnft_instance_id)
						.ok_or(Error::<T>::FnftNotFound)?;

					Self::collect_rewards(
						rewards_pool,
						stake,
						&owner,
						false, // penalize_for_early_unlock
						false, // keep_alive
					)
				})
			})?;

			Self::deposit_event(Event::<T>::Claimed {
				owner: who.clone(),
				fnft_collection_id: *fnft_collection_id,
				fnft_instance_id: *fnft_instance_id,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn split_lock(
			asset_id: T::AssetId,
			old_fnft_asset_account: &T::AccountId,
			new_fnft_asset_account: &T::AccountId,
			old_account_amount: T::Balance,
			new_account_amount: T::Balance,
		) -> DispatchResult {
			// set the lock on the old account to be the amount in the position
			T::Assets::set_lock(
				T::LockId::get(),
				asset_id,
				&old_fnft_asset_account,
				old_account_amount,
			)?;

			dbg!(asset_id);

			// transfer the amount in the new position from the old account to the new account (this
			// should be the total unlocked amount)
			T::Assets::transfer(
				asset_id,
				&old_fnft_asset_account,
				&new_fnft_asset_account,
				new_account_amount,
				false, // not a user account, doesn't need to be kept alive
			)?;

			dbg!();

			// lock assets on new account
			T::Assets::set_lock(
				T::LockId::get(),
				asset_id,
				new_fnft_asset_account,
				new_account_amount,
			)?;

			Ok(())
		}

		/// Transfers the rewards a staker has earned while updating the provided `rewards_pool`.
		///
		/// # Params
		/// * `pool_id` - Pool identifier
		/// * `mut rewards_pool` - Rewards pool to update
		/// * `stake` - Stake position
		/// * `early_unlock` - If there should be an early unlock penalty
		/// * `keep_alive` - If the transaction should be kept alive
		pub(crate) fn collect_rewards(
			rewards_pool: &mut RewardPoolOf<T>,
			stake: &mut StakeOf<T>,
			owner: &T::AccountId,
			penalize_for_early_unlock: bool,
			// REVIEW(benluelo): Is this parameter necessary? The only accounts that this affects
			// are pallet sub accounts.
			keep_alive: bool,
		) -> Result<(), DispatchError> {
			for (asset_id, reward) in &mut rewards_pool.rewards {
				let rate_based_reward = match reward {
					Reward::ProtocolDistribution() => continue,
					Reward::RateBased(ref mut rate_based_reward) => rate_based_reward,
				};

				// REVIEW(benluelo): As far as I can tell, stake.reductions should always contain
				// all of the assets in the pool - review whether this is the case and if so,
				// possible ways to verify it
				let inflation = stake.reductions.get(asset_id).cloned().unwrap_or_else(Zero::zero);
				let claim = if rewards_pool.total_shares.is_zero() {
					Zero::zero()
				} else {
					rate_based_reward
						.total_rewards
						.safe_mul(&stake.share)?
						.safe_div(&rewards_pool.total_shares)?
						.safe_sub(&inflation)?
				};
				let claim = if penalize_for_early_unlock {
					(Perbill::one() - stake.lock.unlock_penalty).mul_ceil(claim)
				} else {
					claim
				};
				let claim = sp_std::cmp::min(
					claim,
					rate_based_reward.total_rewards.safe_sub(&rate_based_reward.claimed_rewards)?,
				);

				dbg!(claim);

				rate_based_reward.claimed_rewards =
					rate_based_reward.claimed_rewards.safe_add(&claim)?;

				if let Some(inflation) = stake.reductions.get_mut(asset_id) {
					*inflation += claim;
				}

				T::Assets::transfer(
					*asset_id,
					&Self::pool_account_id(&stake.staked_asset_id),
					owner,
					claim,
					keep_alive,
				)?;
			}

			Ok(())
		}

		pub(crate) fn pool_account_id(pool_id: &T::AssetId) -> T::AccountId {
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

		pub(crate) fn reward_accumulation_hook_rate_based_reward_update_calculation(
			pool_id: T::AssetId,
			reward_asset_id: T::AssetId,
			reward: &mut RateBasedReward<T::Balance>,
			now_seconds: u64,
		) {
			use RewardAccumulationCalculationError::*;

			let pool_account = Self::pool_account_id(&pool_id);

			match do_rate_based_reward_accumulation::<T>(
				reward_asset_id,
				reward,
				&pool_account,
				now_seconds,
			) {
				Ok(()) => {},
				Err(BackToTheFuture) => {
					Self::deposit_event(Event::<T>::RewardAccumulationHookError {
						pool_id,
						asset_id: reward_asset_id,
						error: RewardAccumulationHookError::BackToTheFuture,
					});
				},
				Err(RewardsPotEmpty) => {
					if !RewardsPotIsEmpty::<T>::contains_key(pool_id, reward_asset_id) {
						RewardsPotIsEmpty::<T>::insert(pool_id, reward_asset_id, ());
						Self::deposit_event(Event::<T>::RewardAccumulationHookError {
							pool_id,
							asset_id: reward_asset_id,
							error: RewardAccumulationHookError::RewardsPotEmpty,
						});
					}
				},
				Err(MaxRewardsAccumulated) => {
					Self::deposit_event(Event::<T>::MaxRewardsAccumulated {
						pool_id,
						asset_id: reward_asset_id,
					});
				},
				Err(MaxRewardsAccumulatedPreviously) => {
					// max rewards were accumulated previously, silently continue since the
					// MaxRewardsAccumulated event has already been emitted for this pool
				},
			}
		}

		pub(crate) fn accumulate_rewards_hook() -> Weight {
			let now_seconds = T::UnixTime::now().as_secs();
			let unix_time_now_weight = T::WeightInfo::unix_time_now();

			let mut total_weight = unix_time_now_weight;

			RewardPools::<T>::translate(|pool_id, mut reward_pool: RewardPoolOf<T>| {
				for (reward_asset_id, reward) in &mut reward_pool.rewards {
					match reward {
						Reward::ProtocolDistribution() => continue,
						Reward::RateBased(rate_based_reward) => {
							total_weight +=
								T::WeightInfo::reward_accumulation_hook_reward_update_calculation();
							Self::reward_accumulation_hook_rate_based_reward_update_calculation(
								pool_id,
								*reward_asset_id,
								rate_based_reward,
								now_seconds,
							);
						},
					}
				}

				// NOTE: `StorageMap::translate` does one read and one write per item
				total_weight += T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1);

				Some(reward_pool)
			});

			total_weight
		}
	}

	impl<T: Config> ProtocolStaking for Pallet<T> {
		type AssetId = T::AssetId;
		type AccountId = T::AccountId;
		type RewardPoolId = T::AssetId;
		type Balance = T::Balance;

		// REVIEW(benluelo): Review the difference between transfer_earning and
		// add_to_rewards_pot, and if we still need both of them.
		#[transactional]
		fn transfer_protocol_distribution(
			from: &Self::AccountId,
			pool_id: &Self::RewardPoolId,
			reward_currency: Self::AssetId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			RewardPools::<T>::try_mutate(pool_id, |maybe_reward_pool| {
				let reward_pool =
					maybe_reward_pool.as_mut().ok_or(Error::<T>::RewardsPoolNotFound)?;

				let pool_account_id = Self::pool_account_id(pool_id);

				let do_transfer = || {
					T::Assets::transfer(reward_currency, from, &pool_account_id, amount, keep_alive)
				};

				// this is basically just the entry api for std's map types, i.e. modify or insert
				match reward_pool.rewards.get_mut(&reward_currency) {
					Some(reward) => match reward {
						Reward::ProtocolDistribution() => {
							// asset already exists as an earnings reward
							do_transfer()?;
						},
						Reward::RateBased(_rate_based_reward) => {
							do_transfer()?;
							// REVIEW(benluelo): Should this be put on hold? (See comment on
							// enclosing function)
							T::Assets::hold(reward_currency, &pool_account_id, amount)?;

							RewardsPotIsEmpty::<T>::remove(pool_id, reward_currency);
						},
					},
					None => {
						// add the asset as a new earnings reward
						//
						// REVIEW(benluelo): Is this a possible attack vector? Since there is a
						// limit on the amount of assets that can be stored per pool, a third party
						// could throoretically "fill up" the assets and prevent the addition of
						// other rewards (both by the pool owner and other third parties).
						// REVIEW(benluelo): Should we add to the reductions of all of the stakes in
						// the pool here?
						reward_pool
							.rewards
							.try_insert(reward_currency, Reward::ProtocolDistribution())
							.map_err(|_| Error::<T>::TooManyRewardAssetTypes)?;

						do_transfer()?;
					},
				}

				Self::deposit_event(Event::RewardTransferred {
					from: from.clone(),
					pool_id: *pool_id,
					reward_currency,
					reward_increment: amount,
				});
				Ok(())
			})
		}
	}
}

#[transactional]
fn add_to_rewards_pot<T: Config>(
	who: T::AccountId,
	pool_id: T::AssetId,
	asset_id: T::AssetId,
	amount: T::Balance,
	keep_alive: bool,
) -> DispatchResult {
	RewardPools::<T>::get(pool_id)
		.ok_or(Error::<T>::RewardsPoolNotFound)?
		.rewards
		.get(&asset_id)
		.ok_or(Error::<T>::RewardAssetNotFound)?;

	let pool_account = Pallet::<T>::pool_account_id(&pool_id);

	T::Assets::transfer(asset_id, &who, &pool_account, amount, keep_alive)?;
	T::Assets::hold(asset_id, &pool_account, amount)?;

	RewardsPotIsEmpty::<T>::remove(pool_id, asset_id);

	Pallet::<T>::deposit_event(Event::<T>::RewardsPotIncreased { pool_id, asset_id, amount });

	Ok(())
}

#[transactional]
fn update_rewards_pool<T: Config>(
	pool_id: T::AssetId,
	reward_updates: BoundedBTreeMap<
		AssetIdOf<T>,
		RewardUpdate<BalanceOf<T>>,
		T::MaxRewardConfigsPerPool,
	>,
) -> DispatchResult {
	RewardPools::<T>::try_mutate(pool_id, |pool| {
		let pool = pool.as_mut().ok_or(Error::<T>::RewardsPoolNotFound)?;

		let pool_account = Pallet::<T>::pool_account_id(&pool_id);

		let now_seconds = T::UnixTime::now().as_secs();

		for (asset_id, update) in reward_updates {
			let reward = pool.rewards.get_mut(&asset_id).ok_or(Error::<T>::RewardAssetNotFound)?;

			match reward {
				Reward::ProtocolDistribution() => continue,
				Reward::RateBased(rate_based_reward) => {
					match do_rate_based_reward_accumulation::<T>(
						asset_id,
						rate_based_reward,
						&pool_account,
						now_seconds,
					) {
						Ok(()) => {},
						Err(RewardAccumulationCalculationError::BackToTheFuture) =>
							return Err(Error::<T>::BackToTheFuture.into()),
						Err(RewardAccumulationCalculationError::MaxRewardsAccumulated) => {
							Pallet::<T>::deposit_event(Event::<T>::MaxRewardsAccumulated {
								pool_id,
								asset_id,
							});
							continue
						},
						Err(
							RewardAccumulationCalculationError::MaxRewardsAccumulatedPreviously,
						) => continue,
						Err(RewardAccumulationCalculationError::RewardsPotEmpty) =>
							return Err(Error::<T>::RewardsPotEmpty.into()),
					}

					rate_based_reward.reward_rate = update.reward_rate;
				},
			}
		}

		Pallet::<T>::deposit_event(Event::<T>::RewardPoolUpdated { pool_id });

		Ok(())
	})
}

/// Calculates the update to the reward and unlocks the accumulated rewards from the pool account.
pub(crate) fn do_rate_based_reward_accumulation<T: Config>(
	asset_id: T::AssetId,
	reward: &mut RateBasedReward<T::Balance>,
	pool_account: &T::AccountId,
	now_seconds: u64,
) -> Result<(), RewardAccumulationCalculationError> {
	if reward.reward_rate.amount.is_zero() {
		return Ok(())
	}

	let elapsed_time = now_seconds
		.safe_sub(&reward.last_updated_timestamp)
		.map_err(|_| RewardAccumulationCalculationError::BackToTheFuture)?;

	let reward_rate_period_seconds = reward.reward_rate.period.as_secs();

	// SAFETY(benluelo): Usage of Div::div: Integer division can only fail if rhs == 0, and
	// reward_rate_period_seconds is a NonZeroU64 here
	let periods_surpassed = elapsed_time.div(reward_rate_period_seconds.get());

	if periods_surpassed.is_zero() {
		Ok(())
	} else {
		let total_locked_rewards: u128 = T::Assets::balance_on_hold(asset_id, pool_account).into();

		// the maximum amount repayable given the reward rate.
		// i.e. if total locked is 50, and the reward rate is 15, then this would be 3
		//
		// SAFETY(benluelo): Usage of Div::div: Integer division can only fail if rhs == 0, and
		// reward.reward_rate.amount is known to be non-zero here as per the check at the beginning
		// of this function
		let maximum_releasable_periods = total_locked_rewards.div(reward.reward_rate.amount.into());

		let releasable_periods_surpassed =
			cmp::min(maximum_releasable_periods, periods_surpassed.into());

		// saturating is safe here since these values are checked against max_rewards anyways, which
		// is <= u128::MAX
		let newly_accumulated_rewards =
			u128::saturating_mul(releasable_periods_surpassed, reward.reward_rate.amount.into());
		let new_total_rewards =
			newly_accumulated_rewards.saturating_add(reward.total_rewards.into());

		// u64::MAX is roughly 584.9 billion years in the future, so saturating at that should be ok
		let last_updated_timestamp = reward.last_updated_timestamp.saturating_add(
			reward_rate_period_seconds
				.get()
				.saturating_mul(releasable_periods_surpassed.saturated_into::<u64>()),
		);

		// TODO(benluelo): This can probably be simplified, review the period calculations
		if u128::saturating_add(new_total_rewards, reward.total_dilution_adjustment.into()) <=
			reward.max_rewards.into()
		{
			let balance_on_hold = T::Assets::balance_on_hold(asset_id, pool_account);
			if !balance_on_hold.is_zero() && balance_on_hold >= newly_accumulated_rewards.into() {
				T::Assets::release(
					asset_id,
					pool_account,
					newly_accumulated_rewards.into(),
					false, // not best effort, entire amount must be released
				)
				.expect("funds should be available to release based on previous check; qed;");

				reward.total_rewards = new_total_rewards.into();
				reward.last_updated_timestamp = last_updated_timestamp;
				Ok(())
			} else {
				Err(RewardAccumulationCalculationError::RewardsPotEmpty)
			}
		} else if reward.total_rewards.saturating_add(reward.total_dilution_adjustment) <
			reward.max_rewards
		{
			// if the new total rewards are less than or equal to the max rewards AND the current
			// total rewards are less than the max rewards (i.e. the newly accumulated rewards is
			// less than the the amount that would be accumulated based on the periods surpassed),
			// then release *up to* the max rewards

			// REVIEW(benluelo): Should max_rewards be max_periods instead? Currently, the
			// max_rewards isn't updatable, and once the max_rewards is hit, it's expected that no
			// more rewards will be accumulated, so it's ok to not reward an entire period's worth
			// of rewards. Review this if the max_rewards ever becomes updateable in the future.

			// SAFETY(benluelo): Usage of Sub::sub: reward.total_rewards is known to be less than
			// reward.max_rewards as per check above
			let rewards_to_release = reward.max_rewards.sub(reward.total_rewards).into();

			let balance_on_hold = T::Assets::balance_on_hold(asset_id, pool_account);
			if !balance_on_hold.is_zero() && balance_on_hold >= newly_accumulated_rewards.into() {
				T::Assets::release(
					asset_id,
					pool_account,
					rewards_to_release.into(),
					false, // not best effort, entire amount must be released
				)
				.expect("funds should be available to release based on previous check; qed;");

				// return an error, but update the reward first
				reward.total_rewards = reward.max_rewards;
				reward.last_updated_timestamp = last_updated_timestamp;
				Err(RewardAccumulationCalculationError::MaxRewardsAccumulated)
			} else {
				Err(RewardAccumulationCalculationError::RewardsPotEmpty)
			}
		} else {
			// at this point, reward.total_rewards is known to be equal to max_rewards which means
			// that the max rewards was hit previously
			Err(RewardAccumulationCalculationError::MaxRewardsAccumulatedPreviously)
		}
	}
}

pub(crate) enum RewardAccumulationCalculationError {
	/// T::UnixTime::now() returned a value in the past.
	BackToTheFuture,
	/// The `max_rewards` for this reward was hit during this calculation.
	MaxRewardsAccumulated,
	/// The `max_rewards` were hit previously; i.e. `total_rewards == max_rewards` at the start of
	/// this calculation.
	MaxRewardsAccumulatedPreviously,
	/// The rewards pot (held balance) for this pool is empty or doesn't have enough held balance
	/// to release for the rewards accumulated.
	RewardsPotEmpty,
}
