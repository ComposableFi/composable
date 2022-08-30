use crate::{Config, Error, PoolConfiguration, PoolCount, Pools};
use composable_maths::dex::stable_swap::{compute_base, compute_d};
use composable_support::math::safe::{safe_multiply_by_rational, SafeAdd, SafeSub};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::CurrencyPair,
	dex::{Fee, FeeConfig, StableSwapPoolInfo},
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
};
use sp_runtime::{
	traits::{Convert, One, Zero},
	DispatchError, Permill,
};
use sp_std::{marker::PhantomData, ops::Mul};

pub(crate) struct StableSwap<T>(PhantomData<T>);

impl<T: Config> StableSwap<T> {
	pub fn do_create_pool(
		who: &T::AccountId,
		pair: CurrencyPair<T::AssetId>,
		amp_coeff: u16,
		fee: FeeConfig,
	) -> Result<T::PoolId, DispatchError> {
		ensure!(amp_coeff > 0, Error::<T>::AmpFactorMustBeGreaterThanZero);
		ensure!(pair.base != pair.quote, Error::<T>::InvalidPair);
		ensure!(fee.fee_rate < Permill::one(), Error::<T>::InvalidFees);

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
						fee_config: fee,
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
		quote_amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		ensure!(pool.pair.contains(asset_id), Error::<T>::InvalidAsset);
		let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
		let pool_base_aum = T::Assets::balance(pair.base, pool_account);
		let pool_quote_aum = T::Assets::balance(pair.quote, pool_account);
		ensure!(
			!pool_base_aum.is_zero() && !pool_quote_aum.is_zero(),
			Error::<T>::NotEnoughLiquidity
		);
		let amp = T::Convert::convert(pool.amplification_coefficient.into());
		let d = Self::get_invariant(pool_base_aum, pool_quote_aum, amp)?;
		let new_quote_amount = pool_quote_aum.safe_add(&quote_amount)?;
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
		pair: CurrencyPair<T::AssetId>,
		quote_amount: T::Balance,
		apply_fees: bool,
	) -> Result<(T::Balance, T::Balance, Fee<T::AssetId, T::Balance>), DispatchError> {
		ensure!(pair == pool.pair, Error::<T>::PairMismatch);
		let base_amount = Self::get_exchange_value(pool, pool_account, pair.base, quote_amount)?;
		let fee = if apply_fees {
			pool.fee_config.calculate_fees(pair.base, base_amount)
		} else {
			Fee::<T::AssetId, T::Balance>::zero(pair.base)
		};

		let base_amount_excluding_fees = base_amount.safe_sub(&fee.fee)?;
		Ok((base_amount_excluding_fees, quote_amount, fee))
	}

