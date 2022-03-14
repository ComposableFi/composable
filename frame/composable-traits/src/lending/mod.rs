pub mod math;

#[cfg(test)]
mod tests;

use crate::{
	defi::{CurrencyPair, DeFiEngine, MoreThanOneFixedU128},
	time::Timestamp,
};
use composable_support::validation::{TryIntoValidated, Validate};
use frame_support::{pallet_prelude::*, sp_runtime::Perquintill, sp_std::vec::Vec};
use scale_info::TypeInfo;
use sp_runtime::{traits::One, Percent};

use self::math::*;

pub type CollateralLpAmountOf<T> = <T as DeFiEngine>::Balance;

pub type BorrowAmountOf<T> = <T as DeFiEngine>::Balance;

#[derive(Encode, Decode, Default, TypeInfo, Debug, Clone, PartialEq)]
pub struct UpdateInput<LiquidationStrategyId> {
	/// Collateral factor of market
	pub collateral_factor: MoreThanOneFixedU128,
	/// warn borrower when loan's collateral/debt ratio
	/// given percentage short to be under collateralized
	pub under_collateralized_warn_percent: Percent,
	/// liquidation engine id
	pub liquidators: Vec<LiquidationStrategyId>,
	pub interest_rate_model: InterestRateModel,
}

/// input to create market extrinsic
///
/// Input to [`Lending::create()`].
#[derive(Encode, Decode, Default, TypeInfo, Debug, Clone, PartialEq)]
pub struct CreateInput<LiquidationStrategyId, AssetId> {
	/// the part of market which can be changed
	pub updatable: UpdateInput<LiquidationStrategyId>,
	/// collateral currency and borrow currency
	/// in case of liquidation, collateral is base and borrow is quote
	pub currency_pair: CurrencyPair<AssetId>,
	/// Reserve factor of market borrow vault.
	pub reserved_factor: Perquintill,
}

#[derive(Clone, Copy, Debug, PartialEq, TypeInfo, Default)]
pub struct MarketModelValid;
#[derive(Clone, Copy, Debug, PartialEq, TypeInfo, Default)]
pub struct CurrencyPairIsNotSame;

impl<LiquidationStrategyId, Asset: Eq>
	Validate<CreateInput<LiquidationStrategyId, Asset>, MarketModelValid> for MarketModelValid
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset>, &'static str> {
		if create_input.updatable.collateral_factor < MoreThanOneFixedU128::one() {
			return Err("collateral factor must be >= 1")
		}

		let interest_rate_model = create_input
			.updatable
			.interest_rate_model
			.try_into_validated::<InteresteRateModelIsValid>()?
			.value();

		Ok(CreateInput {
			updatable: UpdateInput { interest_rate_model, ..create_input.updatable },
			..create_input
		})
	}
}

impl<LiquidationStrategyId, Asset: Eq>
	Validate<CreateInput<LiquidationStrategyId, Asset>, CurrencyPairIsNotSame>
	for CurrencyPairIsNotSame
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset>, &'static str> {
		if create_input.currency_pair.base == create_input.currency_pair.quote {
			Err("currency pair must be different assets")
		} else {
			Ok(create_input)
		}
	}
}

// REVIEW: Are these necessary? The fields are public.
impl<LiquidationStrategyId, AssetId: Copy> CreateInput<LiquidationStrategyId, AssetId> {
	pub fn borrow_asset(&self) -> AssetId {
		self.currency_pair.quote
	}
	pub fn collateral_asset(&self) -> AssetId {
		self.currency_pair.base
	}

	pub fn reserved_factor(&self) -> Perquintill {
		self.reserved_factor
	}
}

#[derive(Encode, Decode, Default, TypeInfo, Debug)]
pub struct MarketConfig<VaultId, AssetId, AccountId, LiquidationStrategyId> {
	/// The owner of this market.
	pub manager: AccountId,
	/// The vault containing the borrow asset.
	pub borrow_asset_vault: VaultId,
	/// The asset being used as collateral.
	pub collateral_asset: AssetId,
	pub collateral_factor: MoreThanOneFixedU128,
	pub interest_rate_model: InterestRateModel,
	pub under_collateralized_warn_percent: Percent,
	pub liquidators: Vec<LiquidationStrategyId>,
}

/// Different ways that a market can be repaid.
// REVIEW: Name is not final
#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq)]
pub enum RepayStrategy<T> {
	/// Attempt to repay the entirety of the remaining debt, repaying interest first and
	/// then principal with whatever is left.
	TotalDebt,
	/// Repay the specified amount.
	///
	/// NOTE: Must be less than the total owing amount + the interest
	PartialAmount(T),
}

