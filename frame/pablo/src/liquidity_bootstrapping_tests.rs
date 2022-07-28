#[cfg(test)]
use crate::{
	liquidity_bootstrapping::PoolIsValid,
	mock,
	mock::{Pablo, *},
	pallet, Error, PoolInitConfiguration,
};
use composable_support::validation::Validated;
use composable_tests_helpers::test::helper::default_acceptable_computation_error;
use composable_traits::{
	defi::CurrencyPair,
	dex::{FeeConfig, LiquidityBootstrappingPoolInfo, Sale},
};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use sp_runtime::Permill;

pub fn valid_pool(
) -> Validated<LiquidityBootstrappingPoolInfo<AccountId, AssetId, BlockNumber>, PoolIsValid<Test>> {
	let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
	let owner = ALICE;
	let duration = MaxSaleDuration::get();
	let start = 0;
	let end = start + duration;
	let initial_weight = MaxInitialWeight::get();
	let final_weight = MinFinalWeight::get();
	let fee = Permill::from_perthousand(1);
	Validated::new(LiquidityBootstrappingPoolInfo {
		owner,
		pair,
		sale: Sale { start, end, initial_weight, final_weight },
		fee_config: FeeConfig {
			fee_rate: fee,
			owner_fee_rate: Permill::zero(),
			protocol_fee_rate: Permill::zero(),
		},
	})
	.expect("impossible; qed;")
}

fn with_pool<T>(
	owner: AccountId,
	sale_duration: BlockNumber,
	initial_weight: Permill,
	final_weight: Permill,
	fee: Permill,
	f: impl FnOnce(
		PoolId,
		&LiquidityBootstrappingPoolInfo<AccountId, AssetId, BlockNumber>,
		&dyn Fn(BlockNumber),
		&dyn Fn(),
	) -> T,
) -> T {
	let random_start = 0xDEADC0DE;
	let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
	let end = random_start + sale_duration;
	let pool = Validated::<_, PoolIsValid<Test>>::new(LiquidityBootstrappingPoolInfo {
		owner,
		pair,
		sale: Sale { start: random_start, end, initial_weight, final_weight },
		fee_config: FeeConfig {
			fee_rate: fee,
			owner_fee_rate: Permill::zero(),
			protocol_fee_rate: Permill::zero(),
		},
	})
	.expect("impossible; qed;");
	new_test_ext().execute_with(|| -> T {
		// Actually create the pool.
		assert_ok!(Pablo::create(
			Origin::signed(owner),
			PoolInitConfiguration::LiquidityBootstrapping(pool.value())
		));

		// Will always start to 0.
		let pool_id = 0;

		// Relative to sale start.
		let set_block = |x: BlockNumber| {
			System::set_block_number(random_start + x);
		};

		// Forward to sale end.
		let end_sale = || {
			set_block(sale_duration + 1);
		};

		f(pool_id, &pool, &set_block, &end_sale)
	})
}

fn within_sale_with_liquidity<T>(
	owner: AccountId,
	sale_duration: BlockNumber,
	initial_weight: Permill,
	final_weight: Permill,
	fee: Permill,
	initial_project_tokens: Balance,
	initial_usdt: Balance,
	f: impl FnOnce(
		PoolId,
		&LiquidityBootstrappingPoolInfo<AccountId, AssetId, BlockNumber>,
		&dyn Fn(BlockNumber),
		&dyn Fn(),
	) -> T,
) -> T {
	with_pool(
		owner,
		sale_duration,
		initial_weight,
		final_weight,
		fee,
		|pool_id, pool, set_block, end_sale| -> T {
			assert_ok!(Tokens::mint_into(PROJECT_TOKEN, &owner, initial_project_tokens));
			assert_ok!(Tokens::mint_into(USDT, &owner, initial_usdt));

			// Add initial liquidity.
			assert_ok!(Pablo::add_liquidity(
				Origin::signed(owner),
				pool_id,
				initial_project_tokens,
				initial_usdt,
				0_u128,
				false
			));

			// Actually start the sale.
			set_block(0);

			f(pool_id, pool, set_block, end_sale)
		},
	)
}

