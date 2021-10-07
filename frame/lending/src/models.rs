use composable_traits::{
	math::{LiftedFixedBalance, SafeArithmetic},
	rate_model::NormalizedCollateralFactor,
};
use sp_runtime::{traits::Saturating, ArithmeticError, Percent};

pub struct BorrowerData {
	pub collateral_balance: LiftedFixedBalance,
	pub collateral_price: LiftedFixedBalance,
	pub borrower_balance_with_interest: LiftedFixedBalance,
	pub borrow_price: LiftedFixedBalance,
	pub collateral_factor: NormalizedCollateralFactor,
	pub under_collaterized_warn_percent: Percent,
}

impl BorrowerData {
	#[inline(always)]
	pub fn new<T: Into<LiftedFixedBalance>>(
		collateral_balance: T,
		collateral_price: T,
		borrower_balance_with_interest: T,
		borrow_price: T,
		collateral_factor: NormalizedCollateralFactor,
		under_collaterized_warn_percent: Percent,
	) -> Self {
		Self {
			collateral_balance: collateral_balance.into(),
			collateral_price: collateral_price.into(),
			borrower_balance_with_interest: borrower_balance_with_interest.into(),
			borrow_price: borrow_price.into(),
			collateral_factor,
			under_collaterized_warn_percent,
		}
	}

	/// Check whether the collateralization is still valid if we subtract an amount from it.
	#[inline(always)]
	pub fn collateralization_still_valid(
		&self,
		collateral_decrease_amount: LiftedFixedBalance,
	) -> Result<bool, ArithmeticError> {
		let collateral = self
			.collateral_balance
			.safe_sub(&collateral_decrease_amount)?
			.safe_mul(&self.collateral_price)?;
		let borrowed = self
			.borrower_balance_with_interest
			.safe_mul(&self.borrow_price)?
			.safe_mul(&self.collateral_factor)?;
		Ok(collateral >= borrowed)
	}

	/// Return the maximum borrowable amount, taking into account the current borrowed amount +
	/// interests
	#[inline(always)]
	pub fn borrow_for_collateral(&self) -> Result<LiftedFixedBalance, ArithmeticError> {
		let max_borrow = self
			.collateral_balance
			.safe_mul(&self.collateral_price)?
			.safe_div(&self.collateral_factor)?;
		let borrowed = self.borrower_balance_with_interest.safe_mul(&self.borrow_price)?;
		max_borrow.saturating_sub(borrowed).safe_div(&self.borrow_price)
	}

	/// Determines whether the loan should trigger a liquidation.
	#[inline(always)]
	pub fn should_liquidate(&self) -> Result<bool, ArithmeticError> {
		let collateral = self.collateral_balance.safe_mul(&self.collateral_price)?;
		let borrowed = self.borrower_balance_with_interest.safe_mul(&self.borrow_price)?;
		let current_collateral_ratio = collateral.safe_div(&borrowed)?;
		Ok(current_collateral_ratio < self.collateral_factor)
	}

	/// Check if loan is about to be under collaterized
	/// safe_collateral_factor = collateral_factor + (collateral_factor *
	/// under_collaterized_warn_percent) For example collateral_factor = 2 and
	/// under_collaterized_warn_percent = 10% then if loan's collateral to debt ratio goes below 2.2
	/// then borrower should be warn so.
	#[inline(always)]
	pub fn should_warn(&self) -> Result<bool, ArithmeticError> {
		let collateral = self.collateral_balance.safe_mul(&self.collateral_price)?;
		let borrowed = self.borrower_balance_with_interest.safe_mul(&self.borrow_price)?;
		let current_collateral_ratio = collateral.safe_div(&borrowed)?;
		let safe_collateral_factor = self.collateral_factor.safe_add(
			&self.collateral_factor.safe_mul(&self.under_collaterized_warn_percent.into())?,
		)?;
		Ok(current_collateral_ratio < safe_collateral_factor)
	}
}
