#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::dex::{PriceAggregate, RemoveLiquidityDryrunResult};

// Pablo Runtime API declaration. Implemented for each runtime at
// `runtime/<runtime-name>/src/lib.rs`.
sp_api::decl_runtime_apis! {
	pub trait PabloRuntimeApi<AccountId, PoolId, AssetId, Balance>
	where
		PoolId: Codec,
		AssetId: Codec + sp_std::cmp::Ord,
		Balance: Codec,
		AccountId: Codec,
	{
		/// Retrieve the price(s) from the given pool calculated for the given `base_asset_id`
		/// and `quote_asset_id` pair.
		fn prices_for(
			pool_id: PoolId,
			base_asset_id: AssetId,
			quote_asset_id: AssetId,
			amount: Balance
		) -> PriceAggregate<SafeRpcWrapper<PoolId>, SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>;

		fn add_liquidity_dryrun(
			who: SafeRpcWrapper<AccountId>,
			pool_id: SafeRpcWrapper<PoolId>,
			base_asset_amount: SafeRpcWrapper<Balance>,
			quote_asset_amount: SafeRpcWrapper<Balance>,
		) -> SafeRpcWrapper<Balance>;

		fn remove_liquidity_dryrun(
			who: SafeRpcWrapper<AccountId>,
			pool_id: SafeRpcWrapper<PoolId>,
			lp_amount: SafeRpcWrapper<Balance>,
			min_base_amount: SafeRpcWrapper<Balance>,
			min_quote_amount: SafeRpcWrapper<Balance>,
		) -> RemoveLiquidityDryrunResult<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>;
	}
}
