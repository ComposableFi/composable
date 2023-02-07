use crate::{
	create_lpt_asset, AssetIdOf, Config, Error, LPTNonce, PoolConfiguration, PoolCount, Pools,
};
use composable_maths::dex::{
	constant_product::{
		compute_deposit_lp, compute_first_deposit_lp, compute_in_given_out, compute_out_given_in,
		compute_redeemed_for_lp,
	},
	PoolWeightMathExt,
};
use composable_support::{
	abstractions::utils::increment::Increment, collections::vec::bounded::BiBoundedVec,
	math::safe::SafeAdd,
};
use composable_traits::dex::{
	normalize_asset_deposit_infos_to_min_ratio, AssetAmount, AssetDepositInfo,
	AssetDepositNormalizationError, BasicPoolInfo, Fee, FeeConfig,
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate, Transfer},
};
use sp_runtime::{
	traits::{Convert, One, Zero},
	ArithmeticError, BoundedBTreeMap, Permill,
};
use sp_std::collections::btree_map::BTreeMap;

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

		// Add new pool
		let pool_id =
			PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
				let pool_id = *pool_count;
				match lp_token_id {
					Some(lp_token) => Pools::<T>::insert(
						pool_id,
						PoolConfiguration::DualAssetConstantProduct(BasicPoolInfo {
							owner: who.clone(),
							assets_weights,
							lp_token,
							fee_config,
						}),
					),
					None => Pools::<T>::insert(
						pool_id,
						PoolConfiguration::DualAssetConstantProduct(BasicPoolInfo {
							owner: who.clone(),
							assets_weights,
							lp_token: create_lpt_asset::<T>(
								LPTNonce::<T>::increment().expect("Does not exceed u64::MAX"),
							)?,
							fee_config,
						}),
					),
				};
				*pool_count = pool_id.safe_add(&T::PoolId::one())?;
				Ok(pool_id)
			})?;

		Ok(dbg!(pool_id))
	}

	/// WARNING! This is not a cheap function to call; it does (at least) one storage read per asset
	/// in the pool!
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
			// TODO(benluelo): This function should return an iterator, rather than eagerly
			// collecting into a map. The caller can collect if they need a map.
			.collect::<BTreeMap<_, _>>()
	}

	pub(crate) fn add_liquidity(
		who: &T::AccountId,
		pool: BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: T::AccountId,
		assets: BiBoundedVec<AssetAmount<T::AssetId, T::Balance>, 1, 2>,
		min_mint_amount: T::Balance,
		keep_alive: bool,
	) -> Result<(T::Balance, BTreeMap<T::AssetId, T::Balance>), DispatchError> {
		let mut pool_assets = Self::get_pool_balances(&pool, &pool_account);

		let assets_with_balances = assets.try_mapped(|asset_amount| {
			if asset_amount.amount.is_zero() {
				return Err(Error::<T>::InvalidAmount)
			};

			let (weight, balance) =
				pool_assets.remove(&asset_amount.asset_id).ok_or(Error::<T>::AssetNotFound)?;

			Ok(AssetDepositInfo {
				asset_id: asset_amount.asset_id,
				deposit_amount: T::Convert::convert(asset_amount.amount),
				existing_balance: balance,
				asset_weight: weight,
			})
		})?;

		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));

		let (amount_of_lp_token_to_mint, actual_amounts_deposited) = if let [single] =
			assets_with_balances.as_slice()
		{
			if lp_total_issuance.is_zero() {
				return Err(Error::<T>::InitialDepositMustContainAllAssets.into())
			}

			let single_deposit = compute_deposit_lp(
				lp_total_issuance,
				single.deposit_amount,
				single.existing_balance,
				single.asset_weight,
				pool.fee_config.fee_rate,
			)?;

			T::Assets::transfer(
				single.asset_id,
				who,
				&pool_account,
				T::Convert::convert(single.deposit_amount),
				keep_alive,
			)?;

			(
				single_deposit.value,
				assets_with_balances
					.into_iter()
					.map(|adi| (adi.asset_id, T::Convert::convert(adi.deposit_amount)))
					.collect(),
			)
		} else {
			// ensure that `assets` contains all of the assets in the pool at this point
			// a bit convoluted, but it works
			ensure!(pool_assets.is_empty(), Error::<T>::UnsupportedOperation);

			if lp_total_issuance.is_zero() {
				let lp_to_mint = compute_first_deposit_lp(
					assets_with_balances
						.iter()
						.map(|adi| (adi.asset_id, adi.deposit_amount, adi.asset_weight)),
					Permill::zero(),
				)?
				.value;

				for deposit in &assets_with_balances {
					T::Assets::transfer(
						deposit.asset_id,
						who,
						&pool_account,
						T::Convert::convert(deposit.deposit_amount),
						keep_alive,
					)?;
				}

				(
					lp_to_mint,
					assets_with_balances
						.into_iter()
						.map(|adi| (adi.asset_id, T::Convert::convert(adi.deposit_amount)))
						.collect(),
				)
			} else {
				let normalized_deposits =
					match normalize_asset_deposit_infos_to_min_ratio(assets_with_balances.into()) {
						Ok(normalized_assets) => normalized_assets,
						Err(AssetDepositNormalizationError::ArithmeticOverflow) =>
							return Err(DispatchError::Arithmetic(ArithmeticError::Overflow)),
						Err(AssetDepositNormalizationError::NotEnoughAssets) => unreachable!(
							"two assets were provided to the normalization function; qed;"
						),
					};

				// since the asset deposits were normalized, the lp_to_mint will be the same for all
				// asset deposits
				let asset_to_calculate_with =
					normalized_deposits.first().expect("2 assets in the vec; qed;");

				// pass 1 as weight since adding liquidity for all assets with normalized deposits
				// see docs on compute_deposit_lp_ for more information
				let lp_to_mint = compute_deposit_lp(
					lp_total_issuance,
					asset_to_calculate_with.deposit_amount,
					asset_to_calculate_with.existing_balance,
					Permill::one(),
					Zero::zero(),
				)?
				.value;

				for normalized_deposit in &normalized_deposits {
					T::Assets::transfer(
						normalized_deposit.asset_id,
						who,
						&pool_account,
						T::Convert::convert(normalized_deposit.deposit_amount),
						keep_alive,
					)?;
				}

				(
					lp_to_mint,
					normalized_deposits
						.into_iter()
						.map(|adi| (adi.asset_id, T::Convert::convert(adi.deposit_amount)))
						.collect(),
				)
			}
		};

		let amount_of_lp_token_to_mint = T::Convert::convert(amount_of_lp_token_to_mint);

		ensure!(
			amount_of_lp_token_to_mint >= min_mint_amount,
			Error::<T>::CannotRespectMinimumRequested
		);

		T::Assets::mint_into(pool.lp_token, who, amount_of_lp_token_to_mint)?;

		Ok((amount_of_lp_token_to_mint, actual_amounts_deposited))
	}

	pub(crate) fn remove_liquidity(
		who: &T::AccountId,
		pool: BasicPoolInfo<T::AccountId, T::AssetId, ConstU32<2>>,
		pool_account: T::AccountId,
		lp_amount: T::Balance,
		mut min_receive: BoundedBTreeMap<T::AssetId, T::Balance, ConstU32<2>>,
	) -> Result<BTreeMap<T::AssetId, T::Balance>, DispatchError> {
		let lp_total_issuance = T::Convert::convert(T::Assets::total_issuance(pool.lp_token));

		let redeemed_assets = Self::get_pool_balances(&pool, &pool_account)
			.into_iter()
			.map(|(id, (_, balance))| {
				let redeemed_amount = compute_redeemed_for_lp(
					lp_total_issuance,
					T::Convert::convert(lp_amount),
					balance,
					Permill::one(),
				)?;

				if let Some(min_amount) = min_receive.remove(&id) {
					ensure!(
						redeemed_amount >= T::Convert::convert(min_amount),
						Error::<T>::CannotRespectMinimumRequested
					);
				}

				Ok::<_, DispatchError>((id, T::Convert::convert(redeemed_amount)))
			})
			.collect::<Result<BTreeMap<_, _>, _>>()?;

		ensure!(min_receive.is_empty(), Error::<T>::AssetNotFound);

		for (id, amount) in &redeemed_assets {
			T::Assets::transfer(
				*id,
				&pool_account,
				who,
				*amount,
				false, // pool account doesn't need to be kept alive
			)?;
		}

		T::Assets::burn_from(pool.lp_token, who, lp_amount)?;

		Ok(redeemed_assets)
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

		let amm_pair = compute_out_given_in::<_>(*w_i, *w_o, *b_i, *b_o, a_sent, fee)?;

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

		let amm_pair = compute_in_given_out(*w_i, *w_o, *b_i, *b_o, a_out, fee)?;

		let a_sent = AssetAmount::new(in_asset_id, T::Convert::convert(amm_pair.value));
		let fee = pool.fee_config.calculate_fees(in_asset_id, T::Convert::convert(amm_pair.fee));

		Ok((out_asset, a_sent, fee))
	}
}
