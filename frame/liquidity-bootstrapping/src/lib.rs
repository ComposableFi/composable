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

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

pub use crate::weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_maths::dex::constant_product::{compute_out_given_in, compute_spot_price};
	use composable_support::validation::{Validate, Validated};
	use composable_traits::{
		currency::LocalAssets,
		defi::CurrencyPair,
		dex::Amm,
		math::{SafeAdd, SafeSub},
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		transactional, PalletId, RuntimeDebug,
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use sp_arithmetic::traits::Saturating;
	use sp_runtime::{
		traits::{
			AccountIdConversion, BlockNumberProvider, CheckedMul, CheckedSub, Convert, One, Zero,
		},
		ArithmeticError, Permill,
	};

	#[derive(Copy, Clone, PartialEq, Eq)]
	pub enum SaleState {
		NotStarted,
		Ongoing,
		Ended,
	}

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Copy, Clone, PartialEq, Eq, TypeInfo)]
	pub struct Sale<BlockNumber> {
		/// Block at which the sale start.
		pub start: BlockNumber,
		/// Block at which the sale stop.
		pub end: BlockNumber,
		/// Initial weight of the base asset of the current pair.
		pub initial_weight: Permill,
		/// Final weight of the base asset of the current pair.
		pub final_weight: Permill,
	}

	impl<BlockNumber: TryInto<u64> + Ord + Copy + Saturating + SafeAdd + SafeSub> Sale<BlockNumber> {
		pub(crate) fn current_weights(
			&self,
			current_block: BlockNumber,
		) -> Result<(Permill, Permill), DispatchError> {
			/* NOTE(hussein-aitlahcen): currently only linear

			Linearly decrease the base asset initial_weight to final_weight.
			Quote asset weight is simple 1-base_asset_weight

				  Assuming final_weight < initial_weight
				  current_weight = initial_weight - (current - start) / (end - start) * (initial_weight - final_weight)
								 = initial_weight - normalized_current / sale_duration * weight_range
								 = initial_weight - point_in_sale * weight_range
			   */
			let normalized_current_block = current_block.safe_sub(&self.start)?;
			let point_in_sale = Permill::from_rational(
				normalized_current_block.try_into().map_err(|_| ArithmeticError::Overflow)?,
				self.duration().try_into().map_err(|_| ArithmeticError::Overflow)?,
			);
			let weight_range = self
				.initial_weight
				.checked_sub(&self.final_weight)
				.ok_or(ArithmeticError::Underflow)?;
			let current_base_weight = self
				.initial_weight
				.checked_sub(
					&point_in_sale.checked_mul(&weight_range).ok_or(ArithmeticError::Overflow)?,
				)
				.ok_or(ArithmeticError::Underflow)?;
			let current_quote_weight = Permill::one()
				.checked_sub(&current_base_weight)
				.ok_or(ArithmeticError::Underflow)?;
			Ok((current_base_weight, current_quote_weight))
		}
	}

	impl<BlockNumber: Copy + Saturating> Sale<BlockNumber> {
		pub(crate) fn duration(&self) -> BlockNumber {
			// NOTE(hussein-aitlahcen): end > start as previously checked by PoolIsValid.
			self.end.saturating_sub(self.start)
		}
	}

	impl<BlockNumber: Ord> Sale<BlockNumber> {
		pub(crate) fn state(&self, current_block: BlockNumber) -> SaleState {
			if current_block < self.start {
				SaleState::NotStarted
			} else if current_block >= self.end {
				SaleState::Ended
			} else {
				SaleState::Ongoing
			}
		}
	}

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Copy, Clone, PartialEq, Eq, TypeInfo)]
	pub struct Pool<AccountId, BlockNumber, AssetId> {
		/// Owner of the pool
		pub owner: AccountId,
		/// Asset pair of the pool along their weight.
		/// Base asset is the project token.
		/// Quote asset is the collateral token.
		pub pair: CurrencyPair<AssetId>,
		/// Sale period of the LBP.
		pub sale: Sale<BlockNumber>,
		/// Trading fees.
		pub fee: Permill,
	}

	#[derive(Copy, Clone, Encode, Decode, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
	pub struct PoolIsValid<T>(PhantomData<T>);

	impl<T: Config> Validate<PoolOf<T>, PoolIsValid<T>> for PoolIsValid<T> {
		fn validate(input: PoolOf<T>) -> Result<PoolOf<T>, &'static str> {
			if input.pair.base == input.pair.quote {
				return Err("Pair elements must be distinct.")
			}

			if input.sale.end <= input.sale.start {
				return Err("Sale end must be after start.")
			}

			if input.sale.duration() < T::MinSaleDuration::get() {
				return Err("Sale duration must be greater than minimum duration.")
			}

			if input.sale.duration() > T::MaxSaleDuration::get() {
				return Err("Sale duration must not exceed maximum duration.")
			}

			if input.sale.initial_weight < input.sale.final_weight {
				return Err("Initial weight must be greater than final weight.")
			}

			if input.sale.initial_weight > T::MaxInitialWeight::get() {
				return Err("Initial weight must not exceed the defined maximum.")
			}

			if input.sale.final_weight < T::MinFinalWeight::get() {
				return Err("Final weight must not be lower than the defined minimum.")
			}

			Ok(input)
		}
	}

	type AssetIdOf<T> = <T as Config>::AssetId;
	type BalanceOf<T> = <T as Config>::Balance;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type PoolIdOf<T> = <T as Config>::PoolId;
	type PoolOf<T> = Pool<AccountIdOf<T>, BlockNumberFor<T>, AssetIdOf<T>>;

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
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Amount of base asset deposited.
			base_amount: T::Balance,
			/// Amount of quote asset deposited.
			quote_amount: T::Balance,
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
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		PoolNotFound,
		PairMismatch,
		MustBeOwner,
		InvalidSaleState,
		InvalidAmount,
		CannotRespectMinimumRequested,
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

		/// Minimum duration for a sale.
		#[pallet::constant]
		type MinSaleDuration: Get<BlockNumberFor<Self>>;

		/// Maximum duration for a sale.
		#[pallet::constant]
		type MaxSaleDuration: Get<BlockNumberFor<Self>>;

		/// Maximum initial weight.
		#[pallet::constant]
		type MaxInitialWeight: Get<Permill>;

		/// Minimum final weight.
		#[pallet::constant]
		type MinFinalWeight: Get<Permill>;

		/// The origin allowed to create new pools.
		type AdminOrigin: EnsureOrigin<Self::Origin>;

		type WeightInfo: WeightInfo;
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
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool.
		///
		/// Emits `PoolCreated` event when successful.
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			pool: Validated<PoolOf<T>, PoolIsValid<T>>,
		) -> DispatchResult {
			let _ = T::AdminOrigin::ensure_origin(origin)?;
			let _ = Self::do_create_pool(pool)?;
			Ok(())
		}

		/// Execute a buy order on a pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::buy())]
		pub fn buy(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::buy(&who, pool_id, asset_id, amount, keep_alive)?;
			Ok(())
		}

		/// Execute a sell order on a pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(T::WeightInfo::sell())]
		pub fn sell(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
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
			pool_id: PoolIdOf<T>,
			pair: CurrencyPair<T::AssetId>,
			quote_amount: BalanceOf<T>,
			min_receive: BalanceOf<T>,
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

		/// Add liquidity to an LBP pool.
		///
		/// Only possible before the sale started.
		///
		/// Emits `LiquidityAdded` event when successful.
		#[pallet::weight(T::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			base_amount: BalanceOf<T>,
			quote_amount: BalanceOf<T>,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::add_liquidity(
				&who,
				pool_id,
				base_amount,
				quote_amount,
				BalanceOf::<T>::zero(),
				keep_alive,
			)?;
			Ok(())
		}

		/// Withdraw the remaining liquidity and destroy the pool.
		///
		/// Emits `PoolDeleted` event when successful.
		#[pallet::weight(T::WeightInfo::remove_liquidity())]
		pub fn remove_liquidity(origin: OriginFor<T>, pool_id: PoolIdOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as Amm>::remove_liquidity(
				&who,
				pool_id,
				BalanceOf::<T>::zero(),
				BalanceOf::<T>::zero(),
				BalanceOf::<T>::zero(),
			)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_create_pool(
			pool: Validated<PoolOf<T>, PoolIsValid<T>>,
		) -> Result<T::PoolId, DispatchError> {
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
					let pool_id = *pool_count;
					Pools::<T>::insert(pool_id, pool.clone().value());
					*pool_count = pool_id.safe_add(&T::PoolId::one())?;
					Ok(pool_id)
				})?;

			Self::deposit_event(Event::PoolCreated { pool_id, owner: pool.owner.clone() });

			Ok(pool_id)
		}

		pub(crate) fn get_pool_ensuring_sale_state(
			pool_id: T::PoolId,
			current_block: BlockNumberFor<T>,
			expected_sale_state: SaleState,
		) -> Result<PoolOf<T>, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			ensure!(
				pool.sale.state(current_block) == expected_sale_state,
				Error::<T>::InvalidSaleState
			);
			Ok(pool)
		}

		pub(crate) fn get_pool(pool_id: T::PoolId) -> Result<PoolOf<T>, DispatchError> {
			Pools::<T>::get(pool_id).ok_or_else(|| Error::<T>::PoolNotFound.into())
		}

		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}

		#[allow(dead_code)]
		pub(crate) fn do_spot_price(
			pool_id: T::PoolId,
			pair: CurrencyPair<AssetIdOf<T>>,
			current_block: BlockNumberFor<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::Ongoing)?;
			ensure!(pair == pool.pair, Error::<T>::PairMismatch);

			let weights = pool.sale.current_weights(current_block)?;

			let (wo, wi) =
				if pair.base == pool.pair.base { weights } else { (weights.1, weights.0) };

			let pool_account = Self::account_id(&pool_id);
			let bi = T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
			let bo = T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
			let base_unit = T::LocalAssets::unit::<u128>(pair.base)?;

			let spot_price = compute_spot_price(wi, wo, bi, bo, base_unit)?;

			Ok(T::Convert::convert(spot_price))
		}

		pub(crate) fn do_get_exchange(
			pool_id: T::PoolId,
			pair: CurrencyPair<AssetIdOf<T>>,
			current_block: BlockNumberFor<T>,
			quote_amount: BalanceOf<T>,
			apply_fees: bool,
		) -> Result<(BalanceOf<T>, BalanceOf<T>), DispatchError> {
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::Ongoing)?;

			ensure!(pair == pool.pair, Error::<T>::PairMismatch);
			ensure!(!quote_amount.is_zero(), Error::<T>::InvalidAmount);

			let weights = pool.sale.current_weights(current_block)?;

			let (wo, wi) =
				if pair.base == pool.pair.base { weights } else { (weights.1, weights.0) };

			let pool_account = Self::account_id(&pool_id);
			let ai = T::Convert::convert(quote_amount);
			let (ai_minus_fees, fees) = if apply_fees {
				let fees = pool.fee.mul_floor(ai);
				// Safe as fees is a fraction of ai
				(ai - fees, fees)
			} else {
				(ai, 0)
			};
			let bi = T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
			let bo = T::Convert::convert(T::Assets::balance(pair.base, &pool_account));

			let base_amount = compute_out_given_in(wi, wo, bi, bo, ai_minus_fees)?;

			Ok((T::Convert::convert(fees), T::Convert::convert(base_amount)))
		}
	}

	impl<T: Config> Amm for Pallet<T> {
		type AssetId = AssetIdOf<T>;
		type Balance = BalanceOf<T>;
		type AccountId = AccountIdOf<T>;
		type PoolId = PoolIdOf<T>;

		fn pool_exists(pool_id: Self::PoolId) -> bool {
			Pools::<T>::contains_key(pool_id)
		}

		fn currency_pair(
			pool_id: Self::PoolId,
		) -> Result<CurrencyPair<Self::AssetId>, DispatchError> {
			Ok(Self::get_pool(pool_id)?.pair)
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let pair = if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
			let current_block = frame_system::Pallet::<T>::current_block_number();
			let (_, base_amount) =
				Self::do_get_exchange(pool_id, pair, current_block, amount, false)?;
			Ok(base_amount)
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
		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			base_amount: Self::Balance,
			quote_amount: Self::Balance,
			_: Self::Balance,
			keep_alive: bool,
		) -> Result<(), DispatchError> {
			let current_block = frame_system::Pallet::<T>::current_block_number();
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::NotStarted)?;

			ensure!(pool.owner == *who, Error::<T>::MustBeOwner);
			ensure!(!base_amount.is_zero() && !quote_amount.is_zero(), Error::<T>::InvalidAmount);

			// NOTE(hussein-aitlahcen): as we only allow the owner to provide liquidity, we don't
			// mint any LP.
			let pool_account = Self::account_id(&pool_id);
			T::Assets::transfer(pool.pair.base, who, &pool_account, base_amount, keep_alive)?;
			T::Assets::transfer(pool.pair.quote, who, &pool_account, quote_amount, keep_alive)?;

			Self::deposit_event(Event::LiquidityAdded { pool_id, base_amount, quote_amount });

			Ok(())
		}

		#[transactional]
		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			_: Self::Balance,
			_: Self::Balance,
			_: Self::Balance,
		) -> Result<(), DispatchError> {
			let current_block = frame_system::Pallet::<T>::current_block_number();
			let pool =
				Self::get_pool_ensuring_sale_state(pool_id, current_block, SaleState::Ended)?;

			ensure!(pool.owner == *who, Error::<T>::MustBeOwner);

			let pool_account = Self::account_id(&pool_id);

			let repatriate = |a| -> Result<BalanceOf<T>, DispatchError> {
				let a_balance = T::Assets::balance(a, &pool_account);
				// NOTE(hussein-aitlahcen): not need to keep the pool account alive.
				T::Assets::transfer(a, &pool_account, who, a_balance, false)?;
				Ok(a_balance)
			};

			let base_amount = repatriate(pool.pair.base)?;
			let quote_amount = repatriate(pool.pair.quote)?;

			Pools::<T>::remove(pool_id);

			Self::deposit_event(Event::PoolDeleted { pool_id, base_amount, quote_amount });

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
			let current_block = frame_system::Pallet::<T>::current_block_number();
			let (_, base_amount) =
				Self::do_get_exchange(pool_id, pair, current_block, quote_amount, true)?;

			ensure!(base_amount >= min_receive, Error::<T>::CannotRespectMinimumRequested);

			let pool_account = Self::account_id(&pool_id);
			T::Assets::transfer(pair.quote, who, &pool_account, quote_amount, keep_alive)?;
			// NOTE(hussein-aitlance): no need to keep alive the pool account
			T::Assets::transfer(pair.base, &pool_account, who, base_amount, false)?;

			Self::deposit_event(Event::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: pair.base,
				quote_asset: pair.quote,
				base_amount,
				quote_amount,
			});

			Ok(base_amount)
		}
	}
}
