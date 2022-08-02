use composable_support::validation::{TryIntoValidated, Validate};
use composable_traits::{
	defi::DeFiEngine,
	oracle::Oracle as OracleTrait,
	undercollateralized_loans::{LoanInput, MarketInput, UndercollateralizedLoans},
};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::Zero;
use sp_std::ops::Rem;
#[derive(Clone, Copy, RuntimeDebug, PartialEq, TypeInfo, Default)]
pub struct CurrencyPairIsNotSame;

impl<LiquidationStrategyId, Asset: Eq, BlockNumber, AccountId>
	Validate<
		MarketInput<LiquidationStrategyId, Asset, BlockNumber, AccountId>,
		CurrencyPairIsNotSame,
	> for CurrencyPairIsNotSame
{
	fn validate(
		create_input: MarketInput<LiquidationStrategyId, Asset, BlockNumber, AccountId>,
	) -> Result<MarketInput<LiquidationStrategyId, Asset, BlockNumber, AccountId>, &'static str> {
		if create_input.currency_pair.base == create_input.currency_pair.quote {
			Err("Base and quote currencies supposed to be different in currency pair")
		} else {
			Ok(create_input)
		}
	}
}

#[derive(RuntimeDebug, PartialEq, TypeInfo, Default, Clone, Copy)]
pub struct AssetIsSupportedByOracle<Oracle: OracleTrait>(PhantomData<Oracle>);

impl<
		LiquidationStrategyId,
		Asset: Copy,
		BlockNumber,
		Oracle: OracleTrait<AssetId = Asset>,
		AccountId,
	>
	Validate<
		MarketInput<LiquidationStrategyId, Asset, BlockNumber, AccountId>,
		AssetIsSupportedByOracle<Oracle>,
	> for AssetIsSupportedByOracle<Oracle>
{
	fn validate(
		create_input: MarketInput<LiquidationStrategyId, Asset, BlockNumber, AccountId>,
	) -> Result<MarketInput<LiquidationStrategyId, Asset, BlockNumber, AccountId>, &'static str> {
		ensure!(
			Oracle::is_supported(create_input.currency_pair.quote)?,
			"Borrow asset is not supported by oracle"
		);
		ensure!(
			Oracle::is_supported(create_input.currency_pair.base)?,
			"Collateral asset is not supported by oracle"
		);
		Ok(create_input)
	}
}

#[derive(RuntimeDebug, PartialEq, TypeInfo, Default, Copy, Clone)]
pub struct BalanceGreaterThenZero;
impl<Balance> Validate<Balance, BalanceGreaterThenZero> for BalanceGreaterThenZero
where
	Balance: Zero + PartialOrd,
{
	fn validate(balance: Balance) -> Result<Balance, &'static str> {
		ensure!(balance > Balance::zero(), "Can not deposit or withdraw zero amount of assets.");
		Ok(balance)
	}
}

#[derive(RuntimeDebug, PartialEq, TypeInfo, Default, Clone, Copy)]
pub struct LoanInputIsValid<Loans: UndercollateralizedLoans>(PhantomData<Loans>);

impl<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy, Loans>
	Validate<
		LoanInput<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>,
		LoanInputIsValid<Loans>,
	> for LoanInputIsValid<Loans>
where
	Balance: Zero + PartialOrd,
	BlockNumber: Zero + Rem<Output = BlockNumber> + PartialOrd + PartialEq + Copy,
	Percent: Zero + PartialOrd,
	Loans: UndercollateralizedLoans + DeFiEngine<AccountId = AccountId>,
{
	fn validate(
		input: LoanInput<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>,
	) -> Result<LoanInput<AccountId, Balance, BlockNumber, Percent, RepaymentStrategy>, &'static str>
	{
		// Check that principal balance	> 0
		let principal = input.principal.try_into_validated::<BalanceGreaterThenZero>()?.value();
		// Check that collateral balance > 0
		let collateral = input.collateral.try_into_validated::<BalanceGreaterThenZero>()?.value();
		// Check that interest rate > 0
		ensure!(input.interest > Percent::zero(), "Can not create loan with zero interest rate.");
		// Check that payment frequency > 0
		ensure!(
			input.payment_frequency > BlockNumber::zero(),
			"Can not create loan with zero payment frequency."
		);
		// Checks that the borrower's account is included in the market's whitelis of borrowers.
		ensure!(
			Loans::is_borrower_account_whitelisted(
				&input.borrower_account_id,
				&input.market_account_id
			)?,
			"Mentioned borrower is not included in the market's whitelist of borrowers."
		);
		//Checks that last interest payment happens when the loan is mature.
		ensure!(
			input.loan_maturity % input.payment_frequency == BlockNumber::zero(),
			"Last interest payment and loan's mature do not fit to each other."
		);

		Ok(LoanInput { principal, collateral, ..input })
	}
}
