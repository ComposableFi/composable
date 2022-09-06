use std::{any::Any, fs::File, ops::Sub};

use crate::labs::twap::Twap;
use frame_support::assert_ok;
use num_traits::{PrimInt, SaturatingMul};
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

const PERIOD: u64 = 1 * DAY; // Default period for twap

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

// #[test] fn should_create_smooth_graph() {
// 	// Read historical data lazily, converting "timestamp (ts)" from a string to
// 	// a timestamp, and selecting only the columns "timestamp" and "mark price"
// 	// where the market index is equal to 0.
// 	let mut df = CsvReader::from_path("final.csv").unwrap().has_header(true).finish().unwrap();
// 	df.try_apply("ts", |s| {
// 		s.utf8()
// 			.unwrap()
// 			.as_datetime(Some("%Y-%m-%d %T%z"), TimeUnit::Milliseconds)
// 			.map(|ca| ca.into_series())
// 			.unwrap()
// 			.timestamp(TimeUnit::Milliseconds)
// 			.map(|x| x / 1000)
// 	})
// 	.unwrap();

// 	// TODO(Cardosaum): Try the ewma from polars.
// 	//
// 	// df.iter().for_each(|x| {
// 	// 	dbg!(x);
// 	// });
// 	// let ts = df["ts"].i64().unwrap().into_no_null_iter().collect();
// 	// let price = df["ts"].i64().unwrap().into_no_null_iter().collect();
// 	// dbg!(&df.filter(
// 	// 	&df["market_index"]
// 	// 		.iter()
// 	// 		.map(|i| Some(i == AnyValue::Int64(0)))
// 	// 		.collect::<BooleanArray>()
// 	// ));
// }

