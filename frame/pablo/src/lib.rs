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
#![allow(dead_code)] // TODO: remove when most of the work is completed.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	use composable_maths::dex::{
		constant_product::{compute_in_given_out, compute_out_given_in},
		stable_swap::compute_d,
	};
	use composable_traits::{
		currency::LocalAssets,
		defi::CurrencyPair,
		dex::{Amm, StableSwapPoolInfo},
		math::{safe_multiply_by_rational, SafeAdd, SafeSub},
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		transactional, PalletId, RuntimeDebug,
	};
	use frame_system::pallet_prelude::OriginFor;
	use sp_runtime::{
		traits::{AccountIdConversion, Convert, One, Zero},
		Permill,
	};
	use sp_std::ops::Mul;

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
	pub enum PoolConfiguration<AccountId, AssetId> {
		StableSwap(StableSwapPoolInfo<AccountId, AssetId>),
	}

	type AssetIdOf<T> = <T as Config>::AssetId;
	type BalanceOf<T> = <T as Config>::Balance;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type PoolIdOf<T> = <T as Config>::PoolId;
	type PoolOf<T> =
		PoolConfiguration<<T as frame_system::Config>::AccountId, <T as Config>::AssetId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
		PoolCreated {
			/// Id of newly created pool.
			pool_id: T::PoolId,
			/// Owner of the pool.
			owner: T::AccountId,
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
			fee: T::Balance,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		PoolNotFound,
		PoolConfigurationNotSupported,
		PairMismatch,
		MustBeOwner,
		InvalidSaleState,
		InvalidAmount,
		CannotRespectMinimumRequested,
		AssetAmountMustBePositiveNumber,
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
			+ Ord;

		/// Type representing the Balance of an account.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ Zero
			+ SafeAdd
			+ SafeSub;

		/// An isomorphism: Balance<->u128
		type Convert: Convert<u128, BalanceOf<Self>> + Convert<BalanceOf<Self>, u128>;

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
			+ SafeAdd
			+ SafeSub;

		type LocalAssets: LocalAssets<AssetIdOf<Self>>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The origin allowed to create new pools.
		type AdminOrigin: EnsureOrigin<Self::Origin>;
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
	#[allow(clippy::disallowed_type)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery, PoolCountOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool.
		///
		/// Emits `PoolCreated` event when successful.
		// TODO: enable weight
		#[pallet::weight(10_000)]
		pub fn create(
			_origin: OriginFor<T>,
			_pool: PoolConfiguration<T::AccountId, T::AssetId>,
		) -> DispatchResult {
			Ok(())
		}

		/// Execute a buy order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
		pub fn buy(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_asset_id: T::AssetId,
			_amount: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Execute a sell order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
		pub fn sell(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_asset_id: T::AssetId,
			_amount: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Execute a specific swap operation.
		///
		/// The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
		pub fn swap(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_pair: CurrencyPair<T::AssetId>,
			_quote_amount: T::Balance,
			_min_receive: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Add liquidity to a stable-swap pool.
		///
		/// Emits `LiquidityAdded` event when successful.
		#[pallet::weight(10_000)]
		pub fn add_liquidity(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_base_amount: T::Balance,
			_quote_amount: T::Balance,
			_min_mint_amount: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Remove liquidity from stable-swap pool.
		///
		/// Emits `LiquidityRemoved` event when successful.
		#[pallet::weight(10_000)]
		pub fn remove_liquidity(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_lp_amount: T::Balance,
			_min_base_amount: T::Balance,
			_min_quote_amount: T::Balance,
		) -> DispatchResult {
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_create_pool(pool: PoolOf<T>) -> Result<T::PoolId, DispatchError> {
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
					let pool_id = *pool_count;
					Pools::<T>::insert(pool_id, pool.clone());
					*pool_count = pool_id.safe_add(&T::PoolId::one())?;
					Ok(pool_id)
				})?;
			match pool {
				PoolConfiguration::StableSwap(stable_swap_pool_info) => {
					let pool = stable_swap_pool_info;
					Self::deposit_event(Event::PoolCreated { pool_id, owner: pool.owner.clone() });
					Ok(pool_id)
				},
			}
		}

		pub(crate) fn get_pool(pool_id: T::PoolId) -> Result<PoolOf<T>, DispatchError> {
			Pools::<T>::get(pool_id).ok_or_else(|| Error::<T>::PoolNotFound.into())
		}

		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}

		fn get_exchange_value_for_stable_swap(
			pool: StableSwapPoolInfo<T::AccountId, T::AssetId>,
			pool_account: T::AccountId,
			asset_id: T::AssetId,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
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

		fn get_stable_swap_invariant(
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

		fn add_liquidity_for_stable_swap(
			who: &T::AccountId,
			pool: StableSwapPoolInfo<T::AccountId, T::AssetId>,
			pool_account: T::AccountId,
			base_amount: T::Balance,
			quote_amount: T::Balance,
			min_mint_amount: T::Balance,
			keep_alive: bool,
		) -> Result<T::Balance, DispatchError> {
			let zero = T::Balance::zero();
			ensure!(base_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
			ensure!(quote_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
			// pool supports only 2 assets
			let pool_base_aum = T::Assets::balance(pool.pair.base, &pool_account);
			let pool_quote_aum = T::Assets::balance(pool.pair.quote, &pool_account);

			let lp_issued = T::Assets::total_issuance(pool.lp_token);
			let amp = T::Convert::convert(pool.amplification_coefficient.into());
			let d0 = Self::get_stable_swap_invariant(pool_base_aum, pool_quote_aum, amp)?;
			let new_base_amount = pool_base_aum.safe_add(&base_amount)?;
			let new_quote_amount = pool_quote_aum.safe_add(&quote_amount)?;
			let d1 = Self::get_stable_swap_invariant(new_base_amount, new_quote_amount, amp)?;
			ensure!(d1 > d0, Error::<T>::AssetAmountMustBePositiveNumber);

			let (mint_amount, base_protocol_fee, quote_protocol_fee) = if lp_issued > zero {
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
				let base_protocol_fee = T::Convert::convert(pool.protocol_fee.mul_floor(base_fee));
				let quote_protocol_fee =
					T::Convert::convert(pool.protocol_fee.mul_floor(quote_fee));
				let base_fee = T::Convert::convert(base_fee);
				let quote_fee = T::Convert::convert(quote_fee);
				let new_base_balance = new_base_amount.safe_sub(&base_fee)?;
				let new_quote_balance = new_quote_amount.safe_sub(&quote_fee)?;

				let d2 = Self::get_stable_swap_invariant(new_base_balance, new_quote_balance, amp)?;
				let mint_amount = T::Convert::convert(safe_multiply_by_rational(
					T::Convert::convert(lp_issued),
					T::Convert::convert(d2.safe_sub(&d0)?),
					T::Convert::convert(d0),
				)?);
				(mint_amount, base_protocol_fee, quote_protocol_fee)
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
				base_protocol_fee,
				keep_alive,
			)?;
			T::Assets::transfer(
				pool.pair.quote,
				&pool_account,
				&pool.owner,
				quote_protocol_fee,
				keep_alive,
			)?;
			T::Assets::mint_into(pool.lp_token, who, mint_amount)?;
			Ok(mint_amount)
		}

		fn remove_liquidity_stable_swap(
			who: &T::AccountId,
			pool: StableSwapPoolInfo<T::AccountId, T::AssetId>,
			pool_account: T::AccountId,
			lp_amount: T::Balance,
			min_base_amount: T::Balance,
			min_quote_amount: T::Balance,
		) -> Result<
			(
				T::Balance, /* base_amount */
				T::Balance, /* quote_amount */
				T::Balance, /* updated_lp */
			),
			DispatchError,
		> {
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
			Ok((base_amount, quote_amount, total_issuance))
		}

		fn do_compute_swap(
			pool_id: T::PoolId,
			pair: CurrencyPair<T::AssetId>,
			quote_amount: T::Balance,
			apply_fees: bool,
			fee: Permill,
			protocol_fee: Permill,
		) -> Result<(T::Balance, T::Balance, T::Balance, T::Balance), DispatchError> {
			let base_amount = Self::get_exchange_value(pool_id, pair.base, quote_amount)?;
			let base_amount_u: u128 = T::Convert::convert(base_amount);

			let (lp_fee, protocol_fee) = if apply_fees {
				let lp_fee = fee.mul_floor(base_amount_u);
				// protocol_fee is computed based on lp_fee
				let protocol_fee = protocol_fee.mul_floor(lp_fee);
				let lp_fee = T::Convert::convert(lp_fee);
				let protocol_fee = T::Convert::convert(protocol_fee);
				(lp_fee, protocol_fee)
			} else {
				(T::Balance::zero(), T::Balance::zero())
			};

			let base_amount_excluding_fees = base_amount.safe_sub(&lp_fee)?;
			Ok((base_amount_excluding_fees, quote_amount, lp_fee, protocol_fee))
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
			match pool {
				PoolConfiguration::StableSwap(stable_swap_pool_info) =>
					Ok(stable_swap_pool_info.pair),
			}
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(stable_swap_pool_info) =>
					Self::get_exchange_value_for_stable_swap(
						stable_swap_pool_info,
						pool_account,
						asset_id,
						amount,
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
			match pool {
				PoolConfiguration::StableSwap(stable_swap_pool_info) => {
					let mint_amount = Self::add_liquidity_for_stable_swap(
						who,
						stable_swap_pool_info,
						pool_account,
						base_amount,
						quote_amount,
						min_mint_amount,
						keep_alive,
					)?;
					Self::deposit_event(Event::<T>::LiquidityAdded {
						who: who.clone(),
						pool_id,
						base_amount,
						quote_amount,
						mint_amount,
					});
				},
			}
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
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			match pool {
				PoolConfiguration::StableSwap(stable_swap_pool_info) => {
					let (base_amount, quote_amount, updated_lp) =
						Self::remove_liquidity_stable_swap(
							who,
							stable_swap_pool_info,
							pool_account,
							lp_amount,
							min_base_amount,
							min_quote_amount,
						)?;
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
			match pool {
				PoolConfiguration::StableSwap(pool) => {
					// /!\ NOTE(hussein-aitlahcen): after this check, do not use pool.pair as the
					// provided pair might have been swapped
					ensure!(pair == pool.pair, Error::<T>::PairMismatch);
					let (base_amount_excluding_fees, quote_amount, lp_fees, protocol_fees) =
						Self::do_compute_swap(
							pool_id,
							pair,
							quote_amount,
							true,
							pool.fee,
							pool.protocol_fee,
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
						&pool.owner,
						protocol_fees,
						false,
					)?;
					T::Assets::transfer(
						pair.base,
						&pool_account,
						who,
						base_amount_excluding_fees,
						false,
					)?;
					Self::deposit_event(Event::<T>::Swapped {
						pool_id,
						who: who.clone(),
						base_asset: pair.base,
						quote_asset: pair.quote,
						base_amount: base_amount_excluding_fees,
						quote_amount,
						fee: lp_fees.safe_add(&protocol_fees)?,
					});

					Ok(base_amount_excluding_fees)
				},
			}
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
			match pool {
				PoolConfiguration::StableSwap(pool) => {
					let pair =
						if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
					// Since when buying asset user can't executed exchange as he don't know how
					// much amount of token he has to trade-in to get expected buy tokens.
					// So we compute price assuming user wants to sell instead of buy.
					// And then do exchange computed amount with token indices flipped.
					let dx = Self::get_exchange_value(pool_id, asset_id, amount)?;
					Self::exchange(who, pool_id, pair, dx, T::Balance::zero(), keep_alive)?;
					Ok(amount)
				},
			}
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
			match pool {
				PoolConfiguration::StableSwap(pool) => {
					let pair =
						if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
					let dy = Self::exchange(
						who,
						pool_id,
						pair,
						amount,
						Self::Balance::zero(),
						keep_alive,
					)?;
					Ok(dy)
				},
			}
		}
	}
}
