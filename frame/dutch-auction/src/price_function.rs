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
	ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Permill, Perquintill,
};

struct LinearDecrease {
	/// Seconds after auction start when the price reaches zero
	total: DurationSeconds,
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
		if duration_since_start >= self.total {
			Ok(LiftedFixedBalance::zero())
		} else {
			// here we violate unit of measure to have best math
			initial_price
				.safe_mul(
					&LiftedFixedBalance::checked_from_integer(
						self.total.safe_sub(&duration_since_start)? as u128,
					)
					.unwrap(),
				)?
				.safe_div(&&LiftedFixedBalance::checked_from_integer(self.total as u128).unwrap())
		}
	}
}

struct StairstepExponentialDecrease {
	// Length of time between price drops
	step: DurationSeconds,
	// Per-step multiplicative factor, usually more than 50%, mostly closer to 100%, but not 100%.
	// Drop per unit of `step`.
	cut: Permill,
}

impl AuctionTimeCurveModel for StairstepExponentialDecrease {
	// returns: top * (cut ^ dur)
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError> {
		let multiplier =
			self.cut.saturating_pow(duration_since_start.safe_div(&self.step)? as usize);
		initial_price.safe_mul(&multiplier.into())
	}
}
