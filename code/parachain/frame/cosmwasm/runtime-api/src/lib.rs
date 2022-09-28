#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use sp_std::collections::btree_map::BTreeMap;
#[cfg(not(feature = "std"))]
use sp_std::vec::Vec;

// Cosmwasm Runtime API declaration.
sp_api::decl_runtime_apis! {
	pub trait CosmwasmRuntimeApi<AccountId, AssetId, Balance, Error>
	where
		AccountId: Codec,
		AssetId: Codec,
		Balance: Codec,
		Error: Codec
	{
		fn query(
			contract: AccountId,
			gas: u64,
			query_request: Vec<u8>,
		) -> Result<Vec<u8>, Error>;

		fn instantiate(
			instantiator: AccountId,
			code_id: u64,
			salt: Vec<u8>,
			admin: Option<AccountId>,
			label: Vec<u8>,
			funds: BTreeMap<AssetId, (Balance, bool)>,
			gas: u64,
			message: Vec<u8>,
		) -> Result<AccountId, Error>;
	}
}