mod sell {
	use super::*;

	#[test]
	fn can_sell_one_to_one() {
		/* 50% weight = constant product, no fees.
		 */
		let unit = 1_000_000_000_000;
		let initial_project_tokens = 1_000_000 * unit;
		let initial_usdt = 1_000_000 * unit;
		let sale_duration = MaxSaleDuration::get();
		let initial_weight = Permill::one() / 2;
		let final_weight = Permill::one() / 2;
		let fee = Permill::zero();
		within_sale_with_liquidity(
			ALICE,
			sale_duration,
			initial_weight,
			final_weight,
			fee,
			initial_project_tokens,
			initial_usdt,
			|pool_id, pool, _, _| {
				// Buy project token
				assert_ok!(Tokens::mint_into(USDT, &BOB, unit));
				assert_ok!(Pablo::sell(
					Origin::signed(BOB),
					pool_id,
					pool.pair.quote,
					unit,
					0_u128,
					false
				));
				assert_ok!(default_acceptable_computation_error(
					Tokens::balance(PROJECT_TOKEN, &BOB),
					unit
				));
			},
		)
	}
}

mod buy {
	use super::*;
	use crate::common_test_functions::assert_has_event;

	#[test]
	fn can_buy_one_to_one() {
		/* 50% weight = constant product, no fees.
		 */
		let unit = 1_000_000_000_000;
		let initial_project_tokens = 1_000_000 * unit;
		let initial_usdt = 1_000_000 * unit;
		let sale_duration = MaxSaleDuration::get();
		let initial_weight = Permill::one() / 2;
		let final_weight = Permill::one() / 2;
		let fee = Permill::zero();
		within_sale_with_liquidity(
			ALICE,
			sale_duration,
			initial_weight,
			final_weight,
			fee,
			initial_project_tokens,
			initial_usdt,
			|created_pool_id, pool, _, _| {
				// Buy project token
				assert_ok!(Tokens::mint_into(USDT, &BOB, unit));
				assert_ok!(Pablo::buy(
					Origin::signed(BOB),
					created_pool_id,
					pool.pair.base,
					unit,
					0_u128,
					false
				));
				assert_has_event::<Test, _>(|e| {
					matches!(
					    e.event,
				        mock::Event::Pablo(crate::Event::Swapped { pool_id, fee, .. }) if pool_id == created_pool_id && fee.asset_id == USDT)
				});
				let price =
					pallet::prices_for::<Test>(created_pool_id, PROJECT_TOKEN, USDT, 1 * unit)
						.unwrap();
				assert_eq!(price.spot_price, 1_000_000_999_997);
				assert_ok!(default_acceptable_computation_error(
					Tokens::balance(PROJECT_TOKEN, &BOB),
					unit
				));
			},
		)
	}
}

mod remove_liquidity {
	use super::*;

	#[test]
	fn cannot_remove_before_sale_end() {
		let owner = ALICE;
		let sale_duration = MaxSaleDuration::get();
		let initial_weight = Permill::one() / 2;
		let final_weight = Permill::one() / 2;
		let fee = Permill::zero();
		let unit = 1_000_000_000_000;
		let initial_project_tokens = 1_000_000 * unit;
		let initial_usdt = 1_000_000 * unit;
		with_pool(owner, sale_duration, initial_weight, final_weight, fee, |pool_id, _, _, _| {
			assert_ok!(Tokens::mint_into(PROJECT_TOKEN, &owner, initial_project_tokens));
			assert_ok!(Tokens::mint_into(USDT, &owner, initial_usdt));
			assert_noop!(
				Pablo::remove_liquidity(Origin::signed(owner), pool_id, 0, 0, 0, false),
				Error::<Test>::InvalidSaleState
			);
		});
	}

