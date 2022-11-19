#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::assets::Asset;
use sp_std::vec::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	// REVIEW(benluelo): Should the AssetId type parameter be removed and then just use CurrencyId directly?
	pub trait AssetsRuntimeApi<AssetId, AccountId, Balance, ForeignAssetId>
	where
		AccountId: Codec,
		Balance: Codec,
		AssetId: Codec,
		ForeignAssetId: Codec,
	{
		fn balance_of(asset_id: SafeRpcWrapper<AssetId>, account_id: AccountId) -> SafeRpcWrapper<Balance>;

		fn list_assets() -> Vec<Asset<Balance, ForeignAssetId>>;
	}
}
