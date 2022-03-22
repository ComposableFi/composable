use composable_support::validation::Validated;
use composable_traits::{
	currency::MathBalance,
	defi::{validate::MoreThanOne, LiftedFixedBalance, MoreThanOneFixedU128},
	math::SafeArithmetic,
};
use frame_benchmarking::Zero;
use sp_runtime::{traits::Saturating, ArithmeticError, FixedPointNumber, FixedU128, Percent};

/// Information about a borrower, including the respective total values of their collateral and
/// borrow assets, and the collateral_factor and under_collateralized_warn_percent for the market.
#[derive(Debug)]
pub struct BorrowerData {
	/// The value of the total amount of collateral asset that the borrower has deposited into the
	/// market.
	pub collateral_balance_total_value: FixedU128,
	/// The value of the borrower's borrow asset balance.
	pub borrow_balance_total_value: FixedU128,
	/// The collateral factor for the market.
	///
	/// See [`MarketConfig::collateral_factor`][composable_traits::lending::MarketConfig::
	/// collateral_factor] for more information.
	pub collateral_factor: Validated<FixedU128, MoreThanOne>,
	pub under_collateralized_warn_percent: Percent,
}

impl BorrowerData {
	#[inline(always)]
	pub fn new<T: MathBalance>(
		collateral_balance_total_value: T,
		borrow_balance_total_value: T,
		collateral_factor: Validated<FixedU128, MoreThanOne>,
		under_collateralized_warn_percent: Percent,
	) -> Self {
		Self {
			collateral_balance_total_value: FixedU128::saturating_from_integer(
				collateral_balance_total_value.into(),
			),
			borrow_balance_total_value: FixedU128::saturating_from_integer(
				borrow_balance_total_value.into(),
			),
			collateral_factor,
			under_collateralized_warn_percent,
		}
	}

	/// The maximum borrowable amount, taking into account the current borrowed amount and
	/// interest accrued.
	///
	/// NOTE: Returns `zero` if the borrower is under-collateralized.
	#[inline(always)]
	pub fn get_borrow_limit(&self) -> Result<LiftedFixedBalance, ArithmeticError> {
		let maximum_borrow_amount = self.max_borrow_for_collateral()?;
		// NOTE(benluelo): `saturating_sub` is used here on purpose.
		// If the borrower is under-collateralized, `borrow_balance_total_value` will be greater
		// than `max_borrow`, in which case we want to return zero (since the borrower can't borrow
		// any more).
		//
		// With `safe_sub`, this would be an error, which *could* be unwrapped to zero, but that's
		// just the behaviour of `saturating_sub` and I see no reason to reinvent the wheel.
		let amount_left_to_borrow =
			maximum_borrow_amount.saturating_sub(self.borrow_balance_total_value);
		Ok(amount_left_to_borrow)
	}

	/// Returns the amount of collateral asset available in the market for the borrower, i.e. the
	/// amount not being held as collateral.
	#[inline(always)]
	fn max_borrow_for_collateral(&self) -> Result<FixedU128, ArithmeticError> {
		self.collateral_balance_total_value.safe_div(&self.collateral_factor)
	}

	/// Determines whether the loan should trigger a liquidation.
	#[inline(always)]
	pub fn should_liquidate(&self) -> Result<bool, ArithmeticError> {
		match self.current_collateral_ratio()? {
			CurrentCollateralRatio::Ratio(ratio) => Ok(ratio < *self.collateral_factor),
			// No liquidation necessary if the borrower's borrow asset balance has no value
			CurrentCollateralRatio::NoBorrowValue => Ok(false),
		}
	}

	/// The current collateral to debt ratio for the borrower. See [`CurrentCollateralRatio`] for
	/// more information.
	#[inline(always)]
	pub fn current_collateral_ratio(
		&self,
	) -> Result<CurrentCollateralRatio<FixedU128>, ArithmeticError> {
		if self.borrow_balance_total_value.is_zero() {
			Ok(CurrentCollateralRatio::NoBorrowValue)
		} else {
			// REVIEW: What errors can occur with safe_div?
			let ratio =
				self.collateral_balance_total_value.safe_div(&self.borrow_balance_total_value)?;
			Ok(CurrentCollateralRatio::Ratio(ratio))
		}
	}

	/// The lowest value the collateral ratio can go before the borrower will be warned about soon
	/// being under-collateralized.
	///
	/// Calculated as follows:
	///
	/// ```python
	/// safe_collateral_factor = collateral_factor + (collateral_factor * under_collateralized_warn_percent)
	/// ```
	///
	/// # Example
	///
	/// ```rust,ignore
	/// let collateral_factor = 2;
	/// let under_collateralized_warn_percent = Percent::from_percent(10); // 10%
	///
	/// let safe_collateral_factor = collateral_factor + (collateral_factor * under_collateralized_warn_percent);
	///                         // = 2 + (2 * 10%)
	///                         // = 2 + 0.2
	///                         // = 2.2
	/// ```
	///
	/// With the above values, if the collateral to debt ratio goes below `2.2`, the account will be
	/// warned about soon being under-collateralized.
	#[inline(always)]
	pub fn minimum_safe_collateral_factor(&self) -> Result<MoreThanOneFixedU128, ArithmeticError> {
		let collateral_factor_warn_percentage =
			self.collateral_factor.safe_mul(&self.under_collateralized_warn_percent.into());
		self.collateral_factor.safe_add(&collateral_factor_warn_percentage?)
	}

	/// Check if a loan is about to go under-collateralized.
	///
	/// Checks the [`current_collateral_ratio`] against the [`minimum_safe_collateral_factor`].
	///
	/// [`minimum_safe_collateral_factor`]: [BorrowerData::minimum_safe_collateral_factor]
	/// [`current_collateral_ratio`]: [BorrowerData::current_collateral_ratio]
	#[inline(always)]
	pub fn should_warn(&self) -> Result<bool, ArithmeticError> {
		match self.current_collateral_ratio()? {
			CurrentCollateralRatio::Ratio(ratio) =>
				Ok(ratio < self.minimum_safe_collateral_factor()?),
			// No liquidation necessary if the borrower's borrow asset balance has no value
			CurrentCollateralRatio::NoBorrowValue => Ok(false),
		}
	}
}

// TODO: Tests?

/// Representation for the collateral ratio of a borrower. It's possible for the borrow value to be
/// zero when calculating this, which would result in a divide by zero error, hence the
/// [`NoBorrowValue`][CurrentCollateralRatio::NoBorrowValue] variant.
pub enum CurrentCollateralRatio<T> {
	/// The current collateral:debt ratio for the borrower.
	Ratio(T),
	/// The total value of the borrow assets owned by the borrower is `0`, either because the
	/// account hasn't borrowed yet *or* the borrow asset has no value.
	NoBorrowValue,
}
