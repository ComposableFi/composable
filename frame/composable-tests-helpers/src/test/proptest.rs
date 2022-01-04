/// Equivalent of assert_ok when inside a proptest context.
#[macro_export]
macro_rules! prop_assert_ok {
    ($cond:expr) => {
        prop_assert_ok!($cond, concat!("ok assertion failed: ", stringify!($cond)))
    };

    ($cond:expr, $($fmt:tt)*) => {
        if let Err(e) = $cond {
            let message = format!($($fmt)*);
            let message = format!("Expected Ok(_), got {:?}, {} at {}:{}", e, message, file!(), line!());
            return ::std::result::Result::Err(
                proptest::test_runner::TestCaseError::fail(message));
        }
    };
}

#[macro_export]
macro_rules! prop_assert_err {
    ($cond:expr, $err:expr) => {
        composable_tests_helpers::prop_assert_err!($cond, $err, concat!("error assertion failed: ", stringify!($cond)))
    };

    ($cond:expr, $err:expr, $($fmt:tt)*) => {

        match $cond {
            Result::Err(e) if e == $err.into() => {}
            v => {
                let message = format!($($fmt)*);
                let message = format!("Expected {:?}, got {:?}, {} at {}:{}", $err, v, message, file!(), line!());
                return ::std::result::Result::Err(
                    proptest::test_runner::TestCaseError::fail(message));
            }
        }
    };
}

#[macro_export]
macro_rules! prop_assert_noop {
	(
		$x:expr,
		$y:expr $(,)?
	) => {
		let h = frame_support::storage_root();
		composable_tests_helpers::prop_assert_err!($x, $y);
		proptest::prop_assert_eq!(h, frame_support::storage_root());
	};
}

/// Accept a `dust` deviation.
#[macro_export]
macro_rules! prop_assert_acceptable_computation_error {
	($x:expr, $y:expr, $precision:expr) => {{
		match composable_tests_helpers::test::helper::acceptable_computation_error(
			$x, $y, $precision,
		) {
			Ok(()) => {},
			Err(q) => {
				prop_assert!(false, "{} * {} / {} = {} != {}", $x, $precision, $y, q, $precision);
			},
		}
	}};
	($x:expr, $y:expr) => {{
		prop_assert_acceptable_computation_error!(
			$x,
			$y,
			composable_tests_helpers::test::helper::DEFAULT_ACCEPTABLE_DEVIATION
		);
	}};
}

/// Accepts -2, -1, 0, 1, 2
#[macro_export]
macro_rules! prop_assert_zero_epsilon {
	($x:expr) => {{
		let epsilon = 2;
		let upper = 0 + epsilon;
		let lower = 0;
		prop_assert!(upper >= $x && $x >= lower, "{} => {} >= {}", upper, $x, lower);
	}};
}

/// Accept a 'dust' deviation
#[macro_export]
macro_rules! prop_assert_epsilon {
	($x:expr, $y:expr) => {{
		let precision = 1000;
		let epsilon = 5;
		let upper = precision + epsilon;
		let lower = precision - epsilon;
		let q = multiply_by_rational($x, precision, $y).expect("qed;");
		prop_assert!(
			upper >= q && q >= lower,
			"({}) => {} >= {} * {} / {} >= {}",
			q,
			upper,
			$x,
			precision,
			$y,
			lower
		);
	}};
}
