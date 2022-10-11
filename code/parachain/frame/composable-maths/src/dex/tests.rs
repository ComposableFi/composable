use crate::dex::constant_product::compute_in_given_out_new;
use proptest::prelude::*;
use sp_runtime::Permill;

/// Tests related to constant product math functions
mod constant_product {
	use super::*;
	/// Tests related to the function `compute_in_given_out_new`
	mod compute_in_given_out_new {
		use super::*;

		/// should always be equal to the number of elements in the `list` array found in
		/// `checked_inputs_and_outputs` function
		const CHECKED_I_AND_O_SIZE: u32 = 5;

		prop_compose! {
			/// Returns (w_i, w_o, b_i, b_o, a_out, f, a_sent, fee)
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
						116,
						11,
					),
					(
						Permill::from_rational::<u32>(1, 3),
						Permill::from_rational::<u32>(2, 3),
						1024,
						2048,
						100,
						Permill::from_percent(10),
						119,
						11,
					),
					(
						Permill::from_rational::<u32>(1, 4),
						Permill::from_rational::<u32>(3, 4),
						10_000,
						30_000,
						100,
						Permill::from_percent(10),
						111,
						11,
					),
					(
						Permill::from_rational::<u32>(2, 5),
						Permill::from_rational::<u32>(3, 5),
						20_000_000,
						30_000_000,
						100_000,
						Permill::from_percent(10),
						111_575,
						11_157,
					),
					(
						Permill::from_rational::<u32>(2, 5),
						Permill::from_rational::<u32>(1, 5),
						20_000_000,
						10_000_000,
						100_000,
						Permill::from_percent(10),
						111_951,
						11_195,
					),
				];

				list[x as usize]
			}
		}

		#[test]
		fn should_return_zero_fee_when_f_is_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 12;
			let a_out = 2;
			let f = Permill::zero();

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f)
				.expect("Input is valid; QED");

			assert_eq!(res.1, 0);
		}

		#[test]
		fn should_return_error_when_w_i_is_zero() {
			let w_i = Permill::zero();
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 12;
			let a_out = 2;
			let f = Permill::from_percent(10);

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f);

			assert_eq!(res, Err(sp_runtime::ArithmeticError::DivisionByZero))
		}

		#[test]
		fn should_return_error_when_b_o_and_a_out_are_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 0;
			let a_out = 0;
			let f = Permill::from_percent(10);

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f);

			assert_eq!(res, Err(sp_runtime::ArithmeticError::DivisionByZero))
		}

		#[test]
		fn should_return_error_when_a_out_is_greater_than_b_o() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 512;
			let b_o = 128;
			let a_out = 256;
			let f = Permill::from_percent(10);

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f);

			// REVIEW: Should a more informative error be returned in this case?
			assert_eq!(res, Err(sp_runtime::ArithmeticError::Overflow))
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(CHECKED_I_AND_O_SIZE))]

			#[test]
			fn should_pass_with_expected_values(i_and_o in checked_inputs_and_outputs()) {
				let w_i = i_and_o.0;
				let w_o = i_and_o.1;
				let b_i = i_and_o.2;
				let b_o = i_and_o.3;
				let a_out = i_and_o.4;
				let f = i_and_o.5;
				let a_sent = i_and_o.6;
				let fee = i_and_o.7;

				let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f).expect("Input is valid; QED");

				prop_assert_eq!(res.0, a_sent);
				prop_assert_eq!(res.1, fee);
			}
		}
	}
}
