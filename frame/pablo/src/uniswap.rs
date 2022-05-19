use crate::{Config, Error, PoolConfiguration, PoolCount, Pools};
use composable_maths::dex::constant_product::{
	compute_deposit_lp, compute_in_given_out, compute_out_given_in,
};
use composable_support::math::safe::{safe_multiply_by_rational, SafeAdd, SafeSub};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::CurrencyPair,
	dex::ConstantProductPoolInfo,
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
	transactional,
};
use sp_runtime::{
	traits::{CheckedAdd, Convert, One, Zero},
	ArithmeticError, Permill,
};

// Uniswap
pub(crate) struct Uniswap<T>(PhantomData<T>);

impl<T: Config> Uniswap<T> {
	#[transactional]
	pub(crate) fn do_create_pool(
		who: &T::AccountId,
		pair: CurrencyPair<T::AssetId>,
		fee: Permill,
		owner_fee: Permill,
	) -> Result<T::PoolId, DispatchError> {
		// NOTE(hussein-aitlahcen): do we allow such pair?
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
					PoolConfiguration::ConstantProduct(ConstantProductPoolInfo {
						owner: who.clone(),
						pair,
						lp_token,
						fee,
						owner_fee,
					}),
				);
				*pool_count = pool_id.safe_add(&T::PoolId::one())?;
				Ok(pool_id)
			})?;

		Ok(pool_id)
	}

	pub(crate) fn get_exchange_value(
		pool: &ConstantProductPoolInfo<T::AccountId, T::AssetId>,
		pool_account: &T::AccountId,
		asset_id: T::AssetId,
		amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		ensure!(
			asset_id == pool.pair.base || asset_id == pool.pair.quote,
			Error::<T>::InvalidAsset
		);
		let amount = T::Convert::convert(amount);
		let half_weight = Permill::from_percent(50);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, pool_account));
		let pool_quote_aum = T::Convert::convert(T::Assets::balance(pool.pair.quote, pool_account));
		ensure!(
			!pool_base_aum.is_zero() && !pool_quote_aum.is_zero(),
			Error::<T>::NotEnoughLiquidity
		);
		let exchange_amount = if asset_id == pool.pair.quote {
			compute_out_given_in(half_weight, half_weight, pool_quote_aum, pool_base_aum, amount)
		} else {
			compute_in_given_out(half_weight, half_weight, pool_quote_aum, pool_base_aum, amount)
		}?;
		Ok(T::Convert::convert(exchange_amount))
	}

	#[transactional]
	pub(crate) fn add_liquidity(
		who: &T::AccountId,
		pool: ConstantProductPoolInfo<T::AccountId, T::AssetId>,
		pool_account: T::AccountId,
		base_amount: T::Balance,
		quote_amount: T::Balance,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		ensure!(base_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
		let pool_quote_aum =
			T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
		let (quote_amount, lp_token_to_mint) = compute_deposit_lp(
			lp_total_issuance,
			T::Convert::convert(base_amount),
			T::Convert::convert(quote_amount),
			pool_base_aum,
			pool_quote_aum,
		)?;
		let quote_amount = T::Convert::convert(quote_amount);
		let lp_token_to_mint = T::Convert::convert(lp_token_to_mint);

		ensure!(quote_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		ensure!(lp_token_to_mint >= min_mint_amount, Error::<T>::CannotRespectMinimumRequested);

		T::Assets::transfer(pool.pair.base, who, &pool_account, base_amount, keep_alive)?;
		T::Assets::transfer(pool.pair.quote, who, &pool_account, quote_amount, keep_alive)?;
		T::Assets::mint_into(pool.lp_token, who, lp_token_to_mint)?;
		Ok((base_amount, quote_amount, lp_token_to_mint))
	}

	#[transactional]
	pub(crate) fn remove_liquidity(
		who: &T::AccountId,
		pool: ConstantProductPoolInfo<T::AccountId, T::AssetId>,
		pool_account: T::AccountId,
		lp_amount: T::Balance,
		min_base_amount: T::Balance,
		min_quote_amount: T::Balance,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
		let pool_quote_aum =
			T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));
		let lp_issued = T::Assets::total_issuance(pool.lp_token);

		let base_amount = T::Convert::convert(safe_multiply_by_rational(
			T::Convert::convert(lp_amount),
			pool_base_aum,
			T::Convert::convert(lp_issued),
		)?);
		let quote_amount = T::Convert::convert(safe_multiply_by_rational(
			T::Convert::convert(lp_amount),
			pool_quote_aum,
			T::Convert::convert(lp_issued),
		)?);

		ensure!(
			base_amount >= min_base_amount && quote_amount >= min_quote_amount,
			Error::<T>::CannotRespectMinimumRequested
		);

		// NOTE(hussein-aitlance): no need to keep alive the pool account
		T::Assets::transfer(pool.pair.base, &pool_account, who, base_amount, false)?;
		T::Assets::transfer(pool.pair.quote, &pool_account, who, quote_amount, false)?;
		T::Assets::burn_from(pool.lp_token, who, lp_amount)?;

		Ok((base_amount, quote_amount, lp_issued.safe_sub(&lp_amount)?))
	}

	pub(crate) fn do_compute_swap(
		pool: &ConstantProductPoolInfo<T::AccountId, T::AssetId>,
		pool_account: &T::AccountId,
		pair: CurrencyPair<T::AssetId>,
		quote_amount: T::Balance,
		apply_fees: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance, T::Balance), DispatchError> {
		ensure!(pair == pool.pair, Error::<T>::PairMismatch);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pair.base, pool_account));
		let pool_quote_aum = T::Convert::convert(T::Assets::balance(pair.quote, pool_account));
		let quote_amount = T::Convert::convert(quote_amount);

		let (lp_fee, owner_fee) = if apply_fees {
			let lp_fee = pool.fee.mul_floor(quote_amount);
			// owner_fee is computed based on lp_fee
			let owner_fee = pool.owner_fee.mul_floor(lp_fee);
			(lp_fee, owner_fee)
		} else {
			(0, 0)
		};
		// Charging fees "on the way in"
		// https://balancer.gitbook.io/balancer/core-concepts/protocol/index#out-given-in
		let quote_amount_excluding_lp_fee = quote_amount.safe_sub(&lp_fee)?;
		let half_weight = Permill::from_percent(50);
		let base_amount = compute_out_given_in(
			half_weight,
			half_weight,
			pool_quote_aum,
			pool_base_aum,
			quote_amount_excluding_lp_fee,
		)?;
		ensure!(base_amount > 0 && quote_amount_excluding_lp_fee > 0, Error::<T>::InvalidAmount);

		Ok((
			T::Convert::convert(base_amount),
			T::Convert::convert(quote_amount_excluding_lp_fee),
			T::Convert::convert(lp_fee),
			T::Convert::convert(owner_fee),
		))
	}
}
