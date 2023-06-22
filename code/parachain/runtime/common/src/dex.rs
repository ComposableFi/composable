use crate::prelude::*;
use crate::{cosmwasm::*, AccountId};
use composable_traits::dex::*;
use cosmwasm_std::{Coin, Response, Uint128, to_binary, QueryResponse};
use primitives::currency::CurrencyId;
use sp_runtime::{traits::Convert, DispatchError};
use sp_std::marker::PhantomData;

pub struct DexPrecompile<Balance, AccountId, Dex>(PhantomData<(Balance, AccountId, Dex)>);

pub enum CosmwasmPrecompileError {
	AssetConversion,
	AccountConversion,
	DispatchError,
	Serde,
}

impl<Dex, Balance, AccountId> DexPrecompile<Balance, AccountId, Dex>
where
	Dex:
		Amm<AssetId = CurrencyId, Balance = Balance, AccountId = AccountId, PoolId = crate::PoolId>,
	Balance: From<Uint128> + Into<u128>,
	AccountId: sp_std::convert::From<sp_runtime::AccountId32>,
{

	pub fn query(sender: &str, msg: QueryMsg) -> Result<QueryResponse, CosmwasmPrecompileError> {
		match msg {
			QueryMsg::Assets { pool_id } => {
				let assets = Dex::assets(pool_id.into()).map_err(|_| CosmwasmPrecompileError::DispatchError)?;
				let assets = assets.into_iter().map(|(k, v)| {
					let fraction = ((v.deconstruct() as u64).into(), 1_000_000u64.into());
					let denom = CosmwasmToSubstrateAssetId::convert(k);
					(denom, fraction)
				}).collect();
				to_binary(&AssetsResponse{ assets }).map_err(|_| CosmwasmPrecompileError::Serde)
			},
			QueryMsg::SpotPrice { base_asset, quote_asset_id, calculate_with_fees } => {
				todo!()
			},
			QueryMsg::LpToken { pool_id } => {
				todo!()
			},
			QueryMsg::RedeemableAssetsForLpTokens { pool_id, lp_amount } => {
				todo!()
			},
			QueryMsg::SimulateAddLiquidity { pool_id, amounts } => {
				todo!()
			},
			QueryMsg::SimulateRemoveLiquidity { pool_id, lp_amount, min_amount } => {
				todo!()
			},
		}
	}

	pub fn execute(sender: &str, msg: ExecuteMsg) -> Result<Response, CosmwasmPrecompileError> {
		match msg {
			ExecuteMsg::AddLiquidity { pool_id, assets, min_mint_amount, keep_alive } => {
				todo!()
			},
			ExecuteMsg::RemoveLiquidity { pool_id, lp_amount, min_receive } => {
				todo!()
			},
			ExecuteMsg::Buy { pool_id, in_asset_id, out_asset, keep_alive } => {
				todo!()
			},
			ExecuteMsg::Swap { pool_id, in_asset, min_receive, keep_alive } => {
				let in_asset_id = CosmwasmToSubstrateAssetId::convert(in_asset.denom.clone())
					.map_err(|_| CosmwasmPrecompileError::AssetConversion)?;
				let in_amount: Balance = in_asset.amount.into();
				
				let min_receive_id = CosmwasmToSubstrateAssetId::convert(min_receive.denom.clone())
					.map_err(|_| CosmwasmPrecompileError::AssetConversion)?;
				let min_receive_amount: Balance = min_receive.amount.into();
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmPrecompileError::AccountConversion)?
					.into();
				let result = Dex::do_swap(
					&who,
					pool_id.into(),
					AssetAmount::new(in_asset_id, in_amount),
					AssetAmount::new(min_receive_id, min_receive_amount),
					keep_alive,
				)
				.map_err(|_| CosmwasmPrecompileError::DispatchError)?;
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
				Ok(Response::new().set_data(to_binary(&result).map_err(|_| CosmwasmPrecompileError::Serde)?))
			},
		}
	}
}
