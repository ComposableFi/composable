use composable_support::math::safe::{SafeArithmetic, SafeMul};
use frame_support::ensure;
use sp_runtime::{
	traits::{AtLeast32Bit, Convert as ConvertTrait, Saturating, Zero},
	ArithmeticError, DispatchError, FixedPointNumber, FixedU128, SaturatedConversion,
};

/// Uniswap V2 TWAP
/// https://docs.uniswap.org/protocol/V2/concepts/core-concepts/oracles

/// Executes `compute_next_price_cumulative` with `previous_timestamp = 0`
/// `current_timestamp =  1` and `previous_price_cumulative = 0`
pub fn compute_initial_price_cumulative<Convert, Balance>(
	current_exchange_rate: FixedU128,
) -> Result<Balance, DispatchError>
where
	Convert: ConvertTrait<u128, Balance>,
	Balance: Zero + SafeArithmetic,
{
	compute_next_price_cumulative::<Convert, Balance, _>(
		0_u64,
		Balance::zero(),
		1_u64,
		current_exchange_rate,
	)
	.map(|(_, x)| x)
}

/// Computes next_price_cumulative as per following equation
/// next_price_cumulative = previous_price_cumulative + (current_timestamp - previous_timestamp) *
/// current_exchange_rate
pub fn compute_next_price_cumulative<Convert, Balance, Timestamp>(
	previous_timestamp: Timestamp,
	previous_price_cumulative: Balance,
	current_timestamp: Timestamp,
	current_exchange_rate: FixedU128,
) -> Result<(Timestamp, Balance), DispatchError>
where
	Balance: SafeArithmetic,
	Timestamp: AtLeast32Bit + SaturatedConversion + Saturating + Ord + Copy,
	Convert: ConvertTrait<u128, Balance>,
{
	ensure!(
		current_timestamp > previous_timestamp,
		DispatchError::Other("Stale TWAP Request Found")
	);
	let elapsed = current_timestamp.saturating_sub(previous_timestamp);
	let new_price_cumulative: u128 = current_exchange_rate
		.safe_mul(&FixedU128::saturated_from(elapsed.saturated_into::<u128>()))?
		.checked_mul_int(1_u128)
		.ok_or(ArithmeticError::Overflow)?;
	let current_price_cumulative =
		previous_price_cumulative.safe_add(&Convert::convert(new_price_cumulative))?;
	Ok((elapsed, current_price_cumulative))
}

/// Computes price of asset as per following equation
/// twap = (current_price_cumulative - previous_price_cumulative) / elapsed
pub fn compute_twap<Convert, Balance, Timestamp>(
	current_price_cumulative: Balance,
	previous_price_cumulative: Balance,
	elapsed: Timestamp,
) -> Result<FixedU128, DispatchError>
where
	Convert: ConvertTrait<Balance, u128>,
	Timestamp: AtLeast32Bit + SaturatedConversion,
{
	Ok(FixedU128::checked_from_rational(
		Convert::convert(current_price_cumulative)
			.saturating_sub(Convert::convert(previous_price_cumulative)),
		elapsed.saturated_into::<u128>(),
	)
	.ok_or(ArithmeticError::Overflow)?)
}

#[cfg(test)]
mod test {
	use crate::dex::price::compute_next_price_cumulative;

	use super::compute_twap;
	use sp_runtime::{
		traits::ConvertInto, ArithmeticError, DispatchError, FixedPointNumber, FixedU128,
	};

	#[test]
	fn compute_next_price_cumulative_works() {
		let previous_timestamp = 10_u32;
		let current_timestamp = 20_u32;
		let previous_price_cumulative = 100_u128;
		let current_exchange_rate = FixedU128::saturating_from_integer(10_u128);
		let price = compute_next_price_cumulative::<ConvertInto, u128, u32>(
			previous_timestamp,
			previous_price_cumulative,
			current_timestamp,
			current_exchange_rate,
		);
		assert_eq!(price, Ok((10_u32, 200_u128)));
	}

	#[test]
	fn compute_next_price_cumulative_0() {
		let previous_timestamp = 10_u32;
		let current_timestamp = 20_u32;
		let previous_price_cumulative = 100_u128;
		let current_exchange_rate = FixedU128::saturating_from_integer(0_u128);
		let price = compute_next_price_cumulative::<ConvertInto, u128, u32>(
			previous_timestamp,
			previous_price_cumulative,
			current_timestamp,
			current_exchange_rate,
		);
		assert_eq!(price, Ok((10_u32, previous_price_cumulative)));
	}

	#[test]
	fn compute_next_price_cumulative_stale_request() {
		let previous_timestamp = 30_u32;
		let current_timestamp = 20_u32;
		let previous_price_cumulative = 100_u128;
		let current_exchange_rate = FixedU128::saturating_from_integer(10_u128);
		let price = compute_next_price_cumulative::<ConvertInto, u128, u32>(
			previous_timestamp,
			previous_price_cumulative,
			current_timestamp,
			current_exchange_rate,
		);
		assert_eq!(price, Err(DispatchError::Other("Stale TWAP Request Found")));
	}

	#[test]
	fn compute_twap_works() {
		let current_price_cumulative = 200_u128;
		let previous_price_cumulative = 100_u128;
		let elapsed = 10_u32;
		let price = compute_twap::<ConvertInto, u128, u32>(
			current_price_cumulative,
			previous_price_cumulative,
			elapsed,
		);
		assert_eq!(price, Ok(FixedU128::saturating_from_integer(10_u128)));
	}

	#[test]
	fn compute_twap_fails() {
		let current_price_cumulative = 200_u128;
		let previous_price_cumulative = 100_u128;
		let elapsed = 0_u32;
		let price = compute_twap::<ConvertInto, u128, u32>(
			current_price_cumulative,
			previous_price_cumulative,
			elapsed,
		);
		assert_eq!(price, Err(DispatchError::Arithmetic(ArithmeticError::Overflow)));
	}

	#[test]
	fn compute_twap_0() {
		let current_price_cumulative = 100_u128;
		let previous_price_cumulative = 200_u128;
		let elapsed = 1_u32;
		let price = compute_twap::<ConvertInto, u128, u32>(
			current_price_cumulative,
			previous_price_cumulative,
			elapsed,
		);
		assert_eq!(price, Ok(FixedU128::saturating_from_integer(0_u128)));
	}
}
