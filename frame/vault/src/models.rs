use crate::Capabilities;
use composable_traits::vault::Deposit;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::Perquintill;

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct VaultInfo<AccountId, Balance, CurrencyId, BlockNumber> {
	pub asset_id: CurrencyId,
	pub lp_token_id: CurrencyId,
	pub manager: AccountId,
	pub deposit: Deposit<Balance, BlockNumber>,
	pub capabilities: Capabilities,
}

#[derive(Copy, Clone, Encode, Decode, MaxEncodedLen, Default, Debug, PartialEq, Eq, TypeInfo)]
pub struct StrategyOverview<Balance> {
	// The allocation of this strategy
	pub allocation: Perquintill,
	/// The reported balance of the strategy
	pub balance: Balance,
	/// Sum of all withdrawn funds.
	pub lifetime_withdrawn: Balance,
	/// Sum of all deposited funds.
	pub lifetime_deposited: Balance,
}
