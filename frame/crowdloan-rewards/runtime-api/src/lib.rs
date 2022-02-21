#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unknown_lints, panics, clippy::too_many_arguments, clippy::unnecessary_mut_passed)]

use codec::Codec;
use composable_support::rpc_helpers::{SafeRpcWrapper, SafeRpcWrapperType};

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait CrowdloanRewardsRuntimeApi<AccountId, Balance>
	where
		AccountId: Codec,
		Balance: SafeRpcWrapperType,
	{
		fn amount_available_to_claim_for(account: AccountId) -> SafeRpcWrapper<Balance>;
	}
}
