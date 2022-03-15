#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use composable_support::rpc_helpers::{SafeRpcWrapper, SafeRpcWrapperType};

sp_api::decl_runtime_apis!{
	pub trait LendingRuntimeApi<AccountId, Balance, MarketId>
	where 
	  AccountId: Codec,
	  MarketId: Codec,
	  Balance: SafeRpcWrapperType {
        fn get_borrow_limit(market_id: MarketId, account: AccountId) -> SafeRpcWrapper<Balance>;
	}
}