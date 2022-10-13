use crate::dex::constant_product::*;
use proptest::prelude::*;
use rust_decimal::prelude::*;
use sp_runtime::{ArithmeticError, Permill};

/// Tests related to constant product math functions
mod constant_product {
	use super::*;

	/// Tests related to the function `compute_in_given_out_new`
	mod compute_in_given_out_new {

		use super::*;

		#[derive(Debug, Eq, PartialEq, Clone, Copy)]
		struct InputsAndOutputs {
			w_i: Permill,
			w_o: Permill,
			b_i: u128,
			b_o: u128,
			a_out: u128,
			f: Permill,
			a_sent: u128,
			fee: u128,
		}

		const CHECKED_I_AND_O_LIST: [InputsAndOutputs; 5] = [
			InputsAndOutputs {
				w_i: Permill::from_percent(50),
				w_o: Permill::from_percent(50),
				b_i: 2048,
				b_o: 2048,
				a_out: 100,
				f: Permill::from_percent(10),
				a_sent: 117,
				fee: 12,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(34),
				w_o: Permill::from_percent(66),
				b_i: 1024,
				b_o: 2048,
				a_out: 100,
				f: Permill::from_percent(10),
				a_sent: 117,
				fee: 12,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(25),
				w_o: Permill::from_percent(75),
				b_i: 10_000,
				b_o: 30_000,
				a_out: 100,
				f: Permill::from_percent(10),
				a_sent: 112,
				fee: 12,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(60),
				b_i: 20_000_000,
				b_o: 30_000_000,
				a_out: 100_000,
				f: Permill::from_percent(10),
				a_sent: 111_576,
				fee: 11_158,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(20),
				b_i: 20_000_000,
				b_o: 10_000_000,
				a_out: 100_000,
				f: Permill::from_percent(10),
				a_sent: 111_952,
				fee: 11_196,
			},
		];

		prop_compose! {
			fn checked_inputs_and_outputs()
			(x in 0..CHECKED_I_AND_O_LIST.len()) -> InputsAndOutputs {
				CHECKED_I_AND_O_LIST[x]
			}

		}

		/// `1_960_897_022_228_042_355_440_212_770_816 / 25` rounded down
		const SAFE_B_I_ASSUMING_1_PERCENT_FEE: u128 = 78_435_880_889_121_694_217_608_510_832;

		prop_compose! {
			#[allow(clippy::useless_conversion)]
			fn range_inputs()
			(
				w_i in 1..100_u32,
				w_o in 1..100_u32,
				b_i in 257_000_000_000_000..SAFE_B_I_ASSUMING_1_PERCENT_FEE,
				b_o in 257_000_000_000_000..Decimal::MAX.to_u128()
					.expect("Decimal::MAX is safe for into ops; QED"),
				a_out in 1_000_000_000_000..256_000_000_000_000_u128,
				f in 0..10_000_u32,
			)
			-> InputsAndOutputs {
				InputsAndOutputs {
					w_i: Permill::from_percent(w_i),
					w_o: Permill::from_percent(w_o),
					b_i,
					b_o,
					a_out,
					f: Permill::from_parts(f),
					a_sent: u128::default(),
					fee: u128::default(),
				}
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

			assert_eq!(res, Err(ConstantProductAmmError::from(ArithmeticError::DivisionByZero)));
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

			assert_eq!(res, Err(ConstantProductAmmError::from(ArithmeticError::DivisionByZero)));
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

			assert_eq!(res, Err(ConstantProductAmmError::CannotTakeMoreThanAvailable))
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(CHECKED_I_AND_O_LIST.len() as u32))]

