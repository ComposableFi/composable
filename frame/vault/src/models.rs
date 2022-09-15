use crate::Capabilities;
use composable_traits::vault::Deposit;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Copy, Clone, Encode, Decode, Default, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct VaultInfo<AccountId, Balance, CurrencyId, BlockNumber> {
	pub asset_id: CurrencyId,
	pub lp_token_id: CurrencyId,
	pub manager: AccountId,
	pub deposit: Deposit<Balance, BlockNumber>,
	pub capabilities: Capabilities,
}
