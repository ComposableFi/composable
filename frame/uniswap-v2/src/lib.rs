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
		defi::CurrencyPair,
		dex::{ConstantProductPoolInfo, CurveAmm},
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
			CheckedSub, IntegerSquareRoot, One, Zero,
		},
		FixedPointNumber, FixedPointOperand, FixedU128, KeyTypeId as CryptoKeyTypeId, Permill,
	};
	use sp_std::{collections::btree_set::BTreeSet, fmt::Debug, iter::FromIterator};

	pub const PALLET_ID: PalletId = PalletId(*b"Uni-swap");
	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"uswp");
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(*b"uswp");

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
		type PoolTokenIndex: Copy + Debug + Eq + Into<u32>;
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
	pub type Pools<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, ConstantProductPoolInfo<T::AccountId>>;

	/// Pool's LP asset
	#[pallet::storage]
	#[pallet::getter(fn pool_lp_asset)]
	pub type PoolLPAsset<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, T::AssetId>;

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
			ensure!(amounts.len() == 2, Error::<T>::IndexOutOfRange);
			ensure!(
				amounts.iter().all(|&x| x >= zero),
				Error::<T>::AssetAmountMustBePositiveNumber
			);

			let pool_lp_asset =
				PoolLPAsset::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
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
			let old_balance_a = old_balances[0];
			let old_balance_b = old_balances[1];
			let amount_a_u: u128 = amounts[0].into();
			let amount_b_u: u128 = amounts[1].into();
			let amount_a_desired = FixedU128::saturating_from_integer(amount_a_u);
			let amount_b_desired = FixedU128::saturating_from_integer(amount_b_u);
			let amount_a;
			let amount_b;

			if old_balance_a == FixedU128::zero() && old_balance_b == FixedU128::zero() {
				amount_a = amount_a_desired;
				amount_b = amount_b_desired;
			} else {
				let amount_b_optimal = Self::quote(amount_a_desired, old_balance_a, old_balance_b)
					.ok_or(Error::<T>::Math)?;

				if amount_b_optimal <= amount_b_desired {
					amount_a = amount_a_desired;
					amount_b = amount_b_optimal;
				} else {
					let amount_a_optimal =
						Self::quote(amount_b_desired, old_balance_b, old_balance_a)
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
					mint_amount_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
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
				mint_amount = mint_amount_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
			}

			let asset_id_a = assets[0];
			let asset_id_b = assets[1];
			let new_balance_a = old_balance_a.checked_add(&amount_a).ok_or(Error::<T>::Math)?;
			let new_balance_b = old_balance_b.checked_add(&amount_b).ok_or(Error::<T>::Math)?;
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_a, |balance| -> DispatchResult {
				*balance = new_balance_a.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_b, |balance| -> DispatchResult {
				*balance = new_balance_b.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;

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

			let pool_lp_asset =
				PoolLPAsset::<T>::get(pool_id).ok_or(Error::<T>::InconsistentStorage)?;
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
			let zero_b = Self::Balance::zero();
			ensure!(dx >= zero_b, Error::<T>::AssetAmountMustBePositiveNumber);

			let pool = Self::get_pool_info(pool_id).ok_or(Error::<T>::PoolNotFound)?;

			let i: usize = i_token.into() as usize;
			let j: usize = j_token.into() as usize;

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
			let x = xp[i].checked_add(&dx_f).ok_or(Error::<T>::Math)?;
			let dy_f = Self::get_y_out(dx_f, xp[i], xp[j]).ok_or(Error::<T>::Math)?;

			let fee_f: FixedU128 = pool.fee.into();
			let dy_fee_f = dy_f.checked_mul(&fee_f).ok_or(Error::<T>::Math)?;
			let dy_fee = dy_fee_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
			let dy_f = dy_f.checked_sub(&dy_fee_f).ok_or(Error::<T>::Math)?;
			ensure!(dy_f >= min_dy_f, Error::<T>::RequiredAmountNotReached);

			let dy: Self::Balance = dy_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
			let asset_id_i = assets[i];
			let asset_id_j = assets[j];
			PoolAssetBalance::<T>::mutate(pool_id, asset_id_i, |balance| -> DispatchResult {
				*balance = x.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
				Ok(())
			})?;
			let bal_j = xp[j].checked_sub(&dy_f).ok_or(Error::<T>::Math)?;
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

						*maybe_pool_info =
							Some(ConstantProductPoolInfo { owner: who.clone(), fee });

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
		pub fn get_pool_info(pool_id: T::PoolId) -> Option<ConstantProductPoolInfo<T::AccountId>> {
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

		fn transfer_liquidity_into_pool(
			pool_account_id: &T::AccountId,
			pool_id: T::PoolId,
			source: &T::AccountId,
			destination_asset_index: usize,
			amount: T::Balance,
		) -> DispatchResult {
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
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
			let assets_pair = PoolAssets::<T>::get(pool_id).ok_or(Error::<T>::PoolNotFound)?;
			let assets = assets_pair.as_slice();
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
