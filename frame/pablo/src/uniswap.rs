use crate::{
	Config, Error, Event, Pallet, PoolConfiguration, PoolConfigurationOf, PoolCount, Pools,
};
use composable_maths::dex::constant_product::{
	compute_deposit_lp, compute_in_given_out, compute_out_given_in,
};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::CurrencyPair,
	dex::{Amm, ConstantProductPoolInfo},
	math::{safe_multiply_by_rational, SafeAdd, SafeSub},
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
	transactional, PalletId,
};
use sp_runtime::{
	traits::{AccountIdConversion, CheckedAdd, Convert, One, Zero},
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

		let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS)?;

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

		<Pallet<T>>::deposit_event(Event::PoolCreated { pool_id, owner: who.clone() });

		Ok(pool_id)
	}

	/// Assume that the pair is valid for the pool
	pub(crate) fn do_compute_swap(
		pool_id: T::PoolId,
		pair: CurrencyPair<T::AssetId>,
		quote_amount: T::Balance,
		apply_fees: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance, T::Balance), DispatchError> {
		let pool = Self::get_pool(pool_id)?;
		let pool_account = Self::account_id(&pool_id);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
		let pool_quote_aum = T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
		let quote_amount = T::Convert::convert(quote_amount);

		// https://uniswap.org/whitepaper.pdf
		// 3.2.1
		// we do not inflate the lp for the owner fees
		// cut is done before enforcing the invariant
		let (lp_fee, owner_fee) = if apply_fees {
			let lp_fee = pool.fee.mul_floor(quote_amount);
			let owner_fee = pool.owner_fee.mul_floor(quote_amount);
			(lp_fee, owner_fee)
		} else {
			(0, 0)
		};

		let quote_amount_excluding_fees = quote_amount.safe_sub(&lp_fee)?.safe_sub(&owner_fee)?;

		let half_weight = Permill::from_percent(50);
		let base_amount = compute_out_given_in(
			half_weight,
			half_weight,
			pool_quote_aum,
			pool_base_aum,
			quote_amount_excluding_fees,
		)?;

		ensure!(base_amount > 0 && quote_amount_excluding_fees > 0, Error::<T>::InvalidAmount);

		Ok((
			T::Convert::convert(base_amount),
			T::Convert::convert(quote_amount_excluding_fees),
			T::Convert::convert(lp_fee),
			T::Convert::convert(owner_fee),
		))
	}
	fn get_pool(
		pool_id: T::PoolId,
	) -> Result<ConstantProductPoolInfo<T::AccountId, T::AssetId>, DispatchError> {
		let pool_config = Pools::<T>::get(pool_id).expect("TODO FIX");
		// .ok_or_else(|| Error::<T>::PoolNotFound.into())?;
		match pool_config as PoolConfiguration<T::AccountId, T::AssetId> {
			PoolConfiguration::StableSwap(_) => Err(Error::<T>::PoolNotFound.into()),
			PoolConfiguration::ConstantProduct(pool) => Ok(pool),
		}
	}

	fn account_id(pool_id: &T::PoolId) -> T::AccountId {
		T::PalletId::get().into_sub_account(pool_id)
	}
}

impl<T: Config> Amm for Uniswap<T> {
	type AssetId = T::AssetId;
	type Balance = T::Balance;
	type AccountId = T::AccountId;
	type PoolId = T::PoolId;

	fn pool_exists(pool_id: Self::PoolId) -> bool {
		Pools::<T>::contains_key(pool_id)
	}

	fn currency_pair(pool_id: Self::PoolId) -> Result<CurrencyPair<Self::AssetId>, DispatchError> {
		let pool = Self::get_pool(pool_id)?;
		Ok(pool.pair)
	}

