use crate::{
	AccountIdOf, AssetIdOf, BalanceOf, Config, Error, LiquidityBootstrappingPoolInfoOf,
	PoolConfiguration, PoolCount, Pools,
};
use composable_maths::dex::constant_product::{compute_out_given_in, compute_spot_price};
use composable_support::{
	math::safe::{SafeAdd, SafeSub},
	validation::{Validate, Validated},
};
use composable_traits::{
	currency::LocalAssets,
	defi::CurrencyPair,
	dex::{Fee, SaleState},
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Transfer},
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::{BlockNumberProvider, Convert, One, Zero};
use sp_std::marker::PhantomData;

#[derive(Copy, Clone, Encode, Decode, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
pub struct PoolIsValid<T>(PhantomData<T>);

impl<T: Config> Validate<LiquidityBootstrappingPoolInfoOf<T>, PoolIsValid<T>> for PoolIsValid<T> {
	fn validate(
		input: LiquidityBootstrappingPoolInfoOf<T>,
	) -> Result<LiquidityBootstrappingPoolInfoOf<T>, &'static str> {
		if input.pair.base == input.pair.quote {
			return Err("Pair elements must be distinct.")
		}

		if input.sale.end <= input.sale.start {
			return Err("Sale end must be after start.")
		}

		if input.sale.duration() < T::LbpMinSaleDuration::get() {
			return Err("Sale duration must be greater than minimum duration.")
		}

		if input.sale.duration() > T::LbpMaxSaleDuration::get() {
			return Err("Sale duration must not exceed maximum duration.")
		}

		if input.sale.initial_weight < input.sale.final_weight {
			return Err("Initial weight must be greater than final weight.")
		}

		if input.sale.initial_weight > T::LbpMaxInitialWeight::get() {
			return Err("Initial weight must not exceed the defined maximum.")
		}

		if input.sale.final_weight < T::LbpMinFinalWeight::get() {
			return Err("Final weight must not be lower than the defined minimum.")
		}

		Ok(input)
	}
}

pub(crate) struct LiquidityBootstrapping<T>(PhantomData<T>);