	pub fn add_liquidity(
		who: &T::AccountId,
		pool_info: StableSwapPoolInfo<T::AccountId, T::AssetId>,
		pool_account: T::AccountId,
		base_amount: T::Balance,
		quote_amount: T::Balance,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<
		(
			T::Balance,
			T::Balance,
			T::Balance,
			Fee<T::AssetId, T::Balance>,
			Fee<T::AssetId, T::Balance>,
		),
		DispatchError,
	> {
		let zero = T::Balance::zero();
		ensure!(base_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
		ensure!(quote_amount > zero, Error::<T>::AssetAmountMustBePositiveNumber);
		let (mint_amount, base_fee, quote_fee) = Self::calculate_mint_amount_and_fees(
			&pool_info,
			&pool_account,
			&base_amount,
			&quote_amount,
		)?;

		ensure!(mint_amount >= min_mint_amount, Error::<T>::CannotRespectMinimumRequested);
		T::Assets::transfer(pool_info.pair.base, who, &pool_account, base_amount, keep_alive)?;
		T::Assets::transfer(pool_info.pair.quote, who, &pool_account, quote_amount, keep_alive)?;
		T::Assets::mint_into(pool_info.lp_token, who, mint_amount)?;
		Ok((base_amount, quote_amount, mint_amount, base_fee, quote_fee))
	}

	pub fn remove_liquidity(
		who: &T::AccountId,
		pool: StableSwapPoolInfo<T::AccountId, T::AssetId>,
		pool_account: T::AccountId,
		lp_amount: T::Balance,
		base_amount: T::Balance,
		quote_amount: T::Balance,
	) -> Result<
		(
			T::Balance, /* base_amount */
			T::Balance, /* quote_amount */
			T::Balance, /* updated_lp */
		),
		DispatchError,
	> {
		let lp_issued = T::Assets::total_issuance(pool.lp_token);
		let total_issuance = lp_issued.safe_sub(&lp_amount)?;

		// NOTE(hussein-aitlance): no need to keep alive the pool account
		T::Assets::transfer(pool.pair.base, &pool_account, who, base_amount, false)?;
		T::Assets::transfer(pool.pair.quote, &pool_account, who, quote_amount, false)?;
		T::Assets::burn_from(pool.lp_token, who, lp_amount)?;
		Ok((base_amount, quote_amount, total_issuance))
	}

	pub(crate) fn calculate_mint_amount_and_fees(
		pool_info: &StableSwapPoolInfo<T::AccountId, T::AssetId>,
		pool_account: &T::AccountId,
		base_amount: &T::Balance,
		quote_amount: &T::Balance,
	) -> Result<(T::Balance, Fee<T::AssetId, T::Balance>, Fee<T::AssetId, T::Balance>), DispatchError>
	{
		let pool_base_aum = T::Assets::balance(pool_info.pair.base, pool_account);
		let pool_quote_aum = T::Assets::balance(pool_info.pair.quote, pool_account);

		let total_lp_issued = T::Assets::total_issuance(pool_info.lp_token);

		let amplification_coefficient =
			T::Convert::convert(pool_info.amplification_coefficient.into());

		let d0 = Self::get_invariant(pool_base_aum, pool_quote_aum, amplification_coefficient)?;

		let new_base_amount = pool_base_aum.safe_add(base_amount)?;
		let new_quote_amount = pool_quote_aum.safe_add(quote_amount)?;

		let d1 = Self::get_invariant(new_base_amount, new_quote_amount, amplification_coefficient)?;

		// REVIEW: Is this necessary with previous `AssetAmountMustBePositiveNumber` checks?
		ensure!(d1 > d0, Error::<T>::AssetAmountMustBePositiveNumber);

		let (mint_amount, base_fee, quote_fee) = if total_lp_issued > T::Balance::zero() {
			// Deposit x + withdraw y should charge about same
			// fees as a swap. Otherwise, one could exchange w/o paying fees.
			// And this formula leads to exactly that equality
			// fee = pool.fee * n_coins / (4 * (n_coins - 1))
			// pool supports only two coins.
			// https://ethereum.stackexchange.com/questions/124850/curve-amm-how-is-fee-calculated-when-adding-liquidity
			let share: Permill = Permill::from_rational(2_u32, 4_u32);
			let updated_fee_config = pool_info.fee_config.mul(share);

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

			// differences from the ideal balances to be used in fee calculation
			let base_difference = Self::abs_difference(ideal_base_balance, new_base_amount)?;
			let quote_difference = Self::abs_difference(ideal_quote_balance, new_quote_amount)?;
			let base_fee = updated_fee_config.calculate_fees(pool_info.pair.base, base_difference);
			let quote_fee =
				updated_fee_config.calculate_fees(pool_info.pair.quote, quote_difference);

			// Substract fees from calculated base/quote amounts to allow for fees
			let new_base_balance = new_base_amount.safe_sub(&base_fee.fee)?;
			let new_quote_balance = new_quote_amount.safe_sub(&quote_fee.fee)?;

			let d2 = Self::get_invariant(
				new_base_balance,
				new_quote_balance,
				amplification_coefficient,
			)?;
			// minted LP is proportional to the delta of the pool invariant caused by imbalanced
			// liquidity
			let mint_amount = T::Convert::convert(safe_multiply_by_rational(
				T::Convert::convert(total_lp_issued),
				T::Convert::convert(d2.safe_sub(&d0)?),
				T::Convert::convert(d0),
			)?);
			(mint_amount, base_fee, quote_fee)
		} else {
			(
				d1,
				Fee::<T::AssetId, T::Balance>::zero(pool_info.pair.base),
				Fee::<T::AssetId, T::Balance>::zero(pool_info.pair.quote),
			)
		};
		Ok((mint_amount, base_fee, quote_fee))
	}
}
