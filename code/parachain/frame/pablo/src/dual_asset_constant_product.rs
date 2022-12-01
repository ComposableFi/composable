use crate::{Config, Error, PoolConfiguration, PoolCount, Pools};
use composable_maths::dex::{
	constant_product::{compute_deposit_lp, compute_in_given_out_new, compute_out_given_in_new},
	PoolWeightMathExt,
};
use composable_support::math::safe::{SafeAdd, SafeSub};
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	dex::{AssetAmount, BasicPoolInfo, Fee, FeeConfig},
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
};
use sp_runtime::{
	traits::{Convert, One, Zero},
	BoundedBTreeMap, Permill,
};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

// Balancer V1 Constant Product Pool
pub(crate) struct DualAssetConstantProduct<T>(PhantomData<T>);

impl<T: Config> DualAssetConstantProduct<T> {
	pub(crate) fn do_create_pool(
		who: &T::AccountId,
		fee_config: FeeConfig,
		assets_weights: BoundedBTreeMap<T::AssetId, Permill, ConstU32<2>>,
	) -> Result<T::PoolId, DispatchError> {
		ensure!(assets_weights.len() == 2, Error::<T>::InvalidPair);
		ensure!(assets_weights.values().non_zero_weights(), Error::<T>::WeightsMustBeNonZero);
		ensure!(
			if let Some(total_weight) = assets_weights.values().sum_weights() {
				total_weight.is_one()
			} else {
				// If `sum_weights` returns `None`, it overflowed and weights are not normalized
				false
			},
			Error::<T>::WeightsMustSumToOne
		);
		ensure!(fee_config.fee_rate < Permill::one(), Error::<T>::InvalidFees);

		let lp_token = T::CurrencyFactory::create(RangeId::LP_TOKENS)?;
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

	fn get_pool_balances(
		pool: &BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: &T::AccountId,
	) -> BTreeMap<T::AssetId, (Permill, u128)> {
		pool.assets_weights
			.iter()
			.map(|(asset_id, weight)| {
				(
					*asset_id,
					(*weight, T::Convert::convert(T::Assets::balance(*asset_id, pool_account))),
				)
			})
			.collect::<BTreeMap<_, _>>()
	}

	pub(crate) fn add_liquidity(
		who: &T::AccountId,
		pool: BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: T::AccountId,
		assets: BTreeMap<T::AssetId, T::Balance>,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		// TODO (vim): Pool weight validation is missing, which would cause the received LP tokens
		//  to be higher than expected if the base token has more than what is allowed by the pool
		//  weight.
		ensure!(!assets.values().any(|balance| balance.is_zero()), Error::<T>::InvalidAmount);
		let pool_assets = Self::get_pool_balances(&pool, &pool_account);
		let assets_vec = pool_assets.keys().copied().collect::<Vec<_>>();
		// This function currently expects liquidity to be provided in all assets in weight ratio
		ensure!(assets_vec == assets.keys().copied().collect::<Vec<_>>(), Error::<T>::PairMismatch);
		// TODO (vim): Change later. Make a vector of keys to easily map base, quote for now
		let first_asset = assets_vec.get(0).expect("Must exist as per previous validations");
		let first_asset_amount =
			assets.get(first_asset).expect("Must exist as per previous validations");
		let second_asset = assets_vec.get(1).expect("Must exist as per previous validations");
		let second_asset_amount =
			assets.get(second_asset).expect("Must exist as per previous validations");

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));
		let (quote_amount, amount_of_lp_token_to_mint) = compute_deposit_lp(
			lp_total_issuance,
			T::Convert::convert(*first_asset_amount),
			T::Convert::convert(*second_asset_amount),
			pool_assets.get(first_asset).expect("Must exist as per previous validations").1,
			pool_assets.get(second_asset).expect("Must exist as per previous validations").1,
		)?;
		let quote_amount = T::Convert::convert(quote_amount);
		let amount_of_lp_token_to_mint = T::Convert::convert(amount_of_lp_token_to_mint);

		ensure!(quote_amount > T::Balance::zero(), Error::<T>::InvalidAmount);
		ensure!(
			amount_of_lp_token_to_mint >= min_mint_amount,
			Error::<T>::CannotRespectMinimumRequested
		);

