use crate::Capabilities;
use composable_traits::vault::Deposit;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct VaultInfo<AccountId, Balance, CurrencyId, BlockNumber> {
	pub asset_id: CurrencyId,
	pub lp_token_id: CurrencyId,
	pub manager: AccountId,
	pub deposit: Deposit<Balance, BlockNumber>,
	pub capabilities: Capabilities,
}

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct StrategyOverview<Balance> {
	/// The reported balance of the strategy.
	///
	/// Added when an account withdraws from a vault and subtracted when an account deposits
	/// into a vault.
	pub balance: Balance,
	/// Sum of all withdrawn funds.
	pub lifetime_withdrawn: Balance,
	/// Sum of all deposited funds.
	pub lifetime_deposited: Balance,
}
