use crate::{mock::*, *};
use composable_support::validation::Validated;
use composable_traits::{defi::CurrencyPair, dex::CurveAmm};
use frame_support::{assert_ok, traits::fungibles::Mutate};
use sp_runtime::Permill;

#[test]
fn test() {
	new_test_ext().execute_with(|| {
		let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
		let owner = ALICE;
		let pool = Pool {
			owner,
			pair,
			sale: Sale {
				start: 100,
				end: 19200 + 100,
				initial_weight: Permill::from_percent(92),
				final_weight: Permill::from_percent(50),
			},
		};
		let pool_id = LBP::do_create_pool(Validated::new(pool).expect("impossible; qed;"))
			.expect("impossible; qed;");

		let unit = 1_000_000_000_000;
		let initial_project_tokens = 200_000_000 * unit;
		let initial_usdt = 5_000_000 * unit;

		assert_ok!(Tokens::mint_into(PROJECT_TOKEN, &ALICE, initial_project_tokens));
		assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));

		assert_ok!(LBP::add_liquidity(
			&ALICE,
			pool_id,
			initial_project_tokens,
			initial_usdt,
			0,
			false
		));

		#[cfg(feature = "visualization")]
		{
			let points = (pool.sale.start..pool.sale.end)
				.map(|block| {
					(
						block,
						LBP::do_spot_price(pool_id, pool.pair, block).expect("impossible; qed;")
							as f64 / unit as f64,
					)
				})
				.collect::<Vec<_>>();
			let max_amount = points.iter().copied().fold(f64::NAN, |x, (_, y)| f64::max(x, y));

			use plotters::prelude::*;
			let area = BitMapBackend::new("./lbp_spot_price.png", (1024, 768)).into_drawing_area();
			area.fill(&WHITE).unwrap();

			let mut chart = ChartBuilder::on(&area)
				.caption("Spot price", ("Arial", 50).into_font())
				.margin(100u32)
				.x_label_area_size(30u32)
				.y_label_area_size(30u32)
				.build_cartesian_2d(pool.sale.start..pool.sale.end, 0f64..max_amount)
				.unwrap();

			chart.configure_mesh().draw().unwrap();
			chart
				.draw_series(LineSeries::new(points, &RED))
				.unwrap()
				.label("base")
				.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
			chart
				.configure_series_labels()
				.background_style(&WHITE.mix(0.8))
				.border_style(&BLACK)
				.draw()
				.unwrap();
		}

		#[cfg(feature = "visualization")]
		{
			use plotters::prelude::*;

			let plot_swap = |pair, swap_amount, name, caption| {
				let points = (pool.sale.start..pool.sale.end)
					.map(|block| {
						(
							block,
							LBP::do_get_exchange(pool_id, pair, block, swap_amount)
								.expect("impossible; qed;") / unit,
						)
					})
					.collect::<Vec<_>>();
				let max_amount = points.iter().copied().map(|(_, x)| x).max().unwrap();

				let area = BitMapBackend::new(name, (1024, 768)).into_drawing_area();
				area.fill(&WHITE).unwrap();

				let mut chart = ChartBuilder::on(&area)
					.caption(caption, ("Arial", 50).into_font())
					.margin(100u32)
					.x_label_area_size(30u32)
					.y_label_area_size(30u32)
					.build_cartesian_2d(pool.sale.start..pool.sale.end, 0..max_amount)
					.unwrap();

				chart.configure_mesh().draw().unwrap();
				chart
					.draw_series(LineSeries::new(points, &BLUE))
					.unwrap()
					.label("Amount received")
					.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
				chart
					.configure_series_labels()
					.background_style(&WHITE.mix(0.8))
					.border_style(&BLACK)
					.draw()
					.unwrap();
			};

			let buy_amount = 10;
			plot_swap(
				pair,
				buy_amount * unit,
				"./lbp_buy_project.png",
				format!("Buy project tokens with {} USDT", buy_amount),
			);
			let sell_amount = 100_000;
			plot_swap(
				pair.swap(),
				100_000 * unit,
				"./lbp_sell_project.png",
				format!("Sell {} project tokens", sell_amount),
			);
		}

		#[cfg(feature = "visualization")]
		{
			use plotters::prelude::*;
			let area = BitMapBackend::new("./lbp_weights.png", (1024, 768)).into_drawing_area();
			area.fill(&WHITE).unwrap();

			let mut chart = ChartBuilder::on(&area)
				.caption("y = weight", ("Arial", 50).into_font())
				.margin(100u32)
				.x_label_area_size(30u32)
				.y_label_area_size(30u32)
				.build_cartesian_2d(pool.sale.start..pool.sale.end, 0..Permill::one().deconstruct())
				.unwrap();

			let points = (pool.sale.start..pool.sale.end)
				.map(|block| (block, pool.sale.current_weights(block).expect("impossible; qed;")));

			chart.configure_mesh().draw().unwrap();
			chart
				.draw_series(LineSeries::new(
					points
						.clone()
						.map(|(block, (base_weight, _))| (block, base_weight.deconstruct())),
					&RED,
				))
				.unwrap()
				.label("base")
				.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
			chart
				.draw_series(LineSeries::new(
					points.map(|(block, (_, quote_weight))| (block, quote_weight.deconstruct())),
					&BLUE,
				))
				.unwrap()
				.label("quote")
				.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
			chart
				.configure_series_labels()
				.background_style(&WHITE.mix(0.8))
				.border_style(&BLACK)
				.draw()
				.unwrap();
		}
	});
}