			#[test]
			fn should_pass_with_expected_values(i_and_o in checked_inputs_and_outputs()) {
				let res = compute_in_given_out_new(
					i_and_o.w_i,
					i_and_o.w_o,
					i_and_o.b_i,
					i_and_o.b_o,
					i_and_o.a_out,
					i_and_o.f)
				.expect("Input is valid; QED");

				prop_assert_eq!(res.0, i_and_o.a_sent);
				prop_assert_eq!(res.1, i_and_o.fee);
			}
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(10_000))]

			#[test]
			fn no_unexpected_errors_in_range(i_and_o in range_inputs()) {
				let res = compute_in_given_out_new(
					i_and_o.w_i,
					i_and_o.w_o,
					i_and_o.b_i,
					i_and_o.b_o,
					i_and_o.a_out,
					i_and_o.f
				);

				prop_assert!(res.is_ok());
			}
		}
	}

	/// Tests related to the function `compute_out_given_in_new`
	mod compute_out_given_in_new {
		use super::*;

		const CHECKED_I_AND_O_LIST: [InputsAndOutputs; 5] = [
			InputsAndOutputs {
				w_i: Permill::from_percent(50),
				w_o: Permill::from_percent(50),
				b_i: 2048,
				b_o: 2048,
				a_sent: 100,
				f: Permill::from_percent(10),
				a_out: 86,
				fee: 10,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(34),
				w_o: Permill::from_percent(66),
				b_i: 1024,
				b_o: 2048,
				a_sent: 100,
				f: Permill::from_percent(10),
				a_out: 86,
				fee: 10,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(25),
				w_o: Permill::from_percent(75),
				b_i: 10_000,
				b_o: 30_000,
				a_sent: 100,
				f: Permill::from_percent(10),
				a_out: 89,
				fee: 10,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(60),
				b_i: 20_000_000,
				b_o: 30_000_000,
				a_sent: 100_000,
				f: Permill::from_percent(10),
				a_out: 89_663,
				fee: 10_000,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(20),
				b_i: 20_000_000,
				b_o: 10_000_000,
				a_sent: 100_000,
				f: Permill::from_percent(10),
				a_out: 89_396,
				fee: 10_000,
			},
		];

		#[derive(Debug, Eq, PartialEq, Clone, Copy)]
		struct InputsAndOutputs {
			w_i: Permill,
			w_o: Permill,
			b_i: u128,
			b_o: u128,
			a_sent: u128,
			f: Permill,
			a_out: u128,
			fee: u128,
		}

		prop_compose! {
			/// Returns (w_i, w_o, b_i, b_o, a_sent, fee, a_out, fee_out)
			fn checked_inputs_and_outputs()
			(x in 0..CHECKED_I_AND_O_LIST.len()) -> InputsAndOutputs {

				CHECKED_I_AND_O_LIST[x]
			}
		}

		prop_compose! {
			fn range_inputs()
			(
				w_i in 1..100_u32,
				w_o in 1..100_u32,
				b_i in 257_000_000_000_000..Decimal::MAX.to_u128()
					.expect("Decimal::MAX is safe for into ops; QED"),
				b_o in 257_000_000_000_000..Decimal::MAX.to_u128()
					.expect("Decimal::MAX is safe for into ops; QED"),
				a_sent in 1_000_000_000_000..256_000_000_000_000_u128,
				f in 0..10_000_u32,
			)
			-> InputsAndOutputs {
				InputsAndOutputs {
					w_i: Permill::from_percent(w_i),
					w_o: Permill::from_percent(w_o),
					b_i,
					b_o,
					a_sent,
					f: Permill::from_parts(f),
					a_out: 0, // Not used in range tests
					fee: 0, // Not used in range tests
				}
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
			#![proptest_config(ProptestConfig::with_cases(CHECKED_I_AND_O_LIST.len() as u32))]

			#[test]
			fn should_pass_with_expected_values(i_and_o in checked_inputs_and_outputs()) {
				let res = compute_out_given_in_new(
					i_and_o.w_i,
					i_and_o.w_o,
					i_and_o.b_i,
					i_and_o.b_o,
					i_and_o.a_sent,
					i_and_o.f
				)
				.expect("Valid input; QED");

				prop_assert_eq!(res.0, i_and_o.a_out);
				prop_assert_eq!(res.1, i_and_o.fee);
			}
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(10_000))]

			#[test]
			fn no_unexpected_errors_in_range(i_and_o in range_inputs()) {
				let res = compute_out_given_in_new(
					i_and_o.w_i,
					i_and_o.w_o,
					i_and_o.b_i,
					i_and_o.b_o,
					i_and_o.a_out,
					i_and_o.f
				);

				prop_assert!(res.is_ok());
			}
		}
	}
}
