use std::ops::Sub;

use crate::labs::twap::Twap;
use frame_support::assert_ok;
use plotters::prelude::*;
use polars::prelude::*;
use proptest::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use rstest::rstest;
use sp_runtime::{offchain::Timestamp, FixedPointNumber, FixedU128};

// -------------------------------------------------------------------------------------------------
//                                             Constants
// -------------------------------------------------------------------------------------------------

const SECOND: u64 = 1000; // 1 second, in millis
const MINUTE: u64 = 60 * SECOND;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;

const PERIOD: u64 = 1 * HOUR; // Default period for twap

// -------------------------------------------------------------------------------------------------
//                                         Helper Functions
// -------------------------------------------------------------------------------------------------

fn from_float(x: f64) -> FixedU128 {
	FixedU128::from_float(x)
}

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[rstest]
#[case(u128::MIN, u64::MIN, PERIOD)]
#[case(u128::MAX, u64::MAX, PERIOD)]
#[case(u128::MIN, u64::MAX, PERIOD)]
#[case(u128::MAX, u64::MIN, PERIOD)]
#[case(0, 0, PERIOD)]
fn should_create_twap_struct_successfully(
	#[case] twap: u128,
	#[case] ts: u64,
	#[case] period: u64,
) {
	let twap = FixedU128::from_inner(twap);
	let t = Twap::new(twap, ts, period);
	assert_eq!(t.twap, twap);
	assert_eq!(t.ts, ts);
	assert_eq!(t.period, period);
}

#[test]
fn should_update_twap_to_correct_value() {
	// Initialize twap to 100,
	// Set timestamp to "Mon Aug  8 11:06:40 PM UTC 2022"
	let ts = 1660000000;
	let mut t = Twap::new(from_float(100.0), ts, PERIOD);

	// After half PERDIOD passes, we update the twap.
	t.accumulate(&from_float(200.0), &(PERIOD / 2));

	// The value should be half the previous price and half the new one.
	assert_eq!(t.twap, from_float(150.0));
}

#[test]
fn should_update_twap_on_accumulate_call() {
	let mut t = Twap::new(from_float(25.0), 0, PERIOD);
	assert_ok!(t.accumulate(&from_float(50.0), &(PERIOD / 2)));
}

// #[rstest]
// #[case(0.1)]
// #[case(0.2)]
// #[case(0.3)]
// #[case(0.4)]
// #[case(0.5)]
// #[case(0.6)]
// #[case(0.7)]
// #[case(0.8)]
// #[case(0.9)]
// #[case(1.)]
// #[case(2.)]
// #[case(3.)]
// #[case(4.)]
// #[case(5.)]
// #[case(6.)]
// #[case(7.)]
// #[case(8.)]
// #[case(9.)]
// #[case(10.)]
// #[case(20.)]
// #[case(30.)]
// #[case(40.)]
// #[case(50.)]
// #[case(60.)]
// #[case(70.)]
// #[case(80.)]
// #[case(90.)]
// #[case(100.)]
// fn should_create_graph(#[case] divisor: f64) {
// 	let p = 3;
// 	let points = 10_f64.powf(p as f64);
// 	let mut v: Vec<f64> = (0..=points as u64).into_iter().map(|x| x.pow(2) as f64).collect();
// 	let ts_p = (PERIOD as f64 / divisor) as u64;
// 	let mut period_bar: Vec<usize> = vec![];
// 	let a: Vec<f64> = v
// 		.iter()
// 		.map(|x| {
// 			let i = x * 2f64 / 10f64;
// 			// let j = rand::thread_rng().gen_range(0..=i as u64) as f64;
// 			let j = Pcg64::seed_from_u64(42).gen_range(0..=i as u64) as f64;
// 			if j % 2.0 == 0.0 {
// 				x + j
// 			} else {
// 				x - j
// 			}
// 		})
// 		.collect();
// 	dbg!(period_bar);

// 	let mut ts = 0u64;
// 	let mut vts = vec![];
// 	let mut t = Twap::new(from_float(0.), ts, PERIOD);
// 	let mut g = 0;
// 	let h: Vec<(FixedU128, _)> = a
// 		.iter()
// 		.map(|x| {
// 			let x = from_float(*x);
// 			ts += ts_p;
// 			t.accumulate(&x, &ts);
// 			// if 0
// 			vts.push(ts);
// 			(x, t.twap)
// 		})
// 		.collect();

// 	dbg!(&ts_p, PERIOD, vts);
// 	// dbg!(&h);

// 	let file_name = format!("period_{}.png", divisor).to_string();
// 	let root = BitMapBackend::new(&file_name, (3840, 2160)).into_drawing_area();
// 	root.fill(&WHITE);
// 	let root = root.margin(10, 10, 10, 10);
// 	// After this point, we should be able to draw construct a chart context
// 	let mut chart = ChartBuilder::on(&root)
// 		// Set the caption of the chart
// 		.caption(
// 			format!(
// 				"Period: {}%, {}, {}, {}",
// 				PERIOD.max(ts_p).sub(PERIOD.min(ts_p)) as f64 / PERIOD as f64 * 100f64,
// 				&divisor,
// 				&ts_p,
// 				PERIOD
// 			),
// 			("sans-serif", 40).into_font(),
// 		)
// 		// Set the size of the label region
// 		.x_label_area_size(20)
// 		.y_label_area_size(40)
// 		// Finally attach a coordinate on the drawing area and make a chart context
// 		.build_cartesian_2d(
// 			0..points as usize,
// 			from_float(0.0).into_inner()..from_float(1000000.0).into_inner(),
// 		)
// 		.unwrap();

// 	// Then we can draw a mesh
// 	chart
// 		.configure_mesh()
// 		// We can customize the maximum number of labels allowed for each axis
// 		// .x_labels(5)
// 		// .y_labels(5)
// 		// We can also change the format of the label text
// 		// .y_label_formatter(&|x| format!("{:.3}", x))
// 		.draw()
// 		.unwrap();

// 	// And we can draw something in the drawing area
// 	chart
// 		// .draw_series(LineSeries::new(a.iter().enumerate().map(|(a, b)| (a, *b)), &RED))
// 		.draw_series(LineSeries::new(
// 			h.iter().enumerate().map(|(i, (a, b))| (i, a.into_inner())),
// 			&RED,
// 		))
// 		.unwrap();
// 	chart
// 		.draw_series(LineSeries::new(
// 			h.iter().enumerate().map(|(i, (a, b))| (i, b.into_inner())),
// 			&BLUE,
// 		))
// 		.unwrap();
// 	// Similarly, we can draw point series
// 	// chart.draw_series(PointSeries::of_element(
// 	// 	vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
// 	// 	5,
// 	// 	&RED,
// 	// 	&|c, s, st| {
// 	// 		return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
// 	// 			+ Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
// 	// 			+ Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
// 	// 	},
// 	// ))?;
// 	root.present().unwrap();
// 	// dbg!(v, j, a);
// }

#[test]
fn should_create_smooth_graph() {
	let mut df = CsvReader::from_path("final.csv").unwrap().has_header(true).finish().unwrap();
	df.try_apply("ts", |s| {
		s.utf8()
			.unwrap()
			.as_datetime(Some("%Y-%m-%d %T%z"), TimeUnit::Nanoseconds)
			.map(|ca| ca.into_series())
	})
	.unwrap();
	dbg!(&df.get_column_names());
}
