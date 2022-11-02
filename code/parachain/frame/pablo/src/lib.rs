#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)]
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
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
pub use pallet::*;

#[cfg(test)]
mod common_test_functions;
#[cfg(test)]
mod dual_asset_constant_product_tests;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod mock_fnft;

pub mod weights;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

mod dual_asset_constant_product;
mod twap;
mod types;

pub use crate::weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		dual_asset_constant_product::DualAssetConstantProduct,
		twap::{update_price_cumulative_state, update_twap_state},
		types::{PriceCumulative, TimeWeightedAveragePrice},
		WeightInfo,
	};
	use codec::FullCodec;
	use composable_support::{
		math::safe::{safe_multiply_by_rational, SafeArithmetic, SafeSub},
		validation::TryIntoValidated,
	};
	use composable_traits::{
		currency::{CurrencyFactory, LocalAssets, RangeId},
		defi::{CurrencyPair, Rate},
		dex::{
			Amm, BasicPoolInfo, Fee, PriceAggregate, RedeemableAssets,
			RemoveLiquiditySimulationResult,
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
	use sp_arithmetic::{fixed_point::FixedU64, FixedPointOperand};

	use composable_maths::dex::{
		constant_product::compute_deposit_lp, price::compute_initial_price_cumulative,
	};
	use composable_traits::{
		currency::BalanceLike,
		dex::{AssetAmount, FeeConfig, SwapResult},
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{AccountIdConversion, BlockNumberProvider, Convert, One, Zero},
		ArithmeticError, FixedPointNumber, Perbill, Permill,
	};
	use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

	#[derive(
		RuntimeDebug, Encode, Decode, MaxEncodedLen, CloneNoBound, PartialEq, Eq, TypeInfo,
	)]
	pub enum PoolInitConfiguration<AccountId: Clone, AssetId: Clone> {
		DualAssetConstantProduct {
			owner: AccountId,
			assets_weights: BoundedBTreeMap<AssetId, Permill, ConstU32<2>>,
			// trading fee
			fee: Permill,
		},
	}

	#[derive(
		RuntimeDebug, Encode, Decode, MaxEncodedLen, CloneNoBound, PartialEqNoBound, Eq, TypeInfo,
	)]
	pub enum PoolConfiguration<AccountId: Clone + PartialEq + Debug, AssetId: Clone + Ord + Debug> {
		DualAssetConstantProduct(BasicPoolInfo<AccountId, AssetId, ConstU32<2>>),
	}

	pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as Config>::Balance;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type PoolConfigurationOf<T> =
		PoolConfiguration<<T as frame_system::Config>::AccountId, <T as Config>::AssetId>;
	pub(crate) type PoolInitConfigurationOf<T> =
		PoolInitConfiguration<<T as frame_system::Config>::AccountId, <T as Config>::AssetId>;
	pub(crate) type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
	pub(crate) type TWAPStateOf<T> = TimeWeightedAveragePrice<MomentOf<T>, <T as Config>::Balance>;
	pub(crate) type PriceCumulativeStateOf<T> =
		PriceCumulative<MomentOf<T>, <T as Config>::Balance>;

	// TODO (vim): Modify events to remove base/quote asset naming and replace with just a map of
	// 	asset->value. Also introduce a  new event for "buy" operation as swap is different.
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
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
		NoLpTokenForLbp,
		NoXTokenForLbp,
		WeightsMustBeNonZero,
		WeightsMustSumToOne,
		StakingPoolConfigError,
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
			+ Into<u128>
			+ Ord;

		/// Type representing the Balance of an account.
		type Balance: BalanceLike + SafeSub + Zero + FixedPointOperand;

		/// An isomorphism: Balance<->u128
		type Convert: Convert<u128, BalanceOf<Self>> + Convert<BalanceOf<Self>, u128>;

		/// Factory to create new lp-token.
		type CurrencyFactory: CurrencyFactory<
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
		>;

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

		/// Required origin for pool creation.
		type PoolCreationOrigin: EnsureOrigin<Self::Origin>;

		/// Required origin to enable TWAP on pool.
		type EnableTwapOrigin: EnsureOrigin<Self::Origin>;

		/// Time provider.
		type Time: Time;

		/// The interval between TWAP computations.
		#[pallet::constant]
		type TWAPInterval: Get<MomentOf<Self>>;

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
			RewardPoolId = Self::AssetId,
		>;

		type ProtocolStaking: ProtocolStaking<
			AccountId = AccountIdOf<Self>,
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			RewardPoolId = Self::AssetId,
		>;

		type WeightInfo: WeightInfo;

		/// AssetId of the PICA asset
		#[pallet::constant]
		type PicaAssetId: Get<Self::AssetId>;

		/// AssetId of the PBLO asset
		#[pallet::constant]
		type PbloAssetId: Get<Self::AssetId>;

		/// AssetId of the xToken variant of PICA asset
		#[pallet::constant]
		type XPicaAssetId: Get<Self::AssetId>;

		/// AssetId of the xToken variant of PBLO asset
		#[pallet::constant]
		type XPbloAssetId: Get<Self::AssetId>;

		#[pallet::constant]
		type PicaStakeFinancialNftCollectionId: Get<Self::AssetId>;

		#[pallet::constant]
		type PbloStakeFinancialNftCollectionId: Get<Self::AssetId>;

		#[pallet::constant]
		type MsPerBlock: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
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

	pub(crate) enum PriceRatio {
		Swapped,
		NotSwapped,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool. Note that this extrinsic does NOT validate if a pool with the same
		/// assets already exists in the runtime.
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
			in_asset_id: T::AssetId,
			out_asset: AssetAmount<T::AssetId, T::Balance>,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::do_buy(&who, pool_id, in_asset_id, out_asset, keep_alive)?;
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
			in_asset: AssetAmount<T::AssetId, T::Balance>,
			min_receive: AssetAmount<T::AssetId, T::Balance>,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::do_swap(&who, pool_id, in_asset, min_receive, keep_alive)?;
			Ok(())
		}

		/// Add liquidity to the given pool.
		///
		/// Emits `LiquidityAdded` event when successful.
		#[pallet::weight(T::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			assets: BTreeMap<T::AssetId, T::Balance>,
			min_mint_amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::add_liquidity(&who, pool_id, assets, min_mint_amount, keep_alive)?;
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
			min_receive: BTreeMap<T::AssetId, T::Balance>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::remove_liquidity(&who, pool_id, lp_amount, min_receive)?;
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
						#[allow(deprecated)]
						if let Ok(assets) = Self::pool_ordered_pair(pool_id) {
							Self::deposit_event(Event::<T>::TwapUpdated {
								pool_id,
								timestamp: updated_twap.timestamp,
								twaps: BTreeMap::from([
									(assets.base, updated_twap.base_twap),
									(assets.quote, updated_twap.quote_twap),
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
		fn default_lp_staking_pool_config(
			pool_id: &T::PoolId,
		) -> Result<
			RewardPoolConfiguration<
				AccountIdOf<T>,
				AssetIdOf<T>,
				BalanceOf<T>,
				T::BlockNumber,
				T::MaxRewardConfigsPerPool,
				T::MaxStakingDurationPresets,
			>,
			DispatchError,
		> {
			// let reward_rate = Perbill::from_percent(REWARD_PERCENTAGE); not sure how this
			// translates to the new model
			let reward_rate = RewardRate::per_second(T::Convert::convert(0));
			let pblo_asset_id: T::AssetId = T::PbloAssetId::get();
			let reward_configs = [(pblo_asset_id, RewardConfig { reward_rate })]
				.into_iter()
				.try_collect()
				.map_err(|_| Error::<T>::StakingPoolConfigError)?;
			let duration_presets = [
				(
					ONE_WEEK,
					FixedU64::from_rational(101, 100)
						.try_into_validated()
						.expect("valid reward multiplier"),
				),
				(
					ONE_MONTH,
					FixedU64::from_rational(11, 10)
						.try_into_validated()
						.expect("valid reward multiplier"),
				),
			]
			.into_iter()
			.try_collect()
			.map_err(|_| Error::<T>::StakingPoolConfigError)?;
			let lock = LockConfig { duration_presets, unlock_penalty: Perbill::from_percent(5) };
			let five_years_block = 5 * 365 * 24 * 60 * 60 / T::MsPerBlock::get();
			// NOTE(connor): `start_block` must greater than current block
			let start_block = frame_system::Pallet::<T>::current_block_number() + 1_u32.into();
			let end_block = start_block + five_years_block.into();
			let minimum_staking_amount: T::Balance = T::Convert::convert(2_000_000_u128);

			Ok(RewardPoolConfiguration::RewardRateBasedIncentive {
				owner: Self::account_id(pool_id),
				asset_id: Self::lp_token(*pool_id)?,
				start_block,
				end_block,
				reward_configs,
				lock,
				share_asset_id: Self::get_x_token_from_pool(*pool_id)?,
				financial_nft_asset_id: Self::get_financial_nft_from_pool(*pool_id)?,
				minimum_staking_amount,
			})
		}

		#[transactional]
		fn create_staking_reward_pool(pool_id: &T::PoolId) -> DispatchResult {
			let lp_pool_config = Self::default_lp_staking_pool_config(pool_id)?;
			T::ManageStaking::create_staking_pool(lp_pool_config)?;
			Ok(())
		}

		/// Note this function does not validate,
		/// 1. if the pool is created by a valid origin.
		/// 2. if a pool exists with the same pair already.
		#[transactional]
		pub fn do_create_pool(
			init_config: PoolInitConfigurationOf<T>,
		) -> Result<T::PoolId, DispatchError> {
			let (owner, pool_id, assets_weights) = match init_config {
				PoolInitConfiguration::DualAssetConstantProduct { owner, fee, assets_weights } => {
					let pool_id = DualAssetConstantProduct::<T>::do_create_pool(
						&owner,
						FeeConfig::default_from(fee),
						assets_weights.clone(),
					)?;
					Self::create_staking_reward_pool(&pool_id)?;
					(owner, pool_id, assets_weights)
				},
			};
			// TODO (vim): We have no way of knowing which amount is for which asset (fixed in a
			// later  stage). For now we assume the input defined order.
			let assets = assets_weights.keys().copied().collect::<Vec<_>>();
			Self::deposit_event(Event::<T>::PoolCreated {
				owner,
				pool_id,
				assets: CurrencyPair::new(assets[0], assets[1]),
			});
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
			#[allow(deprecated)]
			let pair = Self::pool_ordered_pair(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pair = match price_ratio {
				PriceRatio::NotSwapped => pair,
				PriceRatio::Swapped => pair.swap(),
			};
			let pool_base_asset_under_management = T::Assets::balance(pair.base, &pool_account);
			let pool_quote_asset_under_management = T::Assets::balance(pair.quote, &pool_account);

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
			#[allow(deprecated)]
			let currency_pair = Self::pool_ordered_pair(pool_id)?; // update price cumulatives
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
			_: &T::PoolId,
			owner: &T::AccountId,
			fees: &Fee<T::AssetId, T::Balance>,
		) -> Result<(), DispatchError> {
			if !fees.owner_fee.is_zero() {
				T::Assets::transfer(fees.asset_id, who, owner, fees.owner_fee, false)?;
			}
			if !fees.protocol_fee.is_zero() {
				T::ProtocolStaking::transfer_reward(
					who,
					&T::PbloAssetId::get(),
					fees.asset_id,
					fees.protocol_fee,
					false,
				)?;
			}
			Ok(())
		}

		fn get_x_token_from_pool(pool_id: T::PoolId) -> Result<T::AssetId, DispatchError> {
			// Get token asset ID from pool ID
			let pool = Self::get_pool(pool_id)?;
			let token_id = match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => info.lp_token,
			};

			// Match token asset ID with xToken asset ID
			match token_id {
				x if x == T::PicaAssetId::get() => Ok(T::XPicaAssetId::get()),
				x if x == T::PbloAssetId::get() => Ok(T::XPbloAssetId::get()),
				_ => Ok(T::CurrencyFactory::create(RangeId::XTOKEN_ASSETS, T::Balance::default())?),
			}
		}

		fn get_financial_nft_from_pool(pool_id: T::PoolId) -> Result<T::AssetId, DispatchError> {
			// Get token asset ID from pool ID
			let pool = Self::get_pool(pool_id)?;
			let token_id = match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => info.lp_token,
			};

			// Match token asset ID with fNFT asset ID
			match token_id {
				x if x == T::PicaAssetId::get() => Ok(T::PicaStakeFinancialNftCollectionId::get()),
				x if x == T::PbloAssetId::get() => Ok(T::PbloStakeFinancialNftCollectionId::get()),
				_ => Ok(T::CurrencyFactory::create(RangeId::FNFT_ASSETS, T::Balance::default())?),
			}
		}

		fn lp_for_liquidity(
			pool_config: PoolConfiguration<T::AccountId, T::AssetId>,
			pool_account: T::AccountId,
			base_amount: T::Balance,
			quote_amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			match pool_config {
				PoolConfiguration::DualAssetConstantProduct(pool) => {
					let assets = pool.assets_weights.keys().copied().collect::<Vec<_>>();
					let currency_pair = CurrencyPair::new(assets[0], assets[1]);
					let pool_base_aum =
						T::Convert::convert(T::Assets::balance(currency_pair.base, &pool_account));
					let pool_quote_aum =
						T::Convert::convert(T::Assets::balance(currency_pair.quote, &pool_account));

					let lp_total_issuance =
						T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
					let (_, amount_of_lp_token_to_mint) = compute_deposit_lp(
						lp_total_issuance,
						T::Convert::convert(base_amount),
						T::Convert::convert(quote_amount),
						pool_base_aum,
						pool_quote_aum,
					)?;
					Ok(T::Convert::convert(amount_of_lp_token_to_mint))
				},
			}
		}

		#[deprecated(
			note = "This is a temporary function for refactoring/migration purposes. Use `Amm::assets` instead."
		)]
		fn pool_ordered_pair(
			pool_id: T::PoolId,
		) -> Result<CurrencyPair<T::AssetId>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => {
					let assets = info.assets_weights.keys().copied().collect::<Vec<_>>();
					ensure!(assets.len() == 2, Error::<T>::PairMismatch);
					let base_asset = assets.get(0).ok_or(Error::<T>::PairMismatch)?;
					let quote_asset = assets.get(1).ok_or(Error::<T>::PairMismatch)?;
					Ok(CurrencyPair::new(*base_asset, *quote_asset))
				},
			}
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

		fn assets(
			pool_id: Self::PoolId,
		) -> Result<BTreeMap<Self::AssetId, Permill>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			match pool {
				PoolConfiguration::DualAssetConstantProduct(info) =>
					Ok(info.assets_weights.into_inner()),
			}
		}

		fn lp_token(pool_id: Self::PoolId) -> Result<Self::AssetId, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => Ok(info.lp_token),
			}
		}

		fn simulate_add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			#[allow(deprecated)]
			let currency_pair = Self::pool_ordered_pair(pool_id)?;
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

			Self::lp_for_liquidity(pool, pool_account, base_amount, quote_amount)
		}

		fn redeemable_assets_for_lp_tokens(
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_expected_amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<RedeemableAssets<Self::AssetId, Self::Balance>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			#[allow(deprecated)]
			let currency_pair = Self::pool_ordered_pair(pool_id)?;
			ensure!(min_expected_amounts.len() < 3, Error::<T>::MoreThanTwoAssetsNotYetSupported);
			let min_base_amount = *min_expected_amounts
				.get(&currency_pair.base)
				.ok_or(Error::<T>::MissingMinExpectedAmount)?;
			let min_quote_amount = *min_expected_amounts
				.get(&currency_pair.quote)
				.ok_or(Error::<T>::MissingMinExpectedAmount)?;
			match pool {
				PoolConfiguration::DualAssetConstantProduct(BasicPoolInfo { lp_token, .. }) => {
					// TODO (vim): This function must call the relevant calculation through
					//  dual_asset_constant_product.rs. Then most of the logic is removed here.
					let pool_base_aum =
						T::Convert::convert(T::Assets::balance(currency_pair.base, &pool_account));
					let pool_quote_aum =
						T::Convert::convert(T::Assets::balance(currency_pair.quote, &pool_account));
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
							(currency_pair.base, base_amount),
							(currency_pair.quote, quote_amount),
						]),
					})
				},
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
			#[allow(deprecated)]
			let currency_pair = Self::pool_ordered_pair(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::DualAssetConstantProduct(BasicPoolInfo { lp_token, .. }) => {
					let base_amount = *redeemable_assets
						.assets
						.get(&currency_pair.base)
						.ok_or(Error::<T>::InvalidAsset)?;
					let quote_amount = *redeemable_assets
						.assets
						.get(&currency_pair.quote)
						.ok_or(Error::<T>::InvalidAsset)?;
					let lp_issued = T::Assets::total_issuance(lp_token);
					let total_issuance = lp_issued.safe_sub(&lp_amount)?;

					ensure!(
						T::Assets::reducible_balance(currency_pair.base, &pool_account, false) >
							base_amount,
						Error::<T>::NotEnoughLiquidity
					);
					ensure!(
						T::Assets::reducible_balance(currency_pair.quote, &pool_account, false) >
							quote_amount,
						Error::<T>::NotEnoughLiquidity
					);
					ensure!(
						T::Assets::reducible_balance(lp_token, who, false) > lp_amount,
						Error::<T>::NotEnoughLpToken
					);
					Ok(RemoveLiquiditySimulationResult {
						assets: BTreeMap::from([
							(currency_pair.base, base_amount),
							(currency_pair.quote, quote_amount),
							(lp_token, total_issuance),
						]),
					})
				},
			}
		}

		fn spot_price(
			pool_id: Self::PoolId,
			base_asset: AssetAmount<Self::AssetId, Self::Balance>,
			quote_asset_id: Self::AssetId,
		) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => {
					let res = DualAssetConstantProduct::<T>::do_buy(
						&info,
						&pool_account,
						base_asset,
						quote_asset_id,
						false,
					)?;
					Ok(SwapResult::new(quote_asset_id, res.1, res.2.asset_id, res.2.fee))
				},
			}
		}

		#[transactional]
		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			assets: BTreeMap<Self::AssetId, Self::Balance>,
			min_mint_amount: Self::Balance,
			keep_alive: bool,
		) -> Result<(), DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let (added_base_amount, added_quote_amount, minted_lp) = match pool {
				PoolConfiguration::DualAssetConstantProduct(info) =>
					DualAssetConstantProduct::<T>::add_liquidity(
						who,
						info,
						pool_account,
						assets,
						min_mint_amount,
						keep_alive,
					)?,
			};
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
			min_receive: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<(), DispatchError> {
			let redeemable_assets =
				Self::redeemable_assets_for_lp_tokens(pool_id, lp_amount, min_receive)?;
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => {
					let (base_amount, quote_amount, updated_lp) =
						DualAssetConstantProduct::<T>::remove_liquidity(
							who,
							info,
							pool_account,
							lp_amount,
							redeemable_assets.assets,
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
			}
			Ok(())
		}

		#[transactional]
		fn do_swap(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			in_asset: AssetAmount<Self::AssetId, Self::Balance>,
			min_receive: AssetAmount<Self::AssetId, Self::Balance>,
			keep_alive: bool,
		) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let (base_amount, owner, fees) = match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => {
					// NOTE: lp_fees includes owner_fees.
					let (base_amount, quote_amount_excluding_lp_fee, fees) =
						DualAssetConstantProduct::<T>::do_swap(
							&info,
							&pool_account,
							in_asset,
							min_receive,
							true,
						)?;

					ensure!(
						base_amount >= min_receive.amount,
						Error::<T>::CannotRespectMinimumRequested
					);

					T::Assets::transfer(
						in_asset.asset_id,
						who,
						&pool_account,
						quote_amount_excluding_lp_fee,
						keep_alive,
					)?;
					T::Assets::transfer(
						min_receive.asset_id,
						&pool_account,
						who,
						base_amount,
						false,
					)?;
					(base_amount, info.owner, fees)
				},
			};
			Self::disburse_fees(who, &pool_id, &owner, &fees)?;
			Self::update_twap(pool_id)?;
			Self::deposit_event(Event::<T>::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: min_receive.asset_id,
				quote_asset: in_asset.asset_id,
				base_amount,
				quote_amount: in_asset.amount,
				fee: fees,
			});
			Ok(SwapResult::new(min_receive.asset_id, base_amount, fees.asset_id, fees.fee))
		}

		#[transactional]
		fn do_buy(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			in_asset_id: Self::AssetId,
			out_asset: AssetAmount<Self::AssetId, Self::Balance>,
			keep_alive: bool,
		) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let (quote_amount, owner, fees) = match pool {
				PoolConfiguration::DualAssetConstantProduct(info) => {
					// NOTE: lp_fees includes owner_fees.
					let (base_amount, quote_amount_including_lp_fee, fees) =
						DualAssetConstantProduct::<T>::do_buy(
							&info,
							&pool_account,
							out_asset,
							in_asset_id,
							true,
						)?;

					T::Assets::transfer(
						in_asset_id,
						who,
						&pool_account,
						quote_amount_including_lp_fee,
						keep_alive,
					)?;
					T::Assets::transfer(
						out_asset.asset_id,
						&pool_account,
						who,
						base_amount,
						false,
					)?;
					(quote_amount_including_lp_fee, info.owner, fees)
				},
			};
			Self::disburse_fees(who, &pool_id, &owner, &fees)?;
			Self::update_twap(pool_id)?;
			// TODO (vim): Emit a Buy event
			Self::deposit_event(Event::<T>::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: out_asset.asset_id,
				quote_asset: in_asset_id,
				base_amount: out_asset.amount,
				quote_amount,
				fee: fees,
			});
			// TODO (vim): Return a BuyResult type
			Ok(SwapResult::new(out_asset.asset_id, out_asset.amount, fees.asset_id, fees.fee))
		}
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
		let spot_price = <Pallet<T> as Amm>::spot_price(
			pool_id,
			AssetAmount::new(base_asset_id, amount),
			quote_asset_id,
		)?;
		Ok(PriceAggregate {
			pool_id,
			base_asset_id,
			quote_asset_id,
			spot_price: spot_price.value.amount,
		})
	}
}
