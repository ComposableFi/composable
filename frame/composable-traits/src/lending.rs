use crate::{loans::Timestamp, rate_model::*};
use frame_support::{pallet_prelude::*, sp_runtime::Perquintill, sp_std::vec::Vec};
use sp_runtime::Percent;

pub type CollateralLpAmountOf<T> = <T as Lending>::Balance;

pub type BorrowAmountOf<T> = <T as Lending>::Balance;

#[derive(Encode, Decode, Default)]
pub struct MarketConfigInput<AccountId, GroupId> {
	pub reserved: Perquintill,
	pub manager: AccountId,
	/// can pause borrow & deposits of assets
	pub collateral_factor: NormalizedCollateralFactor,
	pub under_collaterized_warn_percent: Percent,
	pub liquidator: Option<GroupId>,
}

#[derive(Encode, Decode, Default)]
pub struct MarketConfig<VaultId, AssetId, AccountId, GroupId> {
	pub manager: AccountId,
	pub borrow: VaultId,
	pub collateral: AssetId,
	pub collateral_factor: NormalizedCollateralFactor,
	pub interest_rate_model: InterestRateModel,
	pub under_collaterized_warn_percent: Percent,
	pub liquidator: Option<GroupId>,
}

/// Basic lending with no its own wrapper (liquidity) token.
///  User will deposit borrow and collateral assets via `Vault`.
/// `Liquidation` is other trait.
/// Based on Blacksmith (Warp v2) IBSLendingPair.sol and Parallel Finance.
/// Fees will be withdrawing to vault.
/// Lenders with be rewarded via vault.
pub trait Lending {
	type AssetId;
	type VaultId;
	type MarketId;
	/// (deposit VaultId, collateral VaultId) <-> MarketId
	type AccountId;
	type Balance;
	type BlockNumber;
	type GroupId;

	/// Generates the underlying owned vault that will hold borrowable asset (may be shared with
	/// specific set of defined collaterals). Creates market for new pair in specified vault. if
	/// market exists under specified manager, updates its parameters `deposit` - asset users want
	/// to borrow. `collateral` - asset users will put as collateral.
	/// ```svgbob
	///  -----------
	///  |  vault  |  I
	///  -----------
	///       |
	/// -------------
	/// |  strategy | P
	/// -------------
	///       |                            M
	///       |                   -------------------
	///       |                   |    ---------    |
	///       -----------------------> |       |    |
	///                           |    | vault |    |
	///       -----------------------> |       |    |
	///       |                   |    ---------    |
	///       |                   -------------------
	///       |
	/// -------------
	/// |  strategy | Q
	/// -------------
	///       |
	///  ----------
	///  |  vault | J
	///  ----------
	/// ```
	/// Let's assume a group of users X want to use a strategy P
	/// and a group of users Y want to use a strategy Q:
	/// Assuming both groups are interested in lending an asset A, they can create two vaults I and
	/// J. They would deposit in I and J, then set P and respectively Q as their strategy.
	/// Now imagine that our lending market M has a good APY, both strategy P and Q
	/// could decide to allocate a share for it, transferring from I and J to the borrow asset vault
	/// of M. Their allocated share could differ because of the strategies being different,
	/// but the lending Market would have all the lendable funds in a single vault.
	fn create(
		borrow_asset: Self::AssetId,
		collateral_asset_vault: Self::AssetId,
		config: MarketConfigInput<Self::AccountId, Self::GroupId>,
		interest_rate_model: &InterestRateModel,
	) -> Result<(Self::MarketId, Self::VaultId), DispatchError>;

	/// AccountId of the market instance
	fn account_id(market_id: &Self::MarketId) -> Self::AccountId;

	/// Deposit collateral in order to borrow.
	fn deposit_collateral(
		market_id: &Self::MarketId,
		account_id: &Self::AccountId,
		amount: CollateralLpAmountOf<Self>,
	) -> Result<(), DispatchError>;

	/// Withdraw a part/total of previously deposited collateral.
	/// In practice if used has borrow user will not withdraw v because it would probably result in
	/// quick liquidation, if he has any borrows. ```python
	/// withdrawable = total_collateral - total_borrows
	/// withdrawable = collateral_balance * collateral_price - borrower_balance_with_interest *
	/// borrow_price * collateral_factor ```
	fn withdraw_collateral(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
		amount: CollateralLpAmountOf<Self>,
	) -> Result<(), DispatchError>;

	/// get all existing markets for current deposit
	fn get_markets_for_borrow(vault: Self::VaultId) -> Vec<Self::MarketId>;

	#[allow(clippy::type_complexity)]
	fn get_all_markets() -> Vec<(
		Self::MarketId,
		MarketConfig<Self::VaultId, Self::AssetId, Self::AccountId, Self::GroupId>,
	)>;

	/// `amount_to_borrow` is the amount of the borrow asset lendings's vault shares the user wants
	/// to borrow. Normalizes amounts for calculations.
	/// Borrows as exact amount as possible with some inaccuracies for oracle price based
	/// normalization. If there is not enough collateral or borrow amounts - fails
	fn borrow(
		market_id: &Self::MarketId,
		debt_owner: &Self::AccountId,
		amount_to_borrow: BorrowAmountOf<Self>,
	) -> Result<(), DispatchError>;

	/// `from` repays some of `beneficiary` debts.
	/// - `market_id`   : the market_id on which to be repaid.
	/// - `repay_amount`: the amount to be repaid in underlying.
	/// Interest will be repaid first and then remaining amount from `repay_amount` will be used to
	/// repay principal value.
	fn repay_borrow(
		market_id: &Self::MarketId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		repay_amount: Option<BorrowAmountOf<Self>>,
	) -> Result<(), DispatchError>;

	/// total debts principals (not includes interest)
	fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	/// Floored down to zero.
	fn total_interest(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	/// ````python
	/// delta_interest_rate = delta_time / period_interest_rate
	/// debt_delta = debt_principal * delta_interest_rate
	/// new_accrued_debt = accrued_debt + debt_delta
	/// total_debt = debt_principal + new_accrued_debt
	/// ```
	fn accrue_interest(market_id: &Self::MarketId, now: Timestamp) -> Result<(), DispatchError>;

	fn total_cash(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	/// utilization_ratio = total_borrows / (total_cash + total_borrows).
	/// utilization ratio is 0 when there are no borrows.
	fn calc_utilization_ratio(
		cash: &Self::Balance,
		borrows: &Self::Balance,
	) -> Result<Percent, DispatchError>;

	/// Borrow asset amount account should repay to be debt free for specific market pair.
	/// Calculate account's borrow balance using the borrow index at the start of block time.
	/// ```python
	/// new_borrow_balance = principal * (market_borrow_index / borrower_borrow_index)
	/// ```
	fn borrow_balance_current(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<Option<BorrowAmountOf<Self>>, DispatchError>;

	fn collateral_of_account(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<CollateralLpAmountOf<Self>, DispatchError>;

	/// Borrower shouldn't borrow more than his total collateral value
	fn collateral_required(
		market_id: &Self::MarketId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Returns the borrow limit for an account.
	/// Calculation uses indexes from start of block time.
	/// Depends on overall collateral put by user into vault.
	/// This borrow limit of specific user, depends only on prices and users collateral, not on
	/// state of vault.
	/// ```python
	/// collateral_balance * collateral_price / collateral_factor - borrower_balance_with_interest * borrow_price
	/// ```
	fn get_borrow_limit(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, DispatchError>;
}