	#[test]
	fn can_remove_after_sale_end() {
		let owner = ALICE;
		let sale_duration = MaxSaleDuration::get();
		let initial_weight = Permill::one() / 2;
		let final_weight = Permill::one() / 2;
		let fee = Permill::zero();
		let unit = 1_000_000_000_000;
		let initial_project_tokens = 1_000_000 * unit;
		let initial_usdt = 1_000_000 * unit;
		with_pool(
			owner,
			sale_duration,
			initial_weight,
			final_weight,
			fee,
			|pool_id, _, _, end_sale| {
				assert_ok!(Tokens::mint_into(PROJECT_TOKEN, &owner, initial_project_tokens));
				assert_ok!(Tokens::mint_into(USDT, &owner, initial_usdt));
				end_sale();
				assert_ok!(Pablo::remove_liquidity(Origin::signed(owner), pool_id, 0, 0, 0, false));
				assert_eq!(Tokens::balance(PROJECT_TOKEN, &owner), initial_project_tokens);
				assert_eq!(Tokens::balance(USDT, &owner), initial_usdt);
			},
		);
	}
}

mod add_liquidity {
	use super::*;

	#[test]
	fn can_add_liquidity_before_sale() {
		let owner = ALICE;
		let sale_duration = MaxSaleDuration::get();
		let initial_weight = Permill::one() / 2;
		let final_weight = Permill::one() / 2;
		let fee = Permill::zero();
		let unit = 1_000_000_000_000;
		let initial_project_tokens = 1_000_000 * unit;
		let initial_usdt = 1_000_000 * unit;
		with_pool(owner, sale_duration, initial_weight, final_weight, fee, |pool_id, _, _, _| {
			assert_ok!(Tokens::mint_into(PROJECT_TOKEN, &owner, initial_project_tokens));
			assert_ok!(Tokens::mint_into(USDT, &owner, initial_usdt));
			assert_ok!(Pablo::add_liquidity(
				Origin::signed(owner),
				pool_id,
				initial_project_tokens,
				initial_usdt,
				0_128,
				false
			));
		});
	}

	#[test]
	fn cannot_add_liquidity_after_sale_started() {
		let owner = ALICE;
		let sale_duration = MaxSaleDuration::get();
		let initial_weight = Permill::one() / 2;
		let final_weight = Permill::one() / 2;
		let fee = Permill::zero();
		let unit = 1_000_000_000_000;
		let initial_project_tokens = 1_000_000 * unit;
		let initial_usdt = 1_000_000 * unit;
		with_pool(
			owner,
			sale_duration,
			initial_weight,
			final_weight,
			fee,
			|pool_id, _, set_sale_block, _| {
				assert_ok!(Tokens::mint_into(PROJECT_TOKEN, &owner, initial_project_tokens));
				assert_ok!(Tokens::mint_into(USDT, &owner, initial_usdt));
				set_sale_block(0);
				assert_noop!(
					Pablo::add_liquidity(
						Origin::signed(owner),
						pool_id,
						initial_project_tokens,
						initial_usdt,
						0_128,
						false
					),
					Error::<Test>::InvalidSaleState
				);
			},
		);
	}
}

mod invalid_pool {
	use super::*;
	use frame_support::assert_err;
	use sp_runtime::DispatchError;

	#[test]
	fn final_weight_below_minimum() {
		new_test_ext().execute_with(|| {
			let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
			let owner = ALICE;
			let duration = MaxSaleDuration::get() - 1;
			let start = 0;
			let end = start + duration;
			let initial_weight = MaxInitialWeight::get();
			let final_weight = MinFinalWeight::get() - Permill::from_parts(1);
			let fee = Permill::from_perthousand(1);
			assert_err!(
				Validated::<_, PoolIsValid<Test>>::new(LiquidityBootstrappingPoolInfo {
					owner,
					pair,
					sale: Sale { start, end, initial_weight, final_weight },
					fee_config: FeeConfig {
						fee_rate: fee,
						owner_fee_rate: Permill::zero(),
						protocol_fee_rate: Permill::zero()
					},
				}),
				DispatchError::Other("Final weight must not be lower than the defined minimum.")
			);
		});
	}

