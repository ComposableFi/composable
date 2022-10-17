use crate::defi::{Rate, ZeroToOneFixedU128};

use super::*;
use proptest::{prop_assert, strategy::Strategy, test_runner::TestRunner};
use sp_runtime::{
	traits::{One, Saturating, Zero},
	FixedPointNumber, FixedU128,
};

// Test jump model
#[test]
fn init_jump_model_works() {
	let base_rate = Rate::saturating_from_rational(2, 100);
	let jump_rate = Rate::saturating_from_rational(10, 100);
	let full_rate = Rate::saturating_from_rational(32, 100);
	let target_utilization = Percent::from_percent(80);
	assert_eq!(
		JumpModel::new(base_rate, jump_rate, full_rate, target_utilization).unwrap(),
		JumpModel {
			base_rate: Rate::from_inner(20_000_000_000_000_000),
			jump_rate: Rate::from_inner(100_000_000_000_000_000),
			full_rate: Rate::from_inner(320_000_000_000_000_000),
			target_utilization: Percent::from_percent(80),
		}
	);
}

#[test]
fn get_borrow_rate_works() {
	// init
	let base_rate = Rate::saturating_from_rational(2, 100);
	let jump_rate = Rate::saturating_from_rational(10, 100);
	let full_rate = Rate::saturating_from_rational(32, 100);
	let target_utilization = Percent::from_percent(80);
	let mut jump_model =
		JumpModel::new(base_rate, jump_rate, full_rate, target_utilization).unwrap();
	// normal rate
	let mut cash: u128 = 500;
	let borrows: u128 = 1000;
	let utilization = Percent::from_rational(borrows, cash + borrows);
	let borrow_rate = jump_model.get_borrow_rate(utilization).unwrap();
	assert_eq!(
		borrow_rate,
		jump_model.jump_rate.saturating_mul(utilization.into()) + jump_model.base_rate,
	);

	// jump rate
	cash = 100;
	let utilization = Percent::from_rational(borrows, cash + borrows);
	let borrow_rate = jump_model.get_borrow_rate(utilization).unwrap();
	let normal_rate =
		jump_model.jump_rate.saturating_mul(target_utilization.into()) + jump_model.base_rate;
	let excess_util = utilization.saturating_sub(target_utilization);
	assert_eq!(
		borrow_rate,
		(jump_model.full_rate - jump_model.jump_rate).saturating_mul(excess_util.into()) /
			FixedU128::saturating_from_rational(20, 100) +
			normal_rate,
	);
}

#[test]
fn get_supply_rate_works() {
	let borrow_rate = Rate::saturating_from_rational(2, 100);
	let util = ZeroToOneFixedU128::saturating_from_rational(50, 100);
	let reserve_factor = ZeroToOneFixedU128::zero();
	let supply_rate = InterestRateModel::get_supply_rate(borrow_rate, util, reserve_factor);
	assert_eq!(
		supply_rate,
		borrow_rate.saturating_mul(ZeroToOneFixedU128::one().saturating_sub(reserve_factor) * util),
	);
}

#[test]
fn curve_model_correctly_calculates_borrow_rate() {
	let mut model = CurveModel::new(Rate::saturating_from_rational(2, 100)).unwrap();
	assert_eq!(
		model.get_borrow_rate(Percent::from_percent(80)).unwrap(),
		// curve model has arbitrary power parameters leading to changes in precision of high
		// power
		Rate::from_inner(Rate::DIV / 100 * 14)
	);
}

#[derive(Debug, Clone)]
struct JumpModelStrategy {
	pub base_rate: ZeroToOneFixedU128,
	pub jump_percentage: ZeroToOneFixedU128,
	pub full_percentage: ZeroToOneFixedU128,
	pub target_utilization: Percent,
}

fn valid_jump_model() -> impl Strategy<Value = JumpModelStrategy> {
	(
		(1..=10_u32).prop_map(|x| ZeroToOneFixedU128::saturating_from_rational(x, 100)),
		(11..=30_u32).prop_map(|x| ZeroToOneFixedU128::saturating_from_rational(x, 100)),
		(31..=50).prop_map(|x| ZeroToOneFixedU128::saturating_from_rational(x, 100)),
		(0..=100_u8).prop_map(Percent::from_percent),
	)
		.prop_filter("Jump rate model", |(base, jump, full, _)| {
			// tried high order strategy - failed as it tries to combine collections with not
			// collection alternative to define arbitrary and proptest attributes with filtering
			// overall cardinality is small, so should work well
			// here we have one liner, not sure why in code we have many lines....
			base <= jump &&
				jump <= full && base <= &JumpModel::MAX_BASE_RATE &&
				jump <= &JumpModel::MAX_JUMP_RATE &&
				full <= &JumpModel::MAX_FULL_RATE
		})
		.prop_map(|(base_rate, jump_percentage, full_percentage, target_utilization)| {
			JumpModelStrategy { base_rate, full_percentage, jump_percentage, target_utilization }
		})
}

