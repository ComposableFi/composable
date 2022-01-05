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
		currency::CurrencyFactory,
		defi::{CurrencyPair, LiftedFixedBalance},
		dex::{CurveAmm, StableSwapPoolInfo},
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
			CheckedSub, One, Zero,
		},
		FixedPointNumber, FixedPointOperand, FixedU128, Permill,
	};
	use sp_std::{collections::btree_set::BTreeSet, fmt::Debug, iter::FromIterator};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: FullCodec
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
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ One
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
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Debug
			+ CheckedAdd
			+ One;
		type PoolTokenIndex: Copy + Debug + Eq + Into<u32> + From<u8>;
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Current number of pools (also ID for the next created pool)
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	// Absence of pool count is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery>;

	/// Existing pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, StableSwapPoolInfo<T::AccountId, T::AssetId>>;

	/// Pair of assets supported by the pool
	#[pallet::storage]
	#[pallet::getter(fn pool_assets)]
	pub type PoolAssets<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, CurrencyPair<T::AssetId>>;

	/// Balance of asset for given pool excluding admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_balance)]
	// Absence of pool asset balance is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type PoolAssetBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Balance,
		ValueQuery,
	>;

	/// Balance of asset for given pool including admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_total_balance)]
	// Absence of pool asset balance is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type PoolAssetTotalBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Balance,
		ValueQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Could not create new asset
		AssetNotCreated,
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
		/// The `AssetChecker` can use this error in case it can't provide better error
		ExternalAssetCheckFailed,
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
			/// Index of sent token.
			sent_token_index: T::PoolTokenIndex,
			/// Amount of sent token.
			sent_amount: T::Balance,
			/// Index of received token.
			received_token_index: T::PoolTokenIndex,
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
		type PoolTokenIndex = T::PoolTokenIndex;

		fn pool_exists(pool_id: T::PoolId) -> bool {
			Pools::<T>::contains_key(pool_id)
		}

		fn pool_count() -> T::PoolId {
			PoolCount::<T>::get()
		}

		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			amounts: Vec<Self::Balance>,
			min_mint_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let zero = Self::Balance::zero();
			ensure!(
				amounts.iter().all(|&x| x >= zero),
				Error::<T>::AssetAmountMustBePositiveNumber
			);

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let pool_lp_asset = pool.lp_token;
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let old_balances = assets
				.iter()
				.map(|asset_id| {
					let balance = PoolAssetBalance::<T>::get(pool_id, asset_id);
					let balance: u128 = (balance).into();
					FixedU128::saturating_from_integer(balance)
				})
				.collect::<Vec<FixedU128>>();

			let n_coins = assets.len();

			ensure!(assets.len() == amounts.len(), Error::<T>::IndexOutOfRange);

			let amplification_coefficient = pool.amplification_coefficient;
			let ann = Self::get_ann(amplification_coefficient, n_coins).ok_or(Error::<T>::Math)?;

			let d0 = Self::get_d(&old_balances, ann).ok_or(Error::<T>::Math)?;

			let token_supply = T::LpToken::total_issuance(pool_lp_asset);
			let token_supply_u: u128 = token_supply.into();
			let token_supply_f = FixedU128::saturating_from_integer(token_supply_u);

			let mut new_balances = old_balances
				.iter()
				.zip(&amounts)
				.map(|(balance, amount)| -> Result<_, _> {
					if token_supply == zero {
						ensure!(amount > &zero, Error::<T>::AssetAmountMustBePositiveNumber);
					}
					balance
						.checked_add(&FixedU128::saturating_from_integer(*amount))
						.ok_or(Error::<T>::Math)
				})
				.collect::<Result<Vec<_>, _>>()?;

			let d1 = Self::get_d(&new_balances, ann).ok_or(Error::<T>::Math)?;
			ensure!(d1 > d0, Error::<T>::AssetAmountMustBePositiveNumber);

			let d1_b = d1.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
			let mut fees = vec![FixedU128::zero(); n_coins];

			// Only account for fees if we are not the first to deposit
			let mint_amount: <T as Config>::Balance = if token_supply > zero {
				// Deposit x + withdraw y would charge about same
				// fees as a swap. Otherwise, one could exchange w/o paying fees.
				// And this formula leads to exactly that equality
				// fee = pool.fee * n_coins / (4 * (n_coins - 1))
				let one = FixedU128::saturating_from_integer(1_u8);
				let four = FixedU128::saturating_from_integer(4_u8);
				let n_coins_f = FixedU128::saturating_from_integer(n_coins as u128);
				let fee_f: FixedU128 = pool.fee.into();
				let fee_f = fee_f
					.checked_mul(&n_coins_f)
					.ok_or(Error::<T>::Math)?
					.checked_div(
						&four
							.checked_mul(&n_coins_f.checked_sub(&one).ok_or(Error::<T>::Math)?)
							.ok_or(Error::<T>::Math)?,
					)
					.ok_or(Error::<T>::Math)?;
				let admin_fee_f: FixedU128 = pool.admin_fee.into();

				for (((asset_id, &old_balance), new_balance), fee) in assets
					.iter()
					.zip(&old_balances)
					.zip(new_balances.iter_mut())
					.zip(fees.iter_mut())
				{
					// ideal_balance = d1 * old_balances[i] / d0
					let ideal_balance = d1
						.checked_mul(&old_balance)
						.map(|x| x.checked_div(&d0))
						.flatten()
						.ok_or(Error::<T>::Math)?;

					// difference = abs(ideal_balance - new_balance)
					let difference = if ideal_balance > *new_balance {
						ideal_balance.checked_sub(&new_balance)
					} else {
						new_balance.checked_sub(&ideal_balance)
					}
					.ok_or(Error::<T>::Math)?;

					*fee = fee_f.checked_mul(&difference).ok_or(Error::<T>::Math)?;

					// new_pool_balance = new_balance - (fees[i] * admin_fee)
					let new_pool_balance = new_balance
						.checked_sub(&fee.checked_mul(&admin_fee_f).ok_or(Error::<T>::Math)?)
						.ok_or(Error::<T>::Math)?;

					PoolAssetBalance::<T>::mutate(
						pool_id,
						asset_id,
						|balance| -> DispatchResult {
							*balance = new_pool_balance
								.checked_mul_int(1_u64)
								.ok_or(Error::<T>::Math)?
								.into();
							Ok(())
						},
					)?;

					*new_balance = new_balance.checked_sub(&fee).ok_or(Error::<T>::Math)?;
				}

				let d2 = Self::get_d(&new_balances, ann).ok_or(Error::<T>::Math)?;

				// mint_amount = token_supply * (d2 - d0) / d0
				token_supply_f
					.checked_mul(
						&d2.checked_sub(&d0)
							.ok_or(Error::<T>::Math)?
							.checked_div(&d0)
							.ok_or(Error::<T>::Math)?,
					)
					.ok_or(Error::<T>::Math)?
					.checked_mul_int(1_u64)
					.ok_or(Error::<T>::Math)?
					.into()
			} else {
				for (asset_id, new_balance) in assets.iter().zip(&new_balances) {
					PoolAssetBalance::<T>::mutate(
						pool_id,
						asset_id,
						|balance| -> DispatchResult {
							*balance =
								new_balance.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
							Ok(())
						},
					)?;
				}
				d1_b
			};

			ensure!(mint_amount >= min_mint_amount, Error::<T>::RequiredAmountNotReached);

			let new_token_supply =
				token_supply.checked_add(&mint_amount).ok_or(Error::<T>::Math)?;

			// Ensure that for all tokens user has sufficient amount
			for (amount, asset) in amounts.iter().zip(assets) {
				ensure!(T::LpToken::balance(*asset, who) >= *amount, Error::<T>::InsufficientFunds);
			}

			// Transfer funds to pool
			for (i, amount) in amounts.iter().enumerate() {
				if amount > &zero {
					Self::transfer_liquidity_into_pool(
						&Self::account_id(&pool_id),
						pool_id,
						who,
						i,
						*amount,
					)?;
				}
			}

			T::LpToken::mint_into(pool_lp_asset, who, mint_amount)?;
			let fees: Vec<T::Balance> = fees
				.iter()
				.map(|b_f| Ok(b_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into()))
				.collect::<Result<Vec<T::Balance>, Error<T>>>()?;

			Self::deposit_event(Event::LiquidityAdded {
				who: who.clone(),
				pool_id,
				token_amounts: amounts,
				fees,
				invariant: d1_b,
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

			let amount_f = {
				let amount_u: u128 = amount.into();
				FixedU128::saturating_from_integer(amount_u)
			};

			let min_amounts_f: Vec<FixedU128> = min_amounts
				.iter()
				.map(|balance| {
					let balance: u128 = (*balance).into();
					FixedU128::saturating_from_integer(balance)
				})
				.collect();

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let pool_lp_asset = pool.lp_token;
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let balances: Vec<<T as Config>::Balance> = assets
				.iter()
				.copied()
				.map(|asset_id| PoolAssetBalance::<T>::get(pool_id, asset_id))
				.collect();

			ensure!(assets.len() == min_amounts.len(), Error::<T>::IndexOutOfRange);

			let token_supply = T::LpToken::total_issuance(pool_lp_asset);
			let token_supply_u: u128 = token_supply.into();

			let amounts_f = assets
				.iter()
				.zip(balances)
				.zip(min_amounts_f)
				.map(
					|((asset_id, old_balance), min_amount_f)| -> Result<FixedU128, DispatchError> {
						let old_balance = {
							let old_balance: u128 = old_balance.into();
							FixedU128::saturating_from_integer(old_balance)
						};
						// value = old_balance * n_amount / token_supply
						let value = old_balance
							.checked_mul(&amount_f)
							.ok_or(Error::<T>::Math)?
							.checked_div(&FixedU128::saturating_from_integer(token_supply_u))
							.ok_or(Error::<T>::Math)?;

						ensure!(value >= min_amount_f, Error::<T>::RequiredAmountNotReached);

						PoolAssetBalance::<T>::mutate(
							pool_id,
							asset_id,
							|balance| -> DispatchResult {
								*balance = old_balance
									.checked_sub(&value)
									.ok_or(Error::<T>::InsufficientFunds)?
									.checked_mul_int(1_u64)
									.ok_or(Error::<T>::Math)?
									.into();
								Ok(())
							},
						)?;
						Ok(value)
					},
				)
				.collect::<Result<Vec<_>, _>>()?;

			let amounts: Vec<T::Balance> = amounts_f
				.iter()
				.map(|b_f| b_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math).map(T::Balance::from))
				.collect::<Result<Vec<T::Balance>, Error<T>>>()?;

			let new_token_supply = token_supply.checked_sub(&amount).ok_or(Error::<T>::Math)?;

			let fees = vec![T::Balance::zero(); assets.len()];

			T::LpToken::burn_from(pool_lp_asset, who, amount)?;

			// Ensure that for all tokens we have sufficient amount
			for (asset_id, amount) in assets.iter().zip(&amounts) {
				ensure!(
					T::LpToken::balance(*asset_id, &Self::account_id(&pool_id)) >= *amount,
					Error::<T>::InsufficientFunds
				);
			}

			for (index, (amount_f, amount)) in amounts_f.iter().zip(&amounts).enumerate() {
				if amount_f > &zero {
					Self::transfer_liquidity_from_pool(
						&Self::account_id(&pool_id),
						pool_id,
						index,
						who,
						*amount,
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
			i_token: T::PoolTokenIndex,
			j_token: T::PoolTokenIndex,
			dx: Self::Balance,
			min_dy: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let prec = T::Precision::get();
			let zero_b = Self::Balance::zero();
			ensure!(dx >= zero_b, Error::<T>::AssetAmountMustBePositiveNumber);

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let i = i_token.into() as usize;
			let j = j_token.into() as usize;

			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let mut balances = Vec::new();
			for asset_id in assets {
				balances.push(PoolAssetBalance::<T>::get(pool_id, asset_id));
			}
			let n_coins = assets.len();

			ensure!(i < n_coins && j < n_coins, Error::<T>::IndexOutOfRange);

			let dx_u: u128 = dx.into();
			let dx_f = FixedU128::saturating_from_integer(dx_u);
			let min_dy_u: u128 = min_dy.into();
			let min_dy_f = FixedU128::saturating_from_integer(min_dy_u);

			let xp: Vec<FixedU128> = balances
				.iter()
				.map(|b| {
					let b_u: u128 = (*b).into();
					FixedU128::saturating_from_integer(b_u)
				})
				.collect();

			// xp[i] + dx
			let xp_i = xp.get(i).ok_or(Error::<T>::InconsistentStorage)?;
			let x = xp_i.checked_add(&dx_f).ok_or(Error::<T>::Math)?;

			let amp_f = pool.amplification_coefficient;
			let ann = Self::get_ann(amp_f, n_coins).ok_or(Error::<T>::Math)?;
			let y = Self::get_y(i, j, x, &xp, ann).ok_or(Error::<T>::Math)?;

			// -1 just in case there were some rounding errors
			// dy = xp[j] - y - 1
			let xp_j = xp.get(j).ok_or(Error::<T>::InconsistentStorage)?;
			let dy_f = xp_j
				.checked_sub(&y)
				.ok_or(Error::<T>::Math)?
				.checked_sub(&prec)
				.ok_or(Error::<T>::Math)?;

			let fee_f: FixedU128 = pool.fee.into();
			let dy_fee_f = dy_f.checked_mul(&fee_f).ok_or(Error::<T>::Math)?;
			let dy_fee = dy_fee_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
			let dy_f = dy_f.checked_sub(&dy_fee_f).ok_or(Error::<T>::Math)?;
			ensure!(dy_f >= min_dy_f, Error::<T>::RequiredAmountNotReached);

			let admin_fee_f: FixedU128 = pool.admin_fee.into();
			let dy_admin_fee_f = dy_fee_f.checked_mul(&admin_fee_f).ok_or(Error::<T>::Math)?;
			let dy: Self::Balance = dy_f.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
			let asset_id_i = assets.get(i).ok_or(Error::<T>::InconsistentStorage)?;
			let asset_id_j = assets.get(j).ok_or(Error::<T>::InconsistentStorage)?;
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_i, |balance| -> DispatchResult {
				*balance = x.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;
			// When rounding errors happen, we undercharge admin fee in favor of LP
			let bal_j = xp_j
				.checked_sub(&dy_f)
				.ok_or(Error::<T>::Math)?
				.checked_sub(&dy_admin_fee_f)
				.ok_or(Error::<T>::Math)?;
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_j, |balance| -> DispatchResult {
				*balance = bal_j.checked_mul_int(1_u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;

			ensure!(T::LpToken::balance(*asset_id_i, &who) >= dx, Error::<T>::InsufficientFunds);

			ensure!(
				T::LpToken::balance(*asset_id_j, &Self::account_id(&pool_id)) >= dy,
				Error::<T>::InsufficientFunds
			);

			Self::transfer_liquidity_into_pool(&Self::account_id(&pool_id), pool_id, &who, i, dx)?;
			Self::transfer_liquidity_from_pool(&Self::account_id(&pool_id), pool_id, j, &who, dy)?;

			Self::deposit_event(Event::TokenExchanged {
				who: who.clone(),
				pool_id,
				sent_token_index: i_token,
				sent_amount: dx,
				received_token_index: j_token,
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

			for ((asset_id, total_balance), admin_fee) in
				assets.iter().zip(&total_balances).zip(&admin_fees)
			{
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
			amplification_coefficient: FixedU128,
			fee: Permill,
			admin_fee: Permill,
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
						let lp_asset = T::CurrencyFactory::create()?;

						*maybe_pool_info = Some(StableSwapPoolInfo {
							owner: who.clone(),
							lp_token: lp_asset,
							amplification_coefficient,
							fee,
							admin_fee,
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
		) -> Option<StableSwapPoolInfo<T::AccountId, T::AssetId>> {
			Pools::<T>::get(pool_id)
		}

		/// Find `ann = amp * n^n` where `amp` - amplification coefficient,
		/// `n` - number of coins.
		pub fn get_ann(amp: FixedU128, n: usize) -> Option<FixedU128> {
			let n_coins = FixedU128::saturating_from_integer(n as u128);
			let mut ann = amp;
			for _ in 0..n {
				ann = ann.checked_mul(&n_coins)?;
			}
			Some(ann)
		}
		/// Find `d` preserving StableSwap invariant.
		/// Here `d` - total amount of coins when they have an equal price,
		/// `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`,
		/// where `n` is number of coins.
		///
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
		pub fn get_d(xp_f: &[FixedU128], ann_f: FixedU128) -> Option<FixedU128> {
			let zero = FixedU128::zero();
			let one = FixedU128::one();
			let prec = T::Precision::get();
			let n = FixedU128::saturating_from_integer(u128::try_from(xp_f.len()).ok()?);
			let sum = xp_f.iter().try_fold(zero, |s, x| s.checked_add(x))?;
			if sum == zero {
				return Some(zero)
			}
			let mut d = sum;

			for _ in 0..255 {
				let mut d_p = d;
				for x in xp_f.iter() {
					// d_p = d_p * d / (x * n)
					d_p = d_p.checked_mul(&d)?.checked_div(&x.checked_mul(&n)?)?;
				}
				let d_prev = d;
				// d = (ann * sum + d_p * n) * d / ((ann - 1) * d + (n + 1) * d_p)
				d = ann_f.checked_mul(&sum)?.checked_add(&d_p.checked_mul(&n)?)?.checked_mul(
					&d.checked_div(
						&ann_f
							.checked_sub(&one)?
							.checked_mul(&d)?
							.checked_add(&n.checked_add(&one)?.checked_mul(&d_p)?)?,
					)?,
				)?;

				if d > d_prev {
					if d.checked_sub(&d_prev)? <= prec {
						return Some(d)
					}
				} else if d_prev.checked_sub(&d)? <= prec {
					return Some(d)
				}
			}
			None
		}

		/// Here `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`, where
		/// `n` is number of coins.
		///
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
		pub fn get_y(
			i: usize,
			j: usize,
			x_f: FixedU128,
			xp_f: &[FixedU128],
			ann_f: FixedU128,
		) -> Option<FixedU128> {
			let zero = FixedU128::zero();
			let prec = T::Precision::get();
			let two = FixedU128::saturating_from_integer(2_u8);
			let n = FixedU128::try_from(xp_f.len() as u128).ok()?;

			// Same coin
			if i == j {
				return None
			}
			// j above n
			if j >= xp_f.len() {
				return None
			}
			if i >= xp_f.len() {
				return None
			}
			let d_f = Self::get_d(xp_f, ann_f)?;
			let mut c = d_f;
			let mut s = zero;

			// Calculate s and c
			// p is implicitly calculated as part of c
			// note that loop makes n - 1 iterations
			for (k, xp_k) in xp_f.iter().enumerate() {
				let x_k: FixedU128;
				if k == i {
					x_k = x_f;
				} else if k != j {
					x_k = *xp_k;
				} else {
					continue
				}
				// s = s + x_k
				s = s.checked_add(&x_k)?;
				// c = c * d / (x_k * n)
				c = c.checked_mul(&d_f)?.checked_div(&x_k.checked_mul(&n)?)?;
			}
			// c = c * d / (ann * n)
			// At this step we have d^n in the numerator of c
			// and n^(n-1) in its denominator.
			// So we multiplying it by remaining d/n
			c = c.checked_mul(&d_f)?.checked_div(&ann_f.checked_mul(&n)?)?;

			// b = s + d / ann
			// We subtract d later
			let b = s.checked_add(&d_f.checked_div(&ann_f)?)?;
			let mut y = d_f;

			for _ in 0..255 {
				let y_prev = y;
				// y = (y^2 + c) / (2 * y + b - d)
				// Subtract d to calculate b finally
				y = y
					.checked_mul(&y)?
					.checked_add(&c)?
					.checked_div(&two.checked_mul(&y)?.checked_add(&b)?.checked_sub(&d_f)?)?;

				// Equality with the specified precision
				if y > y_prev {
					if y.checked_sub(&y_prev)? <= prec {
						return Some(y)
					}
				} else if y_prev.checked_sub(&y)? <= prec {
					return Some(y)
				}
			}

			None
		}
		/// Here `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`, where
		/// `n` is number of coins.
		/// Calculate `x[i]` if one reduces `d` from being calculated for `xp` to `d`.
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
		pub fn get_y_d(
			i: usize,
			d_f: FixedU128,
			xp_f: &[FixedU128],
			ann_f: FixedU128,
		) -> Option<FixedU128> {
			let zero = FixedU128::zero();
			let prec = T::Precision::get();
			let two = FixedU128::saturating_from_integer(2_u8);
			let n = FixedU128::try_from(xp_f.len() as u128).ok()?;

			if i >= xp_f.len() {
				return None
			}

			let mut c = d_f;
			let mut s = zero;

			for (k, xp_k) in xp_f.iter().enumerate() {
				if k == i {
					continue
				}

				let x = xp_k;

				s = s.checked_add(x)?;
				// c = c * d / (x * n)
				c = c.checked_mul(&d_f)?.checked_div(&x.checked_mul(&n)?)?;
			}
			// c = c * d / (ann * n)
			c = c.checked_mul(&d_f)?.checked_div(&ann_f.checked_mul(&n)?)?;
			// b = s + d / ann
			let b = s.checked_add(&d_f.checked_div(&ann_f)?)?;
			let mut y = d_f;

			for _ in 0..255 {
				let y_prev = y;
				// y = (y*y + c) / (2 * y + b - d)
				y = y
					.checked_mul(&y)?
					.checked_add(&c)?
					.checked_div(&two.checked_mul(&y)?.checked_add(&b)?.checked_sub(&d_f)?)?;

				// Equality with the specified precision
				if y > y_prev {
					if y.checked_sub(&y_prev)? <= prec {
						return Some(y)
					}
				} else if y_prev.checked_sub(&y)? <= prec {
					return Some(y)
				}
			}

			None
		}

		fn transfer_liquidity_into_pool(
			pool_account_id: &T::AccountId,
			pool_id: T::PoolId,
			source: &T::AccountId,
			destination_asset_index: usize,
			amount: T::Balance,
		) -> DispatchResult {
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let asset_id =
				assets.get(destination_asset_index).ok_or(Error::<T>::IndexOutOfRange)?;
			T::LpToken::transfer(*asset_id, source, pool_account_id, amount, true)?;
			PoolAssetTotalBalance::<T>::mutate(
				pool_id,
				asset_id,
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
			source_asset_index: usize,
			destination: &T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
			let asset_id = assets.get(source_asset_index).ok_or(Error::<T>::IndexOutOfRange)?;
			T::LpToken::transfer(*asset_id, pool_account_id, destination, amount, true)?;

			PoolAssetTotalBalance::<T>::mutate(
				pool_id,
				asset_id,
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
