use composable_traits::vault::Deposit;
use frame_support::pallet_prelude::*;

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultInfo<AccountId, Balance, CurrencyId, BlockNumber> {
	pub asset_id: CurrencyId,
	pub lp_token_id: CurrencyId,
	pub manager: AccountId,
	pub deposit: Deposit<Balance, BlockNumber>,
}

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq)]
pub struct StrategyOverview<Balance> {
	/// The reported balance of the strategy
	pub balance: Balance,
	/// Sum of all withdrawn funds.
	pub lifetime_withdrawn: Balance,
	/// Sum of all deposited funds.
	pub lifetime_deposited: Balance,
}