#[test]
fn test_empty_drained_market() {
	let mut jump_model = JumpModel::new(
		FixedU128::from_float(0.01),
		FixedU128::from_float(0.11),
		FixedU128::from_float(0.31),
		Percent::zero(),
	)
	.unwrap();
	let borrow_rate = jump_model
		.get_borrow_rate(Percent::zero())
		.expect("borrow rate must be defined");

	assert_eq!(borrow_rate, jump_model.jump_rate);
}

#[test]
fn test_slope() {
	let mut jump_model = JumpModel::new(
		FixedU128::from_float(0.01),
		FixedU128::from_float(0.11),
		FixedU128::from_float(0.31),
		Percent::from_percent(80),
	)
	.unwrap();

	let x1 = 70;
	let x2 = 75;
	let y1 = jump_model.get_borrow_rate(Percent::from_percent(x1)).unwrap();
	let y2 = jump_model.get_borrow_rate(Percent::from_percent(x2)).unwrap();
	let s1 = (y2 - y1) /
		(FixedU128::saturating_from_integer(x2) - FixedU128::saturating_from_integer(x1));

	let x1 = 81;
	let x2 = 86;
	let y1 = jump_model.get_borrow_rate(Percent::from_percent(x1)).unwrap();
	let y2 = jump_model.get_borrow_rate(Percent::from_percent(x2)).unwrap();
	let s2 = (y2 - y1) /
		(FixedU128::saturating_from_integer(x2) - FixedU128::saturating_from_integer(x1));

	assert!(s1 < s2, "slope after target is growing faster")
}

#[test]
fn proptest_jump_model() {
	let mut runner = TestRunner::default();
	runner
		.run(&(valid_jump_model(), 0..=100_u8), |(strategy, utilization)| {
			let base_rate = strategy.base_rate;
			let jump_rate = strategy.jump_percentage;
			let full_rate = strategy.full_percentage;
			let target_utilization = strategy.target_utilization;
			let mut jump_model =
				JumpModel::new(base_rate, jump_rate, full_rate, target_utilization).unwrap();

			let utilization = Percent::from_percent(utilization);
			let borrow_rate =
				jump_model.get_borrow_rate(utilization).expect("borrow rate must be defined");
			prop_assert!(borrow_rate > Rate::zero());
			Ok(())
		})
		.unwrap();
}

#[test]
fn proptest_jump_model_rate() {
	let base_rate = Rate::saturating_from_rational(2, 100);
	let jump_rate = Rate::saturating_from_rational(10, 100);
	let full_rate = Rate::saturating_from_rational(32, 100);
	let strategy = (0..=100_u8, 1..=99_u8)
		.prop_map(|(optimal, utilization)| (optimal, utilization, utilization + 1));

	let mut runner = TestRunner::default();
	runner
		.run(&strategy, |(optimal, previous, next)| {
			let utilization_1 = Percent::from_percent(previous);
			let utilization_2 = Percent::from_percent(next);
			let optimal = Percent::from_percent(optimal);
			let mut model = JumpModel::new(base_rate, jump_rate, full_rate, optimal)
				.expect("model should be defined");
			let rate_1 = model.get_borrow_rate(utilization_1);
			let rate_2 = model.get_borrow_rate(utilization_2);
			if optimal < Percent::from_percent(100) {
				prop_assert!(rate_1 < rate_2);
			}
			Ok(())
		})
		.unwrap();
}

#[cfg(feature = "visualization")]
#[test]
fn jump_model_plotter() {
	use plotters::prelude::*;
	let base_rate = Rate::saturating_from_rational(2, 100);
	let jump_rate = Rate::saturating_from_rational(10, 100);
	let full_rate = Rate::saturating_from_rational(32, 100);
	let optimal = Percent::from_percent(80);
	let mut model = JumpModel::new(base_rate, jump_rate, full_rate, optimal).unwrap();

	let area = BitMapBackend::new("./jump_model_plotter.png", (1024, 768)).into_drawing_area();
	area.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&area)
		.set_label_area_size(LabelAreaPosition::Left, 50)
		.set_label_area_size(LabelAreaPosition::Bottom, 50)
		.build_cartesian_2d(0.0..100.0, 0.0..100.0)
		.unwrap();
	chart
		.configure_mesh()
		.x_desc("Utilization ratio %")
		.y_desc("Borrow rate %")
		.draw()
		.unwrap();
	chart
		.draw_series(LineSeries::new(
			(0..=100).map(|x| {
				let utilization = Percent::from_percent(x);
				let rate = model.get_borrow_rate(utilization).unwrap();
				(x as f64, rate.to_float() * 100.0)
			}),
			&RED,
		))
		.unwrap();
}

