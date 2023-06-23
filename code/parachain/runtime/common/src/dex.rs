use crate::{cosmwasm::*, prelude::*};
use composable_traits::{dex::*, cosmwasm::CosmwasmSubstrateError};
use cosmwasm_std::{to_binary, Coin, QueryResponse, Response, Uint128};
use primitives::currency::CurrencyId;
use sp_runtime::{traits::Convert, Permill, PerThing};
use sp_std::marker::PhantomData;

pub struct DexPrecompile<Balance, AccountId, Dex>(PhantomData<(Balance, AccountId, Dex)>);

impl<Dex, Balance, AccountId> DexPrecompile<Balance, AccountId, Dex>
where
	Dex:
		Amm<AssetId = CurrencyId, Balance = Balance, AccountId = AccountId, PoolId = crate::PoolId>,
	Balance: From<Uint128> + Into<u128>,
	AccountId: sp_std::convert::From<sp_runtime::AccountId32>,
{
	fn to_substrate(
		in_asset: &Coin,
	) -> Result<AssetAmount<CurrencyId, Balance>, CosmwasmSubstrateError> {
		let in_asset_id = CosmwasmToSubstrateAssetId::convert(in_asset.denom.clone())
			.map_err(|_| CosmwasmSubstrateError::AssetConversion)?;
		let in_amount: Balance = in_asset.amount.into();
		Ok(AssetAmount::new(in_asset_id, in_amount))
	}

	fn to_cw(amount: AssetAmount<CurrencyId, Balance>) -> Coin {
		Coin {
			denom: CosmwasmToSubstrateAssetId::convert(amount.asset_id),
			amount: amount.amount.into().into(),
		}
	}

	pub fn query(sender: &str, msg: QueryMsg) -> Result<QueryResponse, CosmwasmSubstrateError> {
		match msg {
			QueryMsg::Assets { pool_id } => {
				let assets = Dex::assets(pool_id.into())
					.map_err(|_| CosmwasmSubstrateError::DispatchError)?;
				let assets = assets
					.into_iter()
					.map(|(k, v)| {
						let fraction = (u64::from(v.deconstruct()).into(), Permill::ACCURACY.into());
						let denom = CosmwasmToSubstrateAssetId::convert(k);
						(denom, fraction)
					})
					.collect();
				to_binary(&AssetsResponse { assets }).map_err(|_| CosmwasmSubstrateError::Serde)
			},
			QueryMsg::SpotPrice { pool_id, base_asset, quote_asset_id, calculate_with_fees } => {
				let quote_asset_id = CosmwasmToSubstrateAssetId::convert(quote_asset_id)
					.map_err(|_| CosmwasmSubstrateError::AssetConversion)?;

				let response = Dex::spot_price(
					pool_id.into(),
					Self::to_substrate(&base_asset)?,
					quote_asset_id,
					calculate_with_fees,
				)
				.map_err(|_| CosmwasmSubstrateError::DispatchError)?;

				to_binary(&SpotPriceResponse {
					value: Self::to_cw(response.value),
					fee: Self::to_cw(response.fee),
				})
				.map_err(|_| CosmwasmSubstrateError::Serde)
			},
			QueryMsg::LpToken { pool_id } => Dex::lp_token(pool_id.into())
				.map_err(|_| CosmwasmSubstrateError::DispatchError)
				.map(CosmwasmToSubstrateAssetId::convert)
				.map(|lp_token| LpTokenResponse { lp_token })
				.map(|x| to_binary(&x))?
				.map_err(|_| CosmwasmSubstrateError::Serde),
			QueryMsg::RedeemableAssetsForLpTokens { pool_id, lp_amount } => {
				let result: Vec<_> =
					Dex::redeemable_assets_for_lp_tokens(pool_id.into(), lp_amount.into())
						.map_err(|_| CosmwasmSubstrateError::DispatchError)
						.map(|x| x.into_iter())?
						.map(|(k, v)| Coin {
							denom: CosmwasmToSubstrateAssetId::convert(k),
							amount: v.into().into(),
						})
						.collect();
				to_binary(&RedeemableAssetsForLpTokensResponse { assets: result })
					.map_err(|_| CosmwasmSubstrateError::Serde)
			},
			QueryMsg::SimulateAddLiquidity { pool_id, amounts } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConversion)?
					.into();
				let amounts = amounts
					.into_iter()
					.map(|x| Self::to_substrate(&x))
					.try_collect::<Vec<_>>()?
					.into_iter()
					.map(|x| (x.asset_id, x.amount))
					.collect();
				let result = Dex::simulate_add_liquidity(&who, pool_id.into(), amounts)
					.map_err(|_| CosmwasmSubstrateError::DispatchError)?
					.into()
					.into();
				to_binary(&SimulateAddLiquidityResponse { amount: result })
					.map_err(|_| CosmwasmSubstrateError::Serde)
			},
			QueryMsg::SimulateRemoveLiquidity { pool_id, lp_amount, min_amount } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConversion)?
					.into();
				let min_amount = min_amount
					.into_iter()
					.map(|x| Self::to_substrate(&x))
					.try_collect::<Vec<_>>()?
					.into_iter()
					.map(|x| (x.asset_id, x.amount))
					.collect();
				let result = Dex::simulate_remove_liquidity(
					&who,
					pool_id.into(),
					lp_amount.into(),
					min_amount,
				)
				.map_err(|_| CosmwasmSubstrateError::DispatchError)?
				.into_iter()
				.map(|(k, v)| Coin {
					denom: CosmwasmToSubstrateAssetId::convert(k),
					amount: v.into().into(),
				})
				.collect();
				to_binary(&SimulateRemoveLiquidityResponse { amounts: result })
					.map_err(|_| CosmwasmSubstrateError::Serde)
			},
		}
	}

	pub fn execute(sender: &str, msg: ExecuteMsg) -> Result<Response, CosmwasmSubstrateError> {
		match msg {
			ExecuteMsg::AddLiquidity { pool_id, assets, min_mint_amount, keep_alive } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConversion)?
					.into();
				let assets = assets
					.into_iter()
					.map(|x| Self::to_substrate(&x))
					.try_collect::<Vec<_>>()?
					.into_iter()
					.map(|x| (x.asset_id, x.amount))
					.collect();
				let result = Dex::add_liquidity(
					&who,
					pool_id.into(),
					assets,
					min_mint_amount.into(),
					keep_alive,
				)
				.map_err(|_| CosmwasmSubstrateError::DispatchError)?
				.into();
				let result = to_binary(&AddLiquidityResponse { lp_amount: result.into() })
					.map_err(|_| CosmwasmSubstrateError::Serde)?;
				Ok(Response::new().set_data(result))
			},

			ExecuteMsg::RemoveLiquidity { pool_id, lp_amount, min_receive } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConversion)?
					.into();
				let min_amount = min_receive
					.into_iter()
					.map(|x| Self::to_substrate(&x))
					.try_collect::<Vec<_>>()?
					.into_iter()
					.map(|x| (x.asset_id, x.amount))
					.collect();
				let result = Dex::simulate_remove_liquidity(
					&who,
					pool_id.into(),
					lp_amount.into(),
					min_amount,
				)
				.map_err(|_| CosmwasmSubstrateError::DispatchError)?
				.into_iter()
				.map(|(k, v)| Coin {
					denom: CosmwasmToSubstrateAssetId::convert(k),
					amount: v.into().into(),
				})
				.collect();
				let result = to_binary(&RemoveLiquidityResponse { assets: result })
					.map_err(|_| CosmwasmSubstrateError::Serde)?;
				Ok(Response::new().set_data(result))
			},
			ExecuteMsg::Buy { pool_id, in_asset_id, out_asset, keep_alive } => {
				let in_asset_id = CosmwasmToSubstrateAssetId::convert(in_asset_id)
					.map_err(|_| CosmwasmSubstrateError::AssetConversion)?;

				let out_asset_id = CosmwasmToSubstrateAssetId::convert(out_asset.denom.clone())
					.map_err(|_| CosmwasmSubstrateError::AssetConversion)?;
				let out_asset_amount: Balance = out_asset.amount.into();

				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConversion)?
					.into();
				let result = Dex::do_buy(
					&who,
					pool_id.into(),
					in_asset_id,
					AssetAmount::new(out_asset_id, out_asset_amount),
					keep_alive,
				)
				.map_err(|_| CosmwasmSubstrateError::DispatchError)?;
				let result = SwapResponse {
					value: Coin {
						denom: CosmwasmToSubstrateAssetId::convert(result.value.asset_id),
						amount: result.value.amount.into().into(),
					},
					fee: Coin {
						denom: CosmwasmToSubstrateAssetId::convert(result.fee.asset_id),
						amount: result.fee.amount.into().into(),
					},
				};
				Ok(Response::new()
					.set_data(to_binary(&result).map_err(|_| CosmwasmSubstrateError::Serde)?))
			},
			ExecuteMsg::Swap { pool_id, in_asset, min_receive, keep_alive } => {
				let in_asset_id = CosmwasmToSubstrateAssetId::convert(in_asset.denom.clone())
					.map_err(|_| CosmwasmSubstrateError::AssetConversion)?;
				let in_amount: Balance = in_asset.amount.into();
				let in_asset = AssetAmount::new(in_asset_id, in_amount);

				let min_receive_id = CosmwasmToSubstrateAssetId::convert(min_receive.denom.clone())
					.map_err(|_| CosmwasmSubstrateError::AssetConversion)?;
				let min_receive_amount: Balance = min_receive.amount.into();
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConversion)?
					.into();
				let result = Dex::do_swap(
					&who,
					pool_id.into(),
					in_asset,
					AssetAmount::new(min_receive_id, min_receive_amount),
					keep_alive,
				)
				.map_err(|_| CosmwasmSubstrateError::DispatchError)?;
				let result = SwapResponse {
					value: Coin {
						denom: CosmwasmToSubstrateAssetId::convert(result.value.asset_id),
						amount: result.value.amount.into().into(),
					},
					fee: Coin {
						denom: CosmwasmToSubstrateAssetId::convert(result.fee.asset_id),
						amount: result.fee.amount.into().into(),
					},
				};
				Ok(Response::new()
					.set_data(to_binary(&result).map_err(|_| CosmwasmSubstrateError::Serde)?))
			},
		}
	}
}
