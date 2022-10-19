use crate::{Config, Error, PoolConfiguration, PoolCount, Pools};
use composable_maths::dex::constant_product::{
	compute_deposit_lp, compute_in_given_out, compute_out_given_in,
};
use composable_support::math::safe::{SafeAdd, SafeSub};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	defi::CurrencyPair,
	dex::{BasicPoolInfo, Fee, FeeConfig},
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
};
use sp_runtime::{
	traits::{Convert, One, Zero},
	BoundedBTreeMap, Permill,
};
use sp_std::vec::Vec;

// Balancer V1 Constant Product Pool
pub(crate) struct DualAssetConstantProduct<T>(PhantomData<T>);

impl<T: Config> DualAssetConstantProduct<T> {
	pub(crate) fn do_create_pool(
		who: &T::AccountId,
		fee_config: FeeConfig,
		assets_weights: BoundedBTreeMap<T::AssetId, Permill, ConstU32<2>>,
	) -> Result<T::PoolId, DispatchError> {
		// TODO(hussein-aitlahcen): refactor all those checks using Validated
		ensure!(assets_weights.len() == 2, Error::<T>::InvalidPair);
		let weights = assets_weights.iter().map(|(_, w)| w).copied().collect::<Vec<_>>();
		ensure!(
			weights[0] != Permill::zero() && weights[1] != Permill::zero(),
			Error::<T>::WeightsMustBeNonZero
		);
		ensure!(
			weights[0].deconstruct() + weights[1].deconstruct() ==
				Permill::from_percent(100).deconstruct(),
			Error::<T>::WeightsMustSumToOne
		);
		ensure!(fee_config.fee_rate < Permill::one(), Error::<T>::InvalidFees);

		let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS, T::Balance::default())?;

