//

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
		dex::{CurveAmm, StableSwapPoolInfo},
		math::LiftedFixedBalance,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		PalletId,
	};
	use scale_info::TypeInfo;
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedDiv, CheckedMul,
			CheckedSub, One, Zero,
		},
		FixedPointNumber, FixedPointOperand, FixedU128, KeyTypeId as CryptoKeyTypeId, Permill,
	};
	use sp_std::{collections::btree_set::BTreeSet, fmt::Debug, iter::FromIterator};

	pub const PALLET_ID: PalletId = PalletId(*b"CurveAmm");
	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"camm");
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(*b"camm");

	pub mod crypto {
		use super::KEY_TYPE;
		use sp_core::sr25519::Signature as Sr25519Signature;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify,
			MultiSignature, MultiSigner,
		};
		app_crypto!(sr25519, KEY_TYPE);

		pub struct TestAuthId;

		// implementation for runtime
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}

		// implementation for mock runtime in test
		impl
			frame_system::offchain::AppCrypto<
				<Sr25519Signature as Verify>::Signer,
				Sr25519Signature,
			> for TestAuthId
		{
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

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
		type PoolTokenIndex: Copy + Debug + Eq + Into<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Current number of pools (also ID for the next created pool)
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery>;

	/// Existing pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, StableSwapPoolInfo<T::AccountId>>;

	/// Pool's LP asset
	#[pallet::storage]
	#[pallet::getter(fn pool_lp_asset)]
	pub type PoolLPAsset<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, T::AssetId>;

	/// List of assets supported by the pool
	#[pallet::storage]
	#[pallet::getter(fn pool_assets)]
	pub type PoolAssets<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, Vec<T::AssetId>>;

	/// Balance of asset for given pool excluding admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_balance)]
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
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `T::PoolId`
		///
		/// \[who, pool_id\]
		PoolCreated { who: T::AccountId, pool_id: T::PoolId },

		/// Liquidity added into the pool `T::PoolId` by `T::AccountId`.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `T::PoolId`
		/// - added token amounts `Vec<T::Balance>`
		/// - charged fees `Vec<T::Balance>`
		/// - actual invariant `T::Balance`
		/// - actual token supply `T::Balance`
		/// - minted amount `T::Balance`
		///
		/// \[who, pool_id, token_amounts, fees, invariant, token_supply, mint_amount\]
		LiquidityAdded {
			who: T::AccountId,
			pool_id: T::PoolId,
			token_amounts: Vec<T::Balance>,
			fees: Vec<T::Balance>,
			invariant: T::Balance,
			token_supply: T::Balance,
			mint_amount: T::Balance,
		},

		/// Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `T::PoolId`
		/// - removed token amounts `Vec<T::Balance>`
		/// - charged fees `Vec<T::Balance>`
		/// - actual token supply `T::Balance`
		///
		/// \[who, pool_id, token_amounts, fees, token_supply\]
		LiquidityRemoved {
			who: T::AccountId,
			pool_id: T::PoolId,
			token_amounts: Vec<T::Balance>,
			fees: Vec<T::Balance>,
			token_supply: T::Balance,
		},

		/// Token exchange happened.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `T::PoolId`
		/// - index of sent token `T::PoolTokenIndex`
		/// - amount of sent token `T::Balance`
		/// - index of received token `T::PoolTokenIndex`
		/// - amount of received token `T::Balance`
		/// - charged fee `T::Balance`
		///
		/// \[who, pool_id, sent_token_index, sent_amount, received_token_index, received_amount,
		/// fee\]
		TokenExchanged {
			who: T::AccountId,
			pool_id: T::PoolId,
			sent_token_index: T::PoolTokenIndex,
			sent_amount: T::Balance,
			received_token_index: T::PoolTokenIndex,
			received_amount: T::Balance,
			fee: T::Balance,
		},

		/// Withdraw admin fees `Vec<T::Balance>` from pool `T::PoolId` by user `T::AccountId`
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `T::PoolId`
		/// - admin fee receiving account identifier `T::AccountId`
		/// - withdrew admin fees `Vec<T::Balance>`
		///
		/// [who, pool_id, admin_fee_account, admin_fees]
		AdminFeesWithdrawn {
			who: T::AccountId,
			pool_id: T::PoolId,
			admin_fee_account: T::AccountId,
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
			let pool_lp_asset =
				PoolLPAsset::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let assets = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let mut balances = Vec::new();
			for asset_id in &assets {
				balances.push(PoolAssetBalance::<T>::get(pool_id, asset_id));
			}

			let n_coins = assets.len();

			ensure!(n_coins == balances.len(), Error::<T>::InconsistentStorage);

			ensure!(n_coins == amounts.len(), Error::<T>::IndexOutOfRange);
			let amp_f = pool.amplification_coefficient;
			let ann = Self::get_ann(amp_f, n_coins).ok_or(Error::<T>::Math)?;

			let old_balances: Vec<FixedU128> = balances
				.iter()
				.map(|b| {
					let b_u: u128 = (*b).into();
					FixedU128::saturating_from_integer(b_u)
				})
				.collect();
			let d0 = Self::get_d(&old_balances, ann).ok_or(Error::<T>::Math)?;

			let token_supply = T::LpToken::total_issuance(pool_lp_asset);
			let token_supply_u: u128 = token_supply.into();
			let token_supply_f = FixedU128::saturating_from_integer(token_supply_u);
			let mut new_balances = old_balances.clone();
			for i in 0..n_coins {
				if token_supply == zero {
					ensure!(amounts[i] > zero, Error::<T>::AssetAmountMustBePositiveNumber);
				}
				let amount_i: u128 = amounts[i].into();
				new_balances[i] = new_balances[i]
					.checked_add(&FixedU128::saturating_from_integer(amount_i))
					.ok_or(Error::<T>::Math)?;
			}

			let d1 = Self::get_d(&new_balances, ann).ok_or(Error::<T>::Math)?;
			ensure!(d1 > d0, Error::<T>::AssetAmountMustBePositiveNumber);
			let d1_b = d1.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
			let mint_amount;
			let mut fees = vec![FixedU128::zero(); n_coins];
			// Only account for fees if we are not the first to deposit
			if token_supply > zero {
				// Deposit x + withdraw y would chargVe about same
				// fees as a swap. Otherwise, one could exchange w/o paying fees.
				// And this formula leads to exactly that equality
				// fee = pool.fee * n_coins / (4 * (n_coins - 1))
				let one = FixedU128::saturating_from_integer(1u8);
				let four = FixedU128::saturating_from_integer(4u8);
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
				for (index, asset_id) in assets.iter().enumerate() {
					// ideal_balance = d1 * old_balances[i] / d0
					let ideal_balance =
						(|| d1.checked_mul(&old_balances[index])?.checked_div(&d0))()
							.ok_or(Error::<T>::Math)?;

					let new_balance = new_balances[index];
					// difference = abs(ideal_balance - new_balance)
					let difference = (if ideal_balance > new_balance {
						ideal_balance.checked_sub(&new_balance)
					} else {
						new_balance.checked_sub(&ideal_balance)
					})
					.ok_or(Error::<T>::Math)?;

					fees[index] = fee_f.checked_mul(&difference).ok_or(Error::<T>::Math)?;
					// new_pool_balance = new_balance - (fees[i] * admin_fee)
					let new_pool_balance =
						(|| new_balance.checked_sub(&fees[index].checked_mul(&admin_fee_f)?))()
							.ok_or(Error::<T>::Math)?;
					PoolAssetBalance::<T>::mutate(
						pool_id,
						asset_id,
						|balance| -> DispatchResult {
							*balance = new_pool_balance
								.checked_mul_int(1u64)
								.ok_or(Error::<T>::Math)?
								.into();
							Ok(())
						},
					)?;

					new_balances[index] =
						new_balances[index].checked_sub(&fees[index]).ok_or(Error::<T>::Math)?;
				}
				let d2 = Self::get_d(&new_balances, ann).ok_or(Error::<T>::Math)?;

				// mint_amount = token_supply * (d2 - d0) / d0
				let mint_amount_f =
					(|| token_supply_f.checked_mul(&d2.checked_sub(&d0)?.checked_div(&d0)?))()
						.ok_or(Error::<T>::Math)?;
				mint_amount = mint_amount_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
			} else {
				for (index, asset_id) in assets.iter().enumerate() {
					PoolAssetBalance::<T>::mutate(
						pool_id,
						asset_id,
						|balance| -> DispatchResult {
							*balance = new_balances[index]
								.checked_mul_int(1u64)
								.ok_or(Error::<T>::Math)?
								.into();
							Ok(())
						},
					)?;
				}
				mint_amount = d1_b;
			}

			ensure!(mint_amount >= min_mint_amount, Error::<T>::RequiredAmountNotReached);

			let new_token_supply =
				token_supply.checked_add(&mint_amount).ok_or(Error::<T>::Math)?;

			// Ensure that for all tokens user has sufficient amount
			for (i, amount) in amounts.iter().enumerate() {
				ensure!(
					T::LpToken::balance(assets[i], who) >= *amount,
					Error::<T>::InsufficientFunds
				);
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
				.map(|b_f| Ok(b_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into()))
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
			let amount_u: u128 = amount.into();
			let amount_f = FixedU128::saturating_from_integer(amount_u);

			let min_amounts_f: Vec<FixedU128> = min_amounts
				.iter()
				.map(|b| {
					let b_u: u128 = (*b).into();
					FixedU128::saturating_from_integer(b_u)
				})
				.collect();

			let pool_lp_asset =
				PoolLPAsset::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let assets = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let n_coins = assets.len();
			let mut balances = Vec::new();
			for asset_id in &assets {
				balances.push(PoolAssetBalance::<T>::get(pool_id, asset_id));
			}

			ensure!(n_coins == balances.len(), Error::<T>::InconsistentStorage);

			ensure!(n_coins == min_amounts.len(), Error::<T>::IndexOutOfRange);

			let token_supply = T::LpToken::total_issuance(pool_lp_asset);
			let token_supply_u: u128 = token_supply.into();

			let mut amounts_f = vec![FixedU128::zero(); n_coins];

			for (index, asset_id) in assets.iter().enumerate() {
				let old_balance_u: u128 = balances[index].into();
				let old_balance = FixedU128::saturating_from_integer(old_balance_u);
				// value = old_balance * n_amount / token_supply
				let value = (|| {
					old_balance
						.checked_mul(&amount_f)?
						.checked_div(&FixedU128::saturating_from_integer(token_supply_u))
				})()
				.ok_or(Error::<T>::Math)?;
				ensure!(value >= min_amounts_f[index], Error::<T>::RequiredAmountNotReached);
				PoolAssetBalance::<T>::mutate(pool_id, asset_id, |balance| -> DispatchResult {
					*balance = old_balance
						.checked_sub(&value)
						.ok_or(Error::<T>::InsufficientFunds)?
						.checked_mul_int(1u64)
						.ok_or(Error::<T>::Math)?
						.into();
					Ok(())
				})?;
				amounts_f[index] = value;
			}

			let amounts: Vec<T::Balance> = amounts_f
				.iter()
				.map(|b_f| Ok(b_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into()))
				.collect::<Result<Vec<T::Balance>, Error<T>>>()?;

			let new_token_supply = token_supply.checked_sub(&amount).ok_or(Error::<T>::Math)?;

			let fees = vec![T::Balance::zero(); n_coins];

			T::LpToken::burn_from(pool_lp_asset, who, amount)?;

			// Ensure that for all tokens we have sufficient amount
			for (index, asset_id) in assets.iter().enumerate() {
				ensure!(
					T::LpToken::balance(*asset_id, &Self::account_id(&pool_id)) >= amounts[index],
					Error::<T>::InsufficientFunds
				);
			}

			for i in 0..n_coins {
				if amounts_f[i] > zero {
					Self::transfer_liquidity_from_pool(
						&Self::account_id(&pool_id),
						pool_id,
						i,
						who,
						amounts[i],
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
		) -> Result<(), DispatchError> {
			let prec = T::Precision::get();
			let zero_b = Self::Balance::zero();
			ensure!(dx >= zero_b, Error::<T>::AssetAmountMustBePositiveNumber);

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let i = i_token.into() as usize;
			let j = j_token.into() as usize;

			let assets = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let mut balances = Vec::new();
			for asset_id in &assets {
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
			let x = xp[i].checked_add(&dx_f).ok_or(Error::<T>::Math)?;

			let amp_f = pool.amplification_coefficient;
			let ann = Self::get_ann(amp_f, n_coins).ok_or(Error::<T>::Math)?;
			let y = Self::get_y(i, j, x, &xp, ann).ok_or(Error::<T>::Math)?;

			// -1 just in case there were some rounding errors
			// dy = xp[j] - y - 1
			let dy_f = xp[j]
				.checked_sub(&y)
				.ok_or(Error::<T>::Math)?
				.checked_sub(&prec)
				.ok_or(Error::<T>::Math)?;

			let fee_f: FixedU128 = pool.fee.into();
			let dy_fee_f = dy_f.checked_mul(&fee_f).ok_or(Error::<T>::Math)?;
			let dy_fee = dy_fee_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
			let dy_f = dy_f.checked_sub(&dy_fee_f).ok_or(Error::<T>::Math)?;
			ensure!(dy_f >= min_dy_f, Error::<T>::RequiredAmountNotReached);

			let admin_fee_f: FixedU128 = pool.admin_fee.into();
			let dy_admin_fee_f = dy_fee_f.checked_mul(&admin_fee_f).ok_or(Error::<T>::Math)?;
			let dy: Self::Balance = dy_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
			let asset_id_i = assets[i];
			let asset_id_j = assets[j];
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_i, |balance| -> DispatchResult {
				*balance = x.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;
			// When rounding errors happen, we undercharge admin fee in favor of LP
			let bal_j = xp[j]
				.checked_sub(&dy_f)
				.ok_or(Error::<T>::Math)?
				.checked_sub(&dy_admin_fee_f)
				.ok_or(Error::<T>::Math)?;
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_j, |balance| -> DispatchResult {
				*balance = bal_j.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;

			ensure!(T::LpToken::balance(asset_id_i, &who) >= dx, Error::<T>::InsufficientFunds);

			ensure!(
				T::LpToken::balance(asset_id_j, &Self::account_id(&pool_id)) >= dy,
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
			Ok(())
		}

		fn withdraw_admin_fees(
			who: &Self::AccountId,
			pool_id: T::PoolId,
			admin_fee_account: &Self::AccountId,
		) -> Result<(), DispatchError> {
			let assets = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let mut balances = Vec::new();
			for asset_id in &assets {
				balances.push(PoolAssetBalance::<T>::get(pool_id, asset_id));
			}
			let mut total_balances = Vec::new();
			for asset_id in &assets {
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
				let new_total_balance = total_balances[index]
					.checked_sub(&admin_fees[index])
					.ok_or(Error::<T>::Math)?;
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
					asset,
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

						*maybe_pool_info = Some(StableSwapPoolInfo {
							owner: who.clone(),
							amplification_coefficient,
							fee,
							admin_fee,
						});

						Ok(())
					})?;

					PoolLPAsset::<T>::try_mutate(pool_id, |lp_asset| -> DispatchResult {
						ensure!(lp_asset.is_none(), Error::<T>::InconsistentStorage);
						let asset = T::CurrencyFactory::create()?;
						*lp_asset = Some(asset);
						Ok(())
					})?;

					for asset_id in &assets {
						PoolAssetBalance::<T>::insert(pool_id, asset_id, T::Balance::zero());
						PoolAssetTotalBalance::<T>::insert(pool_id, asset_id, T::Balance::zero());
					}

					PoolAssets::<T>::try_mutate(pool_id, |pool_assets| -> DispatchResult {
						ensure!(pool_assets.is_none(), Error::<T>::InconsistentStorage);
						*pool_assets = Some(assets);
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
		pub fn get_pool_info(pool_id: T::PoolId) -> Option<StableSwapPoolInfo<T::AccountId>> {
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
			let two = FixedU128::saturating_from_integer(2u8);
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
			let two = FixedU128::saturating_from_integer(2u8);
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
			let assets = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let asset_id = assets[destination_asset_index];
			T::LpToken::transfer(asset_id, source, pool_account_id, amount, true)?;
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
			let assets = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
			let asset_id = assets[source_asset_index];
			T::LpToken::transfer(asset_id, pool_account_id, destination, amount, true)?;

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
			PALLET_ID.into_sub_account(pool_id)
		}
	}
}
