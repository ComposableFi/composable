use super::prelude::*;
use crate::{
	helpers::{accrue_interest_internal, current_interest_rate},
	tests::new_jump_model,
	types::AccruedInterest,
};
use composable_traits::{defi::Rate, lending::math::InterestRate, time::SECONDS_PER_YEAR_NAIVE};
use sp_arithmetic::assert_eq_error_rate;

#[test]
fn current_interest_rate_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		// Create a market
		let ((market_id, _), _) = create_simple_vaulted_market(BTC::instance(), manager);

		assert_eq!(
			current_interest_rate::<Runtime>(market_id.0).unwrap(),
			FixedU128::saturating_from_rational(2_u128, 100_u128)
		);
	})
}

#[test]
fn apr_for_zero() {
	let (_, ref mut interest_rate_model) = new_jump_model();
	let utilization = Percent::from_percent(100);
	let borrow_index = Rate::saturating_from_integer(1_u128);

	let AccruedInterest { accrued_increment: accrued_increase, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			utilization,
			interest_rate_model,
			borrow_index,
			SECONDS_PER_YEAR_NAIVE,
			0,
		)
		.unwrap();
	assert_eq!(accrued_increase, 0);
}

#[test]
fn apr_for_year_for_max() {
	let (_, ref mut interest_rate_model) = new_jump_model();
	let utilization = Percent::from_percent(80);
	let borrow_index = Rate::saturating_from_integer(1_u128);
	let total_borrows = u128::MAX;
	let result = accrue_interest_internal::<Runtime, InterestRateModel>(
		utilization,
		interest_rate_model,
		borrow_index,
		SECONDS_PER_YEAR_NAIVE,
		total_borrows,
	);
	assert_err!(result, ArithmeticError::Overflow);
}

#[test]
fn accrue_interest_base_cases() {
	let (optimal, ref mut interest_rate_model) = new_jump_model();
	let stable_rate = interest_rate_model.get_borrow_rate(optimal).unwrap();
	assert_eq!(stable_rate, ZeroToOneFixedU128::saturating_from_rational(10_u128, 100_u128));
	let borrow_index = Rate::saturating_from_integer(1_u128);
	let delta_time = SECONDS_PER_YEAR_NAIVE;
	let total_issued = 100_000_000_000_000_000_000;
	let accrued_debt = 0;
	let total_borrows = total_issued - accrued_debt;
	let AccruedInterest { accrued_increment: accrued_increase, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			optimal,
			interest_rate_model,
			borrow_index,
			delta_time,
			total_borrows,
		)
		.unwrap();
	assert_eq!(accrued_increase, 10_000_000_000_000_000_000);

	let delta_time = MILLISECS_PER_BLOCK;
	let AccruedInterest { accrued_increment: accrued_increase, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			optimal,
			interest_rate_model,
			borrow_index,
			delta_time,
			total_borrows,
		)
		.unwrap();
	// small increments instead one year lead to some loss by design (until we lift calculation to
	// 256 bit)
	let error = 25;
	assert_eq!(
		accrued_increase,
		10_000_000_000_000_000_000 * MILLISECS_PER_BLOCK as u128 / SECONDS_PER_YEAR_NAIVE as u128 +
			error
	);
}

#[test]
fn accrue_interest_induction() {
	let borrow_index = Rate::saturating_from_integer(1_u128);
	let minimal: u128 = 100;
	let mut runner = TestRunner::default();
	let accrued_debt: u128 = 0;
	runner
		.run(
			&(
				0..=2 * SECONDS_PER_YEAR_NAIVE / MILLISECS_PER_BLOCK,
				(minimal..=minimal * 1_000_000_000),
			),
			|(slot, total_issued)| {
				let (optimal, ref mut interest_rate_model) = new_jump_model();

				let AccruedInterest {
					accrued_increment: accrued_increase_1,
					new_borrow_index: borrow_index_1,
				} = accrue_interest_internal::<Runtime, InterestRateModel>(
					optimal,
					interest_rate_model,
					borrow_index,
					slot * MILLISECS_PER_BLOCK,
					total_issued - accrued_debt,
				)
				.unwrap();

				let AccruedInterest {
					accrued_increment: accrued_increase_2,
					new_borrow_index: borrow_index_2,
				} = accrue_interest_internal::<Runtime, InterestRateModel>(
					optimal,
					interest_rate_model,
					borrow_index,
					(slot + 1) * MILLISECS_PER_BLOCK,
					total_issued - accrued_debt,
				)
				.unwrap();
				prop_assert!(accrued_increase_1 < accrued_increase_2);
				prop_assert!(borrow_index_1 < borrow_index_2);
				Ok(())
			},
		)
		.unwrap();
}

#[test]
fn accrue_interest_plotter() {
	let (optimal, ref mut interest_rate_model) = new_jump_model();
	let borrow_index = MoreThanOneFixedU128::checked_from_integer::<u128>(1).unwrap();
	let total_issued = 10_000_000_000;
	let accrued_debt = 0;
	let total_borrows = total_issued - accrued_debt;
	// no sure how handle in rust previous + next (so map has access to previous result)
	let mut previous = 0;
	const TOTAL_BLOCKS: u64 = 1000;
	let _data: Vec<_> = (0..TOTAL_BLOCKS)
		.map(|x| {
			let AccruedInterest { accrued_increment, .. } =
				accrue_interest_internal::<Runtime, InterestRateModel>(
					optimal,
					interest_rate_model,
					borrow_index,
					MILLISECS_PER_BLOCK,
					total_borrows,
				)
				.unwrap();
			previous += accrued_increment;
			(x, previous)
		})
		.collect();

	let AccruedInterest { accrued_increment: total_accrued, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			optimal,
			interest_rate_model,
			Rate::checked_from_integer::<u128>(1).unwrap(),
			TOTAL_BLOCKS * MILLISECS_PER_BLOCK,
			total_borrows,
		)
		.unwrap();
	assert_eq_error_rate!(previous, total_accrued, 1_000);

	#[cfg(feature = "visualization")]
	{
		let area =
			BitMapBackend::new("./accrue_interest_plotter.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.set_label_area_size(LabelAreaPosition::Left, 80)
			.set_label_area_size(LabelAreaPosition::Bottom, 80)
			.build_cartesian_2d(
				0.0..1100.0,
				total_issued as f64..(total_issued as f64 + 1.1 * total_accrued as f64),
			)
			.unwrap();

		chart.configure_mesh().draw().unwrap();
		chart
			.draw_series(LineSeries::new(
				_data.iter().map(|(x, y)| (*x as f64, total_issued as f64 + *y as f64)),
				&RED,
			))
			.unwrap();
	}
}
