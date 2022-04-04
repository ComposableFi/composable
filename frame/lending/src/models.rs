use composable_support::math::safe::{SafeAdd, SafeDiv, SafeMul};
use composable_traits::{
	currency::MathBalance,
	defi::{LiftedFixedBalance, MoreThanOneFixedU128, Rate},
};
use sp_runtime::{traits::Saturating, ArithmeticError, FixedPointNumber, Percent};

#[derive(Debug)]
pub struct BorrowerData {
	pub collateral_balance_value: LiftedFixedBalance,
	pub borrow_balance_value: LiftedFixedBalance,
	pub collateral_factor: MoreThanOneFixedU128,
	pub under_collaterized_warn_percent: Percent,
}

impl BorrowerData {
	#[inline(always)]
	pub fn new<T: MathBalance>(
		collateral_balance_value: T,
		borrow_balance_value: T,
		collateral_factor: MoreThanOneFixedU128,
		under_collaterized_warn_percent: Percent,
	) -> Self {
		Self {
			collateral_balance_value: LiftedFixedBalance::saturating_from_integer(
				collateral_balance_value.into(),
			),
			borrow_balance_value: LiftedFixedBalance::saturating_from_integer(
				borrow_balance_value.into(),
			),
			collateral_factor,
			under_collaterized_warn_percent,
		}
	}

	/// Return the maximum borrowable amount, taking into account the current borrowed amount +
	/// interests
	#[inline(always)]
	pub fn borrow_for_collateral(&self) -> Result<LiftedFixedBalance, ArithmeticError> {
		let max_borrow = self.collateral_balance_value.safe_div(&self.collateral_factor)?;
		Ok(max_borrow.saturating_sub(self.borrow_balance_value))
	}

	/// Determines whether the loan should trigger a liquidation.
	#[inline(always)]
	pub fn should_liquidate(&self) -> Result<bool, ArithmeticError> {
		if self.borrow_balance_value == 0.into() {
			Ok(false)
		} else {
			Ok(self.current_collateral_ratio()? < self.collateral_factor)
		}
	}

	#[inline(always)]
	pub fn current_collateral_ratio(&self) -> Result<Rate, ArithmeticError> {
		self.collateral_balance_value.safe_div(&self.borrow_balance_value)
	}

	#[inline(always)]
	pub fn safe_collateral_factor(&self) -> Result<MoreThanOneFixedU128, ArithmeticError> {
		self.collateral_factor.safe_add(
			&self.collateral_factor.safe_mul(&self.under_collaterized_warn_percent.into())?,
		)
	}

	/// Check if loan is about to be under collaterized
	/// safe_collateral_factor = collateral_factor + (collateral_factor *
	/// under_collaterized_warn_percent) For example collateral_factor = 2 and
	/// under_collaterized_warn_percent = 10% then if loan's collateral to debt ratio goes below 2.2
	/// then borrower should be warn so.
	#[inline(always)]
	pub fn should_warn(&self) -> Result<bool, ArithmeticError> {
		Ok(self.current_collateral_ratio()? < self.safe_collateral_factor()?)
	}
}
