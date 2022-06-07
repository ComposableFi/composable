#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::dex::PriceAggregate;

// Pablo Runtime API declaration. Implemented for each runtime at
// `runtime/<runtime-name>/src/lib.rs`.
sp_api::decl_runtime_apis! {
	pub trait PabloRuntimeApi<PoolId, AssetId, Balance>
	where
		PoolId: Codec,
		AssetId: Codec,
		Balance: Codec,
	{
		/// Retrieve the price(s) from the given pool calculated for the given `base_asset_id`
		/// and `quote_asset_id` pair.
		fn prices_for(
			pool_id: PoolId,
			base_asset_id: AssetId,
			quote_asset_id: AssetId,
			amount: Balance
		) -> PriceAggregate<SafeRpcWrapper<PoolId>, SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>;

		fn expected_lp_tokens_given_liquidity(
			pool_id: SafeRpcWrapper<PoolId>,
			base_asset_amount: SafeRpcWrapper<Balance>,
			quote_asset_amount: SafeRpcWrapper<Balance>,
		) -> SafeRpcWrapper<Balance>;
	}
}
