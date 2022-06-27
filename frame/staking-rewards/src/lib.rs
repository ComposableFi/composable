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

mod prelude;
#[cfg(test)]
mod test;
#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use composable_support::{
		abstractions::{
			nonce::Nonce,
			utils::{
				increment::{Increment, SafeIncrement},
				start_at::ZeroInit,
			},
		},
		math::safe::SafeArithmetic,
	};
	use composable_traits::{
		currency::{BalanceLike, CurrencyFactory},
		staking::RewardPoolConfiguration::RewardRateBasedIncentive,
		time::DurationSeconds,
	};
	use frame_support::{
		pallet_prelude::*, traits::UnixTime, transactional, BoundedBTreeMap, PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{traits::One, Permill};
	use sp_runtime::traits::BlockNumberProvider;
	use sp_std::collections::btree_map::BTreeMap;
	use std::fmt::Debug;

	use crate::{prelude::*, weights::WeightInfo};

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
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error when creating reward configs.
		RewardConfigProblem,
		/// Invalid end block number provided for creating a pool.
		InvalidEndBlock,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The reward balance type.
		type Balance: Parameter + Member + BalanceLike + FixedPointOperand + From<u128> + Into<u128>;

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
		type PositionId: Parameter + Member + Clone + FullCodec + Zero;

		type AssetId: Parameter
			+ Member
			+ AssetIdLike
			+ MaybeSerializeDeserialize
			+ Ord
			+ From<u128>
			+ Into<u128>;

		/// Is used to create staked asset per `Self::RewardPoolId`
		type CurrencyFactory: CurrencyFactory<Self::AssetId, Self::Balance>;

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
						Error::<T>::InvalidEndBlock
					);
					let pool_id = RewardPoolCount::<T>::increment()?;
					let mut rewards = BTreeMap::new();
					for (i, reward_config) in initial_reward_config.into_iter().enumerate() {
						rewards.insert(reward_config.0, Reward::from(reward_config.1));
					}
					RewardPools::<T>::insert(
						pool_id.clone(),
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
							end_block: end_block.clone(),
							lock,
						},
					);
					(owner, pool_id, end_block)
				},
			};
			Self::deposit_event(Event::<T>::RewardPoolCreated { pool_id, owner, end_block });
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
			Err("Not implemented".into())
		}

		#[transactional]
		fn extend(
			who: &Self::AccountId,
			position: Self::PositionId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::PositionId, DispatchError> {
			Err("Not implemented".into())
		}

		#[transactional]
		fn unstake(
			who: &Self::AccountId,
			position: &Self::PositionId,
			remove_amount: Self::Balance,
		) -> DispatchResult {
			Err("Not implemented".into())
		}

		#[transactional]
		fn split(
			who: &Self::AccountId,
			position: &Self::PositionId,
			ratio: Permill,
		) -> Result<[Self::PositionId; 2], DispatchError> {
			Err("Not implemented".into())
		}
	}
}
