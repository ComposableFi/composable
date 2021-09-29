use composable_traits::{
	math::{LiftedFixedBalance, SafeArithmetic},
	rate_model::NormalizedCollateralFactor,
};
use sp_runtime::{traits::Saturating, ArithmeticError};

pub struct BorrowerData {
	pub collateral_balance: LiftedFixedBalance,
	pub collateral_price: LiftedFixedBalance,
	pub borrower_balance_with_interest: LiftedFixedBalance,
	pub borrow_price: LiftedFixedBalance,
	pub collateral_factor: NormalizedCollateralFactor,
}

impl BorrowerData {
	#[inline(always)]
	pub fn new<T: Into<LiftedFixedBalance>>(
		collateral_balance: T,
		collateral_price: T,
		borrower_balance_with_interest: T,
		borrow_price: T,
		collateral_factor: NormalizedCollateralFactor,
	) -> Self {
		Self {
			collateral_balance: collateral_balance.into(),
			collateral_price: collateral_price.into(),
			borrower_balance_with_interest: borrower_balance_with_interest.into(),
			borrow_price: borrow_price.into(),
			collateral_factor,
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
		Ok(max_borrow.saturating_sub(borrowed))
	}

	pub fn should_liquidate(&self) -> Result<bool, ArithmeticError> {
		let collateral = self.collateral_balance.safe_mul(&self.collateral_price)?;
		let borrowed = self.borrower_balance_with_interest.safe_mul(&self.borrow_price)?;
		let current_collateral_ratio = collateral.safe_div(&borrowed)?;
		Ok(current_collateral_ratio < self.collateral_factor)
	}
}