/// Different ways that a market can be repaid.
// REVIEW: Name is not final
#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq)]
pub enum RepayResult<T> {
	/// Attempt to repay the entirety of the remaining debt, repaying interest first and
	/// thenprincipal with whatever is left.
	Repaid(T),
	/// Repay the specified amount.
	NothingToRepay,
}

/// Basic lending with no its own wrapper (liquidity) token.
///  User will deposit borrow and collateral assets via `Vault`.
/// `Liquidation` is other trait.
/// Based on Blacksmith (Warp v2) IBSLendingPair.sol and Parallel Finance.
/// Fees will be withdrawing to vault.
/// Lenders with be rewarded via vault.
pub trait Lending: DeFiEngine {
	type VaultId;
	type MarketId;
	type BlockNumber;
	/// id of dispatch used to liquidate collateral in case of undercollateralized asset
	type LiquidationStrategyId;
	/// returned from extrinsic is guaranteed to be existing asset id at time of block execution
	//type AssetId;

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
	///
	/// Returned `MarketId` is mapped one to one with (deposit VaultId, collateral VaultId)
	fn create(
		manager: Self::AccountId,
		config: CreateInput<Self::LiquidationStrategyId, Self::MayBeAssetId>,
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
	/// quick liquidation, if he has any borrows.
	///
	/// ```python
	/// withdrawable = total_collateral - total_borrows
	/// withdrawable = collateral_balance * collateral_price - borrower_balance_with_interest *
	/// borrow_price * collateral_factor
	/// ```
	fn withdraw_collateral(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
		amount: CollateralLpAmountOf<Self>,
	) -> Result<(), DispatchError>;

	/// get all existing markets for current deposit
	fn get_markets_for_borrow(vault: Self::VaultId) -> Vec<Self::MarketId>;

	#[allow(clippy::type_complexity)]
	// REVIEW: Why not use a map? Could also return an iterator, that way the caller can do whatever
	// they like with it.
	fn get_all_markets() -> Vec<(
		Self::MarketId,
		MarketConfig<
			Self::VaultId,
			Self::MayBeAssetId,
			Self::AccountId,
			Self::LiquidationStrategyId,
		>,
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

	/// Attempt to repay part or all of `beneficiary`'s debts, paid from `from`.
	/// - `market_id` : id of the market being repaid.
	/// - `repay_amount`: the amount of borrow asset to be repaid. See [`RepayStrategy`] for more
	///   information.
	///
	/// Returns the amount that was repaid.
	// REVIEW: Rename `from` parameter? `payer`, perhaps
	fn repay_borrow(
		market_id: &Self::MarketId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		repay_amount: RepayStrategy<BorrowAmountOf<Self>>,
	) -> Result<BorrowAmountOf<Self>, DispatchError>;

	/// total debts principals (not includes interest)
	fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	/// Floored down to zero.
	// ^ why?
	fn total_interest(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	/// ````python
	/// delta_interest_rate = delta_time / period_interest_rate
	/// debt_delta = debt_principal * delta_interest_rate
	/// new_accrued_debt = accrued_debt + debt_delta
	/// total_debt = debt_principal + new_accrued_debt
	/// ```
	fn accrue_interest(market_id: &Self::MarketId, now: Timestamp) -> Result<(), DispatchError>;

	/// current borrowable balance of market
	fn total_cash(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	/// utilization_ratio = total_borrows / (total_cash + total_borrows).
	/// utilization ratio is 0 when there are no borrows.
	fn calc_utilization_ratio(
		cash: Self::Balance,
		borrows: Self::Balance,
	) -> Result<Percent, DispatchError>;

	/// The amount of *borrow asset* debt remaining for the account in the specified market,
	/// including accrued interest.
	///
	/// Could also be thought of as the amount of *borrow asset* the account must repay to be
	/// totally debt free in the specified market.
	///
	/// Calculate account's borrow balance using the borrow index at the start of block time.
	///
	/// ```python
	/// new_borrow_balance = principal * (market_borrow_index / borrower_borrow_index)
	/// ```
	fn total_debt_with_interest(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<BorrowAmountOf<Self>, DispatchError>;

	fn collateral_of_account(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<CollateralLpAmountOf<Self>, DispatchError>;

	/// Borrower shouldn't borrow more than his total collateral value
	fn collateral_required(
		market_id: &Self::MarketId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Returns the borrow limit for an account in `Oracle` price.
	/// Calculation uses indexes from start of block time.
	/// Depends on overall collateral put by user into vault.
	/// This borrow limit of specific user, depends only on prices and users collateral, not on
	/// state of vault.
	///
	/// REVIEW: Order of operations below?
	/// ```python
	/// collateral_balance * collateral_price / collateral_factor - borrower_balance_with_interest * borrow_price
	/// ```
	fn get_borrow_limit(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, DispatchError>;
}
