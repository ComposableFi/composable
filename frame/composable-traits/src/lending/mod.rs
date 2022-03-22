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
use sp_runtime::{
	traits::{One, Zero},
	Percent,
};

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
#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq)]
pub enum RepayStrategy<T> {
	/// Attempt to repay the entirety of the remaining debt.
	TotalDebt,
	/// Repay the specified amount, repaying interest first and then principal with whatever (if
	/// anything) is left.
	///
	/// NOTE: Must be *less than* the total owing amount + the accrued interest.
	PartialAmount(T),
	// REVIEW: Perhaps add an "interest only" strategy?
}

/// The total amount of debt for an account on a market, if any.
#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq)]
pub enum TotalDebtWithInterest<T> {
	/// The account has some amount of debt on the market.
	Amount(T),
	/// The account has not borrowed from the market yet, or has paid off their debts. There is no
	/// interest or principal left to repay.
	NoDebt,
}

impl<T> TotalDebtWithInterest<T>
where
	T: Zero,
{
	/// Returns the value contained in [`Amount`], or `T::zero()` if `self` is [`NoDebt`].
	///
	/// [`Amount`]: TotalDebtWithInterest::Amount
	/// [`NoDebt`]: TotalDebtWithInterest::NoDebt
	pub fn unwrap_or_zero(self) -> T {
		match self {
			TotalDebtWithInterest::Amount(amount) => amount,
			TotalDebtWithInterest::NoDebt => T::zero(),
		}
	}
}

impl<T> TotalDebtWithInterest<T> {
	///    Returns the contained [`Amount`] value, consuming the self value.
	///
	/// # Panics
	///
	/// Panics if the self value equals [`NoDebt`].
	///
	/// [`Amount`]: TotalDebtWithInterest::Amount
	/// [`NoDebt`]: TotalDebtWithInterest::NoDebt
	// #[cfg(test)]
	pub fn unwrap_amount(self) -> T {
		match self {
			TotalDebtWithInterest::Amount(amount) => amount,
			TotalDebtWithInterest::NoDebt => {
				panic!("called `TotalDebtWithInterest::unwrap_amount()` on a `NoDebt` value")
			},
		}
	}

	/// Returns `true` if the total debt with interest is [`Amount`].
	///
	/// [`Amount`]: TotalDebtWithInterest::Amount
	pub fn is_amount(&self) -> bool {
		matches!(self, Self::Amount(..))
	}

	/// Returns `true` if the total debt with interest is [`NoDebt`].
	///
	/// [`NoDebt`]: TotalDebtWithInterest::NoDebt
	pub fn is_no_debt(&self) -> bool {
		matches!(self, Self::NoDebt)
	}
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

	/// [`AccountId`][Self::AccountId] of the market instance
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

	// REVIEW: what
	/// `amount_to_borrow` is the amount of the borrow asset lendings's vault shares the user wants
	/// to borrow. Amounts are normalized for calculations.
	/// Borrows as exact amount as possible with some inaccuracies for oracle price based
	/// normalization. If there is not enough collateral or borrow amounts - fails
	fn borrow(
		market_id: &Self::MarketId,
		debt_owner: &Self::AccountId,
		amount_to_borrow: BorrowAmountOf<Self>,
	) -> Result<(), DispatchError>;

	/// Attempt to repay part or all of `beneficiary`'s debts, paid from `from`.
	///
	/// - `market_id`: id of the market being repaid.
	/// - `from`: the account repaying the debt.
	/// - `beneficiary`: the account who's debt is being repaid.
	/// - `repay_amount`: the amount of debt to be repaid. See [`RepayStrategy`] for more
	///   information.
	///
	/// Returns the amount that was repaid if the repay was successful.
	///
	/// NOTE: `from` and `beneficiary` can be the same account.
	// REVIEW: Rename `from` parameter? `payer`, perhaps
	fn repay_borrow(
		market_id: &Self::MarketId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		repay_amount: RepayStrategy<BorrowAmountOf<Self>>,
	) -> Result<BorrowAmountOf<Self>, DispatchError>;

	/// The total amount borrowed from the given market, not including interest.
	fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError>;

	/// Floored down to zero.
	// REVIEW: ^ why?
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
	) -> Result<TotalDebtWithInterest<BorrowAmountOf<Self>>, DispatchError>;

	fn collateral_of_account(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<CollateralLpAmountOf<Self>, DispatchError>;

	/// Borrower shouldn't borrow more than his total collateral value
	///
	/// The amount of collateral that would be required in order to borrow `borrow_amount` of borrow
	/// asset.
	///
	/// Can be thought of as the "inverse" of [`Lending::get_borrow_limit`], in that
	/// `get_borrow_limit` returns the maximum amount borrowable with the *current* collateral,
	/// while `collateral_required` returns the amount of collateral asset that would be needed to
	/// borrow the specified amount.
	fn collateral_required(
		market_id: &Self::MarketId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Returns the "borrow limit" for an account in `Oracle` price, i.e. the maximum amount an
	/// account can borrow before going under-collateralized.
	///
	/// REVIEW: What?
	/// The calculation uses indexes from start of block time.
	///
	/// The borrow limit is only affected by the prices of the assets and the amount of collateral
	/// deopsited by the account, and is *specific to this account*. The state of the vault is not
	/// relevant for this calculation.
	///
	/// The calculation is as follows, broken up for clarity:
	///
	/// ```ignore
	/// // total value of the account's collateral
	/// collateral_value = collateral_balance * collateral_price
	///
	/// // available value of the account's collateral, i.e. the amount not held as collateral
	/// collateral_value_available = collateral_value / collateral_factor
	///
	/// // total value of the account's borrowed asset, including interest
	/// value_already_borrowed = borrower_total_balance_with_interest * borrow_price
	///
	/// // the maximum amount the account can borrow
	/// borrow_limit = collateral_value_available - value_already_borrowed
	/// ```
	///
	/// # Example
	///
	/// ```ignore
	/// // Given the following values:
	/// let collateral_balance = 100;
	/// let collateral_price = 50_000;
	/// let collateral_factor = 2;
	/// let borrower_total_balance_with_interest = 100;
	/// let borrow_price = 1_000;
	///
	/// let collateral_value = collateral_balance * collateral_price;
	///                   // = 100 * 50_000
	///                   // = 5_000_000
	///
	/// let collateral_value_available = collateral_value / collateral_factor;
	///                             // = 5_000_000 / 2
	///                             // = 2_500_000
	///
	/// let value_already_borrowed = borrower_total_balance_with_interest * borrow_price;
	///               // = 100 * 1_000
	///               // = 100_000
	///
	/// let borrow_limit = collateral_value_available - value_already_borrowed;
	///               // = 2_500_000 - 100_000
	///               // = 2_400_000
	/// ```
	///
	/// ...meaning the borrower can borrow 2.4m *worth* of the borrow asset.
	///
	/// Given that the price of the borrow asset is `1_000`, they would be able to borrow ***`2,400`
	/// total tokens*** of borrow asset.
	///
	/// The borrow limit will fluctuate as the prices of the borrow and collateral assets fluctuate,
	/// going *up* as either the borrow asset *loses* value or the collateral asset *gains* value,
	/// and going *down* as either the borrow asset *gains* value or the collateral asset *loses*
	/// value.
	///
	/// NOTE: This will return `zero` if the account has not deposited any collateral yet (a newly
	/// created market, for instance) ***OR*** if the account has already borrowed the maximum
	/// amount borrowable with the given amount of collateral deposited.
	fn get_borrow_limit(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, DispatchError>;
}
