//
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		// TODO: enable me after this crate is stablized. todo macros are still denied in the release pipeline, but for
		// regular development allowed.
		// clippy::indexing_slicing,
		// clippy::todo,
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
		currency::CurrencyFactory,
		defi::{CurrencyPair, LiftedFixedBalance},
		dex::{ConstantProductPoolInfo, CurveAmm},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		PalletId,
	};
	use scale_info::TypeInfo;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedDiv, CheckedMul,
			CheckedSub, IntegerSquareRoot, One, Zero,
		},
		FixedPointNumber, FixedPointOperand, FixedU128, Permill,
	};
	use sp_std::{collections::btree_set::BTreeSet, fmt::Debug, iter::FromIterator};

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
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ One
			+ IntegerSquareRoot
			+ FixedPointOperand
			+ Into<LiftedFixedBalance>
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
		type CurrencyFactory: CurrencyFactory<<Self as Config>::AssetId>;
		type LpToken: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
			+ Inspect<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;
		type Precision: Get<FixedU128>;
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Debug
			+ CheckedAdd
			+ Zero
			+ One;
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
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

	/// Pair of assets supported by the pool
	#[pallet::storage]
	#[pallet::getter(fn pool_assets)]
	pub type PoolAssets<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, CurrencyPair<T::AssetId>>;

	/// Balance of asset for given pool excluding admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_balance)]
	#[allow(clippy::disallowed_type)]
	pub type PoolAssetBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Balance,
		ValueQuery,
		PoolAssetBalanceOnEmpty<T>,
	>;

	#[pallet::type_value]
	pub fn PoolAssetBalanceOnEmpty<T: Config>() -> T::Balance {
		Zero::zero()
	}

	/// Balance of asset for given pool including admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_total_balance)]
	#[allow(clippy::disallowed_type)]
	pub type PoolAssetTotalBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Balance,
		ValueQuery,
		PoolAssetTotalBalanceOnEmpty<T>,
	>;

	#[pallet::type_value]
	pub fn PoolAssetTotalBalanceOnEmpty<T: Config>() -> T::Balance {
		Zero::zero()
	}

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
		/// Pool with specified id is not found
		PoolNotFound,
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
			/// Added token amounts.
			token_amounts: Vec<T::Balance>,
			/// Charged fees.
			fees: Vec<T::Balance>,
			/// Invariant after liquidity addition.
			invariant: T::Balance,
			/// Updated lp token supply.
			token_supply: T::Balance,
			/// Amount of minted lp tokens.
			mint_amount: T::Balance,
		},

		/// Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
		LiquidityRemoved {
			/// Account id who removed liquidity.
			who: T::AccountId,
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Removed token amounts.
			token_amounts: Vec<T::Balance>,
			/// Charged fees.
			fees: Vec<T::Balance>,
			/// Updated lp token supply.
			token_supply: T::Balance,
		},

		/// Token exchange happened.
		TokenExchanged {
			/// Account id who exchanged token.
			who: T::AccountId,
			/// Pool id on which exchange done.
			pool_id: T::PoolId,
			/// Id of asset used as input.
			base_asset: T::AssetId,
			/// Amount of sent token.
			sent_amount: T::Balance,
			/// Id of asset used as output.
			quote_asset: T::AssetId,
			/// Amount of received token.
			received_amount: T::Balance,
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

		fn currency_pair(
			pool_id: Self::PoolId,
		) -> Result<CurrencyPair<Self::AssetId>, DispatchError> {
			Ok(PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?)
		}

		fn pool_exists(pool_id: T::PoolId) -> bool {
			Pools::<T>::contains_key(pool_id)
		}

		fn pool_count() -> T::PoolId {
			PoolCount::<T>::get()
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let mut base_asset = assets_pair.base;
			let mut quote_asset = assets_pair.quote;
			if asset_id == quote_asset {
				base_asset = assets_pair.quote;
				quote_asset = assets_pair.base;
			} else if asset_id != base_asset {
				return Err(Error::<T>::AssetNotFound.into())
			}
			let base_asset_balance = PoolAssetBalance::<T>::get(pool_id, base_asset);
			let quote_asset_balance = PoolAssetBalance::<T>::get(pool_id, quote_asset);
			let base_asset_balance_f = Self::to_fixed_point_balance(base_asset_balance);
			let quote_asset_balance_f = Self::to_fixed_point_balance(quote_asset_balance);
			let xp = vec![base_asset_balance_f, quote_asset_balance_f];

			let dx_f = Self::to_fixed_point_balance(amount);

			sp_std::if_std! {
				println!("xp {:?}", xp);
			}
			let xp_i = *xp.get(0).ok_or(Error::<T>::IndexOutOfRange)?;
			let xp_j = *xp.get(1).ok_or(Error::<T>::IndexOutOfRange)?;
			let dy_f = Self::get_y_out(dx_f, xp_i, xp_j).ok_or(Error::<T>::Math)?;

			sp_std::if_std! {
				println!("dx_f {:?}, xp_i {:?}, xp_j {:?}, dy_f {:?}", dx_f, xp_i, xp_j, dy_f);
			}
			let dy = Self::to_balance(dy_f)?;
			Ok(dy)
		}

		fn buy(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let mut exchange_asset = assets_pair.quote;
			if asset_id == assets_pair.quote {
				exchange_asset = assets_pair.base;
			} else if asset_id != assets_pair.base {
				return Err(Error::<T>::AssetNotFound.into())
			}
			// Since when buying asset user can't executed exchange as he don't know how much
			// amount of token he has to trade-in to get expected buy tokens.
			// So we compute price assuming user wants to sell instead of buy.
			// And then do exchange computed amount with token indices flipped.
			let dx = Self::get_exchange_value(pool_id, asset_id, amount)?;
			Self::update_balance(who, &pool_id, &exchange_asset, dx, &asset_id, amount)?;
			Ok(amount)
		}

		fn sell(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let dy = Self::exchange(who, pool_id, asset_id, amount, Self::Balance::zero())?;
			Ok(dy)
		}

		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			amounts: Vec<Self::Balance>,
			min_mint_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let zero = Self::Balance::zero();
			ensure!(amounts.len() == 2, Error::<T>::IndexOutOfRange);
			ensure!(
				amounts.iter().all(|&x| x >= zero),
				Error::<T>::AssetAmountMustBePositiveNumber
			);

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let pool_lp_asset = pool.lp_token;
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let mut balances = Vec::new();
			for asset_id in assets {
				balances.push(PoolAssetBalance::<T>::get(pool_id, asset_id));
			}

			let n_coins = assets.len();

			ensure!(n_coins == balances.len(), Error::<T>::InconsistentStorage);

			ensure!(n_coins == amounts.len(), Error::<T>::IndexOutOfRange);

			let old_balances: Vec<FixedU128> = balances
				.iter()
				.map(|b| {
					let b_u: u128 = (*b).into();
					FixedU128::saturating_from_integer(b_u)
				})
				.collect();
			let old_balance_a = old_balances.get(0).ok_or(Error::<T>::IndexOutOfRange)?;
			let old_balance_b = old_balances.get(1).ok_or(Error::<T>::IndexOutOfRange)?;
			let amount_a_u: u128 = (*amounts.get(0).ok_or(Error::<T>::IndexOutOfRange)?).into();
			let amount_b_u: u128 = (*amounts.get(1).ok_or(Error::<T>::IndexOutOfRange)?).into();
			let amount_a_desired = FixedU128::saturating_from_integer(amount_a_u);
			let amount_b_desired = FixedU128::saturating_from_integer(amount_b_u);
			let amount_a;
			let amount_b;

			if *old_balance_a == FixedU128::zero() && *old_balance_b == FixedU128::zero() {
				amount_a = amount_a_desired;
				amount_b = amount_b_desired;
			} else {
				let amount_b_optimal =
					Self::quote(amount_a_desired, *old_balance_a, *old_balance_b)
						.ok_or(Error::<T>::Math)?;

				if amount_b_optimal <= amount_b_desired {
					amount_a = amount_a_desired;
					amount_b = amount_b_optimal;
				} else {
					let amount_a_optimal =
						Self::quote(amount_b_desired, *old_balance_b, *old_balance_a)
							.ok_or(Error::<T>::Math)?;
					assert!(amount_a_optimal <= amount_a_desired);
					amount_a = amount_a_optimal;
					amount_b = amount_b_desired;
				}
			}

			let token_supply = T::LpToken::total_issuance(pool_lp_asset);
			let token_supply_u: u128 = token_supply.into();
			let token_supply_f = FixedU128::saturating_from_integer(token_supply_u);
			let mint_amount;
			if token_supply_u == 0 {
				let mint_amount_f = amount_a.checked_mul(&amount_b).ok_or(Error::<T>::Math)?;
				let mint_amount_b: T::Balance =
					mint_amount_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
				mint_amount = mint_amount_b.integer_sqrt();
			} else {
				let mint_a = amount_a
					.checked_mul(&token_supply_f)
					.ok_or(Error::<T>::Math)?
					.checked_div(&old_balance_a)
					.ok_or(Error::<T>::Math)?;
				let mint_b = amount_b
					.checked_mul(&token_supply_f)
					.ok_or(Error::<T>::Math)?
					.checked_div(&old_balance_b)
					.ok_or(Error::<T>::Math)?;
				let mint_amount_f = mint_a.min(mint_b);
				mint_amount = mint_amount_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
			}

			let asset_id_a = *assets.get(0).ok_or(Error::<T>::IndexOutOfRange)?;
			let asset_id_b = *assets.get(1).ok_or(Error::<T>::IndexOutOfRange)?;
			let new_balance_a = old_balance_a.checked_add(&amount_a).ok_or(Error::<T>::Math)?;
			let new_balance_b = old_balance_b.checked_add(&amount_b).ok_or(Error::<T>::Math)?;
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_a, |balance| -> DispatchResult {
				*balance = new_balance_a.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_b, |balance| -> DispatchResult {
				*balance = new_balance_b.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;

			ensure!(mint_amount >= min_mint_amount, Error::<T>::RequiredAmountNotReached);

			let new_token_supply =
				token_supply.checked_add(&mint_amount).ok_or(Error::<T>::Math)?;

			// Ensure that for all tokens user has sufficient amount
			for (i, amount) in amounts.iter().enumerate() {
				let asset_i = assets.get(i).ok_or(Error::<T>::IndexOutOfRange)?;
				ensure!(
					T::LpToken::balance(*asset_i, who) >= *amount,
					Error::<T>::InsufficientFunds
				);
			}
			// Transfer funds to pool
			for (i, amount) in amounts.iter().enumerate() {
				if amount > &zero {
					let asset_id = assets.get(i).ok_or(Error::<T>::IndexOutOfRange)?;
					Self::transfer_liquidity_into_pool(
						&Self::account_id(&pool_id),
						pool_id,
						who,
						*asset_id,
						*amount,
					)?;
				}
			}

			T::LpToken::mint_into(pool_lp_asset, who, mint_amount)?;

			let mut invariant = T::Balance::one();
			for asset_id in assets {
				invariant *= PoolAssetBalance::<T>::get(pool_id, asset_id);
			}

			Self::deposit_event(Event::LiquidityAdded {
				who: who.clone(),
				pool_id,
				token_amounts: amounts,
				fees: vec![],
				invariant,
				token_supply: new_token_supply,
				mint_amount,
			});

			Ok(())
		}

		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			amount: Self::Balance,
			min_amounts: Vec<Self::Balance>,
		) -> Result<(), DispatchError> {
			let zero = FixedU128::zero();
			let b_zero = Self::Balance::zero();
			ensure!(amount >= b_zero, Error::<T>::AssetAmountMustBePositiveNumber);
			let amount_u: u128 = amount.into();
			let amount_f = FixedU128::saturating_from_integer(amount_u);

			let min_amounts_f: Vec<FixedU128> = min_amounts
				.iter()
				.map(|b| {
					let b_u: u128 = (*b).into();
					FixedU128::saturating_from_integer(b_u)
				})
				.collect();

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let pool_lp_asset = pool.lp_token;
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let n_coins = assets.len();
			let mut balances = Vec::new();
			for asset_id in assets {
				balances.push(PoolAssetBalance::<T>::get(pool_id, asset_id));
			}

			ensure!(n_coins == balances.len(), Error::<T>::InconsistentStorage);

			ensure!(n_coins == min_amounts.len(), Error::<T>::IndexOutOfRange);

			let token_supply = T::LpToken::total_issuance(pool_lp_asset);
			let token_supply_u: u128 = token_supply.into();

			let mut amounts_f = vec![FixedU128::zero(); n_coins];

			for (index, asset_id) in assets.iter().enumerate() {
				let old_balance_u: u128 =
					(*balances.get(index).ok_or(Error::<T>::IndexOutOfRange)?).into();
				let old_balance = FixedU128::saturating_from_integer(old_balance_u);
				// value = old_balance * n_amount / token_supply
				let value = (|| {
					old_balance
						.checked_mul(&amount_f)?
						.checked_div(&FixedU128::saturating_from_integer(token_supply_u))
				})()
				.ok_or(Error::<T>::Math)?;
				let expected_min_amount =
					min_amounts_f.get(index).ok_or(Error::<T>::IndexOutOfRange)?;
				ensure!(value >= *expected_min_amount, Error::<T>::RequiredAmountNotReached);
				PoolAssetBalance::<T>::mutate(pool_id, asset_id, |balance| -> DispatchResult {
					*balance = old_balance
						.checked_sub(&value)
						.ok_or(Error::<T>::InsufficientFunds)?
						.checked_mul_int(1_u64)
						.ok_or(Error::<T>::Math)?
						.into();
					Ok(())
				})?;
				amounts_f.insert(index, value);
			}

			let amounts: Vec<T::Balance> = amounts_f
				.iter()
				.map(|b_f| Ok(b_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into()))
				.collect::<Result<Vec<T::Balance>, Error<T>>>()?;

			let new_token_supply = token_supply.checked_sub(&amount).ok_or(Error::<T>::Math)?;

			let fees = vec![T::Balance::zero(); n_coins];

			T::LpToken::burn_from(pool_lp_asset, who, amount)?;

			// Ensure that for all tokens we have sufficient amount
			for (index, asset_id) in assets.iter().enumerate() {
				let amount_i = *amounts.get(index).ok_or(Error::<T>::IndexOutOfRange)?;
				ensure!(
					T::LpToken::balance(*asset_id, &Self::account_id(&pool_id)) >= amount_i,
					Error::<T>::InsufficientFunds
				);
			}

			for i in 0..n_coins {
				let amount_f_i = *amounts_f.get(i).ok_or(Error::<T>::IndexOutOfRange)?;
				let amount_i = *amounts.get(i).ok_or(Error::<T>::IndexOutOfRange)?;
				if amount_f_i > zero {
					let asset_id = assets.get(i).ok_or(Error::<T>::IndexOutOfRange)?;
					Self::transfer_liquidity_from_pool(
						&Self::account_id(&pool_id),
						pool_id,
						*asset_id,
						who,
						amount_i,
					)?;
				}
			}

			Self::deposit_event(Event::LiquidityRemoved {
				who: who.clone(),
				pool_id,
				token_amounts: amounts,
				fees,
				token_supply: new_token_supply,
			});

			Ok(())
		}

		fn exchange(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			asset_id: Self::AssetId,
			dx: Self::Balance,
			min_dy: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let zero_b = Self::Balance::zero();
			ensure!(dx >= zero_b, Error::<T>::AssetAmountMustBePositiveNumber);

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;

			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let mut base_asset = assets_pair.base;
			let mut quote_asset = assets_pair.quote;
			if asset_id == quote_asset {
				base_asset = assets_pair.quote;
				quote_asset = assets_pair.base;
			} else if asset_id != base_asset {
				return Err(Error::<T>::AssetNotFound.into())
			}
			let dy = Self::get_exchange_value(pool_id, base_asset, dx)?;
			let dy_f = Self::to_fixed_point_balance(dy);
			let min_dy_f = Self::to_fixed_point_balance(min_dy);

			let fee_f: FixedU128 = pool.fee.into();
			let dy_fee_f = dy_f.checked_mul(&fee_f).ok_or(Error::<T>::Math)?;
			let dy_f = dy_f.checked_sub(&dy_fee_f).ok_or(Error::<T>::Math)?;
			ensure!(dy_f >= min_dy_f, Error::<T>::RequiredAmountNotReached);

			Self::update_balance(who, &pool_id, &base_asset, dx, &quote_asset, dy)?;
			let dy_fee = Self::to_balance(dy_fee_f)?;
			let dy = Self::to_balance(dy_f)?;

			Self::deposit_event(Event::TokenExchanged {
				who: who.clone(),
				pool_id,
				base_asset,
				sent_amount: dx,
				quote_asset,
				received_amount: dy,
				fee: dy_fee,
			});
			Ok(dy)
		}

		fn withdraw_admin_fees(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			admin_fee_account: &Self::AccountId,
		) -> Result<(), DispatchError> {
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let mut balances = Vec::new();
			for asset_id in assets {
				balances.push(PoolAssetBalance::<T>::get(pool_id, asset_id));
			}
			let mut total_balances = Vec::new();
			for asset_id in assets {
				total_balances.push(PoolAssetTotalBalance::<T>::get(pool_id, asset_id));
			}
			let n_coins = assets.len();

			ensure!(n_coins == balances.len(), Error::<T>::InconsistentStorage);

			let admin_fees = total_balances
				.iter()
				.zip(balances)
				.map(|(tb, b)| {
					let admin_fee = tb.checked_sub(&b).ok_or(Error::<T>::Math)?;

					Ok(admin_fee)
				})
				.collect::<Result<Vec<Self::Balance>, DispatchError>>()?;

			let assets = assets.clone();

			for (index, asset_id) in assets.iter().enumerate() {
				let total_balance =
					*total_balances.get(index).ok_or(Error::<T>::IndexOutOfRange)?;
				let admin_fee = *admin_fees.get(index).ok_or(Error::<T>::IndexOutOfRange)?;
				let new_total_balance =
					total_balance.checked_sub(&admin_fee).ok_or(Error::<T>::Math)?;
				PoolAssetTotalBalance::<T>::mutate(
					pool_id,
					asset_id,
					|total_balance| -> DispatchResult {
						*total_balance = new_total_balance;
						Ok(())
					},
				)?;
			}

			for (asset, amount) in assets.into_iter().zip(admin_fees.iter().copied()) {
				T::LpToken::transfer(
					*asset,
					&Self::account_id(&pool_id),
					admin_fee_account,
					amount,
					true,
				)?;
			}

			Self::deposit_event(Event::AdminFeesWithdrawn {
				who: who.clone(),
				pool_id,
				admin_fee_account: admin_fee_account.clone(),
				admin_fees,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn create_pool(
			who: &T::AccountId,
			assets: Vec<T::AssetId>,
			fee: Permill,
			_admin_fee: Permill,
		) -> Result<T::PoolId, DispatchError> {
			// Assets related checks
			ensure!(assets.len() > 1, Error::<T>::NotEnoughAssets);
			let unique_assets = BTreeSet::<T::AssetId>::from_iter(assets.iter().copied());
			ensure!(unique_assets.len() == assets.len(), Error::<T>::DuplicateAssets);

			// Add new pool
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
					let pool_id = *pool_count;

					Pools::<T>::try_mutate_exists(pool_id, |maybe_pool_info| -> DispatchResult {
						// We expect that PoolInfos have sequential keys.
						// No PoolInfo can have key greater or equal to PoolCount
						ensure!(maybe_pool_info.is_none(), Error::<T>::InconsistentStorage);
						let lp_asset = T::CurrencyFactory::reserve_lp_token_id()?;

						*maybe_pool_info = Some(ConstantProductPoolInfo {
							owner: who.clone(),
							lp_token: lp_asset,
							fee,
						});

						Ok(())
					})?;

					for asset_id in &assets {
						PoolAssetBalance::<T>::insert(pool_id, asset_id, T::Balance::zero());
						PoolAssetTotalBalance::<T>::insert(pool_id, asset_id, T::Balance::zero());
					}

					PoolAssets::<T>::try_mutate(pool_id, |pool_assets| -> DispatchResult {
						ensure!(pool_assets.is_none(), Error::<T>::InconsistentStorage);
						*pool_assets = Some(CurrencyPair::new(
							*assets.get(0).ok_or(Error::<T>::IndexOutOfRange)?,
							*assets.get(1).ok_or(Error::<T>::IndexOutOfRange)?,
						));
						Ok(())
					})?;

					*pool_count = pool_id
						.checked_add(&T::PoolId::one())
						.ok_or(Error::<T>::InconsistentStorage)?;

					Ok(pool_id)
				})?;

			Self::deposit_event(Event::PoolCreated { who: who.clone(), pool_id });

			Ok(pool_id)
		}

		/// Return pool information for given pool_id.
		pub fn get_pool_info(
			pool_id: T::PoolId,
		) -> Option<ConstantProductPoolInfo<T::AccountId, T::AssetId>> {
			Pools::<T>::get(pool_id)
		}

		/// given some amount of an asset and pair balances, returns an equivalent amount of the
		/// other asset
		pub fn quote(
			amount_a: FixedU128,
			balance_a: FixedU128,
			balance_b: FixedU128,
		) -> Option<FixedU128> {
			assert!(amount_a > FixedU128::zero());
			assert!(balance_a > FixedU128::zero() && balance_b > FixedU128::zero());
			// optimal amount_b
			amount_a.checked_mul(&balance_b)?.checked_div(&balance_a)
		}

		/// given an input amount of an asset and pair balances, returns the maximum output amount
		/// of the other asset
		pub fn get_y_out(
			dx_f: FixedU128,
			balance_x_f: FixedU128,
			balance_y_f: FixedU128,
		) -> Option<FixedU128> {
			assert!(dx_f > FixedU128::zero());
			assert!(balance_x_f > FixedU128::zero() && balance_y_f > FixedU128::zero());
			let numerator = dx_f.checked_mul(&balance_y_f)?;
			let denominator = balance_x_f.checked_add(&dx_f)?;
			numerator.checked_div(&denominator)
		}

		fn to_fixed_point_balance(balance: T::Balance) -> FixedU128 {
			let b_u: u128 = balance.into();
			FixedU128::saturating_from_integer(b_u)
		}

		fn to_balance(balance_f: FixedU128) -> Result<T::Balance, DispatchError> {
			Ok(balance_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into())
		}

		fn update_balance(
			who: &T::AccountId,
			pool_id: &T::PoolId,
			base_asset: &T::AssetId,
			d_base_amount: T::Balance,
			quote_asset: &T::AssetId,
			d_quote_amount: T::Balance,
		) -> Result<(), DispatchError> {
			ensure!(
				T::LpToken::balance(*base_asset, &who) >= d_base_amount,
				Error::<T>::InsufficientFunds
			);

			ensure!(
				T::LpToken::balance(*quote_asset, &Self::account_id(&pool_id)) >= d_quote_amount,
				Error::<T>::InsufficientFunds
			);
			let pool = Self::get_pool_info(*pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let base_asset_balance = PoolAssetBalance::<T>::get(pool_id, base_asset);
			let quote_asset_balance = PoolAssetBalance::<T>::get(pool_id, quote_asset);
			let base_asset_balance_f = Self::to_fixed_point_balance(base_asset_balance);
			let quote_asset_balance_f = Self::to_fixed_point_balance(quote_asset_balance);
			let d_base_amount_f = Self::to_fixed_point_balance(d_base_amount);
			let d_quote_amount_f = Self::to_fixed_point_balance(d_quote_amount);
			let new_base_asset_balance_f =
				base_asset_balance_f.checked_add(&d_base_amount_f).ok_or(Error::<T>::Math)?;
			let fee_f: FixedU128 = pool.fee.into();
			let amount_fee_f = d_quote_amount_f.checked_mul(&fee_f).ok_or(Error::<T>::Math)?;
			let quote_asset_tx_amount_f =
				d_quote_amount_f.checked_sub(&amount_fee_f).ok_or(Error::<T>::Math)?;
			let new_quote_asset_balance_f = quote_asset_balance_f
				.checked_sub(&quote_asset_tx_amount_f)
				.ok_or(Error::<T>::Math)?;
			PoolAssetBalance::<T>::mutate(pool_id, base_asset, |balance| -> DispatchResult {
				*balance = Self::to_balance(new_base_asset_balance_f)?;
				Ok(())
			})?;
			PoolAssetBalance::<T>::mutate(pool_id, quote_asset, |balance| -> DispatchResult {
				*balance = Self::to_balance(new_quote_asset_balance_f)?;
				Ok(())
			})?;
			let quote_asset_tx_amount = Self::to_balance(quote_asset_tx_amount_f)?;
			Self::transfer_liquidity_into_pool(
				&Self::account_id(&pool_id),
				*pool_id,
				&who,
				*base_asset,
				d_base_amount,
			)?;
			Self::transfer_liquidity_from_pool(
				&Self::account_id(&pool_id),
				*pool_id,
				*quote_asset,
				&who,
				quote_asset_tx_amount,
			)?;
			Ok(())
		}

		fn transfer_liquidity_into_pool(
			pool_account_id: &T::AccountId,
			pool_id: T::PoolId,
			source: &T::AccountId,
			destination_asset: T::AssetId,
			amount: T::Balance,
		) -> DispatchResult {
			T::LpToken::transfer(destination_asset, source, pool_account_id, amount, true)?;
			PoolAssetTotalBalance::<T>::mutate(
				pool_id,
				destination_asset,
				|total_balance| -> DispatchResult {
					*total_balance = total_balance.checked_add(&amount).ok_or(Error::<T>::Math)?;
					Ok(())
				},
			)?;
			Ok(())
		}

		fn transfer_liquidity_from_pool(
			pool_account_id: &T::AccountId,
			pool_id: T::PoolId,
			source_asset: T::AssetId,
			destination: &T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			T::LpToken::transfer(source_asset, pool_account_id, destination, amount, true)?;
			PoolAssetTotalBalance::<T>::mutate(
				pool_id,
				source_asset,
				|total_balance| -> DispatchResult {
					*total_balance = total_balance.checked_sub(&amount).ok_or(Error::<T>::Math)?;
					Ok(())
				},
			)?;
			Ok(())
		}

		pub fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}
	}
}