		T::Assets::transfer(*first_asset, who, &pool_account, *first_asset_amount, keep_alive)?;
		T::Assets::transfer(*second_asset, who, &pool_account, *second_asset_amount, keep_alive)?;
		T::Assets::mint_into(pool.lp_token, who, amount_of_lp_token_to_mint)?;
		Ok((*first_asset_amount, quote_amount, amount_of_lp_token_to_mint))
	}

	pub(crate) fn remove_liquidity(
		who: &T::AccountId,
		pool: BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: T::AccountId,
		lp_amount: T::Balance,
		min_receive: BTreeMap<T::AssetId, T::Balance>,
	) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
		let lp_issued = T::Assets::total_issuance(pool.lp_token);
		let pool_assets = Self::get_pool_balances(&pool, &pool_account);
		// TODO (vim): Business logic of calculating redeemable amounts must be called here
		let assets = pool_assets.keys().copied().collect::<Vec<_>>();

		let first_asset_amount = min_receive.get(&assets[0]).ok_or(Error::<T>::InvalidAsset)?;
		let second_asset_amount = min_receive.get(&assets[1]).ok_or(Error::<T>::InvalidAsset)?;
		T::Assets::transfer(assets[0], &pool_account, who, *first_asset_amount, false)?;
		T::Assets::transfer(assets[1], &pool_account, who, *second_asset_amount, false)?;
		T::Assets::burn_from(pool.lp_token, who, lp_amount)?;

		Ok((*first_asset_amount, *second_asset_amount, lp_issued.safe_sub(&lp_amount)?))
	}

	pub(crate) fn get_exchange_value(
		pool: &BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: &T::AccountId,
		in_asset: AssetAmount<T::AssetId, T::Balance>,
		out_asset_id: T::AssetId,
		apply_fees: bool,
	) -> Result<
		(
			AssetAmount<T::AssetId, T::Balance>,
			AssetAmount<T::AssetId, T::Balance>,
			Fee<T::AssetId, T::Balance>,
		),
		DispatchError,
	> {
		let pool_assets = Self::get_pool_balances(pool, pool_account);
		let a_sent = T::Convert::convert(in_asset.amount);
		let fee = if apply_fees { pool.fee_config.fee_rate } else { Permill::zero() };
		let (w_i, b_i) = pool_assets.get(&in_asset.asset_id).ok_or(Error::<T>::AssetNotFound)?;
		let (w_o, b_o) = pool_assets.get(&out_asset_id).ok_or(Error::<T>::AssetNotFound)?;

		let amm_pair = compute_out_given_in_new::<_>(*w_i, *w_o, *b_i, *b_o, a_sent, fee)?;

		let a_out = AssetAmount::new(out_asset_id, T::Convert::convert(amm_pair.value));
		let a_sent = AssetAmount::new(in_asset.asset_id, in_asset.amount);
		let fee = pool
			.fee_config
			.calculate_fees(in_asset.asset_id, T::Convert::convert(amm_pair.fee));

		Ok((a_out, a_sent, fee))
	}

	pub(crate) fn do_buy(
		pool: &BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: &T::AccountId,
		out_asset: AssetAmount<T::AssetId, T::Balance>,
		in_asset_id: T::AssetId,
		apply_fees: bool,
	) -> Result<
		(
			AssetAmount<T::AssetId, T::Balance>,
			AssetAmount<T::AssetId, T::Balance>,
			Fee<T::AssetId, T::Balance>,
		),
		DispatchError,
	> {
		let pool_assets = Self::get_pool_balances(pool, pool_account);
		let a_out = T::Convert::convert(out_asset.amount);
		let fee = if apply_fees { pool.fee_config.fee_rate } else { Permill::zero() };
		let (w_o, b_o) = pool_assets.get(&out_asset.asset_id).ok_or(Error::<T>::AssetNotFound)?;
		let (w_i, b_i) = pool_assets.get(&in_asset_id).ok_or(Error::<T>::AssetNotFound)?;

		let amm_pair = compute_in_given_out_new(*w_i, *w_o, *b_i, *b_o, a_out, fee)?;

		let a_sent = AssetAmount::new(in_asset_id, T::Convert::convert(amm_pair.value));
		let fee = pool.fee_config.calculate_fees(in_asset_id, T::Convert::convert(amm_pair.fee));

		Ok((out_asset, a_sent, fee))
	}
}
