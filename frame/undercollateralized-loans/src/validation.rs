use codec::FullCodec;
use composable_support::validation::{TryIntoValidated, Validate};
use composable_traits::{
	defi::DeFiEngine,
	oracle::Oracle as OracleTrait,
	undercollateralized_loans::{LoanInput, MarketInput, UndercollateralizedLoans},
};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::Zero;

#[derive(RuntimeDebug, PartialEq, TypeInfo, Default, Clone, Copy)]
pub struct MarketInputIsValid<Oracle, Loans>(PhantomData<(Oracle, Loans)>);

impl<
		Oracle: OracleTrait<AssetId = Asset>,
		Loans: UndercollateralizedLoans,
		Asset: Copy + PartialEq,
		LiquidationStrategyId,
		BlockNumber,
		AccountId,
	>
	Validate<
		MarketInput<LiquidationStrategyId, Asset, BlockNumber, AccountId>,
		MarketInputIsValid<Oracle, Loans>,
	> for MarketInputIsValid<Oracle, Loans>
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
		ensure!(
			create_input.currency_pair.base != create_input.currency_pair.quote,
			"Base and quote currencies are supposed to be different in currency pair"
		);
		ensure!(
			create_input.whitelist.len() <
				Loans::WhiteListBound::get().try_into().expect("This method never panics!"),
			"Payment schedule exceeded maximum size."
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

impl<AccountId, Balance, Loans, Timestamp>
	Validate<LoanInput<AccountId, Balance, Timestamp>, LoanInputIsValid<Loans>>
	for LoanInputIsValid<Loans>
where
	Loans: UndercollateralizedLoans + DeFiEngine<AccountId = AccountId>,
	AccountId: Clone + Eq + PartialEq + FullCodec,
	Balance: Zero + PartialOrd,
	Timestamp: Clone + Ord,
{
	// TODO: @mikolaichuk: add validation of shift value T::MaxDateShiftingInDays
	fn validate(
		input: LoanInput<AccountId, Balance, Timestamp>,
	) -> Result<LoanInput<AccountId, Balance, Timestamp>, &'static str> {
		// Check that principal balance	> 0
		let principal = input.principal.try_into_validated::<BalanceGreaterThenZero>()?.value();
		// Check that collateral balance > 0
		let collateral = input.collateral.try_into_validated::<BalanceGreaterThenZero>()?.value();
		// Checks that the borrower's account is included in the market's whitelis of borrowers.
		ensure!(
			Loans::is_borrower_account_whitelisted(
				&input.borrower_account_id,
				&input.market_account_id
			)?,
			"Mentioned borrower is not included in the market's whitelist of borrowers."
		);
		// Borrower's account should not be included in the market's blacklist.
		ensure!(
			Loans::is_borrower_account_not_blacklisted(
				&input.borrower_account_id,
				&input.market_account_id
			),
			"Mentioned borrower is presented in the market's blacklist of borrowers."
		);
		// Check if payment schedule is empty.
		// We should have at least one payment.
		ensure!(input.payment_schedule.len() > 0, "Payment schedule is empty.");
		// Unwrapped since u32 can be safely converted to usize.
		ensure!(
			input.payment_schedule.len() <
				Loans::ScheduleBound::get().try_into().expect("This method never panics."),
			"Payment schedule exceeded maximum size."
		);
		// Unwrapped since we have checked that shcedule is not empty.
		ensure!(
			input.activation_date <=
				input.payment_schedule.keys().min().cloned().expect("This mehtod never panics."),
			"Contract first date payment is less than activation date."
		);
		// Delayed payments threshold and failed payments shift should not be zero.
		if let Some(treatment) = &input.delayed_payment_treatment {
			ensure!(
				treatment.delayed_payments_threshold > 0,
				"Delayed payments threshold equals zero."
			);
			ensure!(
				treatment.delayed_payments_shift_in_days > 0,
				"Delayed payments shif equals zero."
			);
			ensure!(
				treatment.delayed_payments_shift_in_days < Loans::MaxDateShiftingInDays::get(),
				"Maximum date shifting exceeded."
			);
		};

		Ok(LoanInput { principal, collateral, ..input })
	}
}