	#[test]
	fn initial_weight_above_maximum() {
		new_test_ext().execute_with(|| {
			let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
			let owner = ALICE;
			let duration = MaxSaleDuration::get() - 1;
			let start = 0;
			let end = start + duration;
			let initial_weight = MaxInitialWeight::get() + Permill::from_parts(1);
			let final_weight = MinFinalWeight::get();
			let fee = Permill::from_perthousand(1);
			assert!(Validated::<_, PoolIsValid<Test>>::new(LiquidityBootstrappingPoolInfo {
				owner,
				pair,
				sale: Sale { start, end, initial_weight, final_weight },
				fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
				},
			})
			.is_err());
		});
	}

	#[test]
	fn final_weight_above_initial_weight() {
		new_test_ext().execute_with(|| {
			let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
			let owner = ALICE;
			let duration = MaxSaleDuration::get() - 1;
			let start = 0;
			let end = start + duration;
			let initial_weight = MinFinalWeight::get();
			let final_weight = MaxInitialWeight::get();
			let fee = Permill::from_perthousand(1);
			assert!(Validated::<_, PoolIsValid<Test>>::new(LiquidityBootstrappingPoolInfo {
				owner,
				pair,
				sale: Sale { start, end, initial_weight, final_weight },
				fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
				},
			})
			.is_err());
		});
	}

	#[test]
	fn end_before_start() {
		new_test_ext().execute_with(|| {
			let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
			let owner = ALICE;
			let start = 1;
			let end = 0;
			let initial_weight = MaxInitialWeight::get();
			let final_weight = MinFinalWeight::get();
			let fee = Permill::from_perthousand(1);
			assert!(Validated::<_, PoolIsValid<Test>>::new(LiquidityBootstrappingPoolInfo {
				owner,
				pair,
				sale: Sale { start, end, initial_weight, final_weight },
				fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
				}
			})
			.is_err());
		});
	}

	#[test]
	fn above_maximum_sale_duration() {
		new_test_ext().execute_with(|| {
			let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
			let owner = ALICE;
			let duration = MaxSaleDuration::get() + 1;
			let start = 0;
			let end = start + duration;
			let initial_weight = MaxInitialWeight::get();
			let final_weight = MinFinalWeight::get();
			let fee = Permill::from_perthousand(1);
			assert!(Validated::<_, PoolIsValid<Test>>::new(LiquidityBootstrappingPoolInfo {
				owner,
				pair,
				sale: Sale { start, end, initial_weight, final_weight },
				fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
				}
			})
			.is_err());
		});
	}

	#[test]
	fn below_minimum_sale_duration() {
		new_test_ext().execute_with(|| {
			let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
			let owner = ALICE;
			let duration = MinSaleDuration::get() - 1;
			let start = 0;
			let end = start + duration;
			let initial_weight = MaxInitialWeight::get();
			let final_weight = MinFinalWeight::get();
			let fee = Permill::from_perthousand(1);
			assert!(Validated::<_, PoolIsValid<Test>>::new(LiquidityBootstrappingPoolInfo {
				owner,
				pair,
				sale: Sale { start, end, initial_weight, final_weight },
				fee_config: FeeConfig {
					fee_rate: fee,
					owner_fee_rate: Permill::zero(),
					protocol_fee_rate: Permill::zero()
				}
			})
			.is_err());
		});
	}
}

#[cfg(feature = "visualization")]
mod visualization {
	use super::*;
	use crate::liquidity_bootstrapping::LiquidityBootstrapping;

