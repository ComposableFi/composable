use crate::{Config, Error, PoolConfiguration, PoolCount, Pools};
use composable_maths::dex::constant_product::{
	compute_deposit_lp, compute_deposit_lp_single_asset, compute_in_given_out, compute_out_given_in,
};
use composable_support::math::safe::{SafeAdd, SafeSub};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::CurrencyPair,
	dex::{ConstantProductPoolInfo, Fee, FeeConfig},
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
};
use sp_runtime::{
	traits::{Convert, One, Zero},
	Permill,
};

// Uniswap
pub(crate) struct Uniswap<T>(PhantomData<T>);

impl<T: Config> Uniswap<T> {
	pub(crate) fn do_create_pool(
		who: &T::AccountId,
		pair: CurrencyPair<T::AssetId>,
		fee_config: FeeConfig,
		base_weight: Permill,
	) -> Result<T::PoolId, DispatchError> {
		// TODO(hussein-aitlahcen): refactor all those checks using Validated
		ensure!(base_weight != Permill::zero(), Error::<T>::WeightsMustBeNonZero);
		ensure!(base_weight < Permill::one(), Error::<T>::WeightsMustSumToOne);
		ensure!(pair.base != pair.quote, Error::<T>::InvalidPair);
		ensure!(fee_config.fee_rate < Permill::one(), Error::<T>::InvalidFees);

		let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS, T::Balance::default())?;

		let quote_weight = Permill::one().safe_sub(&base_weight)?;

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
						fee_config,
						base_weight,
						quote_weight,
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
		ensure!(pool.pair.contains(asset_id), Error::<T>::InvalidAsset);
		let amount = T::Convert::convert(amount);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, pool_account));
		let pool_quote_aum = T::Convert::convert(T::Assets::balance(pool.pair.quote, pool_account));
		ensure!(
			!pool_base_aum.is_zero() && !pool_quote_aum.is_zero(),
			Error::<T>::NotEnoughLiquidity
		);
		let exchange_amount = if asset_id == pool.pair.quote {
			compute_out_given_in(
				pool.quote_weight,
				pool.base_weight,
				pool_quote_aum,
				pool_base_aum,
				amount,
			)
		} else {
			compute_in_given_out(
				pool.quote_weight,
				pool.base_weight,
				pool_quote_aum,
				pool_base_aum,
				amount,
			)
		}?;
		Ok(T::Convert::convert(exchange_amount))
	}

	pub(crate) fn add_liquidity(
		who: &T::AccountId,
		pool: &ConstantProductPoolInfo<T::AccountId, T::AssetId>,
		pool_account: &T::AccountId,
		base_amount: T::Balance,
		quote_amount: T::Balance,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance, Fee<T::AssetId, T::Balance>), DispatchError> {
		ensure!(base_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		// single asset case
		if quote_amount.is_zero() {
			return Self::add_liquidity_single_asset(
				who,
				pool,
				pool_account,
				base_amount,
				min_mint_amount,
				keep_alive,
			)
		}
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, pool_account));
		let pool_quote_aum = T::Convert::convert(T::Assets::balance(pool.pair.quote, pool_account));

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
		let (quote_amount, amount_of_lp_token_to_mint) = compute_deposit_lp(
			lp_total_issuance,
			T::Convert::convert(base_amount),
			T::Convert::convert(quote_amount),
			pool_base_aum,
			pool_quote_aum,
		)?;
		let quote_amount = T::Convert::convert(quote_amount);
		let amount_of_lp_token_to_mint = T::Convert::convert(amount_of_lp_token_to_mint);

		ensure!(quote_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		ensure!(
			amount_of_lp_token_to_mint >= min_mint_amount,
			Error::<T>::CannotRespectMinimumRequested
		);

		T::Assets::transfer(pool.pair.base, who, pool_account, base_amount, keep_alive)?;
		T::Assets::transfer(pool.pair.quote, who, pool_account, quote_amount, keep_alive)?;
		T::Assets::mint_into(pool.lp_token, who, amount_of_lp_token_to_mint)?;
		Ok((base_amount, quote_amount, amount_of_lp_token_to_mint, Fee::zero(pool.pair.base)))
	}

	fn add_liquidity_single_asset(
		who: &T::AccountId,
		pool: &ConstantProductPoolInfo<T::AccountId, T::AssetId>,
		pool_account: &T::AccountId,
		amount: T::Balance,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance, Fee<T::AssetId, T::Balance>), DispatchError> {
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, pool_account));
		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
		let fee = pool.fee_config.calculate_fees_for_single_asset(
			pool.pair.base,
			pool.base_weight,
			amount,
		);

		let amount_without_fee = amount.safe_sub(&fee.fee)?;
		let amount_of_lp_token_to_mint = compute_deposit_lp_single_asset(
			T::Convert::convert(amount_without_fee),
			pool_base_aum,
			pool.base_weight,
			lp_total_issuance,
		)?;
		let amount_of_lp_token_to_mint = T::Convert::convert(amount_of_lp_token_to_mint);
		ensure!(
			amount_of_lp_token_to_mint >= min_mint_amount,
			Error::<T>::CannotRespectMinimumRequested
		);

		T::Assets::transfer(pool.pair.base, who, pool_account, amount_without_fee, keep_alive)?;
		T::Assets::mint_into(pool.lp_token, who, amount_of_lp_token_to_mint)?;

		Ok((amount, T::Balance::zero(), amount_of_lp_token_to_mint, fee))
	}

	pub(crate) fn remove_liquidity(
		who: &T::AccountId,
		pool: ConstantProductPoolInfo<T::AccountId, T::AssetId>,
		pool_account: T::AccountId,
		lp_amount: T::Balance,
		base_amount: T::Balance,
		quote_amount: T::Balance,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		let lp_issued = T::Assets::total_issuance(pool.lp_token);
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
	) -> Result<(T::Balance, T::Balance, Fee<T::AssetId, T::Balance>), DispatchError> {
		ensure!(pair == pool.pair, Error::<T>::PairMismatch);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pair.base, pool_account));
		let pool_quote_aum = T::Convert::convert(T::Assets::balance(pair.quote, pool_account));

		let fee = if apply_fees {
			pool.fee_config.calculate_fees(pair.quote, quote_amount)
		} else {
			Fee::<T::AssetId, T::Balance>::zero(pair.quote)
		};
		// Charging fees "on the way in"
		// https://balancer.gitbook.io/balancer/core-concepts/protocol/index#out-given-in
		let quote_amount_excluding_lp_fee = T::Convert::convert(quote_amount.safe_sub(&fee.fee)?);
		let base_amount = compute_out_given_in(
			pool.quote_weight,
			pool.base_weight,
			pool_quote_aum,
			pool_base_aum,
			quote_amount_excluding_lp_fee,
		)?;
		ensure!(base_amount > 0 && quote_amount_excluding_lp_fee > 0, Error::<T>::InvalidAmount);

		Ok((
			T::Convert::convert(base_amount),
			T::Convert::convert(quote_amount_excluding_lp_fee),
			fee,
		))
	}
}
