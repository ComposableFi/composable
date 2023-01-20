#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::{Codec, Decode, Encode};
use composable_support::rpc_helpers::SafeRpcWrapper;
use sp_std::collections::btree_map::BTreeMap;

// Staking Rewards Runtime API declaration. Implemented for each runtime at
// `runtime/<runtime-name>/src/lib.rs`.
sp_api::decl_runtime_apis! {
	pub trait StakingRewardsRuntimeApi<AssetId, FinancialNftInstanceId, Balance>
	where
		AssetId: Codec + sp_std::cmp::Ord,
		FinancialNftInstanceId: Codec,
		Balance: Codec,
	{
		fn get_claimable_amount(
			fnft_collection_id: SafeRpcWrapper<AssetId>,
			fnft_instance_id: SafeRpcWrapper<FinancialNftInstanceId>,
		) -> Result<BTreeMap<AssetId, Balance>, ClaimableAmountError>;
	}
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(serde::Serialize))]
pub enum ClaimableAmountError {
	ArithmeticError(sp_runtime::ArithmeticError),
	StakeNotFound,
	RewardsPoolNotFound,
}
