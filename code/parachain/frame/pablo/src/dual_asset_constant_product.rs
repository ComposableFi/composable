use crate::{AssetIdOf, Config, Error, PoolConfiguration, PoolCount, Pools};
use composable_maths::dex::{
	constant_product::{
		compute_deposit_lp_, compute_first_deposit_lp_, compute_in_given_out_new,
		compute_out_given_in_new, compute_redeemed_for_lp,
	},
	per_thing_acceptable_computation_error, PoolWeightMathExt,
};
use composable_support::math::safe::SafeAdd;
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	dex::{AssetAmount, BasicPoolInfo, Fee, FeeConfig},
};
use frame_support::{
	defensive,
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
		lp_token_id: Option<AssetIdOf<T>>,
	) -> Result<T::PoolId, DispatchError> {
		ensure!(assets_weights.len() == 2, Error::<T>::InvalidPair);
		ensure!(assets_weights.values().non_zero_weights(), Error::<T>::WeightsMustBeNonZero);
		ensure!(
			assets_weights
				.values()
				.sum_weights()
				.map(|total_weight| total_weight.is_one())
				// If `None`, `sum_weights` overflowed - weights are not normalized
				.unwrap_or(false),
			Error::<T>::WeightsMustSumToOne
		);
		ensure!(fee_config.fee_rate < Permill::one(), Error::<T>::InvalidFees);

		// NOTE: Will fully move away from CF at a later date. For now, all pools used in production
		// should be created with a supplied LPT via Pablo's `do_create_pool` function.
		let lp_token = lp_token_id.unwrap_or(T::CurrencyFactory::create(RangeId::LP_TOKENS)?);

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
		assets: BoundedVec<AssetAmount<T::AssetId, T::Balance>, ConstU32<2>>,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<T::Balance, DispatchError> {
		let mut pool_assets = Self::get_pool_balances(&pool, &pool_account);

		let assets_with_balances = assets
			.iter()
			.map(|asset_amount| {
				if asset_amount.amount.is_zero() {
					return Err(Error::<T>::InvalidAmount)
				};

				let balance =
					pool_assets.remove(&asset_amount.asset_id).ok_or(Error::<T>::AssetNotFound)?;

				Ok((asset_amount, balance))
			})
			.collect::<Result<Vec<_>, _>>()?;

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));

		let amount_of_lp_token_to_mint = match assets_with_balances[..] {
			[(single, (single_weight, single_balance))] => {
				if lp_total_issuance.is_zero() {
					return Err(Error::<T>::InitialDepositCannotBeZero.into())
				}

				let single_deposit = compute_deposit_lp_(
					lp_total_issuance,
					T::Convert::convert(single.amount),
					single_balance,
					single_weight,
					pool.fee_config.fee_rate,
				)?;

				T::Assets::transfer(
					single.asset_id,
					who,
					&pool_account,
					single.amount,
					keep_alive,
				)?;

				single_deposit.value
			},
			[(first, (first_weight, first_balance)), (second, (second_weight, second_balance))] => {
				let lp_to_mint = if lp_total_issuance.is_zero() {
					compute_first_deposit_lp_(
						&[
							(T::Convert::convert(first.amount), first_weight),
							(T::Convert::convert(second.amount), second_weight),
						],
						Permill::zero(),
					)?
					.value
				} else {
					let input_ratio_first_to_second = Permill::from_rational(
						first.amount,
						first.amount.safe_add(&second.amount)?,
					);

					ensure!(
						per_thing_acceptable_computation_error(
							input_ratio_first_to_second,
							first_weight
						),
						Error::<T>::IncorrectAssetAmounts
					);

					// pass 1 as weight since adding liquidity for all assets
					// see docs on compute_deposit_lp_ for more information
					sp_std::if_std! {
						let _first_deposit = compute_deposit_lp_(
							lp_total_issuance,
							T::Convert::convert(first.amount),
							first_balance,
							Permill::one(),
							pool.fee_config.fee_rate,
						)?;
					}

					let second_deposit = compute_deposit_lp_(
						lp_total_issuance,
						T::Convert::convert(second.amount),
						second_balance,
						Permill::one(),
						pool.fee_config.fee_rate,
					)?;

					second_deposit.value
				};

				T::Assets::transfer(first.asset_id, who, &pool_account, first.amount, keep_alive)?;
				T::Assets::transfer(
					second.asset_id,
					who,
					&pool_account,
					second.amount,
					keep_alive,
				)?;

				lp_to_mint
			},
			_ => {
				defensive!("this should be unreachable, since the input assets are bounded at 2");
				return Err(Error::<T>::UnsupportedOperation.into())
			},
		};

		let amount_of_lp_token_to_mint = T::Convert::convert(amount_of_lp_token_to_mint);

		ensure!(
			amount_of_lp_token_to_mint >= min_mint_amount,
			Error::<T>::CannotRespectMinimumRequested
		);

		T::Assets::mint_into(pool.lp_token, who, amount_of_lp_token_to_mint)?;

		Ok(amount_of_lp_token_to_mint)
	}

	pub(crate) fn remove_liquidity(
		who: &T::AccountId,
		pool: BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: T::AccountId,
		lp_amount: T::Balance,
		min_receive: BoundedVec<AssetAmount<T::AssetId, T::Balance>, ConstU32<2>>,
	) -> Result<(), DispatchError> {
		let mut pool_assets = Self::get_pool_balances(&pool, &pool_account);

		let min_receive_with_current_balances = min_receive
			.iter()
			.map(|asset_amount| {
				let balance =
					pool_assets.remove(&asset_amount.asset_id).ok_or(Error::<T>::AssetNotFound)?;

				Ok::<_, Error<T>>((asset_amount, balance))
			})
			.collect::<Result<Vec<_>, _>>()?;

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));

		match min_receive_with_current_balances[..] {
			[(single, (single_weight, single_balance))] => {
				let single_redeemed_amount = compute_redeemed_for_lp(
					lp_total_issuance,
					T::Convert::convert(lp_amount),
					single_balance,
					single_weight,
				)?;

				ensure!(
					single_redeemed_amount >= T::Convert::convert(single.amount),
					Error::<T>::CannotRespectMinimumRequested
				);

				T::Assets::transfer(
					single.asset_id,
					&pool_account,
					who,
					T::Convert::convert(single_redeemed_amount),
					false, // pool account doesn't need to be kept alive
				)?;
			},
			[(first_min_receive, (_first_weight, first_balance)), (second_min_receive, (_second_weight, second_balance))] =>
			{
				let first_redeemed_amount = compute_redeemed_for_lp(
					lp_total_issuance,
					T::Convert::convert(lp_amount),
					first_balance,
					Permill::one(),
				)?;
				let second_redeemed_amount = compute_redeemed_for_lp(
					lp_total_issuance,
					T::Convert::convert(lp_amount),
					second_balance,
					Permill::one(),
				)?;

				ensure!(
					first_redeemed_amount >= T::Convert::convert(first_min_receive.amount) &&
						second_redeemed_amount >= T::Convert::convert(second_min_receive.amount),
					Error::<T>::CannotRespectMinimumRequested
				);

				T::Assets::transfer(
					first_min_receive.asset_id,
					&pool_account,
					who,
					T::Convert::convert(first_redeemed_amount),
					false, // pool account doesn't need to be kept alive
				)?;
				T::Assets::transfer(
					second_min_receive.asset_id,
					&pool_account,
					who,
					T::Convert::convert(second_redeemed_amount),
					false, // pool account doesn't need to be kept alive
				)?;
			},
			_ => {
				defensive!("this should be unreachable, since the input assets are bounded at 2");
				return Err(Error::<T>::UnsupportedOperation.into())
			},
		};

		T::Assets::burn_from(pool.lp_token, who, lp_amount)?;

		Ok(())
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
