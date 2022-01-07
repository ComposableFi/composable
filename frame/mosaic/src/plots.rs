mod decay;

fn main() {
	#[cfg(feature = "visualization")]
	{
		plot_linear_decay(5);
	}
}

#[cfg(feature = "visualization")]
fn plot_linear_decay(n: u128) {
	use decay::{BudgetDecay, Decayable};
	use plotters::prelude::*;

	let decay = BudgetDecay::linear(n);
	let mut penalty = 80;
	let blocks: u128 = 100;

	let area = BitMapBackend::new("./linear_decay.png", (1024, 768)).into_drawing_area();
	area.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&area)
		.set_label_area_size(LabelAreaPosition::Left, 40)
		.set_label_area_size(LabelAreaPosition::Bottom, 40)
		.build_cartesian_2d(0.0..100.0, 0.0..100.0)
		.unwrap();
	chart.configure_mesh().draw().unwrap();
	chart
		.draw_series(LineSeries::new(
			(1..=blocks).map(|x| {
				penalty = decay.checked_decay(penalty, x - 1, x).unwrap();
				(x as f64, penalty as f64)
			}),
			&RED,
		))
		.unwrap();
}
