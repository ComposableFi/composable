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
		dex::{CurveAmm, PoolId, PoolInfo, PoolTokenIndex},
		math::LiftedFixedBalance,
		vault::{FundsAvailability, StrategicVault, Vault},
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
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter + Ord + Copy;
		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = <Self as Config>::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;
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
		StorageMap<_, Blake2_128Concat, PoolId, PoolInfo<T::AccountId, T::VaultId, T::Balance>>;

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
		/// Vault Error
		VaultError,
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
		/// - minted lp tokens `Vec<T::Balance>`
		///
		/// \[who, pool_id, token_amounts, minted_lp_tokens\]
		LiquidityAdded {
			who: T::AccountId,
			pool_id: PoolId,
			token_amounts: Vec<T::Balance>,
			minted_lp_tokens: Vec<T::Balance>,
		},

		/// Liquidity removed from pool `PoolId` by `T::AccountId`.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `PoolId`
		/// - removed token amounts `Vec<T::Balance>`
		///
		/// \[who, pool_id, token_amounts\]
		LiquidityRemoved { who: T::AccountId, pool_id: PoolId, token_amounts: Vec<T::Balance> },

		/// Token exchange happened.
		///
		/// Included values are:
		/// - account identifier `T::AccountId`
		/// - pool identifier `PoolId`
		/// - index of sent token `PoolTokenIndex`
		/// - amount of sent token `T::Balance`
		/// - index of received token `PoolTokenIndex`
		/// - amount of received token `T::Balance`
		/// - charged fee `T::Balance`
		///
		/// \[who, pool_id, sent_token_index, sent_amount, received_token_index, received_amount,
		/// fee\]
		TokenExchanged {
			who: T::AccountId,
			pool_id: PoolId,
			sent_token_index: PoolTokenIndex,
			sent_amount: T::Balance,
			received_token_index: PoolTokenIndex,
			received_amount: T::Balance,
			fee: T::Balance,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> CurveAmm for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		type AccountId = T::AccountId;
		type VaultId = T::VaultId;

		fn pool_count() -> PoolId {
			PoolCount::<T>::get()
		}

		fn pool(id: PoolId) -> Option<PoolInfo<Self::AccountId, Self::VaultId, Self::Balance>> {
			Pools::<T>::get(id)
		}

		fn create_pool(
			who: &Self::AccountId,
			assets_vault_ids: Vec<Self::VaultId>,
			amplification_coefficient: Self::Balance,
			fee: Permill,
		) -> Result<PoolId, DispatchError> {
			// Assets related checks
			ensure!(assets_vault_ids.len() > 1, Error::<T>::NotEnoughAssets);
			let unique_assets = BTreeSet::<T::VaultId>::from_iter(assets_vault_ids.iter().copied());
			ensure!(unique_assets.len() == assets_vault_ids.len(), Error::<T>::DuplicateAssets);

			// Add new pool
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<PoolId, DispatchError> {
					let pool_id = *pool_count;

					Pools::<T>::try_mutate_exists(pool_id, |maybe_pool_info| -> DispatchResult {
						// We expect that PoolInfos have sequential keys.
						// No PoolInfo can have key greater or equal to PoolCount
						ensure!(maybe_pool_info.is_none(), Error::<T>::InconsistentStorage);

						*maybe_pool_info = Some(PoolInfo {
							owner: who.clone(),
							assets_vault_ids,
							amplification_coefficient,
							fee,
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
		) -> Result<Vec<Self::Balance>, DispatchError> {
			let zero = Self::Balance::zero();
			ensure!(amounts.iter().all(|&x| x >= zero), Error::<T>::WrongAssetAmount);

			let (provider, pool_id, token_amounts, minted_lp_tokens) =
				Pools::<T>::try_mutate(pool_id, |pool| -> Result<_, DispatchError> {
					let pool = pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

					let n_coins = pool.assets_vault_ids.len();

					ensure!(n_coins == amounts.len(), Error::<T>::IndexOutOfRange);

					let mut lp_tokens = Vec::new();
					// Transfer funds to pool
					for (i, amount) in amounts.iter().enumerate() {
						if amount > &zero {
							let lp = <T::Vault as Vault>::deposit(
								&pool.assets_vault_ids[i],
								who,
								*amount,
							)?;
							lp_tokens.push(lp);
						}
					}

					Ok((who.clone(), pool_id, amounts, lp_tokens))
				})?;

			Self::deposit_event(Event::LiquidityAdded {
				who: provider,
				pool_id,
				token_amounts,
				minted_lp_tokens: minted_lp_tokens.clone(),
			});

			Ok(minted_lp_tokens)
		}

		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: PoolId,
			lp_tokens: Vec<Self::Balance>,
		) -> Result<(), DispatchError> {
			let b_zero = Self::Balance::zero();
			ensure!(lp_tokens.iter().all(|&x| x >= b_zero), Error::<T>::WrongAssetAmount);

			let (provider, pool_id, token_amounts) =
				Pools::<T>::try_mutate(pool_id, |pool| -> Result<_, DispatchError> {
					let pool = pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

					let n_coins = pool.assets_vault_ids.len();

					ensure!(n_coins == lp_tokens.len(), Error::<T>::IndexOutOfRange);

					let mut amounts = Vec::new();

					for i in 0..n_coins {
						let lp = <T::Vault as Vault>::withdraw(
							&pool.assets_vault_ids[i],
							who,
							lp_tokens[i],
						)?;
						amounts.push(lp);
					}

					Ok((who.clone(), pool_id, amounts))
				})?;

			Self::deposit_event(Event::LiquidityRemoved { who: provider, pool_id, token_amounts });

			Ok(())
		}

		fn exchange(
			who: &Self::AccountId,
			pool_id: PoolId,
			i: PoolTokenIndex,
			j: PoolTokenIndex,
			dx: Self::Balance,
			min_dy: Self::Balance,
		) -> Result<(), DispatchError> {
			let prec = T::Precision::get();
			let zero_b = Self::Balance::zero();
			ensure!(dx >= zero_b, Error::<T>::WrongAssetAmount);

			let (provider, pool_id, dy, fee) =
				Pools::<T>::try_mutate(pool_id, |pool| -> Result<_, DispatchError> {
					let pool = pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

					let i = i as usize;
					let j = j as usize;

					let n_coins = pool.assets_vault_ids.len();

					ensure!(i < n_coins && j < n_coins, Error::<T>::IndexOutOfRange);

					let pool_account_ids: Vec<Self::AccountId> = pool
						.assets_vault_ids
						.iter()
						.map(|vault_id| T::Vault::account_id(vault_id))
						.collect();
					let pool_asset_ids: Vec<Self::AssetId> = pool
						.assets_vault_ids
						.iter()
						.map(|vault_id| {
							Ok(T::Vault::asset_id(vault_id).ok().ok_or(Error::VaultError)?.into())
						})
						.collect::<Result<Vec<T::AssetId>, Error<T>>>()?;
					let mut balances = Vec::new();
					for (vault_id, account_id) in
						pool.assets_vault_ids.iter().zip(pool_account_ids.iter())
					{
						let res =
							<T::Vault as StrategicVault>::available_funds(vault_id, account_id);
						let bal = match res {
							Ok(FundsAvailability::Withdrawable(b)) => b,
							_ => Self::Balance::zero(),
						};
						balances.push(bal);
					}
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

					let amp_f = FixedU128::saturating_from_integer(
						u128::try_from(pool.amplification_coefficient)
							.ok()
							.ok_or(Error::<T>::Math)?,
					);
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

					let dy: Self::Balance =
						dy_f.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();

					// pool.balances[i] = x.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();
					// When rounding errors happen, we undercharge admin fee in favor of LP
					// pool.balances[j] = xp[j] - dy_f - dy_admin_fee_f
					// let bal_j = xp[j]
					// 	.checked_sub(&dy_f)
					// 	.ok_or(Error::<T>::Math)?;
					// pool.balances[j] =
					// bal_j.checked_mul_int(1u64).ok_or(Error::<T>::Math)?.into();

					ensure!(
						T::LpToken::balance(pool_asset_ids[i], &who) >= dx,
						Error::<T>::InsufficientFunds
					);

					ensure!(
						T::LpToken::balance(pool_asset_ids[j], &pool_account_ids[j]) >= dy,
						Error::<T>::InsufficientFunds
					);

					<T::Vault as StrategicVault>::deposit(&pool.assets_vault_ids[i], &who, dx)?;
					<T::Vault as StrategicVault>::withdraw(&pool.assets_vault_ids[j], &who, dy)?;

					Ok((who.clone(), pool_id, dy, dy_fee))
				})?;

			Self::deposit_event(Event::TokenExchanged {
				who: provider,
				pool_id,
				sent_token_index: i,
				sent_amount: dx,
				received_token_index: j,
				received_amount: dy,
				fee,
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

		pub fn account_id(pool_id: &PoolId) -> T::AccountId {
			PALLET_ID.into_sub_account(pool_id)
		}
	}
}