	fn get_exchange_value(
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		let pool = Self::get_pool(pool_id)?;
		let pool_account = Self::account_id(&pool_id);
		let amount = T::Convert::convert(amount);
		let half_weight = Permill::from_percent(50);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
		let pool_quote_aum =
			T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));
		let exchange_amount = if asset_id == pool.pair.quote {
			compute_out_given_in(half_weight, half_weight, pool_quote_aum, pool_base_aum, amount)
		} else {
			compute_in_given_out(half_weight, half_weight, pool_quote_aum, pool_base_aum, amount)
		}?;
		Ok(T::Convert::convert(exchange_amount))
	}

	#[transactional]
	fn buy(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError> {
		let pool = Self::get_pool(pool_id)?;
		let pair = if asset_id == pool.pair.base { pool.pair } else { pool.pair.swap() };
		let quote_amount = Self::get_exchange_value(pool_id, asset_id, amount)?;
		<Self as Amm>::exchange(who, pool_id, pair, quote_amount, T::Balance::zero(), keep_alive)
	}

	#[transactional]
	fn sell(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError> {
		let pool = Self::get_pool(pool_id)?;
		let pair = if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
		<Self as Amm>::exchange(who, pool_id, pair, amount, T::Balance::zero(), keep_alive)
	}

	#[transactional]
	fn add_liquidity(
		who: &Self::AccountId,
		pool_id: Self::PoolId,
		base_amount: Self::Balance,
		quote_amount: Self::Balance,
		min_mint_amount: Self::Balance,
		keep_alive: bool,
	) -> Result<(), DispatchError> {
		ensure!(base_amount > T::Balance::zero(), Error::<T>::InvalidAmount);

		let pool = Self::get_pool(pool_id)?;
		let pool_account = Self::account_id(&pool_id);
		let pool_base_aum = T::Convert::convert(T::Assets::balance(pool.pair.base, &pool_account));
		let pool_quote_aum =
			T::Convert::convert(T::Assets::balance(pool.pair.quote, &pool_account));

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
		let (quote_amount, lp_to_mint) = compute_deposit_lp(
			lp_total_issuance,
			T::Convert::convert(base_amount),
			T::Convert::convert(quote_amount),
			pool_base_aum,
			pool_quote_aum,
		)?;
		let quote_amount = T::Convert::convert(quote_amount);
		let lp_to_mint = T::Convert::convert(lp_to_mint);

		ensure!(quote_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		ensure!(lp_to_mint >= min_mint_amount, Error::<T>::CannotRespectMinimumRequested);

		T::Assets::transfer(pool.pair.base, who, &pool_account, base_amount, keep_alive)?;
		T::Assets::transfer(pool.pair.quote, who, &pool_account, quote_amount, keep_alive)?;
		T::Assets::mint_into(pool.lp_token, who, lp_to_mint)?;

		<Pallet<T>>::deposit_event(Event::<T>::LiquidityAddedToConstantProductPool {
			pool_id,
			who: who.clone(),
			base_amount,
			quote_amount,
			minted_lp: lp_to_mint,
		});

		Ok(())
	}

	#[transactional]
	fn remove_liquidity(
		who: &Self::AccountId,
		pool_id: T::PoolId,
		lp_amount: Self::Balance,
		min_base_amount: Self::Balance,
		min_quote_amount: Self::Balance,
	) -> Result<(), DispatchError> {
		let pool = Self::get_pool(pool_id)?;

		let pool_account = Self::account_id(&pool_id);
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

		<Pallet<T>>::deposit_event(Event::<T>::LiquidityRemoved {
			pool_id,
			who: who.clone(),
			base_amount,
			quote_amount,
			total_issuance: lp_issued.safe_sub(&lp_amount)?,
		});

		Ok(())
	}

	#[transactional]
	fn exchange(
		who: &Self::AccountId,
		pool_id: T::PoolId,
		pair: CurrencyPair<Self::AssetId>,
		quote_amount: Self::Balance,
		min_receive: Self::Balance,
		keep_alive: bool,
	) -> Result<Self::Balance, DispatchError> {
		let pool = Self::get_pool(pool_id)?;
		// /!\ NOTE(hussein-aitlahcen): after this check, do not use pool.pair as the provided
		// pair might have been swapped
		ensure!(pair == pool.pair, Error::<T>::PairMismatch);

		let (base_amount, quote_amount, lp_fees, owner_fees) =
			Self::do_compute_swap(pool_id, pair, quote_amount, true)?;
		let total_fees = lp_fees.safe_add(&owner_fees)?;
		let quote_amount_including_fees = quote_amount.safe_add(&total_fees)?;

		ensure!(base_amount >= min_receive, Error::<T>::CannotRespectMinimumRequested);

		let pool_account = Self::account_id(&pool_id);
		T::Assets::transfer(
			pair.quote,
			who,
			&pool_account,
			quote_amount_including_fees,
			keep_alive,
		)?;
		// NOTE(hussein-aitlance): no need to keep alive the pool account
		T::Assets::transfer(pair.quote, &pool_account, &pool.owner, owner_fees, false)?;
		T::Assets::transfer(pair.base, &pool_account, who, base_amount, false)?;

		<Pallet<T>>::deposit_event(Event::<T>::Swapped {
			pool_id,
			who: who.clone(),
			base_asset: pair.base,
			quote_asset: pair.quote,
			base_amount,
			quote_amount,
			fee: total_fees,
		});

		Ok(base_amount)
	}
}

#[cfg(test)]
mod tests {
	use crate::mock::{new_test_ext, ALICE, BOB, BTC, USDT};
	use composable_traits::{defi::CurrencyPair, dex::Amm};
	use frame_support::assert_ok;
	use sp_arithmetic::Permill;

	#[test]
	fn test() {
		new_test_ext().execute_with(|| {});
	}
}
