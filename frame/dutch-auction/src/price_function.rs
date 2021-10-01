//! Price function for auction with price going to minimal possible value.
//! Linear, step-wise exponential, and continuous exponential, others, configured from MakerDao
//! https://github.com/makerdao/dss/blob/master/src/abaci.sol

use composable_traits::{
	auction::{LinearDecrease, StairstepExponentialDecrease},
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

pub trait AuctionTimeCurveModel {
	/// return current auction price
	fn price(
		&self,
		initial_price: LiftedFixedBalance,
		duration_since_start: DurationSeconds,
	) -> Result<LiftedFixedBalance, ArithmeticError>;
}

/// Price calculation when price is decreased linearly in proportion to time:
/// Returns y = initial_price * ((tau - duration_since_start) / tau)
impl AuctionTimeCurveModel for LinearDecrease {
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
						self.total.saturating_sub(duration_since_start) as u128,
					)
					.unwrap(),
				)?
				.safe_div(&&LiftedFixedBalance::checked_from_integer(self.total as u128).unwrap())
		}
	}
}

/// returns: top * (cut ^ dur)
impl AuctionTimeCurveModel for StairstepExponentialDecrease {
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
