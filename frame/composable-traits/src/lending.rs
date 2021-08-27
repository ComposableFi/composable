use codec::Codec;
use frame_support::{pallet_prelude::*, sp_std::fmt::Debug};

use crate::vault::Deposit;

#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct AccountConfig<AccountId, AssetId>
where
	AccountId: core::cmp::Ord,
{
	pub deposit: AssetId,
	pub collateral: AssetId,
	pub manager: AccountId,
}

pub trait Composable {
	type Error;
	type Balance;
	type BlockNumber;
	type AccountId: core::cmp::Ord;
}

/// basic lending
// assumption that user will deposit borrow and collateral assets via vault
// does not have wrapped tokens, so no need to proxy vault calls
// liquidation is other trait
// ASK: not clear how Vault will prevent withdrawing collateral?
// based on Blacksmith (Warp v2) IBSLendingPair.sol, but without wrapper token and proxy methods
pub trait Lending: Composable {
	type AssetId;
	type VaultId: Clone + Codec + Debug + PartialEq;
	type PairId: Clone + Codec + Debug + PartialEq;

	/// creates market for new pair in specified vault
	fn create(
		vault: Self::VaultId,
		deposit: Deposit<Self::Balance, Self::BlockNumber>,
		config: AccountConfig<Self::AccountId, Self::AssetId>,
	) -> Result<Self::PairId, Self::Error>;

	fn get_pair_in_vault(vault: Self::VaultId) -> Result<Vec<Self::PairId>, Self::Error>;

	fn get_pairs_all() -> Result<Vec<Self::PairId>, Self::Error>;

	fn borrow(
		pair: Self::PairId,
		debt_owner: &Self::AccountId,
		amount_to_borrow: Self::Balance,
	) -> Result<(), Self::Error>;

	fn repay(
		pair: Self::PairId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		repay_amount: Self::Balance,
	) -> Result<(), Self::Error>;

	/// part or whole deposited assets + interest to account
	fn redeem(
		pair: Self::PairId,
		to: &Self::AccountId,
		redeem_amount: Self::Balance,
	) -> Result<(), Self::Error>;

	fn calculate_liquidation_fee(amount: Self::Balance) -> Self::Balance;

	fn total_borrows(pair: Self::PairId) -> Result<Self::Balance, Self::Error>;

	fn accrue_interest(pair: Self::PairId) -> Result<(), Self::Error>;

	fn borrow_balance_current(
		pair: Self::PairId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;

	fn withdraw_fees(to_withdraw: Self::Balance) -> Result<(), Self::Error>;

	fn collateral_of_account(
		pair: Self::PairId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;

	fn collateral_required(
		pair: Self::PairId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, Self::Error>;

	fn get_borrow_limit(
		pair: Self::PairId,
		account: Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;
}
