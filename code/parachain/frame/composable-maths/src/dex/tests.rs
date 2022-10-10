use crate::dex::constant_product::compute_out_given_in_new;
use proptest::prelude::*;
use sp_runtime::Permill;

/// Tests related to constant product math functions
mod constant_product {
	use super::*;
	/// Tests related to the function `compute_out_given_in_new`
	mod compute_out_given_in_new {
		use super::*;

		/// should always be equal to the number of elements in the `list` array found in
		/// `checked_inputs_and_outputs` function
		const CHECKED_I_AND_O_SIZE: u32 = 5;

		prop_compose! {
			/// Returns (w_i, w_o, b_i, b_o, a_sent, fee, a_out, fee_out)
			fn checked_inputs_and_outputs()
			(x in 0..CHECKED_I_AND_O_SIZE) -> (Permill, Permill, u128, u128, u128, Permill, u128, u128) {
				let list = [
					(
						Permill::from_rational::<u32>(1, 2),
						Permill::from_rational::<u32>(1, 2),
						2048,
						2048,
						100,
						Permill::from_percent(10),
						86,
						10,
					),
					(
						Permill::from_rational::<u32>(1, 3),
						Permill::from_rational::<u32>(2, 3),
						1024,
						2048,
						100,
						Permill::from_percent(10),
						84,
						10,
					),
					(
						Permill::from_rational::<u32>(1, 4),
						Permill::from_rational::<u32>(3, 4),
						10_000,
						30_000,
						100,
						Permill::from_percent(10),
						89,
						10,
					),
					(
						Permill::from_rational::<u32>(2, 5),
						Permill::from_rational::<u32>(3, 5),
						20_000_000,
						30_000_000,
						100_000,
						Permill::from_percent(10),
						89_663,
						10_000,
					),
					(
						Permill::from_rational::<u32>(2, 5),
						Permill::from_rational::<u32>(1, 5),
						20_000_000,
						10_000_000,
						100_000,
						Permill::from_percent(10),
						89_396,
						10_000,
					),
				];

				list[x as usize]
			}
		}

		#[test]
		fn should_return_zero_fee_when_fee_is_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 12;
			let a_sent = 2;
			let fee = Permill::zero();

			let res = compute_out_given_in_new(w_i, w_o, b_i, b_o, a_sent, fee)
				.expect("Valid input; QED");

			assert_eq!(res.1, 0);
		}

		#[test]
		fn should_return_error_if_w_o_is_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::zero();
			let b_i = 12;
			let b_o = 12;
			let a_sent = 2;
			let fee = Permill::zero();

			let res = compute_out_given_in_new(w_i, w_o, b_i, b_o, a_sent, fee);

			assert_eq!(res, Err(sp_runtime::ArithmeticError::DivisionByZero));
		}

		#[test]
		fn should_return_error_if_b_i_and_a_sent_are_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 0;
			let b_o = 12;
			let a_sent = 0;
			let fee = Permill::zero();

			let res = compute_out_given_in_new(w_i, w_o, b_i, b_o, a_sent, fee);

			assert_eq!(res, Err(sp_runtime::ArithmeticError::DivisionByZero));
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(CHECKED_I_AND_O_SIZE))]

			#[test]
			fn should_pass_with_expected_values(i_and_o in checked_inputs_and_outputs()) {
				let w_i = i_and_o.0;
				let w_o = i_and_o.1;
				let b_i = i_and_o.2;
				let b_o = i_and_o.3;
				let a_sent = i_and_o.4;
				let fee = i_and_o.5;
				let a_out = i_and_o.6;
				let fee_out = i_and_o.7;

				let res = compute_out_given_in_new(w_i, w_o, b_i, b_o, a_sent, fee)
					.expect("Valid input; QED");

				prop_assert_eq!(res.0, a_out);
				prop_assert_eq!(res.1, fee_out);
			}
		}
	}
}
