/// Equivalent of assert_ok when inside a proptest context.
#[macro_export]
macro_rules! prop_assert_ok {
    ($cond:expr) => {
        prop_assert_ok!($cond, concat!("assertion failed: ", stringify!($cond)))
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

/// Accept a `dust` deviation.
#[macro_export]
macro_rules! prop_assert_acceptable_computation_error {
	($x:expr, $y:expr, $precision:expr) => {{
		match composable_helpers::test::helper::acceptable_computation_error($x, $y, $precision) {
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
			composable_helpers::test::helper::DEFAULT_ACCEPTABLE_DEVIATION
		);
	}};
}
