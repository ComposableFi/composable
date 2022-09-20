#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;

// Pablo Runtime API declaration. Implemented for each runtime at
// `runtime/<runtime-name>/src/lib.rs`.
sp_api::decl_runtime_apis! {
	pub trait CosmwasmRuntimeApi<AccountId, QueryRequest, Binary>
	where
		AccountId: Codec,
		QueryRequest: Codec,
		Binary: Codec,
	{
		fn query(
			contract: AccountId,
			gas: u64,
			query_request: QueryRequest,
		) -> Binary;
	}
}
