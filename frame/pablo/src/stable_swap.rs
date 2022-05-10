use crate::{Config, Error, PoolConfiguration, PoolCount, Pools};
use composable_maths::dex::stable_swap::{compute_base, compute_d};
use composable_support::math::safe::{safe_multiply_by_rational, SafeAdd, SafeSub};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::CurrencyPair,
	dex::StableSwapPoolInfo,
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
};
use sp_runtime::{
	traits::{CheckedAdd, Convert, One, Zero},
	ArithmeticError, DispatchError, Permill,
};
use sp_std::{marker::PhantomData, ops::Mul};

pub(crate) struct StableSwap<T>(PhantomData<T>);

impl<T: Config> StableSwap<T> {
	pub fn do_create_pool(
		who: &T::AccountId,
		pair: CurrencyPair<T::AssetId>,
		amp_coeff: u16,
		fee: Permill,
		owner_fee: Permill,
	) -> Result<T::PoolId, DispatchError> {
		ensure!(amp_coeff > 0, Error::<T>::AmpFactorMustBeGreaterThanZero);
		ensure!(pair.base != pair.quote, Error::<T>::InvalidPair);

		let total_fees = fee.checked_add(&owner_fee).ok_or(ArithmeticError::Overflow)?;
		ensure!(total_fees < Permill::one(), Error::<T>::InvalidFees);

		let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS, T::Balance::default())?;
		// Add new pool
		let pool_id =
			PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
				let pool_id = *pool_count;

				Pools::<T>::insert(
					pool_id,
					PoolConfiguration::StableSwap(StableSwapPoolInfo {
						owner: who.clone(),
						pair,
						lp_token,
						amplification_coefficient: amp_coeff,
						fee,
						owner_fee,
					}),
				);
				*pool_count = pool_id.safe_add(&T::PoolId::one())?;
				Ok(pool_id)
			})?;

		Ok(pool_id)
	}

	fn get_invariant(
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

	pub fn get_exchange_value(
		pool: &StableSwapPoolInfo<T::AccountId, T::AssetId>,
		pool_account: &T::AccountId,
		asset_id: T::AssetId,
		amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
		let pool_base_aum = T::Assets::balance(pair.base, pool_account);
		let pool_quote_aum = T::Assets::balance(pair.quote, pool_account);
		ensure!(!pool_base_aum.is_zero() && !pool_quote_aum.is_zero(), Error::<T>::NotEnoughLiquidity);
		let amp = T::Convert::convert(pool.amplification_coefficient.into());
		let d = Self::get_invariant(pool_base_aum, pool_quote_aum, amp)?;
		let new_quote_amount = pool_quote_aum.safe_add(&amount)?;
		let new_base_amount = T::Convert::convert(compute_base(
			T::Convert::convert(new_quote_amount),
			T::Convert::convert(amp),
			T::Convert::convert(d),
		)?);
		let exchange_value = pool_base_aum.safe_sub(&new_base_amount)?;
		Ok(exchange_value)
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

	pub fn do_compute_swap(
		pool: &StableSwapPoolInfo<T::AccountId, T::AssetId>,
		pool_account: &T::AccountId,
		quote_amount: T::Balance,
		apply_fees: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance, T::Balance), DispatchError> {
		let base_amount =
			Self::get_exchange_value(pool, pool_account, pool.pair.base, quote_amount)?;
		let base_amount_u: u128 = T::Convert::convert(base_amount);

		let (lp_fee, owner_fee) = if apply_fees {
			let lp_fee = pool.fee.mul_floor(base_amount_u);
			// owner_fee is computed based on lp_fee
			let owner_fee = pool.owner_fee.mul_floor(lp_fee);
			let lp_fee = T::Convert::convert(lp_fee);
			let owner_fee = T::Convert::convert(owner_fee);
			(lp_fee, owner_fee)
		} else {
			(T::Balance::zero(), T::Balance::zero())
		};

		let base_amount_excluding_fees = base_amount.safe_sub(&lp_fee)?;
		Ok((base_amount_excluding_fees, quote_amount, lp_fee, owner_fee))
	}

	pub fn add_liquidity(
		who: &T::AccountId,
		pool: StableSwapPoolInfo<T::AccountId, T::AssetId>,
		pool_account: T::AccountId,
		base_amount: T::Balance,
		quote_amount: T::Balance,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		let zero = T::Balance::zero();
		ensure!(base_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
		ensure!(quote_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
		// pool supports only 2 assets
		let pool_base_aum = T::Assets::balance(pool.pair.base, &pool_account);
		let pool_quote_aum = T::Assets::balance(pool.pair.quote, &pool_account);

		let lp_issued = T::Assets::total_issuance(pool.lp_token);
		let amp = T::Convert::convert(pool.amplification_coefficient.into());
		let d0 = Self::get_invariant(pool_base_aum, pool_quote_aum, amp)?;
		let new_base_amount = pool_base_aum.safe_add(&base_amount)?;
		let new_quote_amount = pool_quote_aum.safe_add(&quote_amount)?;
		let d1 = Self::get_invariant(new_base_amount, new_quote_amount, amp)?;
		ensure!(d1 > d0, Error::<T>::AssetAmountMustBePositiveNumber);

		let (mint_amount, base_owner_fee, quote_owner_fee) = if lp_issued > zero {
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
			let base_owner_fee = T::Convert::convert(pool.owner_fee.mul_floor(base_fee));
			let quote_owner_fee = T::Convert::convert(pool.owner_fee.mul_floor(quote_fee));
			let base_fee = T::Convert::convert(base_fee);
			let quote_fee = T::Convert::convert(quote_fee);
			let new_base_balance = new_base_amount.safe_sub(&base_fee)?;
			let new_quote_balance = new_quote_amount.safe_sub(&quote_fee)?;

			let d2 = Self::get_invariant(new_base_balance, new_quote_balance, amp)?;
			let mint_amount = T::Convert::convert(safe_multiply_by_rational(
				T::Convert::convert(lp_issued),
				T::Convert::convert(d2.safe_sub(&d0)?),
				T::Convert::convert(d0),
			)?);
			(mint_amount, base_owner_fee, quote_owner_fee)
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
			base_owner_fee,
			keep_alive,
		)?;
		T::Assets::transfer(
			pool.pair.quote,
			&pool_account,
			&pool.owner,
			quote_owner_fee,
			keep_alive,
		)?;
		T::Assets::mint_into(pool.lp_token, who, mint_amount)?;
		Ok((base_amount, quote_amount, mint_amount))
	}

	pub fn remove_liquidity(
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
}
