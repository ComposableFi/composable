#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
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

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
mod maths;

pub mod weights;
pub use crate::weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		maths::{compute_base, compute_d},
		weights::WeightInfo,
	};
	use codec::{Codec, FullCodec};
	use composable_support::math::safe::{safe_multiply_by_rational, SafeAdd, SafeSub};
	use composable_traits::{
		currency::{CurrencyFactory, RangeId},
		defi::CurrencyPair,
		dex::{Amm, StableSwapPoolInfo},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		transactional, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use scale_info::TypeInfo;
	use sp_runtime::{
		traits::{AccountIdConversion, CheckedAdd, Convert, One, Zero},
		ArithmeticError, Permill,
	};
	use sp_std::{fmt::Debug, ops::Mul};

	type PoolOf<T> =
		StableSwapPoolInfo<<T as frame_system::Config>::AccountId, <T as Config>::AssetId>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ Zero
			+ One
			+ SafeAdd
			+ SafeSub;
		type Convert: Convert<u128, Self::Balance> + Convert<Self::Balance, u128>;
		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId, Self::Balance>;
		type Assets: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Inspect<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Debug
			+ One
			+ Zero
			+ SafeAdd
			+ SafeSub;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Current number of pools (also ID for the next created pool)
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	// Absence of pool count is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery, PoolCountOnEmpty<T>>;

	#[pallet::type_value]
	pub fn PoolCountOnEmpty<T: Config>() -> T::PoolId {
		Zero::zero()
	}

	/// Existing pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, StableSwapPoolInfo<T::AccountId, T::AssetId>>;

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		AssetAmountMustBePositiveNumber,
		InvalidFees,
		InvalidPair,
		PoolNotFound,
		PairMismatch,
		CannotRespectMinimumRequested,
		AmpFactorMustBeGreaterThanZero,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
		PoolCreated {
			/// Account id of creator.
			who: T::AccountId,
			/// Id of newly created pool.
			pool_id: T::PoolId,
		},

		/// Liquidity added into the pool `T::PoolId` by `T::AccountId`.
		LiquidityAdded {
			/// Account id who added liquidity.
			who: T::AccountId,
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Base amount added to pool.
			base_amount: T::Balance,
			/// Quote amount added to pool.
			quote_amount: T::Balance,
			/// Amount of minted lp tokens.
			mint_amount: T::Balance,
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
		/// Emits `PoolCreated` event when successful.
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			pair: CurrencyPair<T::AssetId>,
			amplification_coefficient: u16,
			fee: Permill,
			owner_fee: Permill,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = Self::do_create_pool(&who, pair, amplification_coefficient, fee, owner_fee)?;
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

		/// Add liquidity to a stable-swap pool.
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

		/// Remove liquidity from stable-swap pool.
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

		fn lp_token(pool_id: Self::PoolId) -> Result<Self::AssetId, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			Ok(pool.lp_token)
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
			let pool_base_aum = T::Assets::balance(pair.base, &pool_account);
			let pool_quote_aum = T::Assets::balance(pair.quote, &pool_account);
			let amp = T::Convert::convert(pool.amplification_coefficient.into());
			let d = Self::get_d(pool_base_aum, pool_quote_aum, amp)?;
			let new_quote_amount = pool_quote_aum.safe_add(&amount)?;
			let new_base_amount = Self::get_base(new_quote_amount, amp, d)?;
			let exchange_value = pool_base_aum.safe_sub(&new_base_amount)?;
			Ok(exchange_value)
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
			let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
			// Since when buying asset user can't executed exchange as he don't know how much
			// amount of token he has to trade-in to get expected buy tokens.
			// So we compute price assuming user wants to sell instead of buy.
			// And then do exchange computed amount with token indices flipped.
			let dx = Self::get_exchange_value(pool_id, asset_id, amount)?;
			Self::exchange(who, pool_id, pair, dx, min_receive, keep_alive)?;
			Ok(amount)
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
			let pair = if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
			let dy = Self::exchange(who, pool_id, pair, amount, min_receive, keep_alive)?;
			Ok(dy)
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
			let zero = Self::Balance::zero();
			ensure!(base_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
			ensure!(quote_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
			// pool supports only 2 assets
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pool_base_aum = T::Assets::balance(pool.pair.base, &pool_account);
			let pool_quote_aum = T::Assets::balance(pool.pair.quote, &pool_account);

			let lp_issued = T::Assets::total_issuance(pool.lp_token);
			let amp = T::Convert::convert(pool.amplification_coefficient.into());
			let d0 = Self::get_d(pool_base_aum, pool_quote_aum, amp)?;
			let new_base_amount = pool_base_aum.safe_add(&base_amount)?;
			let new_quote_amount = pool_quote_aum.safe_add(&quote_amount)?;
			let d1 = Self::get_d(new_base_amount, new_quote_amount, amp)?;
			ensure!(d1 > d0, Error::<T>::AssetAmountMustBePositiveNumber);

			let (mint_amount, base_owner_fee, quote_owner_fee) = if lp_issued > zero {
				// Deposit x + withdraw y sould charge about same
				// fees as a swap. Otherwise, one could exchange w/o paying fees.
				// And this formula leads to exactly that equality
				// fee = pool.fee * n_coins / (4 * (n_coins - 1))
				// pool supports only two coins.
				let share: Permill = Permill::from_rational(2_u32, 4_u32);
				let fee = pool.fee.mul(share);

				let ideal_base_balance = T::Convert::convert(safe_multiply_by_rational(
					T::Convert::convert(d1),
					T::Convert::convert(pool_base_aum),
					T::Convert::convert(d0),
				)?);
				let ideal_quote_balance = T::Convert::convert(safe_multiply_by_rational(
					T::Convert::convert(d1),
					T::Convert::convert(pool_quote_aum),
					T::Convert::convert(d0),
				)?);

				let base_difference = Self::abs_difference(ideal_base_balance, new_base_amount)?;
				let quote_difference = Self::abs_difference(ideal_quote_balance, new_quote_amount)?;

				let base_fee = fee.mul_floor(T::Convert::convert(base_difference));
				let quote_fee = fee.mul_floor(T::Convert::convert(quote_difference));
				let base_owner_fee = T::Convert::convert(pool.owner_fee.mul_floor(base_fee));
				let quote_owner_fee = T::Convert::convert(pool.owner_fee.mul_floor(quote_fee));
				let base_fee = T::Convert::convert(base_fee);
				let quote_fee = T::Convert::convert(quote_fee);
				let new_base_balance = new_base_amount.safe_sub(&base_fee)?;
				let new_quote_balance = new_quote_amount.safe_sub(&quote_fee)?;

				let d2 = Self::get_d(new_base_balance, new_quote_balance, amp)?;
				let mint_amount = T::Convert::convert(safe_multiply_by_rational(
					T::Convert::convert(lp_issued),
					T::Convert::convert(d2.safe_sub(&d0)?),
					T::Convert::convert(d0),
				)?);
				(mint_amount, base_owner_fee, quote_owner_fee)
			} else {
				(d1, T::Balance::zero(), T::Balance::zero())
			};

			ensure!(mint_amount >= min_mint_amount, Error::<T>::CannotRespectMinimumRequested);

			T::Assets::transfer(pool.pair.base, who, &pool_account, base_amount, keep_alive)?;
			T::Assets::transfer(pool.pair.quote, who, &pool_account, quote_amount, keep_alive)?;
			// owner's fee is transferred upfront.
			T::Assets::transfer(
				pool.pair.base,
				&pool_account,
				&pool.owner,
				base_owner_fee,
				keep_alive,
			)?;
			T::Assets::transfer(
				pool.pair.quote,
				&pool_account,
				&pool.owner,
				quote_owner_fee,
				keep_alive,
			)?;
			T::Assets::mint_into(pool.lp_token, who, mint_amount)?;

			Self::deposit_event(Event::<T>::LiquidityAdded {
				who: who.clone(),
				pool_id,
				base_amount,
				quote_amount,
				mint_amount,
			});
			Ok(())
		}

		// NOTE: This function does not charge fees as it does not impact pool's invariant in
		// imbalanced way.
		#[transactional]
		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_base_amount: Self::Balance,
			min_quote_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let pool = Self::get_pool(pool_id)?;

			let pool_account = Self::account_id(&pool_id);
			let pool_base_aum = T::Assets::balance(pool.pair.base, &pool_account);
			let pool_quote_aum = T::Assets::balance(pool.pair.quote, &pool_account);
			let lp_issued = T::Assets::total_issuance(pool.lp_token);
			let base_amount = T::Convert::convert(safe_multiply_by_rational(
				T::Convert::convert(lp_amount),
				T::Convert::convert(pool_base_aum),
				T::Convert::convert(lp_issued),
			)?);
			let quote_amount = T::Convert::convert(safe_multiply_by_rational(
				T::Convert::convert(lp_amount),
				T::Convert::convert(pool_quote_aum),
				T::Convert::convert(lp_issued),
			)?);

			ensure!(
				base_amount >= min_base_amount && quote_amount >= min_quote_amount,
				Error::<T>::CannotRespectMinimumRequested
			);

			let total_issuance = lp_issued.safe_sub(&lp_amount)?;

			// NOTE(hussein-aitlance): no need to keep alive the pool account
			T::Assets::transfer(pool.pair.base, &pool_account, who, base_amount, false)?;
			T::Assets::transfer(pool.pair.quote, &pool_account, who, quote_amount, false)?;
			T::Assets::burn_from(pool.lp_token, who, lp_amount)?;
			Self::deposit_event(Event::<T>::LiquidityRemoved {
				pool_id,
				who: who.clone(),
				base_amount,
				quote_amount,
				total_issuance,
			});
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
			// /!\ NOTE(hussein-aitlahcen): after this check, do not use pool.pair as the provided
			// pair might have been swapped
			ensure!(pair == pool.pair, Error::<T>::PairMismatch);
			let (base_amount_excluding_fees, quote_amount, lp_fees, owner_fees) =
				Self::do_compute_swap(pool_id, pair, quote_amount, true)?;

			ensure!(
				base_amount_excluding_fees >= min_receive,
				Error::<T>::CannotRespectMinimumRequested
			);
			let pool_account = Self::account_id(&pool_id);
			T::Assets::transfer(pair.quote, who, &pool_account, quote_amount, keep_alive)?;

			// NOTE(hussein-aitlance): no need to keep alive the pool account
			T::Assets::transfer(pair.base, &pool_account, &pool.owner, owner_fees, false)?;
			T::Assets::transfer(pair.base, &pool_account, who, base_amount_excluding_fees, false)?;
			Self::deposit_event(Event::<T>::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: pair.base,
				quote_asset: pair.quote,
				base_amount: base_amount_excluding_fees,
				quote_amount,
				fee: lp_fees.safe_add(&owner_fees)?,
			});

			Ok(base_amount_excluding_fees)
		}
	}

	impl<T: Config> Pallet<T> {
		#[transactional]
		pub fn do_create_pool(
			who: &T::AccountId,
			pair: CurrencyPair<T::AssetId>,
			amplification_coefficient: u16,
			fee: Permill,
			owner_fee: Permill,
		) -> Result<T::PoolId, DispatchError> {
			ensure!(amplification_coefficient > 0, Error::<T>::AmpFactorMustBeGreaterThanZero);
			ensure!(pair.base != pair.quote, Error::<T>::InvalidPair);

			let total_fees = fee.checked_add(&owner_fee).ok_or(ArithmeticError::Overflow)?;
			ensure!(total_fees < Permill::one(), Error::<T>::InvalidFees);

			// TODO: pass from ED from above
			let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS, T::Balance::zero())?;
			// Add new pool
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
					let pool_id = *pool_count;

					Pools::<T>::insert(
						pool_id,
						StableSwapPoolInfo {
							owner: who.clone(),
							pair,
							lp_token,
							amplification_coefficient,
							fee,
							owner_fee,
						},
					);
					*pool_count = pool_id.safe_add(&T::PoolId::one())?;
					Ok(pool_id)
				})?;

			Self::deposit_event(Event::PoolCreated { who: who.clone(), pool_id });

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

		pub fn get_d(
			base_asset_aum: T::Balance,
			quote_asset_aum: T::Balance,
			amp_coeff: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let d = compute_d(
				T::Convert::convert(base_asset_aum),
				T::Convert::convert(quote_asset_aum),
				T::Convert::convert(amp_coeff),
			)?;

			Ok(T::Convert::convert(d))
		}

		pub fn get_base(
			new_quote: T::Balance,
			amp_coeff: T::Balance,
			d: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let base = compute_base(
				T::Convert::convert(new_quote),
				T::Convert::convert(amp_coeff),
				T::Convert::convert(d),
			)?;
			Ok(T::Convert::convert(base))
		}

		fn abs_difference(
			new_balance: T::Balance,
			old_balance: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let difference = if old_balance > new_balance {
				old_balance.safe_sub(&new_balance)
			} else {
				new_balance.safe_sub(&old_balance)
			}?;
			Ok(difference)
		}

		pub(crate) fn do_compute_swap(
			pool_id: T::PoolId,
			pair: CurrencyPair<T::AssetId>,
			quote_amount: T::Balance,
			apply_fees: bool,
		) -> Result<(T::Balance, T::Balance, T::Balance, T::Balance), DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let base_amount = Self::get_exchange_value(pool_id, pair.base, quote_amount)?;
			let base_amount_u: u128 = T::Convert::convert(base_amount);

			let (lp_fee, owner_fee) = if apply_fees {
				let lp_fee = pool.fee.mul_floor(base_amount_u);
				let owner_fee = pool.owner_fee.mul_floor(lp_fee);
				let lp_fee = T::Convert::convert(lp_fee);
				let owner_fee = T::Convert::convert(owner_fee);
				(lp_fee, owner_fee)
			} else {
				(T::Balance::zero(), T::Balance::zero())
			};

			let base_amount_excluding_fees = base_amount.safe_sub(&lp_fee)?;
			Ok((base_amount_excluding_fees, quote_amount, lp_fee, owner_fee))
		}
	}
}
