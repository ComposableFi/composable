#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait CrowdloanRewardsRuntimeApi<AccountId, Balance>
	where
		AccountId: Codec,
	Balance: Codec
	{
		fn amount_available_to_claim_for(account: AccountId) -> SafeRpcWrapper<Balance>;
	}
}