	#[test]
	fn plot() {
		new_test_ext().execute_with(|| {
			let pair = CurrencyPair::new(PROJECT_TOKEN, USDT);
			let owner = ALICE;
			let two_days = 48 * 3600 / 12;
			let window = 100;
			let pool = LiquidityBootstrappingPoolInfo {
				owner,
				pair,
				sale: Sale {
					start: window,
					end: two_days + window,
					initial_weight: Permill::from_percent(92),
					final_weight: Permill::from_percent(50),
				},
				fee_config: Permill::from_perthousand(1),
			};
			let pool_id =
				Pablo::do_create_pool(&owner, PoolInitConfiguration::LiquidityBootstrapping(pool))
					.expect("impossible; qed;");

			let unit = 1_000_000_000_000;
			let initial_project_tokens = 100_000_000 * unit;
			let initial_usdt = 1_000_000 * unit;

			assert_ok!(Tokens::mint_into(PROJECT_TOKEN, &ALICE, initial_project_tokens));
			assert_ok!(Tokens::mint_into(USDT, &ALICE, initial_usdt));
			assert_ok!(Pablo::add_liquidity(
				Origin::signed(ALICE),
				pool_id,
				initial_project_tokens,
				initial_usdt,
				0,
				false
			));
			let pool_account = Pablo::account_id(&pool_id);

			{
				let points = (pool.sale.start..pool.sale.end)
					.map(|block| {
						(
							block,
							LiquidityBootstrapping::<Test>::do_spot_price(
								pool,
								pool_account,
								pool.pair,
								block,
							)
							.expect("impossible; qed;") as f64 /
								unit as f64,
						)
					})
					.collect::<Vec<_>>();
				let max_amount = points.iter().copied().fold(f64::NAN, |x, (_, y)| f64::max(x, y));

				use plotters::prelude::*;
				let area = BitMapBackend::new("./plots/lbp/lbp_spot_price.png", (1024, 768))
					.into_drawing_area();
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

			{
				use plotters::prelude::*;

				let plot_swap = |pair, swap_amount, name, caption| {
					let points = (pool.sale.start..pool.sale.end)
						.map(|block| {
							let (fees, base_amount) =
								LiquidityBootstrapping::<Test>::do_get_exchange(
									pool,
									&pool_account,
									pair,
									block,
									swap_amount,
									true,
								)
								.expect("impossible; qed;");
							(block, fees / unit, base_amount / unit)
						})
						.collect::<Vec<_>>();
					let amounts =
						points.clone().iter().copied().map(|(x, _, y)| (x, y)).collect::<Vec<_>>();
					let amounts_with_fees =
						points.into_iter().map(|(x, y, z)| (x, y + z)).collect::<Vec<_>>();

					let max_amount =
						amounts_with_fees.iter().copied().map(|(_, x)| x).max().unwrap();

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
						.draw_series(LineSeries::new(amounts, &BLUE))
						.unwrap()
						.label("Received tokens fees applied")
						.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
					chart
						.draw_series(LineSeries::new(amounts_with_fees, &RED))
						.unwrap()
						.label("Received tokens fees not applied")
						.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
					chart
						.configure_series_labels()
						.background_style(&WHITE.mix(0.8))
						.border_style(&BLACK)
						.draw()
						.unwrap();
				};

				let buy_amount = 500;
				plot_swap(
					pair,
					buy_amount * unit,
					"./plots/lbp/lbp_buy_project.png",
					format!("Buy project tokens with {} USDT", buy_amount),
				);
				let sell_amount = 100_000;
				plot_swap(
					pair.swap(),
					sell_amount * unit,
					"./plots/lbp/lbp_sell_project.png",
					format!("Sell {} project tokens", sell_amount),
				);
			}

			{
				use plotters::prelude::*;
				let area = BitMapBackend::new("./plots/lbp/lbp_weights.png", (1024, 768))
					.into_drawing_area();
				area.fill(&WHITE).unwrap();

				let mut chart = ChartBuilder::on(&area)
					.caption("y = weight", ("Arial", 50).into_font())
					.margin(100u32)
					.x_label_area_size(30u32)
					.y_label_area_size(30u32)
					.build_cartesian_2d(
						pool.sale.start..pool.sale.end,
						0..Permill::one().deconstruct(),
					)
					.unwrap();

				let points = (pool.sale.start..pool.sale.end).map(|block| {
					(block, pool.sale.current_weights(block).expect("impossible; qed;"))
				});

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
						points
							.map(|(block, (_, quote_weight))| (block, quote_weight.deconstruct())),
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
}
