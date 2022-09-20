#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	no_mangle_generic_items,
	non_shorthand_field_patterns,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	trivial_casts,
	trivial_numeric_casts,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_extern_crates,
	unused_parens,
	while_true,
	clippy::unseparated_literal_suffix
)]
#![cfg_attr(
	test,
	allow(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::identity_op,
		clippy::indexing_slicing,
		clippy::panic,
		clippy::todo,
		clippy::unwrap_used,
	)
)]
#![allow(dead_code)] // TODO: remove when most of the work is completed.

pub use pallet::*;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(test)]
mod common_test_functions;
#[cfg(test)]
mod liquidity_bootstrapping_tests;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod mock_fnft;
#[cfg(test)]
mod stable_swap_tests;
#[cfg(test)]
mod uniswap_tests;

pub mod weights;

mod liquidity_bootstrapping;
mod stable_swap;
mod twap;
mod types;
mod uniswap;

pub use crate::weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		stable_swap::StableSwap,
		twap::{update_price_cumulative_state, update_twap_state},
		types::{PriceCumulative, TimeWeightedAveragePrice},
		uniswap::Uniswap,
		WeightInfo,
	};
	use codec::FullCodec;
	use composable_support::math::safe::{
		safe_multiply_by_rational, SafeAdd, SafeArithmetic, SafeDiv, SafeSub,
	};
	use composable_traits::{
		currency::{CurrencyFactory, LocalAssets},
		defi::{CurrencyPair, Rate},
		dex::{
			Amm, ConstantProductPoolInfo, Fee, LiquidityBootstrappingPoolInfo, PriceAggregate,
			RedeemableAssets, RemoveLiquiditySimulationResult, RewardPoolType,
			SingleAssetAccountsStorageAction, StableSwapPoolInfo, StakingRewardPool, MAX_REWARDS,
		},
		staking::{
			lock::LockConfig, ManageStaking, ProtocolStaking, RewardConfig,
			RewardPoolConfiguration, RewardRate,
		},
		time::{ONE_MONTH, ONE_WEEK},
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			Time, TryCollect,
		},
		transactional, BoundedBTreeMap, PalletId, RuntimeDebug,
	};

	use crate::liquidity_bootstrapping::LiquidityBootstrapping;
	use composable_maths::dex::{
		constant_product::{
			compute_asset_for_redeemable_lp_tokens, compute_deposit_lp,
			compute_deposit_lp_single_asset,
		},
		price::compute_initial_price_cumulative,
	};
	use composable_support::validation::Validated;
	use composable_traits::{currency::BalanceLike, dex::FeeConfig};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use sp_runtime::{
		traits::{AccountIdConversion, BlockNumberProvider, Convert, One, Zero},
		ArithmeticError, FixedPointNumber, Perbill, Permill,
	};
	use sp_std::{collections::btree_map::BTreeMap, vec, vec::Vec};

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
	pub enum PoolInitConfiguration<AccountId, AssetId, BlockNumber> {
		StableSwap {
			owner: AccountId,
			pair: CurrencyPair<AssetId>,
			amplification_coefficient: u16,
			// trading fee
			fee: Permill,
		},
		ConstantProduct {
			owner: AccountId,
			pair: CurrencyPair<AssetId>,
			// trading fee
			fee: Permill,
			base_weight: Permill,
		},
		LiquidityBootstrapping(LiquidityBootstrappingPoolInfo<AccountId, AssetId, BlockNumber>),
	}

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
	pub enum PoolConfiguration<AccountId, AssetId, BlockNumber> {
		StableSwap(StableSwapPoolInfo<AccountId, AssetId>),
		ConstantProduct(ConstantProductPoolInfo<AccountId, AssetId>),
		LiquidityBootstrapping(LiquidityBootstrappingPoolInfo<AccountId, AssetId, BlockNumber>),
	}

	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type LiquidityBootstrappingPoolInfoOf<T> = LiquidityBootstrappingPoolInfo<
		<T as frame_system::Config>::AccountId,
		<T as Config>::AssetId,
		<T as frame_system::Config>::BlockNumber,
	>;
	type PoolIdOf<T> = <T as Config>::PoolId;
	type PoolConfigurationOf<T> = PoolConfiguration<
		<T as frame_system::Config>::AccountId,
		<T as Config>::AssetId,
		<T as frame_system::Config>::BlockNumber,
	>;
	pub(crate) type PoolInitConfigurationOf<T> = PoolInitConfiguration<
		<T as frame_system::Config>::AccountId,
		<T as Config>::AssetId,
		<T as frame_system::Config>::BlockNumber,
	>;

	type StakingRewardPoolsOf<T> = BoundedVec<
		StakingRewardPool<<T as Config>::RewardPoolId>,
		<T as Config>::MaxStakingRewardPools,
	>;
	type RewardConfigsOf<T> = BoundedBTreeMap<
		<T as Config>::AssetId,
		RewardConfig<<T as Config>::AssetId, <T as Config>::Balance>,
		<T as Config>::MaxRewardConfigsPerPool,
	>;
	pub(crate) type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
	pub(crate) type TWAPStateOf<T> = TimeWeightedAveragePrice<MomentOf<T>, <T as Config>::Balance>;
	pub(crate) type PriceCumulativeStateOf<T> =
		PriceCumulative<MomentOf<T>, <T as Config>::Balance>;

	type DurationPresets<T> =
		BoundedBTreeMap<u64, Perbill, <T as Config>::MaxStakingDurationPresets>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
		PoolCreated {
			/// Id of newly created pool.
			pool_id: T::PoolId,
			/// Owner of the pool.
			owner: T::AccountId,
			// Pool assets
			assets: CurrencyPair<AssetIdOf<T>>,
		},
		/// The sale ended, the funds repatriated and the pool deleted.
		PoolDeleted {
			/// Pool that was removed.
			pool_id: T::PoolId,
			/// Amount of base asset repatriated.
			base_amount: T::Balance,
			/// Amount of quote asset repatriated.
			quote_amount: T::Balance,
		},

		/// Liquidity added into the pool `T::PoolId`.
		LiquidityAdded {
			/// Account id who added liquidity.
			who: T::AccountId,
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Amount of base asset deposited.
			base_amount: T::Balance,
			/// Amount of quote asset deposited.
			quote_amount: T::Balance,
			/// Amount of minted lp.
			minted_lp: T::Balance,
		},
		/// Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
		LiquidityRemoved {
			/// Account id who removed liquidity.
			who: T::AccountId,
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Amount of base asset removed from pool.
			base_amount: T::Balance,
			/// Amount of quote asset removed from pool.
			quote_amount: T::Balance,
			/// Updated lp token supply.
			total_issuance: T::Balance,
		},
		/// Token exchange happened.
		Swapped {
			/// Pool id on which exchange done.
			pool_id: T::PoolId,
			/// Account id who exchanged token.
			who: T::AccountId,
			/// Id of asset used as input.
			base_asset: T::AssetId,
			/// Id of asset used as output.
			quote_asset: T::AssetId,
			/// Amount of base asset received.
			base_amount: T::Balance,
			/// Amount of quote asset provided.
			quote_amount: T::Balance,
			/// Charged fees.
			fee: Fee<T::AssetId, T::Balance>,
		},
		/// TWAP updated.
		TwapUpdated {
			/// Pool id on which exchange done.
			pool_id: T::PoolId,
			/// TWAP Timestamp
			timestamp: MomentOf<T>,
			/// Map of asset_id -> twap
			twaps: BTreeMap<T::AssetId, Rate>,
		},
		/// Balance of assets for withdrawing with single coin updated.
		SingleAssetAccountsStorageUpdated {
			/// Account of user.
			who: T::AccountId,
			/// Pool id on which user deposit/withdraw.
			pool_id: T::PoolId,
			/// Available amount of LP.
			balance: T::Balance,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		PoolNotFound,
		NotEnoughLiquidity,
		NotEnoughLpToken,
		PairMismatch,
		MustBeOwner,
		InvalidSaleState,
		InvalidAmount,
		InvalidAsset,
		CannotRespectMinimumRequested,
		AssetAmountMustBePositiveNumber,
		InvalidPair,
		InvalidFees,
		AmpFactorMustBeGreaterThanZero,
		MissingAmount,
		MissingMinExpectedAmount,
		MoreThanTwoAssetsNotYetSupported,
		SingleAssetNotYetSupported,
		NoLpTokenForLbp,
		WeightsMustBeNonZero,
		WeightsMustSumToOne,
		StakingPoolConfigError,
		NotEnoughLpTokenForSingleAssetWithdraw,
		LpTokenNotFoundForSingleAssetWithdrawal,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Type representing the unique ID of an asset.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ Clone
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ From<u128>
			+ Ord;

		/// Type representing the Balance of an account.
		type Balance: BalanceLike + SafeSub;

		/// An isomorphism: Balance<->u128
		type Convert: Convert<u128, BalanceOf<Self>> + Convert<BalanceOf<Self>, u128>;

		/// Factory to create new lp-token.
		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId, Self::Balance>;

		/// Dependency allowing this pallet to transfer funds from one account to another.
		type Assets: Transfer<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Mutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Inspect<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

		/// Type representing the unique ID of a pool.
		type PoolId: FullCodec
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

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Used for spot price calculation for LBP
		type LocalAssets: LocalAssets<AssetIdOf<Self>>;

		/// Minimum duration for a sale.
		#[pallet::constant]
		type LbpMinSaleDuration: Get<BlockNumberFor<Self>>;

		/// Maximum duration for a sale.
		#[pallet::constant]
		type LbpMaxSaleDuration: Get<BlockNumberFor<Self>>;

		/// Maximum initial weight.
		#[pallet::constant]
		type LbpMaxInitialWeight: Get<Permill>;

		/// Minimum final weight.
		#[pallet::constant]
		type LbpMinFinalWeight: Get<Permill>;

		/// Required origin for pool creation.
		type PoolCreationOrigin: EnsureOrigin<Self::Origin>;

		/// Required origin to enable TWAP on pool.
		type EnableTwapOrigin: EnsureOrigin<Self::Origin>;

		/// Time provider.
		type Time: Time;

		/// The interval between TWAP computations.
		#[pallet::constant]
		type TWAPInterval: Get<MomentOf<Self>>;

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

		type MaxStakingRewardPools: Get<u32>;

		type MaxRewardConfigsPerPool: Get<u32>;

		type MaxStakingDurationPresets: Get<u32>;

		type ManageStaking: ManageStaking<
			AccountId = AccountIdOf<Self>,
			AssetId = <Self as Config>::AssetId,
			BlockNumber = <Self as frame_system::Config>::BlockNumber,
			Balance = Self::Balance,
			RewardConfigsLimit = Self::MaxRewardConfigsPerPool,
			StakingDurationPresetsLimit = Self::MaxStakingDurationPresets,
			RewardPoolId = Self::RewardPoolId,
		>;

		type ProtocolStaking: ProtocolStaking<
			AccountId = AccountIdOf<Self>,
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			RewardPoolId = Self::RewardPoolId,
		>;

		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type MsPerBlock: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
	pub fn PoolCountOnEmpty<T: Config>() -> T::PoolId {
		Zero::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	#[allow(clippy::disallowed_types)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery, PoolCountOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolConfigurationOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn twap)]
	#[pallet::unbounded]
	pub type TWAPState<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, TWAPStateOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn price_cumulative)]
	#[pallet::unbounded]
	pub type PriceCumulativeState<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, PriceCumulativeStateOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn staking_reward_pools)]
	pub type StakingRewardPools<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, StakingRewardPoolsOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn accounts)]
	#[pallet::unbounded]
	pub type SingleAssetAccountsStorage<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::PoolId,
		T::Balance,
		OptionQuery,
	>;

	pub(crate) enum PriceRatio {
		Swapped,
		NotSwapped,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool.
		///
		/// Emits `PoolCreated` event when successful.
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(origin: OriginFor<T>, pool: PoolInitConfigurationOf<T>) -> DispatchResult {
			T::PoolCreationOrigin::ensure_origin(origin)?;
			let _ = Self::do_create_pool(pool)?;
			Ok(())
		}

		/// Execute a buy order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::buy())]
		pub fn buy(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			asset_id: T::AssetId,
			amount: T::Balance,
			min_receive: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::buy(&who, pool_id, asset_id, amount, min_receive, keep_alive)?;
			Ok(())
		}

		/// Execute a sell order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::sell())]
		pub fn sell(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			asset_id: T::AssetId,
			amount: T::Balance,
			min_receive: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::sell(&who, pool_id, asset_id, amount, min_receive, keep_alive)?;
			Ok(())
		}

		/// Execute a specific swap operation.
		///
		/// The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::swap())]
		pub fn swap(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			pair: CurrencyPair<T::AssetId>,
			quote_amount: T::Balance,
			min_receive: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::exchange(
				&who,
				pool_id,
				pair,
				quote_amount,
				min_receive,
				keep_alive,
			)?;
			Ok(())
		}

		/// Add liquidity to the given pool.
		///
		/// Emits `LiquidityAdded` event when successful.
		#[pallet::weight(T::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			base_amount: T::Balance,
			quote_amount: T::Balance,
			min_mint_amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::add_liquidity(
				&who,
				pool_id,
				base_amount,
				quote_amount,
				min_mint_amount,
				keep_alive,
			)?;
			Ok(())
		}

		/// Remove liquidity from the given pool.
		///
		/// Emits `LiquidityRemoved` event when successful.
		#[pallet::weight(T::WeightInfo::remove_liquidity())]
		pub fn remove_liquidity(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			lp_amount: T::Balance,
			min_base_amount: T::Balance,
			min_quote_amount: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::remove_liquidity(
				&who,
				pool_id,
				lp_amount,
				min_base_amount,
				min_quote_amount,
			)?;
			Ok(())
		}

		/// Remove liquidity from the given pool with a single asset.
		///
		/// Emits `LiquidityRemoved` event when successful.
		#[pallet::weight(T::WeightInfo::remove_liquidity_single_asset())]
		pub fn remove_liquidity_single_asset(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			lp_amount: T::Balance,
			min_amount: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				SingleAssetAccountsStorage::<T>::contains_key(&who, &pool_id),
				Error::<T>::LpTokenNotFoundForSingleAssetWithdrawal
			);
			<Self as Amm>::remove_liquidity_single_asset(&who, pool_id, lp_amount, min_amount)?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn enable_twap(origin: OriginFor<T>, pool_id: T::PoolId) -> DispatchResult {
			T::EnableTwapOrigin::ensure_origin(origin)?;
			if TWAPState::<T>::contains_key(pool_id) {
				// pool_id is already enabled for TWAP
				return Ok(())
			}
			let current_timestamp = T::Time::now();
			let rate_base = Self::do_get_exchange_rate(pool_id, PriceRatio::NotSwapped)?;
			let rate_quote = Self::do_get_exchange_rate(pool_id, PriceRatio::Swapped)?;
			let base_price_cumulative =
				compute_initial_price_cumulative::<T::Convert, _>(rate_base)?;
			let quote_price_cumulative =
				compute_initial_price_cumulative::<T::Convert, _>(rate_quote)?;
			TWAPState::<T>::insert(
				pool_id,
				TimeWeightedAveragePrice {
					base_price_cumulative,
					quote_price_cumulative,
					timestamp: current_timestamp,
					base_twap: rate_base,
					quote_twap: rate_quote,
				},
			);
			PriceCumulativeState::<T>::insert(
				pool_id,
				PriceCumulative {
					timestamp: current_timestamp,
					base_price_cumulative,
					quote_price_cumulative,
				},
			);
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(_block_number: T::BlockNumber) -> Weight {
			let mut weight: Weight = 0;
			let twap_enabled_pools: Vec<T::PoolId> =
				PriceCumulativeState::<T>::iter_keys().collect();
			for pool_id in twap_enabled_pools {
				let result = PriceCumulativeState::<T>::try_mutate(
					pool_id,
					|prev_price_cumulative| -> Result<(), DispatchError> {
						let (base_price_cumulative, quote_price_cumulative) =
							update_price_cumulative_state::<T>(pool_id, prev_price_cumulative)?;
						// if update_twap_state fails, return Err() so effect of
						// update_price_cumulative_state is also gets reverted.
						TWAPState::<T>::try_mutate(
							pool_id,
							|prev_twap_state| -> Result<(), DispatchError> {
								update_twap_state::<T>(
									base_price_cumulative,
									quote_price_cumulative,
									prev_twap_state,
								)
							},
						)
					},
				);
				if result.is_ok() {
					weight += 1;
					if let Some(updated_twap) = TWAPState::<T>::get(pool_id) {
						if let Ok(currency_pair) = Self::currency_pair(pool_id) {
							Self::deposit_event(Event::<T>::TwapUpdated {
								pool_id,
								timestamp: updated_twap.timestamp,
								twaps: BTreeMap::from([
									(currency_pair.base, updated_twap.base_twap),
									(currency_pair.quote, updated_twap.quote_twap),
								]),
							});
						}
					}
				}
			}
			weight
		}
	}

	impl<T: Config> Pallet<T> {
		fn default_pblo_staking_pool_config(
			pool_id: &T::PoolId,
			pair: &CurrencyPair<T::AssetId>,
		) -> Result<
			RewardPoolConfiguration<
				T::AccountId,
				T::AssetId,
				T::BlockNumber,
				RewardConfigsOf<T>,
				DurationPresets<T>,
			>,
			DispatchError,
		> {
			let max_rewards: T::Balance = T::Convert::convert(MAX_REWARDS);
			let reward_rate = RewardRate::per_second(T::Convert::convert(0));
			let reward_configs = [
				(
					pair.base,
					RewardConfig {
						asset_id: pair.base,
						max_rewards,
						reward_rate: reward_rate.clone(),
					},
				),
				(pair.quote, RewardConfig { asset_id: pair.quote, max_rewards, reward_rate }),
			]
			.into_iter()
			.try_collect()
			.map_err(|_| Error::<T>::StakingPoolConfigError)?;
			let duration_presets =
				[(ONE_WEEK, Perbill::from_percent(1)), (ONE_MONTH, Perbill::from_percent(10))]
					.into_iter()
					.try_collect()
					.map_err(|_| Error::<T>::StakingPoolConfigError)?;
			let lock = LockConfig { duration_presets, unlock_penalty: Perbill::from_percent(5) };

			let five_years_block = 5 * 365 * 24 * 60 * 60 / T::MsPerBlock::get();
			let end_block =
				frame_system::Pallet::<T>::current_block_number() + five_years_block.into();
			Ok(RewardPoolConfiguration::<_, _, _, _, _>::RewardRateBasedIncentive {
				owner: Self::account_id(pool_id),
				asset_id: primitives::currency::CurrencyId::PBLO.0.into(),
				end_block,
				reward_configs,
				lock,
			})
		}

		fn default_lp_staking_pool_config(
			pool_id: &T::PoolId,
		) -> Result<
			RewardPoolConfiguration<
				T::AccountId,
				T::AssetId,
				T::BlockNumber,
				RewardConfigsOf<T>,
				DurationPresets<T>,
			>,
			DispatchError,
		> {
			let max_rewards: T::Balance = T::Convert::convert(MAX_REWARDS);
			// let reward_rate = Perbill::from_percent(REWARD_PERCENTAGE); not sure how this
			// translates to the new model
			let reward_rate = RewardRate::per_second(T::Convert::convert(0));
			let pblo_asset_id: T::AssetId = primitives::currency::CurrencyId::PBLO.0.into();
			let reward_configs = [(
				pblo_asset_id,
				RewardConfig { asset_id: pblo_asset_id, max_rewards, reward_rate },
			)]
			.into_iter()
			.try_collect()
			.map_err(|_| Error::<T>::StakingPoolConfigError)?;
			let duration_presets =
				[(ONE_WEEK, Perbill::from_percent(1)), (ONE_MONTH, Perbill::from_percent(10))]
					.into_iter()
					.try_collect()
					.map_err(|_| Error::<T>::StakingPoolConfigError)?;
			let lock = LockConfig { duration_presets, unlock_penalty: Perbill::from_percent(5) };
			let five_years_block = 5 * 365 * 24 * 60 * 60 / T::MsPerBlock::get();
			let end_block =
				frame_system::Pallet::<T>::current_block_number() + five_years_block.into();
			Ok(RewardPoolConfiguration::<_, _, _, _, _>::RewardRateBasedIncentive {
				owner: Self::account_id(pool_id),
				asset_id: Self::lp_token(*pool_id)?,
				end_block,
				reward_configs,
				lock,
			})
		}

		#[transactional]
		fn create_staking_reward_pool(
			pool_id: &T::PoolId,
			pair: CurrencyPair<T::AssetId>,
		) -> DispatchResult {
			let pblo_pool_config = Self::default_pblo_staking_pool_config(pool_id, &pair)?;
			let lp_pool_config = Self::default_lp_staking_pool_config(pool_id)?;
			let pblo_staking_pool_id = T::ManageStaking::create_staking_pool(pblo_pool_config)?;
			let lp_staking_pool_id = T::ManageStaking::create_staking_pool(lp_pool_config)?;
			let pblo_staking_pool = StakingRewardPool {
				pool_id: pblo_staking_pool_id,
				pool_type: RewardPoolType::PBLO,
			};
			let lp_staking_pool =
				StakingRewardPool { pool_id: lp_staking_pool_id, pool_type: RewardPoolType::LP };
			let staking_reward_pools =
				BoundedVec::try_from(vec![pblo_staking_pool, lp_staking_pool])
					.map_err(|_| Error::<T>::StakingPoolConfigError)?;

			StakingRewardPools::<T>::insert(pool_id, staking_reward_pools);
			Ok(())
		}

		#[transactional]
		pub fn do_create_pool(
			init_config: PoolInitConfigurationOf<T>,
		) -> Result<T::PoolId, DispatchError> {
			let (owner, pool_id, pair) = match init_config {
				PoolInitConfiguration::StableSwap {
					owner,
					pair,
					amplification_coefficient,
					fee,
				} => {
					let pool_id = StableSwap::<T>::do_create_pool(
						&owner,
						pair,
						amplification_coefficient,
						FeeConfig::default_from(fee),
					)?;
					Self::create_staking_reward_pool(&pool_id, pair)?;
					(owner, pool_id, pair)
				},
				PoolInitConfiguration::ConstantProduct { owner, pair, fee, base_weight } => {
					let pool_id = Uniswap::<T>::do_create_pool(
						&owner,
						pair,
						FeeConfig::default_from(fee),
						base_weight,
					)?;
					Self::create_staking_reward_pool(&pool_id, pair)?;
					(owner, pool_id, pair)
				},
				PoolInitConfiguration::LiquidityBootstrapping(pool_config) => {
					let validated_pool_config =
						Validated::new(pool_config.clone()).map_err(DispatchError::Other)?;
					(
						pool_config.owner,
						LiquidityBootstrapping::<T>::do_create_pool(validated_pool_config)?,
						pool_config.pair,
					)
				},
			};
			Self::deposit_event(Event::<T>::PoolCreated { owner, pool_id, assets: pair });
			Ok(pool_id)
		}

		pub(crate) fn get_pool(
			pool_id: T::PoolId,
		) -> Result<PoolConfigurationOf<T>, DispatchError> {
			Pools::<T>::get(pool_id).ok_or_else(|| Error::<T>::PoolNotFound.into())
		}

		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account_truncating(pool_id)
		}

		pub(crate) fn do_get_exchange_rate(
			pool_id: T::PoolId,
			price_ratio: PriceRatio,
		) -> Result<Rate, DispatchError> {
			let pair = Self::currency_pair(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pair = match price_ratio {
				PriceRatio::NotSwapped => pair,
				PriceRatio::Swapped => pair.swap(),
			};
			let pool_base_asset_under_management =
				T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
			let pool_quote_asset_under_management =
				T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));

			ensure!(
				pool_base_asset_under_management > Zero::zero(),
				Error::<T>::NotEnoughLiquidity
			);
			ensure!(
				pool_quote_asset_under_management > Zero::zero(),
				Error::<T>::NotEnoughLiquidity
			);

			Ok(Rate::checked_from_rational(
				pool_base_asset_under_management,
				pool_quote_asset_under_management,
			)
			.ok_or(ArithmeticError::Overflow)?)
		}

		fn update_twap(pool_id: T::PoolId) -> Result<(), DispatchError> {
			let currency_pair = Self::currency_pair(pool_id)?; // update price cumulative
			let (base_price_cumulative, quote_price_cumulative) =
				PriceCumulativeState::<T>::try_mutate(
					pool_id,
					|prev_price_cumulative| -> Result<(T::Balance, T::Balance), DispatchError> {
						update_price_cumulative_state::<T>(pool_id, prev_price_cumulative)
					},
				)?;
			if base_price_cumulative != T::Balance::zero() &&
				quote_price_cumulative != T::Balance::zero()
			{
				// update TWAP
				let updated_twap = TWAPState::<T>::try_mutate(
					pool_id,
					|prev_twap_state| -> Result<Option<TWAPStateOf<T>>, DispatchError> {
						update_twap_state::<T>(
							base_price_cumulative,
							quote_price_cumulative,
							prev_twap_state,
						)
						.map_or_else(|_| Ok(None), |_| Ok(prev_twap_state.clone()))
					},
				)?;
				if let Some(updated_twap) = updated_twap {
					Self::deposit_event(Event::<T>::TwapUpdated {
						pool_id,
						timestamp: updated_twap.timestamp,
						twaps: BTreeMap::from([
							(currency_pair.base, updated_twap.base_twap),
							(currency_pair.quote, updated_twap.quote_twap),
						]),
					});
				}
				return Ok(())
			}
			Ok(())
		}

		#[transactional]
		fn disburse_fees(
			who: &T::AccountId,
			pool_id: &T::PoolId,
			owner: &T::AccountId,
			fees: &Fee<T::AssetId, T::Balance>,
		) -> Result<(), DispatchError> {
			if !fees.owner_fee.is_zero() {
				T::Assets::transfer(fees.asset_id, who, owner, fees.owner_fee, false)?;
			}
			if !fees.protocol_fee.is_zero() {
				let staking_reward_pools = StakingRewardPools::<T>::get(&pool_id)
					.ok_or(Error::<T>::StakingPoolConfigError)?;
				for staking_reward_pool in staking_reward_pools {
					if staking_reward_pool.pool_type == RewardPoolType::PBLO {
						T::ProtocolStaking::transfer_reward(
							who,
							&staking_reward_pool.pool_id,
							fees.asset_id,
							fees.protocol_fee,
						)?;
					}
				}
			}
			Ok(())
		}
	}

	impl<T: Config> Amm for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		type AccountId = T::AccountId;
		type PoolId = T::PoolId;

		fn pool_exists(pool_id: Self::PoolId) -> bool {
			Pools::<T>::contains_key(pool_id)
		}

		fn get_price_of_lp_token(pool_id: Self::PoolId) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(StableSwapPoolInfo { pair, lp_token, .. }) |
				PoolConfiguration::ConstantProduct(ConstantProductPoolInfo {
					pair,
					lp_token,
					..
				}) => calculate_price_of_lp_token::<T>(pair, lp_token, &pool_account),
				PoolConfiguration::LiquidityBootstrapping(_) =>
					Err(Error::<T>::NoLpTokenForLbp.into()),
			}
		}

		fn currency_pair(
			pool_id: Self::PoolId,
		) -> Result<CurrencyPair<Self::AssetId>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			match pool {
				PoolConfiguration::StableSwap(info) => Ok(info.pair),
				PoolConfiguration::ConstantProduct(info) => Ok(info.pair),
				PoolConfiguration::LiquidityBootstrapping(info) => Ok(info.pair),
			}
		}

		fn lp_token(pool_id: Self::PoolId) -> Result<Self::AssetId, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			match pool {
				PoolConfiguration::StableSwap(info) => Ok(info.lp_token),
				PoolConfiguration::ConstantProduct(info) => Ok(info.lp_token),
				PoolConfiguration::LiquidityBootstrapping(_) =>
					Err(Error::<T>::NoLpTokenForLbp.into()),
			}
		}

		fn simulate_add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let currency_pair = Self::currency_pair(pool_id)?;
			ensure!(amounts.len() < 3, Error::<T>::MoreThanTwoAssetsNotYetSupported);
			let base_amount = *amounts.get(&currency_pair.base).ok_or(Error::<T>::MissingAmount)?;
			let quote_amount =
				*amounts.get(&currency_pair.quote).ok_or(Error::<T>::MissingAmount)?;
			ensure!(
				T::Assets::reducible_balance(currency_pair.base, who, false) >= base_amount,
				Error::<T>::NotEnoughLiquidity
			);
			ensure!(
				T::Assets::reducible_balance(currency_pair.quote, who, false) >= quote_amount,
				Error::<T>::NotEnoughLiquidity
			);

			lp_for_liquidity::<T>(pool, pool_account, base_amount, quote_amount)
		}

		fn update_accounts_deposited_one_asset_storage(
			who: Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			action: SingleAssetAccountsStorageAction,
		) -> Result<(), DispatchError> {
			match action {
				SingleAssetAccountsStorageAction::Depositing => {
					let available_lp = SingleAssetAccountsStorage::<T>::try_mutate_exists(
						&who,
						&pool_id,
						|exist_amount| -> Result<Self::Balance, DispatchError> {
							match exist_amount {
								Some(amount) => {
									*amount = amount.safe_add(&lp_amount)?;
									Ok(*amount)
								},
								None => {
									*exist_amount = Some(lp_amount);
									Ok(lp_amount)
								},
							}
						},
					)?;
					Self::deposit_event(Event::<T>::SingleAssetAccountsStorageUpdated {
						who,
						pool_id,
						balance: available_lp,
					});
				},
				SingleAssetAccountsStorageAction::Withdrawing => {
					let new_lp_amount = SingleAssetAccountsStorage::<T>::try_mutate(
						&who,
						&pool_id,
						|exist_amount| -> Result<Self::Balance, DispatchError> {
							match exist_amount {
								Some(amount) => {
									*amount = amount.safe_sub(&lp_amount)?;
									Ok(*amount)
								},
								None =>
									Err(Error::<T>::LpTokenNotFoundForSingleAssetWithdrawal.into()),
							}
						},
					)?;
					if new_lp_amount == Self::Balance::zero() {
						SingleAssetAccountsStorage::<T>::remove(&who, &pool_id);
					}
					Self::deposit_event(Event::<T>::SingleAssetAccountsStorageUpdated {
						who,
						pool_id,
						balance: new_lp_amount,
					});
				},
			}
			Ok(())
		}

		fn redeemable_assets_for_lp_tokens(
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_expected_amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<RedeemableAssets<Self::AssetId, Self::Balance>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let currency_pair = Self::currency_pair(pool_id)?;
			ensure!(min_expected_amounts.len() < 3, Error::<T>::MoreThanTwoAssetsNotYetSupported);
			let min_base_amount = *min_expected_amounts
				.get(&currency_pair.base)
				.ok_or(Error::<T>::MissingMinExpectedAmount)?;
			let min_quote_amount = *min_expected_amounts
				.get(&currency_pair.quote)
				.ok_or(Error::<T>::MissingMinExpectedAmount)?;
			match pool {
				PoolConfiguration::StableSwap(StableSwapPoolInfo { pair, lp_token, .. }) => {
					let pool_base_aum =
						T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
					let pool_quote_aum =
						T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
					let lp_issued = T::Assets::total_issuance(lp_token);

					let base_amount = T::Convert::convert(safe_multiply_by_rational(
						T::Convert::convert(lp_amount),
						pool_base_aum,
						T::Convert::convert(lp_issued),
					)?);
					let quote_amount = T::Convert::convert(safe_multiply_by_rational(
						T::Convert::convert(lp_amount),
						pool_quote_aum,
						T::Convert::convert(lp_issued),
					)?);

					ensure!(
						base_amount >= min_base_amount && quote_amount >= min_quote_amount,
						Error::<T>::CannotRespectMinimumRequested
					);
					Ok(RedeemableAssets {
						assets: BTreeMap::from([
							(pair.base, base_amount),
							(pair.quote, quote_amount),
						]),
					})
				},
				PoolConfiguration::ConstantProduct(ConstantProductPoolInfo {
					pair,
					lp_token,
					..
				}) => {
					let pool_base_aum =
						T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
					let lp_issued = T::Convert::convert(T::Assets::total_issuance(lp_token));

					let pool_quote_aum =
						T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
					let base_amount = T::Convert::convert(safe_multiply_by_rational(
						T::Convert::convert(lp_amount),
						pool_base_aum,
						lp_issued,
					)?);
					let quote_amount = T::Convert::convert(safe_multiply_by_rational(
						T::Convert::convert(lp_amount),
						pool_quote_aum,
						lp_issued,
					)?);

					ensure!(
						base_amount >= min_base_amount && quote_amount >= min_quote_amount,
						Error::<T>::CannotRespectMinimumRequested
					);
					Ok(RedeemableAssets {
						assets: BTreeMap::from([
							(pair.base, base_amount),
							(pair.quote, quote_amount),
						]),
					})
				},

				// for LBP just return 0 for both balances as it does not have LP Token
				PoolConfiguration::LiquidityBootstrapping(LiquidityBootstrappingPoolInfo {
					pair,
					..
				}) => Ok(RedeemableAssets {
					assets: BTreeMap::from([
						(pair.base, Self::Balance::zero()),
						(pair.quote, Self::Balance::zero()),
					]),
				}),
			}
		}

		fn redeemable_single_asset_for_lp_tokens(
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_expected_amount: Self::Balance,
		) -> Result<RedeemableAssets<Self::AssetId, Self::Balance>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(_) =>
					Err(Error::<T>::SingleAssetNotYetSupported.into()),
				PoolConfiguration::ConstantProduct(ConstantProductPoolInfo {
					pair,
					lp_token,
					base_weight,
					..
				}) => {
					let pool_base_aum =
						T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
					let lp_issued = T::Convert::convert(T::Assets::total_issuance(lp_token));
					let base_amount = compute_asset_for_redeemable_lp_tokens(
						pool_base_aum,
						base_weight,
						T::Convert::convert(lp_amount),
						lp_issued,
					)?;
					let base_amount = T::Convert::convert(base_amount);
					ensure!(
						base_amount >= min_expected_amount,
						Error::<T>::CannotRespectMinimumRequested
					);
					Ok(RedeemableAssets { assets: BTreeMap::from([(pair.base, base_amount)]) })
				},

				// for LBP just return 0 for both balances as it does not have LP Token
				PoolConfiguration::LiquidityBootstrapping(LiquidityBootstrappingPoolInfo {
					pair,
					..
				}) => Ok(RedeemableAssets {
					assets: BTreeMap::from([(pair.base, Self::Balance::zero())]),
				}),
			}
		}

		fn simulate_remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_expected_amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<RemoveLiquiditySimulationResult<Self::AssetId, Self::Balance>, DispatchError> {
			ensure!(min_expected_amounts.len() < 3, Error::<T>::MoreThanTwoAssetsNotYetSupported);
			let redeemable_assets =
				Self::redeemable_assets_for_lp_tokens(pool_id, lp_amount, min_expected_amounts)?;
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(StableSwapPoolInfo { pair, lp_token, .. }) |
				PoolConfiguration::ConstantProduct(ConstantProductPoolInfo {
					pair,
					lp_token,
					..
				}) => {
					let base_amount = *redeemable_assets
						.assets
						.get(&pair.base)
						.ok_or(Error::<T>::InvalidAsset)?;
					let quote_amount = *redeemable_assets
						.assets
						.get(&pair.quote)
						.ok_or(Error::<T>::InvalidAsset)?;
					let lp_issued = T::Assets::total_issuance(lp_token);
					let total_issuance = lp_issued.safe_sub(&lp_amount)?;

					ensure!(
						T::Assets::reducible_balance(pair.base, &pool_account, false) > base_amount,
						Error::<T>::NotEnoughLiquidity
					);
					ensure!(
						T::Assets::reducible_balance(pair.quote, &pool_account, false) >
							quote_amount,
						Error::<T>::NotEnoughLiquidity
					);
					ensure!(
						T::Assets::reducible_balance(lp_token, who, false) > lp_amount,
						Error::<T>::NotEnoughLpToken
					);
					Ok(RemoveLiquiditySimulationResult {
						assets: BTreeMap::from([
							(pair.base, base_amount),
							(pair.quote, quote_amount),
							(lp_token, total_issuance),
						]),
					})
				},
				PoolConfiguration::LiquidityBootstrapping(LiquidityBootstrappingPoolInfo {
					pair,
					..
				}) => Ok(RemoveLiquiditySimulationResult {
					assets: BTreeMap::from([
						(pair.base, Self::Balance::zero()),
						(pair.quote, Self::Balance::zero()),
					]),
				}),
			}
		}

		fn simulate_remove_liquidity_single_asset(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_expected_amount: Self::Balance,
		) -> Result<RemoveLiquiditySimulationResult<Self::AssetId, Self::Balance>, DispatchError> {
			let redeemable_assets = Self::redeemable_single_asset_for_lp_tokens(
				pool_id,
				lp_amount,
				min_expected_amount,
			)?;
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(StableSwapPoolInfo { pair, lp_token, .. }) |
				PoolConfiguration::ConstantProduct(ConstantProductPoolInfo {
					pair,
					lp_token,
					..
				}) => {
					let base_amount = *redeemable_assets
						.assets
						.get(&pair.base)
						.ok_or(Error::<T>::InvalidAsset)?;
					let lp_issued = T::Assets::total_issuance(lp_token);
					let total_issuance = lp_issued.safe_sub(&lp_amount)?;

					ensure!(
						T::Assets::reducible_balance(pair.base, &pool_account, false) > base_amount,
						Error::<T>::NotEnoughLiquidity
					);
					ensure!(
						T::Assets::reducible_balance(lp_token, who, false) > lp_amount,
						Error::<T>::NotEnoughLpToken
					);
					Ok(RemoveLiquiditySimulationResult {
						assets: BTreeMap::from([
							(pair.base, base_amount),
							(lp_token, total_issuance),
						]),
					})
				},
				PoolConfiguration::LiquidityBootstrapping(LiquidityBootstrappingPoolInfo {
					pair,
					..
				}) => Ok(RemoveLiquiditySimulationResult {
					assets: BTreeMap::from([
						(pair.base, Self::Balance::zero()),
						(pair.quote, Self::Balance::zero()),
					]),
				}),
			}
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			quote_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(info) => StableSwap::<T>::get_exchange_value(
					&info,
					&pool_account,
					asset_id,
					quote_amount,
				),
				PoolConfiguration::ConstantProduct(info) =>
					Uniswap::<T>::get_exchange_value(&info, &pool_account, asset_id, quote_amount),
				PoolConfiguration::LiquidityBootstrapping(info) =>
					LiquidityBootstrapping::<T>::get_exchange_value(
						info,
						pool_account,
						asset_id,
						quote_amount,
					),
			}
		}

		#[transactional]
		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			base_amount: Self::Balance,
			quote_amount: Self::Balance,
			min_mint_amount: Self::Balance,
			keep_alive: bool,
		) -> Result<(), DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let (added_base_amount, added_quote_amount, minted_lp) = match pool {
				PoolConfiguration::StableSwap(info) => {
					let (added_base_amount, added_quote_amount, minted_lp, base_fee, quote_fee) =
						StableSwap::<T>::add_liquidity(
							who,
							&info,
							&pool_account,
							base_amount,
							quote_amount,
							min_mint_amount,
							keep_alive,
						)?;
					// imbalance fees
					Self::disburse_fees(&pool_account, &pool_id, &info.owner, &base_fee)?;
					Self::disburse_fees(&pool_account, &pool_id, &info.owner, &quote_fee)?;
					(added_base_amount, added_quote_amount, minted_lp)
				},
				PoolConfiguration::ConstantProduct(info) => {
					let (added_base_amount, added_quote_amount, minted_lp, fee) =
						Uniswap::<T>::add_liquidity(
							who,
							&info,
							&pool_account,
							base_amount,
							quote_amount,
							min_mint_amount,
							keep_alive,
						)?;
					Self::disburse_fees(&pool_account, &pool_id, &info.owner, &fee)?;
					(added_base_amount, added_quote_amount, minted_lp)
				},
				PoolConfiguration::LiquidityBootstrapping(info) =>
					LiquidityBootstrapping::<T>::add_liquidity(
						who,
						info,
						&pool_account,
						base_amount,
						quote_amount,
						min_mint_amount,
						keep_alive,
					)?,
			};
			if added_base_amount.is_zero() || added_quote_amount.is_zero() {
				Self::update_accounts_deposited_one_asset_storage(
					who.clone(),
					pool_id,
					minted_lp,
					SingleAssetAccountsStorageAction::Depositing,
				)?;
			}
			Self::update_twap(pool_id)?;
			Self::deposit_event(Event::<T>::LiquidityAdded {
				who: who.clone(),
				pool_id,
				base_amount: added_base_amount,
				quote_amount: added_quote_amount,
				minted_lp,
			});
			Ok(())
		}

		#[transactional]
		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_base_amount: Self::Balance,
			min_quote_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let currency_pair = Self::currency_pair(pool_id)?;
			let redeemable_assets = Self::redeemable_assets_for_lp_tokens(
				pool_id,
				lp_amount,
				BTreeMap::from([
					(currency_pair.base, min_base_amount),
					(currency_pair.quote, min_quote_amount),
				]),
			)?;
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(info) => {
					let base_amount = *redeemable_assets
						.assets
						.get(&info.pair.base)
						.ok_or(Error::<T>::InvalidAsset)?;
					let quote_amount = *redeemable_assets
						.assets
						.get(&info.pair.quote)
						.ok_or(Error::<T>::InvalidAsset)?;
					let (base_amount, quote_amount, updated_lp) =
						StableSwap::<T>::remove_liquidity(
							who,
							info,
							pool_account,
							lp_amount,
							base_amount,
							quote_amount,
						)?;
					Self::update_twap(pool_id)?;
					Self::deposit_event(Event::<T>::LiquidityRemoved {
						pool_id,
						who: who.clone(),
						base_amount,
						quote_amount,
						total_issuance: updated_lp,
					});
				},
				PoolConfiguration::ConstantProduct(info) => {
					let base_amount = *redeemable_assets
						.assets
						.get(&info.pair.base)
						.ok_or(Error::<T>::InvalidAsset)?;
					let quote_amount = *redeemable_assets
						.assets
						.get(&info.pair.quote)
						.ok_or(Error::<T>::InvalidAsset)?;
					let (base_amount, quote_amount, updated_lp) = Uniswap::<T>::remove_liquidity(
						who,
						info,
						pool_account,
						lp_amount,
						base_amount,
						quote_amount,
					)?;
					Self::update_twap(pool_id)?;
					Self::deposit_event(Event::<T>::LiquidityRemoved {
						pool_id,
						who: who.clone(),
						base_amount,
						quote_amount,
						total_issuance: updated_lp,
					});
				},
				PoolConfiguration::LiquidityBootstrapping(info) => {
					let (base_amount, quote_amount) =
						LiquidityBootstrapping::<T>::remove_liquidity(
							who,
							pool_id,
							info,
							pool_account,
							lp_amount,
							min_base_amount,
							min_quote_amount,
						)?;
					Self::update_twap(pool_id)?;
					Pools::<T>::remove(pool_id);
					Self::deposit_event(Event::PoolDeleted { pool_id, base_amount, quote_amount });
				},
			}
			Ok(())
		}

		#[transactional]
		fn remove_liquidity_single_asset(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			// LP provider can withdraw liquidity with one asset only if he deposited liquidity with
			// one asset.
			ensure!(
				SingleAssetAccountsStorage::<T>::get(who, &pool_id) >= Some(lp_amount),
				Error::<T>::NotEnoughLpTokenForSingleAssetWithdraw
			);
			let redeemable_assets =
				Self::redeemable_single_asset_for_lp_tokens(pool_id, lp_amount, min_amount)?;
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(_) =>
					return Err(Error::<T>::SingleAssetNotYetSupported.into()),
				PoolConfiguration::ConstantProduct(info) => {
					let base_amount = *redeemable_assets
						.assets
						.get(&info.pair.base)
						.ok_or(Error::<T>::InvalidAsset)?;
					let (base_amount, quote_amount, updated_lp) = Uniswap::<T>::remove_liquidity(
						who,
						info,
						pool_account,
						lp_amount,
						base_amount,
						Self::Balance::zero(),
					)?;
					Self::update_accounts_deposited_one_asset_storage(
						who.clone(),
						pool_id,
						lp_amount,
						SingleAssetAccountsStorageAction::Withdrawing,
					)?;
					Self::update_twap(pool_id)?;
					Self::deposit_event(Event::<T>::LiquidityRemoved {
						pool_id,
						who: who.clone(),
						base_amount,
						quote_amount,
						total_issuance: updated_lp,
					});
				},
				PoolConfiguration::LiquidityBootstrapping(_) =>
					return Err(Error::<T>::SingleAssetNotYetSupported.into()),
			}
			Ok(())
		}

		#[transactional]
		fn exchange(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			pair: CurrencyPair<Self::AssetId>,
			quote_amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let (base_amount, owner, fees) = match pool {
				PoolConfiguration::StableSwap(info) => {
					// NOTE: lp_fees includes owner_fees.
					let (base_amount_excluding_fees, quote_amount, fees) =
						StableSwap::<T>::do_compute_swap(
							&info,
							&pool_account,
							pair,
							quote_amount,
							true,
						)?;

					ensure!(
						base_amount_excluding_fees >= min_receive,
						Error::<T>::CannotRespectMinimumRequested
					);
					T::Assets::transfer(pair.quote, who, &pool_account, quote_amount, keep_alive)?;

					// NOTE(hussein-aitlance): no need to keep alive the pool account
					T::Assets::transfer(
						pair.base,
						&pool_account,
						who,
						base_amount_excluding_fees,
						false,
					)?;
					(base_amount_excluding_fees, info.owner, fees)
				},
				PoolConfiguration::ConstantProduct(info) => {
					// NOTE: lp_fees includes owner_fees.
					let (base_amount, quote_amount_excluding_lp_fee, fees) =
						Uniswap::<T>::do_compute_swap(
							&info,
							&pool_account,
							pair,
							quote_amount,
							true,
						)?;

					ensure!(base_amount >= min_receive, Error::<T>::CannotRespectMinimumRequested);

					T::Assets::transfer(
						pair.quote,
						who,
						&pool_account,
						quote_amount_excluding_lp_fee,
						keep_alive,
					)?;
					// NOTE(hussein-aitlance): no need to keep alive the pool account
					T::Assets::transfer(pair.base, &pool_account, who, base_amount, false)?;
					(base_amount, info.owner, fees)
				},
				PoolConfiguration::LiquidityBootstrapping(info) => {
					let current_block = frame_system::Pallet::<T>::current_block_number();
					let (fees, base_amount) = LiquidityBootstrapping::<T>::do_get_exchange(
						info.clone(),
						&pool_account,
						pair,
						current_block,
						quote_amount,
						true,
					)?;

					ensure!(base_amount >= min_receive, Error::<T>::CannotRespectMinimumRequested);

					T::Assets::transfer(pair.quote, who, &pool_account, quote_amount, keep_alive)?;
					// NOTE(hussein-aitlance): no need to keep alive the pool account
					T::Assets::transfer(pair.base, &pool_account, who, base_amount, false)?;
					(base_amount, info.owner, fees)
				},
			};
			Self::disburse_fees(who, &pool_id, &owner, &fees)?;
			Self::update_twap(pool_id)?;
			Self::deposit_event(Event::<T>::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: pair.base,
				quote_asset: pair.quote,
				base_amount,
				quote_amount,
				fee: fees,
			});
			Ok(base_amount)
		}

		#[transactional]
		fn buy(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			match pool {
				PoolConfiguration::StableSwap(info) => {
					let pair =
						if asset_id == info.pair.base { info.pair } else { info.pair.swap() };
					// Since when buying asset user can't executed exchange as he don't know how
					// much amount of quote token he has to trade-in to get expected buy tokens.
					// So we compute price assuming user wants to sell instead of buy.
					// And then do exchange computed amount with token indices flipped.
					let dx = Self::get_exchange_value(pool_id, asset_id, amount)?;
					Self::exchange(who, pool_id, pair, dx, min_receive, keep_alive)?;
					Ok(amount)
				},
				PoolConfiguration::ConstantProduct(info) => {
					let pair =
						if asset_id == info.pair.base { info.pair } else { info.pair.swap() };
					let quote_amount = Self::get_exchange_value(pool_id, asset_id, amount)?;
					Self::exchange(who, pool_id, pair, quote_amount, min_receive, keep_alive)
				},
				PoolConfiguration::LiquidityBootstrapping(info) => {
					let pair =
						if asset_id == info.pair.base { info.pair } else { info.pair.swap() };
					let quote_amount = Self::get_exchange_value(pool_id, asset_id, amount)?;
					Self::exchange(who, pool_id, pair, quote_amount, min_receive, keep_alive)
				},
			}
		}

		#[transactional]
		fn sell(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			match pool {
				PoolConfiguration::StableSwap(info) => {
					let pair =
						if asset_id == info.pair.base { info.pair.swap() } else { info.pair };
					Self::exchange(who, pool_id, pair, amount, min_receive, keep_alive)
				},
				PoolConfiguration::ConstantProduct(info) => {
					let pair =
						if asset_id == info.pair.base { info.pair.swap() } else { info.pair };
					Self::exchange(who, pool_id, pair, amount, min_receive, keep_alive)
				},
				PoolConfiguration::LiquidityBootstrapping(info) => {
					let pair =
						if asset_id == info.pair.base { info.pair.swap() } else { info.pair };
					Self::exchange(who, pool_id, pair, amount, min_receive, keep_alive)
				},
			}
		}
	}

	/// Calculate current price of 1 LP token.
	fn calculate_price_of_lp_token<T: Config>(
		pair: CurrencyPair<T::AssetId>,
		lp_token: T::AssetId,
		pool_account: &T::AccountId,
	) -> Result<T::Balance, DispatchError> {
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pair.base, pool_account));
		let pool_quote_aum = T::Convert::convert(T::Assets::balance(pair.quote, pool_account));
		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(lp_token));
		let pool_general_aum = pool_base_aum.safe_add(&pool_quote_aum)?;
		Ok(T::Convert::convert(pool_general_aum.safe_div(&lp_total_issuance)?))
	}

	/// Retrieve the price(s) from the given pool calculated for the given `base_asset_id`
	/// and `quote_asset_id` pair.
	pub fn prices_for<T: Config>(
		pool_id: T::PoolId,
		base_asset_id: T::AssetId,
		quote_asset_id: T::AssetId,
		amount: T::Balance,
	) -> Result<PriceAggregate<T::PoolId, T::AssetId, T::Balance>, DispatchError> {
		// quote_asset_id is always known given the base as no multi-asset pool support is
		// implemented as of now.
		let spot_price = <Pallet<T> as Amm>::get_exchange_value(pool_id, base_asset_id, amount)?;
		Ok(PriceAggregate { pool_id, base_asset_id, quote_asset_id, spot_price })
	}

	fn lp_for_liquidity<T: Config>(
		pool_config: PoolConfiguration<T::AccountId, T::AssetId, T::BlockNumber>,
		pool_account: T::AccountId,
		base_amount: T::Balance,
		quote_amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		match pool_config {
			PoolConfiguration::StableSwap(pool) => {
				let (amount_of_lp_token_to_mint, ..) =
					StableSwap::<T>::calculate_mint_amount_and_fees(
						&pool,
						&pool_account,
						&base_amount,
						&quote_amount,
					)?;
				Ok(amount_of_lp_token_to_mint)
			},
			PoolConfiguration::ConstantProduct(pool) => {
				let base_amount = T::Convert::convert(base_amount);
				let pool_base_aum =
					T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
				let lp_total_issuance =
					T::Convert::convert(T::Assets::total_issuance(pool.lp_token));

				let amount_of_lp_token_to_mint = if quote_amount.is_zero() {
					let weight = pool.base_weight;
					compute_deposit_lp_single_asset(
						base_amount,
						pool_base_aum,
						weight,
						lp_total_issuance,
					)?
				} else {
					let pool_quote_aum =
						T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));

					let (_, amount_of_lp_token_to_mint) = compute_deposit_lp(
						lp_total_issuance,
						base_amount,
						T::Convert::convert(quote_amount),
						pool_base_aum,
						pool_quote_aum,
					)?;
					amount_of_lp_token_to_mint
				};
				Ok(T::Convert::convert(amount_of_lp_token_to_mint))
			},
			PoolConfiguration::LiquidityBootstrapping(_pool) => Ok(T::Balance::zero()),
		}
	}
}