		// Add new pool
		let pool_id =
			PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
				let pool_id = *pool_count;
				Pools::<T>::insert(
					pool_id,
					PoolConfiguration::DualAssetConstantProduct(BasicPoolInfo {
						owner: who.clone(),
						assets_weights,
						lp_token,
						fee_config,
					}),
				);
				*pool_count = pool_id.safe_add(&T::PoolId::one())?;
				Ok(pool_id)
			})?;

		Ok(pool_id)
	}

	pub(crate) fn get_exchange_value(
		pool: &BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: &T::AccountId,
		in_asset_id: T::AssetId,
		in_asset_amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		ensure!(pool.assets_weights.contains_key(&in_asset_id), Error::<T>::InvalidAsset);
		let amount = T::Convert::convert(in_asset_amount);
		let pool_assets = Self::get_pool_balances(pool, pool_account);
		// TODO (vim): We have no way of knowing which amount is for which asset (fixed in a later
		//  stage). For now we assume the pool defined order which is wrong as the order is based on
		// Ord
		let base_asset = pool_assets[0];
		let quote_asset = pool_assets[1];
		ensure!(
			!pool_assets.iter().any(|(_, _, balance)| balance.is_zero()),
			Error::<T>::NotEnoughLiquidity
		);

		// TODO (vim): Following does not work for "buy" as this causes "out_given_in" to be used in
		//  case user wanting to buy the quote asset of the pool.
		let exchange_amount = if in_asset_id == quote_asset.0 {
			compute_out_given_in(quote_asset.1, base_asset.1, quote_asset.2, base_asset.2, amount)
		} else {
			compute_in_given_out(quote_asset.1, base_asset.1, quote_asset.2, base_asset.2, amount)
		}?;
		Ok(T::Convert::convert(exchange_amount))
	}

	fn get_pool_balances(
		pool: &BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: &T::AccountId,
	) -> Vec<(T::AssetId, Permill, u128)> {
		pool.assets_weights
			.iter()
			.map(|(asset_id, weight)| {
				(
					*asset_id,
					*weight,
					T::Convert::convert(T::Assets::balance(*asset_id, pool_account)),
				)
			})
			// TODO (vim): Make a map here later when the out_asset is provided.
			.collect::<Vec<_>>()
	}

	pub(crate) fn add_liquidity(
		who: &T::AccountId,
		pool: BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: T::AccountId,
		base_amount: T::Balance,
		quote_amount: T::Balance,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		// TODO (vim): Pool weight validation is missing, which would cause the received LP tokens
		//  to be higher than expected if the base token has more than what is allowed by the pool
		//  weight.
		ensure!(base_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		let pool_assets = Self::get_pool_balances(&pool, &pool_account);
		// TODO (vim): We have no way of knowing which amount is for which asset (fixed in a later
		//  stage). For now we assume the pool defined order
		let base_asset = pool_assets[0];
		let quote_asset = pool_assets[1];

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
		let (quote_amount, amount_of_lp_token_to_mint) = compute_deposit_lp(
			lp_total_issuance,
			T::Convert::convert(base_amount),
			T::Convert::convert(quote_amount),
			base_asset.2,
			quote_asset.2,
		)?;
		let quote_amount = T::Convert::convert(quote_amount);
		let amount_of_lp_token_to_mint = T::Convert::convert(amount_of_lp_token_to_mint);

		ensure!(quote_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		ensure!(
			amount_of_lp_token_to_mint >= min_mint_amount,
			Error::<T>::CannotRespectMinimumRequested
		);

		T::Assets::transfer(base_asset.0, who, &pool_account, base_amount, keep_alive)?;
		T::Assets::transfer(quote_asset.0, who, &pool_account, quote_amount, keep_alive)?;
		T::Assets::mint_into(pool.lp_token, who, amount_of_lp_token_to_mint)?;
		Ok((base_amount, quote_amount, amount_of_lp_token_to_mint))
	}

	pub(crate) fn remove_liquidity(
		who: &T::AccountId,
		pool: BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: T::AccountId,
		lp_amount: T::Balance,
		base_amount: T::Balance,
		quote_amount: T::Balance,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		let lp_issued = T::Assets::total_issuance(pool.lp_token);
		let pool_assets = Self::get_pool_balances(&pool, &pool_account);
		// TODO (vim): We have no way of knowing which amount is for which asset (fixed in a later
		// stage). For now we assume the pool defined order
		let base_asset = pool_assets[0];
		let quote_asset = pool_assets[1];
		T::Assets::transfer(base_asset.0, &pool_account, who, base_amount, false)?;
		T::Assets::transfer(quote_asset.0, &pool_account, who, quote_amount, false)?;
		T::Assets::burn_from(pool.lp_token, who, lp_amount)?;

		Ok((base_amount, quote_amount, lp_issued.safe_sub(&lp_amount)?))
	}

	pub(crate) fn do_compute_swap(
		pool: &BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: &T::AccountId,
		pair: CurrencyPair<T::AssetId>,
		quote_amount: T::Balance,
		apply_fees: bool,
	) -> Result<(T::Balance, T::Balance, Fee<T::AssetId, T::Balance>), DispatchError> {
		let pool_assets = Self::get_pool_balances(pool, pool_account);
		// TODO (vim): We have no way of knowing which amount is for which asset (fixed in a later
		// stage). For now we assume the pool defined order
		let (base_asset, quote_asset) = if pool_assets[0].0 == pair.base {
			(pool_assets[0], pool_assets[1])
		} else {
			(pool_assets[1], pool_assets[0])
		};
		ensure!(CurrencyPair::new(base_asset.0, quote_asset.0) == pair, Error::<T>::PairMismatch);

		let fee = if apply_fees {
			pool.fee_config.calculate_fees(pair.quote, quote_amount)
		} else {
			Fee::<T::AssetId, T::Balance>::zero(pair.quote)
		};
		// Charging fees "on the way in"
		// https://balancer.gitbook.io/balancer/core-concepts/protocol/index#out-given-in
		let quote_amount_excluding_lp_fee = T::Convert::convert(quote_amount.safe_sub(&fee.fee)?);
		let base_amount = compute_out_given_in(
			// TODO bug (possibly the cause of https://app.clickup.com/t/39ny0d0) weights are swapped when the pair sent is different from the pool pair
			quote_asset.1,
			base_asset.1,
			quote_asset.2,
			base_asset.2,
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