#[rstest]
#[case(1 * HOUR, 0)]
#[case(2 * HOUR, 0)]
#[case(3 * HOUR, 0)]
#[case(4 * HOUR, 0)]
#[case(5 * HOUR, 0)]
#[case(6 * HOUR, 0)]
#[case(7 * HOUR, 0)]
#[case(8 * HOUR, 0)]
#[case(9 * HOUR, 0)]
#[case(10 * HOUR, 0)]
#[case(11 * HOUR, 0)]
#[case(12 * HOUR, 0)]
#[case(1 * DAY, 0)]
#[case(2 * DAY, 0)]
#[case(3 * DAY, 0)]
#[case(4 * DAY, 0)]
#[case(5 * DAY, 0)]
#[case(6 * DAY, 0)]
#[case(7 * DAY, 0)]
#[case(1 * HOUR, 1)]
#[case(2 * HOUR, 1)]
#[case(3 * HOUR, 1)]
#[case(4 * HOUR, 1)]
#[case(5 * HOUR, 1)]
#[case(6 * HOUR, 1)]
#[case(7 * HOUR, 1)]
#[case(8 * HOUR, 1)]
#[case(9 * HOUR, 1)]
#[case(10 * HOUR, 1)]
#[case(11 * HOUR, 1)]
#[case(12 * HOUR, 1)]
#[case(1 * DAY, 1)]
#[case(2 * DAY, 1)]
#[case(3 * DAY, 1)]
#[case(4 * DAY, 1)]
#[case(5 * DAY, 1)]
#[case(6 * DAY, 1)]
#[case(7 * DAY, 1)]
#[case(1 * HOUR, 2)]
#[case(2 * HOUR, 2)]
#[case(3 * HOUR, 2)]
#[case(4 * HOUR, 2)]
#[case(5 * HOUR, 2)]
#[case(6 * HOUR, 2)]
#[case(7 * HOUR, 2)]
#[case(8 * HOUR, 2)]
#[case(9 * HOUR, 2)]
#[case(10 * HOUR, 2)]
#[case(11 * HOUR, 2)]
#[case(12 * HOUR, 2)]
#[case(1 * DAY, 2)]
#[case(2 * DAY, 2)]
#[case(3 * DAY, 2)]
#[case(4 * DAY, 2)]
#[case(5 * DAY, 2)]
#[case(6 * DAY, 2)]
#[case(7 * DAY, 2)]
#[case(1 * HOUR, 3)]
#[case(2 * HOUR, 3)]
#[case(3 * HOUR, 3)]
#[case(4 * HOUR, 3)]
#[case(5 * HOUR, 3)]
#[case(6 * HOUR, 3)]
#[case(7 * HOUR, 3)]
#[case(8 * HOUR, 3)]
#[case(9 * HOUR, 3)]
#[case(10 * HOUR, 3)]
#[case(11 * HOUR, 3)]
#[case(12 * HOUR, 3)]
#[case(1 * DAY, 3)]
#[case(2 * DAY, 3)]
#[case(3 * DAY, 3)]
#[case(4 * DAY, 3)]
#[case(5 * DAY, 3)]
#[case(6 * DAY, 3)]
#[case(7 * DAY, 3)]
#[case(1 * HOUR, 4)]
#[case(2 * HOUR, 4)]
#[case(3 * HOUR, 4)]
#[case(4 * HOUR, 4)]
#[case(5 * HOUR, 4)]
#[case(6 * HOUR, 4)]
#[case(7 * HOUR, 4)]
#[case(8 * HOUR, 4)]
#[case(9 * HOUR, 4)]
#[case(10 * HOUR, 4)]
#[case(11 * HOUR, 4)]
#[case(12 * HOUR, 4)]
#[case(1 * DAY, 4)]
#[case(2 * DAY, 4)]
#[case(3 * DAY, 4)]
#[case(4 * DAY, 4)]
#[case(5 * DAY, 4)]
#[case(6 * DAY, 4)]
#[case(7 * DAY, 4)]
#[case(1 * HOUR, 5)]
#[case(2 * HOUR, 5)]
#[case(3 * HOUR, 5)]
#[case(4 * HOUR, 5)]
#[case(5 * HOUR, 5)]
#[case(6 * HOUR, 5)]
#[case(7 * HOUR, 5)]
#[case(8 * HOUR, 5)]
#[case(9 * HOUR, 5)]
#[case(10 * HOUR, 5)]
#[case(11 * HOUR, 5)]
#[case(12 * HOUR, 5)]
#[case(1 * DAY, 5)]
#[case(2 * DAY, 5)]
#[case(3 * DAY, 5)]
#[case(4 * DAY, 5)]
#[case(5 * DAY, 5)]
#[case(6 * DAY, 5)]
#[case(7 * DAY, 5)]
#[case(1 * HOUR, 6)]
#[case(2 * HOUR, 6)]
#[case(3 * HOUR, 6)]
#[case(4 * HOUR, 6)]
#[case(5 * HOUR, 6)]
#[case(6 * HOUR, 6)]
#[case(7 * HOUR, 6)]
#[case(8 * HOUR, 6)]
#[case(9 * HOUR, 6)]
#[case(10 * HOUR, 6)]
#[case(11 * HOUR, 6)]
#[case(12 * HOUR, 6)]
#[case(1 * DAY, 6)]
#[case(2 * DAY, 6)]
#[case(3 * DAY, 6)]
#[case(4 * DAY, 6)]
#[case(5 * DAY, 6)]
#[case(6 * DAY, 6)]
#[case(7 * DAY, 6)]
#[case(1 * HOUR, 7)]
#[case(2 * HOUR, 7)]
#[case(3 * HOUR, 7)]
#[case(4 * HOUR, 7)]
#[case(5 * HOUR, 7)]
#[case(6 * HOUR, 7)]
#[case(7 * HOUR, 7)]
#[case(8 * HOUR, 7)]
#[case(9 * HOUR, 7)]
#[case(10 * HOUR, 7)]
#[case(11 * HOUR, 7)]
#[case(12 * HOUR, 7)]
#[case(1 * DAY, 7)]
#[case(2 * DAY, 7)]
#[case(3 * DAY, 7)]
#[case(4 * DAY, 7)]
#[case(5 * DAY, 7)]
#[case(6 * DAY, 7)]
#[case(7 * DAY, 7)]
#[case(1 * HOUR, 8)]
#[case(2 * HOUR, 8)]
#[case(3 * HOUR, 8)]
#[case(4 * HOUR, 8)]
#[case(5 * HOUR, 8)]
#[case(6 * HOUR, 8)]
#[case(7 * HOUR, 8)]
#[case(8 * HOUR, 8)]
#[case(9 * HOUR, 8)]
#[case(10 * HOUR, 8)]
#[case(11 * HOUR, 8)]
#[case(12 * HOUR, 8)]
#[case(1 * DAY, 8)]
#[case(2 * DAY, 8)]
#[case(3 * DAY, 8)]
#[case(4 * DAY, 8)]
#[case(5 * DAY, 8)]
#[case(6 * DAY, 8)]
#[case(7 * DAY, 8)]
#[case(1 * HOUR, 9)]
#[case(2 * HOUR, 9)]
#[case(3 * HOUR, 9)]
#[case(4 * HOUR, 9)]
#[case(5 * HOUR, 9)]
#[case(6 * HOUR, 9)]
#[case(7 * HOUR, 9)]
#[case(8 * HOUR, 9)]
#[case(9 * HOUR, 9)]
#[case(10 * HOUR, 9)]
#[case(11 * HOUR, 9)]
#[case(12 * HOUR, 9)]
#[case(1 * DAY, 9)]
#[case(2 * DAY, 9)]
#[case(3 * DAY, 9)]
#[case(4 * DAY, 9)]
#[case(5 * DAY, 9)]
#[case(6 * DAY, 9)]
#[case(7 * DAY, 9)]
#[case(1 * HOUR, 10)]
#[case(2 * HOUR, 10)]
#[case(3 * HOUR, 10)]
#[case(4 * HOUR, 10)]
#[case(5 * HOUR, 10)]
#[case(6 * HOUR, 10)]
#[case(7 * HOUR, 10)]
#[case(8 * HOUR, 10)]
#[case(9 * HOUR, 10)]
#[case(10 * HOUR, 10)]
#[case(11 * HOUR, 10)]
#[case(12 * HOUR, 10)]
#[case(1 * DAY, 10)]
#[case(2 * DAY, 10)]
#[case(3 * DAY, 10)]
#[case(4 * DAY, 10)]
#[case(5 * DAY, 10)]
#[case(6 * DAY, 10)]
#[case(7 * DAY, 10)]
#[case(1 * HOUR, 11)]
#[case(2 * HOUR, 11)]
#[case(3 * HOUR, 11)]
#[case(4 * HOUR, 11)]
#[case(5 * HOUR, 11)]
#[case(6 * HOUR, 11)]
#[case(7 * HOUR, 11)]
#[case(8 * HOUR, 11)]
#[case(9 * HOUR, 11)]
#[case(10 * HOUR, 11)]
#[case(11 * HOUR, 11)]
#[case(12 * HOUR, 11)]
#[case(1 * DAY, 11)]
#[case(2 * DAY, 11)]
#[case(3 * DAY, 11)]
#[case(4 * DAY, 11)]
#[case(5 * DAY, 11)]
#[case(6 * DAY, 11)]
#[case(7 * DAY, 11)]
#[case(1 * HOUR, 12)]
#[case(2 * HOUR, 12)]
#[case(3 * HOUR, 12)]
#[case(4 * HOUR, 12)]
#[case(5 * HOUR, 12)]
#[case(6 * HOUR, 12)]
#[case(7 * HOUR, 12)]
#[case(8 * HOUR, 12)]
#[case(9 * HOUR, 12)]
#[case(10 * HOUR, 12)]
#[case(11 * HOUR, 12)]
#[case(12 * HOUR, 12)]
#[case(1 * DAY, 12)]
#[case(2 * DAY, 12)]
#[case(3 * DAY, 12)]
#[case(4 * DAY, 12)]
#[case(5 * DAY, 12)]
#[case(6 * DAY, 12)]
#[case(7 * DAY, 12)]
fn test_polars(#[case] period: u64, #[case] dataset: i32) {
	let mut twap: Option<Twap> = None;
	let df = LazyCsvReader::new("eda/raw_data/final.csv".into())
		.has_header(true)
		.finish()
		.unwrap()
		.filter(col("market_index").eq(lit(dataset)))
		.select(&[col("ts"), col("mark_price_before")])
		.with_column(
			col("ts")
				.str()
				.strptime(StrpTimeOptions {
					date_dtype: DataType::Datetime(TimeUnit::Milliseconds, None),
					fmt: Some("%Y-%m-%d %T%z".to_string()),
					strict: true,
					exact: true,
				})
				.dt()
				.timestamp(TimeUnit::Milliseconds)
				.alias("timestamp"),
		)
		.sort("timestamp", SortOptions { descending: false, nulls_last: true })
		.with_column(
			as_struct(&[cols(vec!["timestamp", "mark_price_before"])])
				.map(
					move |data| {
						Ok(Series::from_vec(
							"parsed_twap",
							data.iter()
								.map(move |i| match i {
									AnyValue::Struct(v, t) => {
										let (now, price) = match v[..] {
											[AnyValue::Int64(now), AnyValue::Int64(price)] => (
												now as u64,
												FixedU128::from_inner(
													(((price as u64).saturating_mul(10.pow(7)))
														as u128)
														.saturating_mul(10.pow(5)),
												),
											),
											_ => panic!(
												"Could not extranct `now` and `price` values"
											),
										};
										match twap {
											Some(ref mut t) => {
												// dbg!("==================================================");
												// dbg!(&t);
												//
												// dbg!(&t, &price, &now);
												// dbg!(dbg!(t.accumulate(&price, &now))
												// 	.expect(
												// 		format!(
												// 		"Failed to accumulate twap, {now} {price}"
												// 	)
												// 		.as_str(),
												// 	)
												// 	.to_float())
												let x = t
													.accumulate(&price, &now)
													.expect(
														format!(
														"Failed to accumulate twap, {now} {price}"
													)
														.as_str(),
													)
													.to_float();

												// dbg!(&t);
												// dbg!("--------------------------------------------------");
												x
											},
											None => {
												twap = Some(Twap::new(price, now, period));
												// dbg!(&twap);
												twap.unwrap().get_twap().to_float()
											},
										}
									},
									_ => panic!("Failed to parse a struct field"),
								})
								.collect::<Vec<f64>>(),
						))
					},
					GetOutput::from_type(DataType::Float64),
				)
				.alias("twap"),
		)
		.with_column(
			col("mark_price_before")
				.map(
					|p| {
						Ok(Series::from_vec(
							"price",
							p.iter()
								.map(|x| match x {
									AnyValue::Int64(i) => i as f64 / 10.0_f64.powf(6.),
									err => panic!("Failed to parse int: {err}"),
								})
								.collect::<Vec<f64>>(),
						))
					},
					GetOutput::from_type(DataType::Int64),
				)
				.alias("price"),
		)
		.with_column(
			as_struct(&[cols(vec!["twap", "price"])])
				.map(
					|data| {
						Ok(Series::from_vec(
							"parsed_diff",
							data.iter()
								.map(|i| match i {
									AnyValue::Struct(s, _) => match s[..] {
										[AnyValue::Float64(twap), AnyValue::Float64(price)] =>
											twap.max(price) - twap.min(price),
										_ => panic!("failed to get `twap` and `price` values"),
									},
									_ => panic!("failed to parse tuple data"),
								})
								.collect::<Vec<f64>>(),
						))
					},
					GetOutput::from_type(DataType::Float64),
				)
				.alias("diff"),
		)
		.collect()
		.unwrap();

	// dbg!(&df);
	// dbg!(&df.select(["diff"]).unwrap().describe(None));
	let x_lim_0 = df["timestamp"].min::<i64>().unwrap();
	let x_lim_1 = df["timestamp"].max::<i64>().unwrap();
	let y_lim_0 = df["price"].min::<f64>().unwrap().min(df["twap"].min::<f64>().unwrap());
	let y_lim_1 = df["price"].max::<f64>().unwrap().max(df["twap"].max::<f64>().unwrap());

	let file_name = format!("eda/imgs/test_{dataset:02}_{period:010}.png");
	dbg!(&file_name);
	let root = BitMapBackend::new(&file_name, (3840, 2160)).into_drawing_area();
	root.fill(&WHITE).unwrap();
	let mut chart = ChartBuilder::on(&root)
		.margin(10)
		.caption(format!("Dataset = {dataset:02}, Period = {period:010}"), ("sans-serif", 40))
		.set_label_area_size(LabelAreaPosition::Left, 60)
		.set_label_area_size(LabelAreaPosition::Bottom, 40)
		.build_cartesian_2d(x_lim_0..x_lim_1, y_lim_0..y_lim_1)
		.unwrap();
	chart
		.configure_mesh()
		.disable_x_mesh()
		.disable_y_mesh()
		.x_labels(30)
		.max_light_lines(4)
		.y_desc("price/twap")
		.draw()
		.unwrap();

	let price_plot = df["timestamp"].iter().zip(df["price"].iter()).map(|(ts, price)| {
		(
			match ts {
				AnyValue::Int64(ts) => ts,
				_ => panic!(),
			},
			match price {
				AnyValue::Float64(price) => price,
				_ => panic!(),
			},
		)
	});
	let twap_plot = df["timestamp"].iter().zip(df["twap"].iter()).map(|(ts, price)| {
		(
			match ts {
				AnyValue::Int64(ts) => ts,
				_ => panic!(),
			},
			match price {
				AnyValue::Float64(price) => price,
				_ => panic!(),
			},
		)
	});
	chart.draw_series(LineSeries::new(price_plot, &BLUE)).unwrap();
	chart.draw_series(LineSeries::new(twap_plot, &RED)).unwrap();
	root.present().unwrap();

	let out_file = std::fs::File::create(std::path::Path::new("twap.csv")).unwrap();
	CsvWriter::new(out_file).has_header(true).finish(&mut df.clone());
}
