use num_traits::One;

use super::Validate;

/// >= 1
pub struct GeOne;

impl<T: One + PartialOrd> Validate<T, GeOne> for GeOne {
	fn validate(input: T) -> Result<T, &'static str> {
		if input >= T::one() {
			Ok(input)
		} else {
			Err("Input value was < 1")
		}
	}
}

#[test]
fn test_ge_one() {
	use crate::validation::Validate;
	use frame_support::{assert_err, assert_ok};

	// unsigned
	assert_ok!(GeOne::validate(1_u32), 1);
	assert_ok!(GeOne::validate(2_u32), 2);
	assert_ok!(GeOne::validate(u32::MAX), u32::MAX);
	assert_err!(GeOne::validate(0_u32), "Input value was < 1");

	// signed
	assert_ok!(GeOne::validate(1_i32), 1);
	assert_ok!(GeOne::validate(2_i32), 2);
	assert_err!(GeOne::validate(-1_i32), "Input value was < 1");
	assert_err!(GeOne::validate(0_i32), "Input value was < 1");
	assert_ok!(GeOne::validate(i32::MAX), i32::MAX);
	assert_err!(GeOne::validate(i32::MIN), "Input value was < 1");

	// floats
	assert_ok!(GeOne::validate(1.0_f32), 1.0);
	assert_ok!(GeOne::validate(1.1_f32), 1.1);
	assert_err!(GeOne::validate(0.9_f32), "Input value was < 1");
	assert_ok!(GeOne::validate(2.0_f32), 2.0);
	assert_err!(GeOne::validate(-1.0_f32), "Input value was < 1");
	assert_err!(GeOne::validate(0.0_f32), "Input value was < 1");
	assert_ok!(GeOne::validate(f32::MAX), f32::MAX);
	assert_err!(GeOne::validate(f32::MIN), "Input value was < 1");
}
