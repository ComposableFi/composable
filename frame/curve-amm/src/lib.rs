//!

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
		dex::{CurveAmm, PoolId, PoolInfo},
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
		FixedPointNumber, FixedPointOperand, FixedU128, KeyTypeId as CryptoKeyTypeId,
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
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Current number of pools (also ID for the next created pool)
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	pub type PoolCount<T: Config> = StorageValue<_, PoolId, ValueQuery>;

	/// Existing pools
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> =
		StorageMap<_, Blake2_128Concat, PoolId, PoolInfo<T::AccountId, T::AssetId, T::Balance>>;

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
		WrongAssetAmount,
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
		/// Pool with specified id `PoolId` was created successfully by `T::AccountId`.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `PoolId`
		///
		/// \[who, pool_id\]
		PoolCreated { who: T::AccountId, pool_id: PoolId },

		/// Liquidity added into the pool `PoolId` by `T::AccountId`.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `PoolId`
		/// - added token amounts `Vec<T::Balance>`
		/// - actual invariant `T::Balance`
		/// - actual token supply `T::Balance`
		/// - minted amount `T::Balance`
		///
		/// \[who, pool_id, token_amounts, invariant, token_supply, mint_amount\]
		LiquidityAdded {
			who: T::AccountId,
			pool_id: PoolId,
			token_amounts: Vec<T::Balance>,
			invariant: T::Balance,
			token_supply: T::Balance,
			mint_amount: T::Balance,
		},

		/// Liquidity removed from pool `PoolId` by `T::AccountId` in balanced way.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `PoolId`
		/// - removed token amounts `Vec<T::Balance>`
		/// - actual token supply `T::Balance`
		///
		/// \[who, pool_id, token_amounts, token_supply\]
		LiquidityRemoved {
			who: T::AccountId,
			pool_id: PoolId,
			token_amounts: Vec<T::Balance>,
			token_supply: T::Balance,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> CurveAmm for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		type AccountId = T::AccountId;

		fn pool_count() -> PoolId {
			PoolCount::<T>::get()
		}

		fn pool(id: PoolId) -> Option<PoolInfo<Self::AccountId, Self::AssetId, Self::Balance>> {
			Pools::<T>::get(id)
		}
		fn create_pool(
			who: &Self::AccountId,
			assets: Vec<Self::AssetId>,
			amplification_coefficient: Self::Balance,
		) -> Result<PoolId, DispatchError> {
			// Assets related checks
			ensure!(assets.len() > 1, Error::<T>::NotEnoughAssets);
			let unique_assets = BTreeSet::<T::AssetId>::from_iter(assets.iter().copied());
			ensure!(unique_assets.len() == assets.len(), Error::<T>::DuplicateAssets);

			// Add new pool
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<PoolId, DispatchError> {
					let pool_id = *pool_count;

					Pools::<T>::try_mutate_exists(pool_id, |maybe_pool_info| -> DispatchResult {
						// We expect that PoolInfos have sequential keys.
						// No PoolInfo can have key greater or equal to PoolCount
						ensure!(maybe_pool_info.is_none(), Error::<T>::InconsistentStorage);

						let asset = T::CurrencyFactory::create()?;

						let empty_balances = vec![Self::Balance::zero(); assets.len()];

						*maybe_pool_info = Some(PoolInfo {
							owner: who.clone(),
							pool_asset: asset,
							assets,
							amplification_coefficient,
							balances: empty_balances,
						});

						Ok(())
					})?;

					*pool_count = pool_id.checked_add(1).ok_or(Error::<T>::InconsistentStorage)?;

					Ok(pool_id)
				})?;

			Self::deposit_event(Event::PoolCreated { who: who.clone(), pool_id });

			Ok(pool_id)
		}

		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: PoolId,
			amounts: Vec<Self::Balance>,
			min_mint_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let zero = Self::Balance::zero();
			ensure!(amounts.iter().all(|&x| x >= zero), Error::<T>::WrongAssetAmount);

			let (provider, pool_id, token_amounts, invariant, token_supply, mint_amount) =
				Pools::<T>::try_mutate(pool_id, |pool| -> Result<_, DispatchError> {
					let pool = pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

					let n_coins = pool.assets.len();

					ensure!(n_coins == pool.balances.len(), Error::<T>::InconsistentStorage);

					ensure!(n_coins == amounts.len(), Error::<T>::IndexOutOfRange);
					let amp_f = FixedU128::saturating_from_integer(
						u128::try_from(pool.amplification_coefficient)
							.ok()
							.ok_or(Error::<T>::Math)?,
					);
					let ann = Self::get_ann(amp_f, n_coins).ok_or(Error::<T>::Math)?;

					let old_balances: Vec<FixedU128> = pool
						.balances
						.iter()
						.map(|b| {
							let b_u: u128 = (*b).into();
							FixedU128::saturating_from_integer(b_u)
						})
						.collect();
					let d0 = Self::get_d(&old_balances, ann).ok_or(Error::<T>::Math)?;

					let token_supply = T::LpToken::total_issuance(pool.pool_asset);
					let token_supply_u: u128 = token_supply.into();
					let token_supply_f = FixedU128::saturating_from_integer(token_supply_u);
					let mut new_balances = old_balances;
					for i in 0..n_coins {
						if token_supply == zero {
							ensure!(amounts[i] > zero, Error::<T>::WrongAssetAmount);
						}
						let amount_i: u128 = amounts[i].into();
						new_balances[i] = new_balances[i]
							.checked_add(&FixedU128::saturating_from_integer(amount_i))
							.ok_or(Error::<T>::Math)?;
					}

					let d1 = Self::get_d(&new_balances, ann).ok_or(Error::<T>::Math)?;
					ensure!(d1 > d0, Error::<T>::WrongAssetAmount);
					let _new_balances_b: Vec<T::Balance> = new_balances
						.iter()
						.map(|b_f| Ok(b_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into()))
						.collect::<Result<Vec<T::Balance>, Error<T>>>()?;
					let d1_b = d1.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
					let mint_amount;
					if token_supply > zero {
						// mint_amount = token_supply * (d1 - d0) / d0
						let mint_amount_f = (|| {
							token_supply_f.checked_mul(&d1.checked_sub(&d0)?)?.checked_div(&d0)
						})()
						.ok_or(Error::<T>::Math)?;
						mint_amount =
							mint_amount_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
					} else {
						mint_amount = d1_b;
					}

					ensure!(mint_amount >= min_mint_amount, Error::<T>::RequiredAmountNotReached);

					let new_token_supply =
						token_supply.checked_add(&mint_amount).ok_or(Error::<T>::Math)?;

					// Ensure that for all tokens user has sufficient amount
					for (i, amount) in amounts.iter().enumerate() {
						ensure!(
							T::LpToken::balance(pool.assets[i], who) >= *amount,
							Error::<T>::InsufficientFunds
						);
					}
					// Transfer funds to pool
					for (i, amount) in amounts.iter().enumerate() {
						if amount > &zero {
							Self::transfer_liquidity_into_pool(
								&Self::account_id(&pool_id),
								pool,
								who,
								i,
								*amount,
							)?;
						}
					}

					T::LpToken::mint_into(pool.pool_asset, who, mint_amount)?;

					Ok((who.clone(), pool_id, amounts, d1_b, new_token_supply, mint_amount))
				})?;

			Self::deposit_event(Event::LiquidityAdded {
				who: provider,
				pool_id,
				token_amounts,
				invariant,
				token_supply,
				mint_amount,
			});

			Ok(())
		}

		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: PoolId,
			amount: Self::Balance,
			min_amounts: Vec<Self::Balance>,
		) -> Result<(), DispatchError> {
			let zero = FixedU128::zero();
			let b_zero = Self::Balance::zero();
			ensure!(amount >= b_zero, Error::<T>::WrongAssetAmount);
			let amount_u: u128 = amount.into();
			let amount_f = FixedU128::saturating_from_integer(amount_u);

			let min_amounts_f: Vec<FixedU128> = min_amounts
				.iter()
				.map(|b| {
					let b_u: u128 = (*b).into();
					FixedU128::saturating_from_integer(b_u)
				})
				.collect();

			let (provider, pool_id, token_amounts, token_supply) =
				Pools::<T>::try_mutate(pool_id, |pool| -> Result<_, DispatchError> {
					let pool = pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

					let n_coins = pool.assets.len();

					ensure!(n_coins == pool.balances.len(), Error::<T>::InconsistentStorage);

					ensure!(n_coins == min_amounts.len(), Error::<T>::IndexOutOfRange);

					let token_supply = T::LpToken::total_issuance(pool.pool_asset);
					let token_supply_u: u128 = token_supply.into();

					let mut amounts_f = vec![FixedU128::zero(); n_coins];

					for i in 0..n_coins {
						let old_balance_u: u128 = pool.balances[i].into();
						let old_balance = FixedU128::saturating_from_integer(old_balance_u);
						// value = old_balance * n_amount / token_supply
						let value = (|| {
							old_balance
								.checked_mul(&amount_f)?
								.checked_div(&FixedU128::saturating_from_integer(token_supply_u))
						})()
						.ok_or(Error::<T>::Math)?;
						ensure!(value >= min_amounts_f[i], Error::<T>::RequiredAmountNotReached);

						amounts_f[i] = value;
					}

					let amounts: Vec<T::Balance> = amounts_f
						.iter()
						.map(|b_f| Ok(b_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into()))
						.collect::<Result<Vec<T::Balance>, Error<T>>>()?;

					let new_token_supply =
						token_supply.checked_sub(&amount).ok_or(Error::<T>::Math)?;

					T::LpToken::burn_from(pool.pool_asset, who, amount)?;

					// Ensure that for all tokens we have sufficient amount
					for (i, amounts_i) in amounts.iter().enumerate() {
						ensure!(
							T::LpToken::balance(pool.assets[i], &Self::account_id(&pool_id)) >=
								*amounts_i,
							Error::<T>::InsufficientFunds
						);
					}

					for i in 0..n_coins {
						if amounts_f[i] > zero {
							// pool.balances[i] = old_balance - value
							Self::transfer_liquidity_from_pool(
								&Self::account_id(&pool_id),
								pool,
								i,
								who,
								amounts[i],
							)?;
						}
					}

					Ok((who.clone(), pool_id, amounts, new_token_supply))
				})?;

			Self::deposit_event(Event::LiquidityRemoved {
				who: provider,
				pool_id,
				token_amounts,
				token_supply,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
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
			// let n = FixedU128::try_from(xp_f.len() as u128).ok()?;
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
				d = ann_f
					.checked_mul(&sum)?
					.checked_add(&d_p.checked_mul(&n)?)?
					.checked_mul(&d)?
					.checked_div(
					&ann_f
						.checked_sub(&one)?
						.checked_mul(&d)?
						.checked_add(&n.checked_add(&one)?.checked_mul(&d_p)?)?,
				)?;

				if d > d_prev {
					if d.checked_sub(&d_prev)? <= prec {
						// return Some(d.checked_mul_int(1u64)?.into());
						return Some(d)
					}
				} else if d_prev.checked_sub(&d)? <= prec {
					// return Some(d.checked_mul_int(1u64)?.into());
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
			pool: &mut PoolInfo<T::AccountId, T::AssetId, T::Balance>,
			source: &T::AccountId,
			destination_asset_index: usize,
			amount: T::Balance,
		) -> DispatchResult {
			T::LpToken::transfer(
				pool.assets[destination_asset_index],
				source,
				pool_account_id,
				amount,
				true,
			)?;

			pool.balances[destination_asset_index] = pool.balances[destination_asset_index]
				.checked_add(&amount)
				.ok_or(Error::<T>::InconsistentStorage)?;

			Ok(())
		}

		fn transfer_liquidity_from_pool(
			pool_account_id: &T::AccountId,
			pool: &mut PoolInfo<T::AccountId, T::AssetId, T::Balance>,
			source_asset_index: usize,
			destination: &T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			T::LpToken::transfer(
				pool.assets[source_asset_index],
				pool_account_id,
				destination,
				amount,
				true,
			)?;

			pool.balances[source_asset_index] = pool.balances[source_asset_index]
				.checked_sub(&amount)
				.ok_or(Error::<T>::InconsistentStorage)?;

			Ok(())
		}

		fn account_id(pool_id: &PoolId) -> T::AccountId {
			PALLET_ID.into_sub_account(pool_id)
		}
	}
}
