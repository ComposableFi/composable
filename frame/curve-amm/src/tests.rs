use sp_runtime::{
	traits::{Saturating, Zero},
	FixedPointNumber, FixedU128,
};
use sp_std::cmp::Ordering;
// use composable_traits::dex::CurveAmm;
use crate::mock::CurveAmm;

#[test]
fn compute_d_works() {
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();
	let d = CurveAmm::get_d(&xp, ann);
	// expected d is 1.978195735374521596
	// expected precision is 1e-13
	let delta = d
		.map(|x| {
			x.saturating_sub(FixedU128::saturating_from_rational(
				1978195735374521596u128,
				10_000_000_000_000_000u128,
			))
			.saturating_abs()
		})
		.map(|x| x.cmp(&FixedU128::saturating_from_rational(1u128, 10_000_000_000_000u128)));
	assert_eq!(delta, Some(Ordering::Less));
}

#[test]
fn compute_d_empty() {
	let xp = vec![];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();
	let result = CurveAmm::get_d(&xp, ann);
	assert_eq!(result, Some(FixedU128::zero()));
}

#[test]
fn get_y_successful() {
	let i = 0;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);
	// expected y is 1.247108067356516682
	// expected precision is 1e-13
	let delta = result
		.map(|x| {
			x.saturating_sub(FixedU128::saturating_from_rational(
				1247108067356516682u128,
				10_000_000_000_000_000u128,
			))
			.saturating_abs()
		})
		.map(|x| x.cmp(&FixedU128::saturating_from_rational(1u128, 10_000_000_000_000u128)));
	assert_eq!(delta, Some(Ordering::Less));
}

#[test]
fn get_y_same_coin() {
	let i = 1;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

#[test]
fn get_y_i_greater_than_n() {
	let i = 33;
	let j = 1;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}

#[test]
fn get_y_j_greater_than_n() {
	let i = 1;
	let j = 33;
	let x = FixedU128::saturating_from_rational(111u128, 100u128);
	let xp = vec![
		FixedU128::saturating_from_rational(11u128, 10u128),
		FixedU128::saturating_from_rational(88u128, 100u128),
	];
	let amp = FixedU128::saturating_from_rational(292u128, 100u128);
	let ann = CurveAmm::get_ann(amp, xp.len()).unwrap();

	let result = CurveAmm::get_y(i, j, x, &xp, ann);

	assert_eq!(result, None);
}