impl<T: Config> LiquidityBootstrapping<T> {
	pub(crate) fn do_create_pool(
		pool: Validated<LiquidityBootstrappingPoolInfoOf<T>, PoolIsValid<T>>,
	) -> Result<T::PoolId, DispatchError> {
		let pool_id =
			PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
				let pool_id = *pool_count;
				Pools::<T>::insert(
					pool_id,
					PoolConfiguration::LiquidityBootstrapping(pool.clone().value()),
				);
				*pool_count = pool_id.safe_add(&T::PoolId::one())?;
				Ok(pool_id)
			})?;

		Ok(pool_id)
	}

	fn ensure_sale_state(
		pool: &LiquidityBootstrappingPoolInfoOf<T>,
		current_block: BlockNumberFor<T>,
		expected_sale_state: SaleState,
	) -> Result<(), DispatchError> {
		ensure!(
			pool.sale.state(current_block) == expected_sale_state,
			Error::<T>::InvalidSaleState
		);
		Ok(())
	}

	#[allow(dead_code)]
	pub(crate) fn do_spot_price(
		pool: LiquidityBootstrappingPoolInfoOf<T>,
		pool_account: AccountIdOf<T>,
		pair: CurrencyPair<AssetIdOf<T>>,
		current_block: BlockNumberFor<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		Self::ensure_sale_state(&pool, current_block, SaleState::Ongoing)?;
		ensure!(pair == pool.pair, Error::<T>::PairMismatch);

		let weights = pool.sale.current_weights(current_block)?;

		let (wo, wi) = if pair.base == pool.pair.base { weights } else { (weights.1, weights.0) };

		let bi = T::Convert::convert(T::Assets::balance(pair.quote, &pool_account));
		let bo = T::Convert::convert(T::Assets::balance(pair.base, &pool_account));
		let base_unit = T::LocalAssets::unit::<u128>(pair.base)?;

		let spot_price = compute_spot_price(wi, wo, bi, bo, base_unit)?;

		Ok(T::Convert::convert(spot_price))
	}

	pub(crate) fn do_get_exchange(
		pool: LiquidityBootstrappingPoolInfoOf<T>,
		pool_account: &AccountIdOf<T>,
		pair: CurrencyPair<AssetIdOf<T>>,
		current_block: BlockNumberFor<T>,
		quote_amount: BalanceOf<T>,
		apply_fees: bool,
	) -> Result<(Fee<AssetIdOf<T>, BalanceOf<T>>, BalanceOf<T>), DispatchError> {
		Self::ensure_sale_state(&pool, current_block, SaleState::Ongoing)?;
		ensure!(pair == pool.pair, Error::<T>::PairMismatch);
		ensure!(!quote_amount.is_zero(), Error::<T>::InvalidAmount);

		let weights = pool.sale.current_weights(current_block)?;
		let (wo, wi) = if pair.base == pool.pair.base { weights } else { (weights.1, weights.0) };
		let fee = if apply_fees {
			pool.fee_config.calculate_fees(pair.quote, quote_amount)
		} else {
			Fee::<T::AssetId, T::Balance>::zero(pair.quote)
		};

		let bi = T::Convert::convert(T::Assets::balance(pair.quote, pool_account));
		let bo = T::Convert::convert(T::Assets::balance(pair.base, pool_account));
		let ai_except_fees = quote_amount.safe_sub(&fee.fee)?;
		let base_amount =
			compute_out_given_in(wi, wo, bi, bo, T::Convert::convert(ai_except_fees))?;
		Ok((fee, T::Convert::convert(base_amount)))
	}

	pub(crate) fn get_exchange_value(
		pool: LiquidityBootstrappingPoolInfoOf<T>,
		pool_account: AccountIdOf<T>,
		asset_id: T::AssetId,
		amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		ensure!(pool.pair.contains(asset_id), Error::<T>::InvalidAsset);
		let pair = if asset_id == pool.pair.base { pool.pair.swap() } else { pool.pair };
		let current_block = frame_system::Pallet::<T>::current_block_number();
		let (_, base_amount) =
			Self::do_get_exchange(pool, &pool_account, pair, current_block, amount, false)?;
		Ok(base_amount)
	}

	pub(crate) fn add_liquidity(
		who: &T::AccountId,
		pool: LiquidityBootstrappingPoolInfoOf<T>,
		pool_account: AccountIdOf<T>,
		base_amount: T::Balance,
		quote_amount: T::Balance,
		_: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		let current_block = frame_system::Pallet::<T>::current_block_number();
		Self::ensure_sale_state(&pool, current_block, SaleState::NotStarted)?;

		ensure!(pool.owner == *who, Error::<T>::MustBeOwner);
		ensure!(!base_amount.is_zero() && !quote_amount.is_zero(), Error::<T>::InvalidAmount);

		// NOTE(hussein-aitlahcen): as we only allow the owner to provide liquidity, we don't
		// mint any LP.
		T::Assets::transfer(pool.pair.base, who, &pool_account, base_amount, keep_alive)?;
		T::Assets::transfer(pool.pair.quote, who, &pool_account, quote_amount, keep_alive)?;

		Ok((base_amount, quote_amount, T::Balance::zero()))
	}

	pub(crate) fn remove_liquidity(
		who: &T::AccountId,
		_pool_id: T::PoolId,
		pool: LiquidityBootstrappingPoolInfoOf<T>,
		pool_account: AccountIdOf<T>,
		_: T::Balance,
		_: T::Balance,
		_: T::Balance,
	) -> Result<(BalanceOf<T>, BalanceOf<T>), DispatchError> {
		let current_block = frame_system::Pallet::<T>::current_block_number();
		Self::ensure_sale_state(&pool, current_block, SaleState::Ended)?;

		ensure!(pool.owner == *who, Error::<T>::MustBeOwner);

		let repatriate = |a| -> Result<BalanceOf<T>, DispatchError> {
			let a_balance = T::Assets::balance(a, &pool_account);
			// NOTE(hussein-aitlahcen): not need to keep the pool account alive.
			T::Assets::transfer(a, &pool_account, who, a_balance, false)?;
			Ok(a_balance)
		};

		let base_amount = repatriate(pool.pair.base)?;
		let quote_amount = repatriate(pool.pair.quote)?;

		Ok((base_amount, quote_amount))
	}
}
