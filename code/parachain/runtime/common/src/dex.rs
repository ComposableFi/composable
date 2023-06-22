use composable_traits::dex::*;
use sp_runtime::DispatchError;
use sp_std::marker::PhantomData;



pub struct DexPrecompile<AssetId, Balance, AccountId, PoolId, Dex>(
	PhantomData<(AssetId, Balance, AccountId, PoolId, Dex)>,
);

impl<Dex, AssetId, Balance, AccountId, PoolId>
	DexPrecompile<AssetId, Balance, AccountId, PoolId, Dex>
where
	Dex: Amm<AssetId = AssetId, Balance = Balance, AccountId = AccountId, PoolId = PoolId>,
{
	pub fn execute(who: &AccountId, msg: &ExecuteMsg) -> Result<Response, DispatchError> {
		match msg {
			ExecuteMsg::AddLiquidity { pool_id, assets, min_mint_amount, keep_alive } => todo!(),
			ExecuteMsg::RemoveLiquidity { pool_id, lp_amount, min_receive } => todo!(),
			ExecuteMsg::Buy { pool_id, in_asset_id, out_asset, keep_alive } => todo!(),
			ExecuteMsg::Swap { pool_id, in_asset, min_receive, keep_alive } => {
				let in_asset_id = AssetToDenom::convert(in_asset.denom)
				.map_err(|_| CosmwasmVMError::AssetConversion)?;
			let min_receive_asset_id = AssetToDenom::convert(min_receive.denom)
				.map_err(|_| CosmwasmVMError::AssetConversion)?;
			let in_asset_amount = in_asset.amount.into();
			let min_receive_amount = min_receive.amount.into();
			let who = AccountAddrConvert::convert(
				vm.0.data().cosmwasm_message_info.sender.clone().into_string(),
			},
		}
	}
}