#[cfg(feature = "visualization")]
#[test]
fn curve_model_plotter() {
	use plotters::prelude::*;
	let base_rate = Rate::saturating_from_rational(3, 100);
	let mut model = CurveModel::new(base_rate).unwrap();

	let area = BitMapBackend::new("./curve_model_plotter.png", (1024, 768)).into_drawing_area();
	area.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&area)
		.set_label_area_size(LabelAreaPosition::Left, 50)
		.set_label_area_size(LabelAreaPosition::Bottom, 50)
		.build_cartesian_2d(0.0..100.0, 0.0..100.0)
		.unwrap();
	chart
		.configure_mesh()
		.x_desc("Utilization ratio %")
		.y_desc("Borrow rate %")
		.draw()
		.unwrap();
	chart
		.draw_series(LineSeries::new(
			(0..=100).map(|x| {
				let utilization = Percent::from_percent(x);
				let rate = model.get_borrow_rate(utilization).unwrap();
				(x as f64, rate.to_float() * 100.0)
			}),
			&RED,
		))
		.unwrap();
}

// ISSUE: given ideal real interest rate, borrow rate growth infinitely, which is wrong
#[cfg(feature = "visualization")]
#[test]
fn dynamic_pid_model_plotter() {
	use plotters::prelude::*;
	use sp_runtime::FixedI128;
	let proportional_parameter = FixedI128::saturating_from_rational(40, 100);
	let integral_parameter = FixedI128::saturating_from_rational(50, 100);
	let derivative_parameter = FixedI128::saturating_from_rational(30, 100);
	let target_utilization = FixedU128::saturating_from_rational(80, 100);
	let initial_interest_rate = FixedU128::saturating_from_rational(13, 100);
	let mut model = DynamicPIDControllerModel::new(
		proportional_parameter,
		integral_parameter,
		derivative_parameter,
		initial_interest_rate,
		target_utilization,
	)
	.unwrap();

	let area =
		BitMapBackend::new("./dynamic_pid_model_plotter.png", (1024, 768)).into_drawing_area();
	area.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&area)
		.set_label_area_size(LabelAreaPosition::Left, 50)
		.set_label_area_size(LabelAreaPosition::Bottom, 50)
		.set_label_area_size(LabelAreaPosition::Right, 100)
		.build_cartesian_2d(0.0..100.0, 0.0..150.0)
		.unwrap();
	chart.configure_mesh().x_desc("Time").y_desc("%").draw().unwrap();

	chart
		.draw_series(LineSeries::new(
			[
				0, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
				80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
				80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
				80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
			]
			.iter()
			.enumerate()
			.map(|(i, x)| {
				let utilization = Percent::from_percent(*x);
				let rate = model.get_borrow_rate(utilization).unwrap();
				(i as f64, rate.to_float())
			}),
			&RED,
		))
		.unwrap()
		.label("Interest rate %");

	chart
		.draw_series(LineSeries::new(
			[
				80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
				80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
				80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
				80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
			]
			.iter()
			.enumerate()
			.map(|(i, x)| (i as f64, *x as f64)),
			&BLUE,
		))
		.unwrap()
		.label("Target Utilization ratio %");

	chart.configure_series_labels().border_style(&BLACK).draw().unwrap();
}

#[cfg(feature = "visualization")]
#[test]
fn double_exponents_model_plotter() {
	use plotters::prelude::*;
	let coefficients: [u8; 16] = [10, 10, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
	let mut model = DoubleExponentModel::new(coefficients).unwrap();
	let area =
		BitMapBackend::new("./double_exponents_model_plotter.png", (1024, 768)).into_drawing_area();
	area.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&area)
		.set_label_area_size(LabelAreaPosition::Left, 50)
		.set_label_area_size(LabelAreaPosition::Bottom, 50)
		.build_cartesian_2d(0.0..100.0, 0.0..100.0)
		.unwrap();
	chart
		.configure_mesh()
		.x_desc("Utilization ratio %")
		.y_desc("Borrow rate %")
		.draw()
		.unwrap();
	chart
		.draw_series(LineSeries::new(
			(1..=100).map(|x| {
				let utilization = Percent::from_percent(x);
				let rate = model.get_borrow_rate(utilization).unwrap();
				(x as f64, rate.to_float() * 100.0)
			}),
			&RED,
		))
		.unwrap();
}
