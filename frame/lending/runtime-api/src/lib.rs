#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use composable_support::rpc_helpers::SafeRpcWrapper;
use composable_traits::defi::Rate;

// Lending Runtime API declaration. Implemented for each runtime at
// `runtime/<runtime-name>/src/lib.rs`.
sp_api::decl_runtime_apis! {
	pub trait LendingRuntimeApi<MarketId>
	where
		MarketId: Codec,
	{
		/// Retrieve the current interest rate for the given `market_id`.
		fn current_interest_rate(market_id: MarketId) -> SafeRpcWrapper<Rate>;
	}
}
