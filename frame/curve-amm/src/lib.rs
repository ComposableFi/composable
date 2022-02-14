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
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
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
#![allow(clippy::all)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::{CurrencyFactory, RangeId},
		defi::CurrencyPair,
		dex::{CurveAmm, SafeConvert, StableSwapPoolInfo},
		math::{safe_multiply_by_rational, SafeArithmetic},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		transactional, PalletId,
	};
	use frame_system::WeightInfo;
	use scale_info::TypeInfo;
	use sp_runtime::{
		traits::{AccountIdConversion, CheckedAdd, Convert, One, Zero},
		ArithmeticError, Permill,
	};
	use sp_std::fmt::Debug;

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
			+ SafeArithmetic;
		type Number: Parameter + Copy + Eq + Ord + SafeArithmetic + Zero + One;
		type Convert: SafeConvert<Self::Number, Self::Balance>
			+ SafeConvert<Self::Balance, Self::Number>
			+ Convert<u16, Self::Number>
			+ Convert<Permill, Self::Number>;
		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId>;
		type Assets: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Inspect<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;
		type Precision: Get<Self::Number>;
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
			+ SafeArithmetic;

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
	#[allow(clippy::disallowed_type)]
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
		/// Given asset is not used in pool.
		AssetNotFound,
		/// Values in the storage are inconsistent
		InconsistentStorage,
		/// Not enough assets provided
		NotEnoughAssets,
		/// Some provided assets are not unique
		DuplicateAssets,
		/// Error occurred while performing math calculations
		Math,
		/// Specified asset amount is wrong
		AssetAmountMustBePositiveNumber,
		/// Required amount of some token did not reached during adding or removing liquidity
		RequiredAmountNotReached,
		/// Source does not have required amount of coins to complete operation
		InsufficientFunds,
		/// Specified index is out of range
		IndexOutOfRange,
		InvalidFees,
		InvalidPair,
		PoolNotFound,
		InvalidAmount,
		MissingAmount,
		PairMismatch,
		CannotRespectMinimumRequested,
		CouldNotComputeQuote,
		CouldNotComputeInvariant,
		ConversionError,
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

		/// Withdraw admin fees `Vec<T::Balance>` from pool `T::PoolId` by user `T::AccountId`
		AdminFeesWithdrawn {
			/// Account id which withdrew admin fee.
			who: T::AccountId,
			/// Pool id from which fee withdrew.
			pool_id: T::PoolId,
			/// Account id to which fee deposited.
			admin_fee_account: T::AccountId,
			/// Amounts of fees.
			admin_fees: Vec<T::Balance>,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> CurveAmm for Pallet<T> {
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
			let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
			let pool_base_aum = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::balance(pair.base, &pool_account),
			)
			.map_err(|_| Error::<T>::ConversionError)?;
			let pool_quote_aum = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::balance(pair.quote, &pool_account),
			)
			.map_err(|_| Error::<T>::ConversionError)?;
			let amount = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(amount)
				.map_err(|_| Error::<T>::ConversionError)?;
			let n = 2_u16;
			let ann = Self::get_ann(pool.amplification_coefficient, n)?;
			let d = Self::get_d(pool_base_aum, pool_quote_aum, ann)?;
			let new_base_amount = pool_base_aum.safe_add(&amount)?;
			let new_quote_amount = Self::get_quote(new_base_amount, ann, d)?;
			let exchange_value = pool_quote_aum.safe_sub(&new_quote_amount)?;
			let exchange_amount =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(exchange_value)
					.map_err(|_| Error::<T>::ConversionError)?;
			Ok(exchange_amount)
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
			let pair = if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
			// Since when buying asset user can't executed exchange as he don't know how much
			// amount of token he has to trade-in to get expected buy tokens.
			// So we compute price assuming user wants to sell instead of buy.
			// And then do exchange computed amount with token indices flipped.
			let dx = Self::get_exchange_value(pool_id, asset_id, amount)?;
			Self::exchange(who, pool_id, pair, dx, T::Balance::zero(), keep_alive)?;
			Ok(amount)
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
			let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
			let dy = Self::exchange(who, pool_id, pair, amount, Self::Balance::zero(), keep_alive)?;
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
			let zero = T::Number::zero();
			// pool supports only 2 assets
			let n = 2_u16;
			let pool = Self::get_pool(pool_id)?;
			let pool_account = Self::account_id(&pool_id);
			let pool_base_aum = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::balance(pool.pair.base, &pool_account),
			)
			.map_err(|_| Error::<T>::ConversionError)?;
			let pool_quote_aum = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::balance(pool.pair.quote, &pool_account),
			)
			.map_err(|_| Error::<T>::ConversionError)?;

			let base_amount =
				<T::Convert as SafeConvert<T::Balance, T::Number>>::convert(base_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let quote_amount =
				<T::Convert as SafeConvert<T::Balance, T::Number>>::convert(quote_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let lp_issued = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::total_issuance(pool.lp_token),
			)
			.map_err(|_| Error::<T>::ConversionError)?;
			let ann = Self::get_ann(pool.amplification_coefficient, n)?;
			let d0 = Self::get_d(pool_base_aum, pool_quote_aum, ann)?;
			let new_base_amount = pool_base_aum.safe_add(&base_amount)?;
			let new_quote_amount = pool_quote_aum.safe_add(&quote_amount)?;
			let d1 = Self::get_d(new_base_amount, new_quote_amount, ann)?;
			ensure!(d1 > d0, Error::<T>::AssetAmountMustBePositiveNumber);

			let (mint_amount, base_protocol_fee, quote_protocol_fee) = if lp_issued > zero {
				// Deposit x + withdraw y sould charge about same
				// fees as a swap. Otherwise, one could exchange w/o paying fees.
				// And this formula leads to exactly that equality
				// fee = pool.fee * n_coins / (4 * (n_coins - 1))
				let one = <T::Convert as Convert<u16, T::Number>>::convert(1_u16);
				let four = <T::Convert as Convert<u16, T::Number>>::convert(4_u16);
				// pool supports only two coins.
				let n = <T::Convert as Convert<u16, T::Number>>::convert(2_u16);
				let share = n.safe_div(&four.safe_mul(&n.safe_sub(&one)?)?)?;
				let fee = <T::Convert as Convert<Permill, T::Number>>::convert(pool.fee);
				let protocol_fee =
					<T::Convert as Convert<Permill, T::Number>>::convert(pool.protocol_fee);
				let fee = fee.safe_mul(&share)?;
				// let fee: T::Balance = T::Convert::convert(fee);

				// ideal_balance = d1 * old_balances[i] / d0
				let ideal_base_balance = d1.safe_mul(&pool_base_aum)?.safe_div(&d0)?;
				let ideal_quote_balance = d1.safe_mul(&pool_quote_aum)?.safe_div(&d0)?;

				let base_difference = Self::abs_difference(ideal_base_balance, new_base_amount)?;
				let quote_difference = Self::abs_difference(ideal_quote_balance, new_quote_amount)?;

				let base_fee = fee.safe_mul(&base_difference)?;
				let quote_fee = fee.safe_mul(&quote_difference)?;
				let base_protocol_fee = protocol_fee.safe_mul(&base_fee)?;
				let quote_protocol_fee = protocol_fee.safe_mul(&quote_fee)?;
				let new_base_balance = new_base_amount.safe_sub(&base_fee)?;
				let new_quote_balance = new_quote_amount.safe_sub(&quote_fee)?;

				let d2 = Self::get_d(new_base_balance, new_quote_balance, ann)?;
				let mint_amount = lp_issued.safe_mul(&d2.safe_sub(&d0)?)?.safe_div(&d0)?;
				(mint_amount, base_protocol_fee, quote_protocol_fee)
			} else {
				(d1, T::Number::zero(), T::Number::zero())
			};

			let mint_amount =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(mint_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let base_amount =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(base_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let quote_amount =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(quote_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let base_protocol_fee =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(base_protocol_fee)
					.map_err(|_| Error::<T>::ConversionError)?;
			let quote_protocol_fee =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(quote_protocol_fee)
					.map_err(|_| Error::<T>::ConversionError)?;
			ensure!(mint_amount >= min_mint_amount, Error::<T>::RequiredAmountNotReached);

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
			let pool_base_aum = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::balance(pool.pair.base, &pool_account),
			)
			.map_err(|_| Error::<T>::ConversionError)?;
			let pool_quote_aum = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::balance(pool.pair.quote, &pool_account),
			)
			.map_err(|_| Error::<T>::ConversionError)?;
			let lp_issued = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(
				T::Assets::total_issuance(pool.lp_token),
			)
			.map_err(|_| Error::<T>::ConversionError)?;

			let lp_amount = <T::Convert as SafeConvert<T::Balance, T::Number>>::convert(lp_amount)
				.map_err(|_| Error::<T>::ConversionError)?;
			let base_amount = lp_amount.safe_mul(&pool_base_aum)?.safe_div(&lp_issued)?;
			let quote_amount = lp_amount.safe_mul(&pool_quote_aum)?.safe_div(&lp_issued)?;
			let min_base_amount =
				<T::Convert as SafeConvert<T::Balance, T::Number>>::convert(min_base_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let min_quote_amount =
				<T::Convert as SafeConvert<T::Balance, T::Number>>::convert(min_quote_amount)
					.map_err(|_| Error::<T>::ConversionError)?;

			ensure!(
				base_amount >= min_base_amount && quote_amount >= min_quote_amount,
				Error::<T>::CannotRespectMinimumRequested
			);

			let total_issuance = lp_issued.safe_sub(&lp_amount)?;

			let base_amount =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(base_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let quote_amount =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(quote_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let lp_amount = <T::Convert as SafeConvert<T::Number, T::Balance>>::convert(lp_amount)
				.map_err(|_| Error::<T>::ConversionError)?;
			let total_issuance =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(total_issuance)
					.map_err(|_| Error::<T>::ConversionError)?;
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
			base_amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			// /!\ NOTE(hussein-aitlahcen): after this check, do not use pool.pair as the provided
			// pair might have been swapped
			ensure!(pair == pool.pair, Error::<T>::PairMismatch);
			let (base_amount, quote_amount_excluding_fees, lp_fees, protocol_fees) =
				Self::do_compute_swap(pool_id, pair, base_amount, true)?;

			ensure!(
				quote_amount_excluding_fees >= min_receive,
				Error::<T>::CannotRespectMinimumRequested
			);
			let pool_account = Self::account_id(&pool_id);
			let user_base_bal = T::Assets::balance(pool.pair.base, &who);
			let user_quote_bal = T::Assets::balance(pool.pair.quote, &who);
			sp_std::if_std! {
				println!("base {:?}, quote {:?}, user_base_bal {:?}, user_quote_bal {:?}, base_amount {:?}", pool.pair.base, pool.pair.quote, user_base_bal, user_quote_bal, base_amount);
			}
			T::Assets::transfer(pair.base, who, &pool_account, base_amount, keep_alive)?;

			// NOTE(hussein-aitlance): no need to keep alive the pool account
			T::Assets::transfer(pair.quote, &pool_account, &pool.owner, protocol_fees, false)?;
			T::Assets::transfer(
				pair.quote,
				&pool_account,
				who,
				quote_amount_excluding_fees,
				false,
			)?;
			Self::deposit_event(Event::<T>::Swapped {
				pool_id,
				who: who.clone(),
				base_asset: pair.base,
				quote_asset: pair.quote,
				base_amount,
				quote_amount: quote_amount_excluding_fees,
				fee: lp_fees.safe_add(&protocol_fees)?,
			});

			Ok(quote_amount_excluding_fees)
		}
	}

	impl<T: Config> Pallet<T> {
		#[transactional]
		pub fn do_create_pool(
			who: &T::AccountId,
			pair: CurrencyPair<T::AssetId>,
			amplification_coefficient: u16,
			fee: Permill,
			protocol_fee: Permill,
		) -> Result<T::PoolId, DispatchError> {
			ensure!(pair.base != pair.quote, Error::<T>::InvalidPair);

			let total_fees = fee.checked_add(&protocol_fee).ok_or(ArithmeticError::Overflow)?;
			ensure!(total_fees < Permill::one(), Error::<T>::InvalidFees);

			let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS)?;
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
							protocol_fee,
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
			Pools::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound.into())
		}

		/// Account of a pool
		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}

		/// Find `ann = amp * n^n` where `amp` - amplification coefficient,
		/// `n` - number of coins.
		pub fn get_ann(amp: u16, n: u16) -> Result<T::Number, DispatchError> {
			let mut ann = <T::Convert as Convert<u16, T::Number>>::convert(amp);
			for _ in 0..n {
				ann = ann.safe_mul(&<T::Convert as Convert<u16, T::Number>>::convert(n))?;
			}
			Ok(ann)
		}

		/// # Notes
		///
		/// D invariant calculation in non-overflowing integer operations iteratively
		///
		/// ```pseudocode
		///  A * sum(x_i) * n^n + D = A * D * n^n + D^(n+1) / (n^n * prod(x_i))
		/// ```
		///
		/// Converging solution:
		///
		/// ```pseudocode
		/// D[j + 1] = (A * n^n * sum(x_i) - D[j]^(n+1) / (n^n * prod(x_i))) / (A * n^n - 1)
		/// ```
		pub fn get_d(
			base_asset_aum: T::Number,
			quote_asset_aum: T::Number,
			ann: T::Number,
		) -> Result<T::Number, DispatchError> {
			let zero = T::Number::zero();
			let one = T::Number::one();
			let prec = T::Precision::get();
			let sum = base_asset_aum.safe_add(&quote_asset_aum)?;
			// currently only 2 assets are supported.
			if sum == zero {
				return Ok(zero)
			}

			let n = <T::Convert as Convert<u16, T::Number>>::convert(2_u16);
			let mut d = sum;

			for _ in 0..255 {
				let mut d_p = d;
				// d_p = d_p * d / (x * n)

				d_p = d_p.safe_mul(&d.safe_div(&base_asset_aum.safe_mul(&n)?)?)?;
				d_p = d_p.safe_mul(&d.safe_div(&quote_asset_aum.safe_mul(&n)?)?)?;

				let d_prev = d;
				// d = (ann * sum + d_p * n) * d / ((ann - 1) * d + (n + 1) * d_p)

				sp_std::if_std! {
						println!("d {:?} ", d);
				}
				let t1 = ann.safe_mul(&sum)?.safe_add(&d_p.safe_mul(&n)?)?;
				let t2 = ann
					.safe_sub(&one)?
					.safe_mul(&d)?
					.safe_add(&n.safe_add(&one)?.safe_mul(&d_p)?)?;
				d = t1.safe_mul(&d)?.safe_div(&t2)?;
				sp_std::if_std! {
						println!("t1 {:?}, t2 {:?}, d {:?} ", t1, t2, d);
				}

				if d > d_prev {
					if d.safe_sub(&d_prev)? <= prec {
						return Ok(d)
					}
				} else if d_prev.safe_sub(&d)? <= prec {
					return Ok(d)
				}
			}
			return Err(Error::<T>::CouldNotComputeInvariant.into())
		}

		/// See https://github.com/equilibrium-eosdt/equilibrium-curve-amm/blob/master/docs/deducing-get_y-formulas.pdf
		/// for detailed explanation about formulas this function uses.
		///
		/// # Notes
		///
		/// Done by solving quadratic equation iteratively.
		///
		/// ```pseudocode
		/// x_1^2 + x_1 * (sum' - (A * n^n - 1) * D / (A * n^n)) = D^(n+1) / (n^2n * prod' * A)
		/// x_1^2 + b * x_1 = c
		///
		/// x_1 = (x_1^2 + c) / (2 * x_1 + b)
		/// ```
		pub fn get_quote(
			new_base: T::Number,
			ann: T::Number,
			d: T::Number,
		) -> Result<T::Number, DispatchError> {
			let prec = T::Precision::get();
			let n = <T::Convert as Convert<u16, T::Number>>::convert(2_u16);
			let two = <T::Convert as Convert<u16, T::Number>>::convert(2_u16);
			// s and p are same as input base amount as pool supports only 2 assets.
			let s = new_base;
			let p = new_base;
			let d_ann = d.safe_div(&ann)?;
			let d_n = d.safe_div(&n)?;
			let b = s.safe_add(&d_ann)?; // substract d later, otherwise Underflows
			let c = d_ann.safe_mul(&d_n)?.safe_div(&p)?.safe_mul(&d_n)?;
			sp_std::if_std! {
				println!("s {:?}, p {:?}, d_ann {:?}, d_n {:?}, b {:?}, c {:?}",s, p, d_ann, d_n, b, c);
			}

			let mut y = d;

			for _ in 0..255 {
				let y_prev = y;
				// y = (y^2 + c) / (2y + b)
				let tt1 = y.safe_mul(&y)?.safe_add(&c)?;
				let tt2 = two.safe_mul(&y)?.safe_add(&b)?.safe_sub(&d)?;
				sp_std::if_std! {
					println!("tt1 {:?}, tt2 {:?}",tt1, tt2);
				}

				y = tt1.safe_div(&tt2)?;

				if y > y_prev {
					if y.safe_sub(&y_prev)? <= prec {
						return Ok(y)
					}
				} else if y_prev.safe_sub(&y)? <= prec {
					return Ok(y)
				}
			}

			return Err(Error::<T>::CouldNotComputeQuote.into())
		}

		fn abs_difference(
			new_balance: T::Number,
			old_balance: T::Number,
		) -> Result<T::Number, DispatchError> {
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
			base_amount: T::Balance,
			apply_fees: bool,
		) -> Result<(T::Balance, T::Balance, T::Balance, T::Balance), DispatchError> {
			let pool = Self::get_pool(pool_id)?;
			let quote_amount = Self::get_exchange_value(pool_id, pair.base, base_amount)?;
			let quote_amount =
				<T::Convert as SafeConvert<T::Balance, T::Number>>::convert(quote_amount)
					.map_err(|_| Error::<T>::ConversionError)?;
			let fee = <T::Convert as Convert<Permill, T::Number>>::convert(pool.fee);
			let protocol_fee =
				<T::Convert as Convert<Permill, T::Number>>::convert(pool.protocol_fee);

			let (lp_fee, protocol_fee) = if apply_fees {
				let lp_fee = fee.safe_mul(&quote_amount)?;
				let protocol_fee = protocol_fee.safe_mul(&lp_fee)?;
				(lp_fee, protocol_fee)
			} else {
				(T::Number::zero(), T::Number::zero())
			};

			let quote_amount_excluding_fees =
				quote_amount.safe_sub(&lp_fee)?.safe_sub(&protocol_fee)?;

			let quote_amount_excluding_fees =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(
					quote_amount_excluding_fees,
				)
				.map_err(|_| Error::<T>::ConversionError)?;
			let lp_fee = <T::Convert as SafeConvert<T::Number, T::Balance>>::convert(lp_fee)
				.map_err(|_| Error::<T>::ConversionError)?;
			let protocol_fee =
				<T::Convert as SafeConvert<T::Number, T::Balance>>::convert(protocol_fee)
					.map_err(|_| Error::<T>::ConversionError)?;
			Ok((base_amount, quote_amount_excluding_fees, lp_fee, protocol_fee))
		}
	}
}
