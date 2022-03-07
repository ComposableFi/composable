//
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
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
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use crate::weights::WeightInfo;

pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		types::{PriceCumulatives, TWAP},
		weights::WeightInfo,
	};
	use codec::{Codec, FullCodec};
	use composable_maths::dex::{
		constant_product::{compute_deposit_lp, compute_in_given_out, compute_out_given_in},
		price::{
			compute_initial_price_cumulative, compute_next_price_cumulative, compute_price_average,
		},
	};
	use composable_traits::{
		currency::{CurrencyFactory, RangeId},
		defi::{CurrencyPair, Rate},
		dex::{Amm, ConstantProductPoolInfo},
		math::{safe_multiply_by_rational, SafeArithmetic, SafeSub, SafeAdd},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			Time,
		},
		transactional, PalletId,
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use scale_info::TypeInfo;
	use sp_runtime::{
		traits::{AccountIdConversion, CheckedAdd, Convert, One, Zero},
		ArithmeticError, FixedPointNumber, Permill,
	};
	use sp_std::fmt::Debug;

	type PoolOf<T> =
		ConstantProductPoolInfo<<T as frame_system::Config>::AccountId, <T as Config>::AssetId>;

	type MomentOf<T> = <<T as Config>::Time as Time>::Moment;

	type PriceCumulativesOf<T> = PriceCumulatives<MomentOf<T>, <T as Config>::Balance>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;

		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Zero
			+ Ord
      + SafeArithmetic;

		type Convert: Convert<u128, Self::Balance> + Convert<Self::Balance, u128>;

		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId>;

		type Assets: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Inspect<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;

		/// Time provider.
		type Time: Time;

		/// The interval between TWAP computations.
		#[pallet::constant]
		type TWAPInterval: Get<MomentOf<Self>>;

		/// Type allowing us to refer a pool.
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Debug
      + SafeArithmetic
			+ Zero
			+ One;

		/// Pallet unique ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Extrinsics weights implementation.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	// TODO(hussein-aitlahcen): MaxEncodedLen not yet implemented for trait AT `Time::Moment`.
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Current number of pools (also ID for the next created pool)
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	#[allow(clippy::disallowed_type)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery, PoolCountOnEmpty<T>>;

	#[pallet::type_value]
	pub fn PoolCountOnEmpty<T: Config>() -> T::PoolId {
		Zero::zero()
	}

	/// Existing pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		ConstantProductPoolInfo<T::AccountId, T::AssetId>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn price_cumulatives)]
	pub type PriceCumulativesState<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, PriceCumulativesOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn twap)]
	pub type TWAPState<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, TWAP, OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		InvalidFees,
		InvalidPair,
		PoolNotFound,
		InvalidAmount,
		MissingAmount,
		PairMismatch,
		CannotRespectMinimumRequested,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
		PoolCreated {
			/// Id of newly created pool.
			pool_id: T::PoolId,
			/// Account id of creator.
			who: T::AccountId,
		},

		/// Liquidity added into the pool `T::PoolId` by `T::AccountId`.
		LiquidityAdded {
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Account id who added liquidity.
			who: T::AccountId,
			/// Amount of base asset deposited.
			base_amount: T::Balance,
			/// Amount of quote asset deposited.
			quote_amount: T::Balance,
			/// Amount of minted lp.
			minted_lp: T::Balance,
		},

		/// Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
		LiquidityRemoved {
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Account id who removed liquidity.
			who: T::AccountId,
			/// Amount of base asset deposited.
			base_amount: T::Balance,
			/// Amount of quote asset deposited.
			quote_amount: T::Balance,
			/// New total LP issuance.
			total_issuance: T::Balance,
		},

		/// Token exchange happened.
		Swapped {
			/// Account id who exchanged token.
			who: T::AccountId,
			/// Pool id on which exchange done.
			pool_id: T::PoolId,
			/// Id of asset used as input.
			base_asset: T::AssetId,
			/// Id of asset used as output.
			quote_asset: T::AssetId,
			/// Amount of base asset received.
			base_amount: T::Balance,
			/// Amount of quote asset provided.
			quote_amount: T::Balance,
			/// Charged fees.
			fee: T::Balance,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool.
		///
		/// Emits `PoolCreated` even when successful.
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			pair: CurrencyPair<T::AssetId>,
			fee: Permill,
			owner_fee: Permill,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = Self::do_create_pool(&who, pair, fee, owner_fee)?;
			Ok(())
		}

		/// Execute a buy order on a pool.
		///
		/// The `base_amount` always represent the base asset amount (A/B => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::buy())]
		pub fn buy(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			asset_id: T::AssetId,
			amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::buy(&who, pool_id, asset_id, amount, keep_alive)?;
			Ok(())
		}

		/// Execute a sell order on a pool.
		///
		/// The `base_amount` always represent the base asset amount (A/B => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::sell())]
		pub fn sell(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			asset_id: T::AssetId,
			amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::sell(&who, pool_id, asset_id, amount, keep_alive)?;
			Ok(())
		}

		/// Execute a specific swap operation.
		///
		/// Buy operation if the pair is the original pool pair (A/B).
		/// Sell operation if the pair is the original pool pair swapped (B/A).
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

		/// Add liquidity to a constant_product pool.
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
			let _ = <Self as Amm>::add_liquidity(
				&who,
				pool_id,
				base_amount,
				quote_amount,
				min_mint_amount,
				keep_alive,
			)?;
			Ok(())
		}

		/// Remove liquidity from constant_product pool.
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
			let _ = <Self as Amm>::remove_liquidity(
				&who,
				pool_id,
				lp_amount,
				min_base_amount,
				min_quote_amount,
			)?;
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			let current_timestamp = T::Time::now();
			// TODO(hussein-aitlahcen): probably worth making twap enabled/disabled per-pool using
			// democracy as this can grow a lot.
			Pools::<T>::iter_keys().fold(Weight::zero(), |weight, pool_id| {
				let rate_base = Self::do_get_exchange_rate(pool_id, false);
				let rate_quote = Self::do_get_exchange_rate(pool_id, true);
				match (rate_base, rate_quote) {
					(Ok(rate_base), Ok(rate_quote)) => PriceCumulativesState::<T>::try_mutate(
						pool_id,
						|price_cumulatives| -> Result<Weight, DispatchError> {
							match *price_cumulatives {
								// Initialization
								None => {
									let (price_cumulative_base, price_cumulative_quote) = (
										compute_initial_price_cumulative::<T::Convert, _>(
											rate_base,
										)?,
										compute_initial_price_cumulative::<T::Convert, _>(
											rate_quote,
										)?,
									);
									*price_cumulatives = Some(PriceCumulatives {
										timestamp: current_timestamp,
										price_cumulative_base,
										price_cumulative_quote,
									});
									TWAPState::<T>::insert(
										pool_id,
										TWAP {
											average_price_base: rate_base,
											average_price_quote: rate_quote,
										},
									);
									Ok(weight + 1)
								},
								// Update
								Some(PriceCumulatives {
									timestamp: previous_timestamp,
									price_cumulative_base: previous_price_cumulative_base,
									price_cumulative_quote: previous_price_cumulative_quote,
								}) => {
									let f = |previous_cumulative_price, exchange_rate| {
										compute_next_price_cumulative::<T::Convert, _, _>(
											previous_timestamp,
											previous_cumulative_price,
											current_timestamp,
											exchange_rate,
										)
									};
									let (
										(elapsed, price_cumulative_base),
										(_, price_cumulative_quote),
									) = (
										f(previous_price_cumulative_base, rate_base)?,
										f(previous_price_cumulative_quote, rate_quote)?,
									);
									if elapsed >= T::TWAPInterval::get() {
										*price_cumulatives = Some(PriceCumulatives {
											timestamp: current_timestamp,
											price_cumulative_base,
											price_cumulative_quote,
										});
										let avg = |current: T::Balance,
										           previous: T::Balance|
										 -> Result<Rate, DispatchError> {
											compute_price_average::<T::Convert, _, _>(
												current, previous, elapsed,
											)
										};
										let average_price_base = avg(
											price_cumulative_base,
											previous_price_cumulative_base,
										)?;
										let average_price_quote = avg(
											price_cumulative_quote,
											previous_price_cumulative_quote,
										)?;
										TWAPState::<T>::insert(
											pool_id,
											TWAP { average_price_base, average_price_quote },
										);
										Ok(weight + 1)
									} else {
										Ok(weight)
									}
								},
							}
						},
					)
					.unwrap_or(weight),
					_ => {
						log::warn!("Failed to get exchange rate for pool: {:?}", pool_id,);
						weight
					},
				}
			})
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

		fn currency_pair(
			pool_id: Self::PoolId,
		) -> Result<CurrencyPair<Self::AssetId>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			Ok(pool.pair)
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let amount = T::Convert::convert(amount);
			let half_weight = Permill::from_percent(50);
			let pool_base_aum =
				T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
			let pool_quote_aum =
				T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));
			let exchange_amount = if asset_id == pool.pair.quote {
				compute_out_given_in(
					half_weight,
					half_weight,
					pool_quote_aum,
					pool_base_aum,
					amount,
				)
			} else {
				compute_in_given_out(
					half_weight,
					half_weight,
					pool_quote_aum,
					pool_base_aum,
					amount,
				)
			}?;
			Ok(T::Convert::convert(exchange_amount))
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
			ensure!(base_amount > T::Balance::zero(), Error::<T>::InvalidAmount);

			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pool_base_aum =
				T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
			let pool_quote_aum =
				T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));

			let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
			let (quote_amount, lp_to_mint) = compute_deposit_lp(
				lp_total_issuance,
				T::Convert::convert(base_amount),
				T::Convert::convert(quote_amount),
				pool_base_aum,
				pool_quote_aum,
			)?;
			let quote_amount = T::Convert::convert(quote_amount);
			let lp_to_mint = T::Convert::convert(lp_to_mint);

			ensure!(quote_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
			ensure!(lp_to_mint >= min_mint_amount, Error::<T>::CannotRespectMinimumRequested);

			T::Assets::transfer(pool.pair.base, who, &pool_account, base_amount, keep_alive)?;
			T::Assets::transfer(pool.pair.quote, who, &pool_account, quote_amount, keep_alive)?;
			T::Assets::mint_into(pool.lp_token, who, lp_to_mint)?;

			Self::deposit_event(Event::<T>::LiquidityAdded {
				pool_id,
				who: who.clone(),
				base_amount,
				quote_amount,
				minted_lp: lp_to_mint,
			});

			Ok(())
		}

		#[transactional]
		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			lp_amount: Self::Balance,
			min_base_amount: Self::Balance,
			min_quote_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let pool = Self::get_pool(pool_id)?;

			let pool_account = Self::account_id(&pool_id);
			let pool_base_aum =
				T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
			let pool_quote_aum =
				T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));
			let lp_issued = T::Assets::total_issuance(pool.lp_token);

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

			// NOTE(hussein-aitlance): no need to keep alive the pool account
			T::Assets::transfer(pool.pair.base, &pool_account, who, base_amount, false)?;
			T::Assets::transfer(pool.pair.quote, &pool_account, who, quote_amount, false)?;
			T::Assets::burn_from(pool.lp_token, who, lp_amount)?;

			Self::deposit_event(Event::<T>::LiquidityRemoved {
				pool_id,
				who: who.clone(),
				base_amount,
				quote_amount,
				total_issuance: lp_issued.safe_sub(&lp_amount)?,
			});

			Ok(())
		}

		#[transactional]
		fn buy(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
			let quote_amount = Self::get_exchange_value(pool_id, asset_id, amount)?;
			<Self as Amm>::exchange(
				who,
				pool_id,
				pair,
				quote_amount,
				T::Balance::zero(),
				keep_alive,
			)
		}

		#[transactional]
		fn sell(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pair = if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
			<Self as Amm>::exchange(who, pool_id, pair, amount, T::Balance::zero(), keep_alive)
		}

		#[transactional]
		fn exchange(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			pair: CurrencyPair<Self::AssetId>,
			quote_amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			// /!\ NOTE(hussein-aitlahcen): after this check, do not use pool.pair as the provided
			// pair might have been swapped
			ensure!(pair == pool.pair, Error::<T>::PairMismatch);

			let (base_amount, quote_amount, lp_fees, owner_fees) =
				Self::do_compute_swap(pool_id, pair, quote_amount, true)?;
			let total_fees = lp_fees.safe_add(&owner_fees)?;
			let quote_amount_including_fees = quote_amount.safe_add(&total_fees)?;

			ensure!(base_amount >= min_receive, Error::<T>::CannotRespectMinimumRequested);

			let pool_account = Self::account_id(&pool_id);
			T::Assets::transfer(
				pair.quote,
				who,
				&pool_account,
				quote_amount_including_fees,
				keep_alive,
			)?;
			// NOTE(hussein-aitlance): no need to keep alive the pool account
			T::Assets::transfer(pair.quote, &pool_account, &pool.owner, owner_fees, false)?;
			T::Assets::transfer(pair.base, &pool_account, who, base_amount, false)?;

			Self::deposit_event(Event::<T>::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: pair.base,
				quote_asset: pair.quote,
				base_amount,
				quote_amount,
				fee: total_fees,
			});

			Ok(base_amount)
		}
	}

	impl<T: Config> Pallet<T> {
		#[transactional]
		pub fn do_create_pool(
			who: &T::AccountId,
			pair: CurrencyPair<T::AssetId>,
			fee: Permill,
			owner_fee: Permill,
		) -> Result<T::PoolId, DispatchError> {
			// NOTE(hussein-aitlahcen): do we allow such pair?
			ensure!(pair.base != pair.quote, Error::<T>::InvalidPair);

			let total_fees = fee.checked_add(&owner_fee).ok_or(ArithmeticError::Overflow)?;
			ensure!(total_fees < Permill::one(), Error::<T>::InvalidFees);

			let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS)?;

			// Add new pool
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
					let pool_id = *pool_count;
					Pools::<T>::insert(
						pool_id,
						ConstantProductPoolInfo {
							owner: who.clone(),
							pair,
							lp_token,
							fee,
							owner_fee,
						},
					);
					*pool_count = pool_id.safe_add(&T::PoolId::one())?;
					Ok(pool_id)
				})?;

			Self::deposit_event(Event::PoolCreated { pool_id, who: who.clone() });

			Ok(pool_id)
		}

		/// Return pool information for given pool_id.
		pub(crate) fn get_pool(pool_id: T::PoolId) -> Result<PoolOf<T>, DispatchError> {
			Pools::<T>::get(pool_id).ok_or_else(|| Error::<T>::PoolNotFound.into())
		}

		/// Account of a pool
		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}

		/// Assume that the pair is valid for the pool
		pub(crate) fn do_compute_swap(
			pool_id: T::PoolId,
			pair: CurrencyPair<T::AssetId>,
			quote_amount: T::Balance,
			apply_fees: bool,
		) -> Result<(T::Balance, T::Balance, T::Balance, T::Balance), DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pool_base_aum = T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
			let pool_quote_aum = T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
			let quote_amount = T::Convert::convert(quote_amount);

			// https://uniswap.org/whitepaper.pdf
			// 3.2.1
			// we do not inflate the lp for the owner fees
			// cut is done before enforcing the invariant
			let (lp_fee, owner_fee) = if apply_fees {
				let lp_fee = pool.fee.mul_floor(quote_amount);
				let owner_fee = pool.owner_fee.mul_floor(quote_amount);
				(lp_fee, owner_fee)
			} else {
				(0, 0)
			};

			let quote_amount_excluding_fees =
				quote_amount.safe_sub(&lp_fee)?.safe_sub(&owner_fee)?;

			let half_weight = Permill::from_percent(50);
			let base_amount = compute_out_given_in(
				half_weight,
				half_weight,
				pool_quote_aum,
				pool_base_aum,
				quote_amount_excluding_fees,
			)?;

			ensure!(base_amount > 0 && quote_amount_excluding_fees > 0, Error::<T>::InvalidAmount);

			Ok((
				T::Convert::convert(base_amount),
				T::Convert::convert(quote_amount_excluding_fees),
				T::Convert::convert(lp_fee),
				T::Convert::convert(owner_fee),
			))
		}

		pub(crate) fn do_get_exchange_rate(
			pool_id: T::PoolId,
			swap: bool,
		) -> Result<Rate, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pair = if swap { pool.pair.swap() } else { pool.pair };
			let pool_base_aum = T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
			let pool_quote_aum = T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
			Ok(Rate::checked_from_rational(pool_base_aum, pool_quote_aum)
				.ok_or(ArithmeticError::Overflow)?)
		}
	}
}
