//! Price function for auction with price going to minimal possible value.
//! Linear, step-wise exponential, and continuous exponential, others, configured from MakerDao
//! https://github.com/makerdao/dss/blob/master/src/abaci.sol

use composable_traits::{
	loans::DurationSeconds,
	math::{LiftedFixedBalance, SafeArithmetic},
};

use sp_runtime::{
	traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
		Saturating, Zero,
	},
	ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Perquintill,
};

struct LinearDecrease {
	/// Seconds after auction start when the price reaches zero
	total_duration: DurationSeconds,
}

trait AuctionTimeCurveModel {
	/// return current auction price
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError>;
}

impl AuctionTimeCurveModel for LinearDecrease {
	// Price calculation when price is decreased linearly in proportion to time:
	// Returns y = top * ((tau - dur) / tau)
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		if duration_since_start >= self.total_duration {
			Ok(LiftedFixedBalance::zero())
		} else {
			// here we violate unit of measure to have best math
			initial_price
				.safe_mul(
					&LiftedFixedBalance::checked_from_integer(
						self.total_duration.safe_sub(&duration_since_start)? as u128,
					)
					.unwrap(),
				)?
				.safe_div(
					&&LiftedFixedBalance::checked_from_integer(self.total_duration as u128)
						.unwrap(),
				)
		}
	}
}
