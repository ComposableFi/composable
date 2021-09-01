use crate::vault::Deposit;
use codec::Codec;
use frame_support::{
	pallet_prelude::*,
	sp_runtime::Permill,
	sp_std::{fmt::Debug, vec::Vec},
};

#[derive(Encode, Decode, Default)]
pub struct LendingConfigInput<AccountId>
where
	AccountId: core::cmp::Ord,
{
	/// can pause borrow & deposits of assets
	pub manager: AccountId,
	pub reserve_factor: Permill,
	pub collateral_factor: Permill,
}

/// Basic lending with no its own wrapper (liquidity) token.
///  User will deposit borrow and collateral assets via `Vault`.
/// `Liquidation` is other trait.
/// Based on Blacksmith (Warp v2) IBSLendingPair.sol and Parallel Finance.
/// Fees will be withdrawing to vault.
/// Lenders with be rewarded via vault.
pub trait Lending {
	type VaultId: Codec;
	type LendingId: Codec;
	/// (deposit VaultId, collateral VaultId) <-> PairId
	type AccountId: core::cmp::Ord + Clone + Codec;
	type Error;
	type Balance;
	type BlockNumber;

	/// creates market for new pair in specified vault. if market exists under specified manager,
	/// updates its parameters `deposit` - asset users want to borrow.
	/// `collateral` - asset users will put as collateral.
	fn create_or_update(
		deposit: Self::VaultId,
		collateral: Self::VaultId,
		config: LendingConfigInput<Self::AccountId>,
	) -> Result<(), DispatchError>;

	/// account id of pallet
	fn account_id(lending_id: &Self::LendingId) -> Self::AccountId;

	fn get_pair_in_vault(vault: Self::VaultId) -> Result<Vec<Self::LendingId>, Self::Error>;

	fn get_pairs_all() -> Result<Vec<Self::LendingId>, Self::Error>;

	fn borrow(
		lending_id: &Self::LendingId,
		debt_owner: &Self::AccountId,
		amount_to_borrow: Self::Balance,
	) -> Result<(), Self::Error>;

	/// `from` repays some of `beneficiary` debts.
	///
	/// - `pair`        : the pair to be repaid.
	/// - `repay_amount`: the amount to be repaid.
	fn repay_borrow(
		lending_id: &Self::LendingId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		repay_amount: Self::Balance,
	) -> Result<(), Self::Error>;

	fn total_borrows(lending_id: &Self::LendingId) -> Result<Self::Balance, Self::Error>;

	fn accrue_interest(lending_id: &Self::LendingId) -> Result<(), Self::Error>;

	fn borrow_balance_current(
		lending_id: &Self::LendingId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;

	fn collateral_of_account(
		lending_id: &Self::LendingId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;

	/// Borrower shouldn't borrow more than his total collateral value
	fn collateral_required(
		lending_id: &Self::LendingId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, Self::Error>;

	fn get_borrow_limit(
		lending_id: &Self::LendingId,
		account: Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;
}
