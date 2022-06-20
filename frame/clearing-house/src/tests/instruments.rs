use crate::{
	mock::{
		self as mock,
		assets::AssetId,
		runtime::{ExtBuilder, MarketId, Runtime, TestPallet, VammId},
	},
	tests::{as_inner, bounded_decimal, zero_to_one_open_interval, Market, Position},
	Direction,
};
use composable_traits::{clearing_house::Instruments, time::DurationSeconds};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedI128, FixedPointNumber};

// -------------------------------------------------------------------------------------------------
//                                           Helpers
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct MarketVars {
	vamm_id: VammId,
	asset_id: AssetId,
	margin_ratio_initial: FixedI128,
	margin_ratio_maintenance: FixedI128,
	minimum_trade_size: FixedI128,
	cum_funding_rate_long: FixedI128,
	cum_funding_rate_short: FixedI128,
	funding_rate_ts: DurationSeconds,
	funding_frequency: DurationSeconds,
	funding_period: DurationSeconds,
}

impl From<MarketVars> for Market {
	fn from(vars: MarketVars) -> Self {
		let MarketVars {
			vamm_id,
			asset_id,
			margin_ratio_initial,
			margin_ratio_maintenance,
			minimum_trade_size,
			cum_funding_rate_long,
			cum_funding_rate_short,
			funding_rate_ts,
			funding_frequency,
			funding_period,
		} = vars;
		Self {
			vamm_id,
			asset_id,
			margin_ratio_initial,
			margin_ratio_maintenance,
			minimum_trade_size,
			cum_funding_rate_long,
			cum_funding_rate_short,
			funding_rate_ts,
			funding_frequency,
			funding_period,
			..Default::default()
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                         Prop Compose
// -------------------------------------------------------------------------------------------------

prop_compose! {
	fn any_duration()(duration in any::<DurationSeconds>()) -> DurationSeconds {
		duration
	}
}

prop_compose! {
	fn nonzero_duration()(
		duration in any_duration().prop_filter("Zero duration not allowed", |n| n > &0)
	) -> DurationSeconds {
		duration
	}
}

prop_compose! {
	fn funding_params()(
		(funding_frequency, funding_freq_mul) in nonzero_duration()
			.prop_flat_map(|n| (Just(n), 1..=DurationSeconds::MAX.div_euclid(n)))
	) -> (DurationSeconds, DurationSeconds) {
		(funding_frequency, funding_frequency * funding_freq_mul)
	}
}

prop_compose! {
	fn any_minimum_trade_size()(inner in 0..as_inner(1_000_000_000)) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn initial_gt_maintenance_margin_ratio()(
		(initial, maintenance) in zero_to_one_open_interval()
			.prop_flat_map(|num|
				(Just(num), (0.0..num).prop_filter("Zero MMR not allowed", |n| n > &0.0))
			)
	) -> (FixedI128, FixedI128) {
		(FixedI128::from_float(initial), FixedI128::from_float(maintenance))
	}
}

prop_compose! {
	fn any_market_vars()(
		vamm_id in any::<VammId>(),
		asset_id in any::<AssetId>(),
		(
			margin_ratio_initial,
			margin_ratio_maintenance
		) in initial_gt_maintenance_margin_ratio(),
		minimum_trade_size in any_minimum_trade_size(),
		cum_funding_rate in bounded_decimal(),
		funding_rate_ts in any_duration(),
		(funding_frequency, funding_period) in funding_params(),
	) -> MarketVars {
		MarketVars {
			vamm_id,
			asset_id,
			margin_ratio_initial,
			margin_ratio_maintenance,
			minimum_trade_size,
			cum_funding_rate_long: cum_funding_rate,
			cum_funding_rate_short: cum_funding_rate,
			funding_rate_ts,
			funding_frequency,
			funding_period,
		}
	}
}

prop_compose! {
	fn any_position()(
		market_id in any::<MarketId>(),
		base_asset_amount in bounded_decimal(),
		quote_asset_notional_amount in bounded_decimal(),
		last_cum_funding in bounded_decimal(),
	) -> Position {
		Position {
			market_id,
			base_asset_amount,
			quote_asset_notional_amount,
			last_cum_funding
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                        Property Tests
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn funding_rate_query_leaves_storage_intact(market_vars in any_market_vars()) {
		ExtBuilder::default().build().execute_with(|| {
			let market = market_vars.into();
			assert_storage_noop!(
				assert_ok!(<TestPallet as Instruments>::funding_rate(&market))
			);
		})
	}

	#[test]
	#[ignore = "TWAP is now stored in market; this test should be modified or discarded"]
	fn funding_rate_query_fails_if_oracle_twap_fails(market_vars in any_market_vars()) {
		ExtBuilder { oracle_twap: None, ..Default::default() }.build().execute_with(|| {
			let market = market_vars.into();
			assert_noop!(
				<TestPallet as Instruments>::funding_rate(&market),
				mock::oracle::Error::<Runtime>::CantComputeTwap
			);
		})
	}

	#[test]
	fn funding_rate_query_fails_if_vamm_twap_fails(market_vars in any_market_vars()) {
		ExtBuilder { vamm_twap: None, ..Default::default() }.build().execute_with(|| {
			let market = market_vars.into();
			assert_noop!(
				<TestPallet as Instruments>::funding_rate(&market),
				mock::vamm::Error::<Runtime>::FailedToCalculateTwap
			);
		})
	}

	#[test]
	fn unrealized_funding_query_leaves_storage_intact(
		market_vars in any_market_vars(), position in any_position()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let market = market_vars.into();
			assert_storage_noop!(
				assert_ok!(<TestPallet as Instruments>::unrealized_funding(&market, &position))
			);
		})
	}

	#[test]
	fn unrealized_funding_sign_is_correct(
		(base_amount, quote_amount) in prop_oneof![
			Just((-10, -100)),
			Just((0, 0)),
			Just((10, 100))
		],
		cum_funding_delta in
			prop_oneof![Just(-1), Just(0), Just(1)].prop_map(|n| FixedI128::from((n, 10))),
	) {
		let position = Position {
			market_id: 0,
			base_asset_amount: base_amount.into(),
			quote_asset_notional_amount: quote_amount.into(),
			last_cum_funding: 0.into(),
		};
		let cum_funding_rate = position.last_cum_funding + cum_funding_delta;

		ExtBuilder::default().build().execute_with(|| {
			let market = Market {
				cum_funding_rate_long: cum_funding_rate,
				cum_funding_rate_short: cum_funding_rate,
				..Default::default()
			};

			let unrealized_funding = <TestPallet as Instruments>::unrealized_funding(
				&market, &position
			).unwrap();

			// Positive unrealized funding means the position receive funds
			//
			//     position sign | funding rate sign | unrealized funding sign
			// ---------------------------------------------------------------
			//                -1 |                -1 | -1
			//                -1 |                 1 |  1
			//                 1 |                -1 |  1
			//                 1 |                 1 | -1
			//                 - |                 0 |  0
			//                 0 |                 - |  0
			let direction = position.direction();
			if cum_funding_delta.is_zero() || direction.is_none() {
				assert!(unrealized_funding.is_zero());
			} else  {
				assert!(match direction.unwrap() {
					Direction::Long =>
						cum_funding_delta.is_positive() != unrealized_funding.is_positive(),
					_ => cum_funding_delta.is_positive() == unrealized_funding.is_positive(),
				});
			}
		})
	}
}
