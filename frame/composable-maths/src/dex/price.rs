use composable_traits::{defi::Rate, math::{SafeArithmetic, SafeMul}};
use frame_support::ensure;
use sp_runtime::{
	traits::{AtLeast32Bit, Convert, Saturating, Zero},
	ArithmeticError, DispatchError, FixedPointNumber, SaturatedConversion,
};

/// Uniswap V2 TWAP
/// https://docs.uniswap.org/protocol/V2/concepts/core-concepts/oracles
pub fn compute_initial_price_cumulative<C, Balance>(
	current_exchange_rate: Rate,
) -> Result<Balance, DispatchError>
where
	C: Convert<u128, Balance>,
	Balance: Zero + SafeArithmetic,
{
	compute_next_price_cumulative::<C, Balance, _>(
		0_u64,
		Balance::zero(),
		1_u64,
		current_exchange_rate,
	)
	.map(|(_, x)| x)
}

pub fn compute_next_price_cumulative<C, Balance, Timestamp>(
	previous_timestamp: Timestamp,
	previous_price_cumulative: Balance,
	current_timestamp: Timestamp,
	current_exchange_rate: Rate,
) -> Result<(Timestamp, Balance), DispatchError>
where
	Balance: SafeArithmetic,
	Timestamp: AtLeast32Bit + SaturatedConversion + Saturating + Ord + Copy,
	C: Convert<u128, Balance>,
{
	ensure!(current_timestamp > previous_timestamp, ArithmeticError::Underflow);
	let elapsed = current_timestamp.saturating_sub(previous_timestamp);
	let weighted_price: u128 = current_exchange_rate
		.safe_mul(&Rate::saturated_from(elapsed.saturated_into::<u128>()))?
		.checked_mul_int(1_u128)
		.ok_or(ArithmeticError::Overflow)?;
	let current_price_cumulative =
		previous_price_cumulative.safe_add(&C::convert(weighted_price))?;
	Ok((elapsed, current_price_cumulative))
}

pub fn compute_price_average<C, Balance, Timestamp>(
	current_price_cumulative: Balance,
	previous_price_cumulative: Balance,
	elapsed: Timestamp,
) -> Result<Rate, DispatchError>
where
	C: Convert<Balance, u128>,
	Timestamp: AtLeast32Bit + SaturatedConversion,
{
	Ok(Rate::checked_from_rational(
		C::convert(current_price_cumulative).saturating_sub(C::convert(previous_price_cumulative)),
		elapsed.saturated_into::<u128>(),
	)
	.ok_or(ArithmeticError::Overflow)?)
}
