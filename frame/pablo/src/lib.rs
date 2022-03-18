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

#[cfg(test)]
mod common_test_functions;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod stable_swap_tests;
#[cfg(test)]
mod uniswap_tests;

mod stable_swap;
mod uniswap;

#[frame_support::pallet]
pub mod pallet {
	use crate::{stable_swap::StableSwap, uniswap::Uniswap, PoolConfiguration::ConstantProduct};
	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::CurrencyFactory,
		defi::CurrencyPair,
		dex::{Amm, ConstantProductPoolInfo, StableSwapPoolInfo},
		math::{SafeAdd, SafeSub},
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		transactional, PalletId, RuntimeDebug,
	};

	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{AccountIdConversion, Convert, One, Zero},
		Permill,
	};

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
	pub enum PoolInitConfiguration<AssetId> {
		StableSwap {
			pair: CurrencyPair<AssetId>,
			amplification_coefficient: u16,
			fee: Permill,
			protocol_fee: Permill,
		},
		ConstantProduct {
			pair: CurrencyPair<AssetId>,
			fee: Permill,
			owner_fee: Permill,
		},
	}

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
	pub enum PoolConfiguration<AccountId, AssetId> {
		StableSwap(StableSwapPoolInfo<AccountId, AssetId>),
		ConstantProduct(ConstantProductPoolInfo<AccountId, AssetId>),
	}

	type AssetIdOf<T> = <T as Config>::AssetId;
	type BalanceOf<T> = <T as Config>::Balance;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type PoolIdOf<T> = <T as Config>::PoolId;
	type PoolConfigurationOf<T> =
		PoolConfiguration<<T as frame_system::Config>::AccountId, <T as Config>::AssetId>;
	type PoolInitConfigurationOf<T> = PoolInitConfiguration<<T as Config>::AssetId>;

	// TODO refactor event publishing with cu-23v2y3n
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
		InvalidPair,
		InvalidFees,
		AmpFactorMustBeGreaterThanZero,

		// ConstantProduct Specific: Possibly rename
		MissingAmount,
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

		/// Factory to create new lp-token.
		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId>;

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

		#[pallet::constant]
		type PalletId: Get<PalletId>;
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
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolConfigurationOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool.
		///
		/// Emits `PoolCreated` event when successful.
		// TODO: enable weight
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>, pool: PoolInitConfigurationOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = Self::do_create_pool(&who, pool)?;
			Ok(())
		}

		/// Execute a buy order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
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

		/// Execute a sell order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
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
		/// The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
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
		#[pallet::weight(10_000)]
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
		#[pallet::weight(10_000)]
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

	impl<T: Config> Pallet<T> {
		#[transactional]
		pub(crate) fn do_create_pool(
			who: &T::AccountId,
			init_config: PoolInitConfigurationOf<T>,
		) -> Result<T::PoolId, DispatchError> {
			match init_config {
				PoolInitConfiguration::StableSwap {
					pair,
					amplification_coefficient,
					fee,
					protocol_fee,
				} => {
					let pool_id = StableSwap::<T>::do_create_pool(
						&who,
						pair,
						amplification_coefficient,
						fee,
						protocol_fee,
					)?;
					Self::deposit_event(Event::PoolCreated { owner: who.clone(), pool_id });

					Ok(pool_id)
				},
				PoolInitConfiguration::ConstantProduct { pair, fee, owner_fee } => {
					let pool_id = Uniswap::<T>::do_create_pool(&who, pair, fee, owner_fee)?;
					Self::deposit_event(Event::PoolCreated { owner: who.clone(), pool_id });
					Ok(pool_id)
				},
			}
		}

		pub(crate) fn get_pool(
			pool_id: T::PoolId,
		) -> Result<PoolConfigurationOf<T>, DispatchError> {
			Pools::<T>::get(pool_id).ok_or_else(|| Error::<T>::PoolNotFound.into())
		}

		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
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
				ConstantProduct(constant_product_pool_info) => Ok(constant_product_pool_info.pair),
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
					StableSwap::<T>::get_exchange_value(
						&stable_swap_pool_info,
						&pool_account,
						asset_id,
						amount,
					),
				ConstantProduct(constant_product_pool_info) => Uniswap::<T>::get_exchange_value(
					&constant_product_pool_info,
					&pool_account,
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
					let mint_amount = StableSwap::<T>::add_liquidity(
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
						minted_lp: mint_amount,
					});
				},
				ConstantProduct(constant_product_pool_info) => {
					let mint_amount = Uniswap::<T>::add_liquidity(
						who,
						constant_product_pool_info,
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
						minted_lp: mint_amount,
					});
				},
			}
			// TODO refactor event publishing with cu-23v2y3n
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
						StableSwap::<T>::remove_liquidity(
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
				ConstantProduct(constant_product_pool_info) => {
					let (base_amount, quote_amount, updated_lp) = Uniswap::<T>::remove_liquidity(
						who,
						constant_product_pool_info,
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
			// TODO refactor event publishing with cu-23v2y3n
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
						StableSwap::<T>::do_compute_swap(&pool, &pool_account, quote_amount, true)?;

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
				ConstantProduct(constant_product_pool_info) => {
					let (base_amount, quote_amount_excluding_fees, lp_fees, owner_fees) =
						Uniswap::<T>::do_compute_swap(
							&constant_product_pool_info,
							&pool_account,
							pair,
							quote_amount,
							true,
						)?;
					let total_fees = lp_fees.safe_add(&owner_fees)?;
					let quote_amount_including_fees =
						quote_amount_excluding_fees.safe_add(&total_fees)?;

					ensure!(base_amount >= min_receive, Error::<T>::CannotRespectMinimumRequested);

					T::Assets::transfer(
						pair.quote,
						who,
						&pool_account,
						quote_amount_including_fees,
						keep_alive,
					)?;
					// NOTE(hussein-aitlance): no need to keep alive the pool account
					T::Assets::transfer(
						pair.quote,
						&pool_account,
						&constant_product_pool_info.owner,
						owner_fees,
						false,
					)?;
					T::Assets::transfer(pair.base, &pool_account, who, base_amount, false)?;

					Self::deposit_event(Event::<T>::Swapped {
						pool_id,
						who: who.clone(),
						base_asset: pair.base,
						quote_asset: pair.quote,
						base_amount,
						quote_amount: quote_amount_excluding_fees,
						fee: total_fees,
					});

					Ok(base_amount)
				},
			}

			// TODO refactor event publishing with cu-23v2y3n
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
				ConstantProduct(constant_product_pool) => {
					let pair = if asset_id == constant_product_pool.pair.base {
						constant_product_pool.pair
					} else {
						constant_product_pool.pair.swap()
					};
					let quote_amount = Self::get_exchange_value(pool_id, asset_id, amount)?;
					Self::exchange(who, pool_id, pair, quote_amount, T::Balance::zero(), keep_alive)
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
					Self::exchange(who, pool_id, pair, amount, Self::Balance::zero(), keep_alive)
				},
				ConstantProduct(constant_product_pool) => {
					let pair = if asset_id == constant_product_pool.pair.base {
						constant_product_pool.pair.swap()
					} else {
						constant_product_pool.pair
					};
					Self::exchange(who, pool_id, pair, amount, T::Balance::zero(), keep_alive)
				},
			}
		}
	}
}
