use crate::{
	mock::{
		self as mock,
		assets::AssetId,
		runtime::{ExtBuilder, MarketId, Runtime, TestPallet, VammId},
	},
	tests::{as_inner, zero_to_one_open_interval, Market, Position},
};
use composable_traits::{clearing_house::Instruments, time::DurationSeconds};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedI128};

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn bounded_decimal()(
		inner in as_inner(-1_000_000_000)..as_inner(1_000_000_000)
	) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

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
	fn any_market()(
		vamm_id in any::<VammId>(),
		asset_id in any::<AssetId>(),
		(
			margin_ratio_initial,
			margin_ratio_maintenance
		) in initial_gt_maintenance_margin_ratio(),
		minimum_trade_size in any_minimum_trade_size(),
		cum_funding_rate in bounded_decimal(),
		funding_rate_ts in any_duration(),
		(funding_frequency, funding_period) in funding_params()
	) -> Market {
		Market {
			vamm_id,
			asset_id,
			margin_ratio_initial,
			margin_ratio_maintenance,
			minimum_trade_size,
			net_base_asset_amount: 0.into(),
			cum_funding_rate,
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

// ----------------------------------------------------------------------------------------------------
//                                          Instruments trait
// ----------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn funding_rate_query_leaves_storage_intact(market in any_market()) {
		ExtBuilder::default().build().execute_with(|| {
			assert_storage_noop!(
				assert_ok!(<TestPallet as Instruments>::funding_rate(&market))
			);
		})
	}
}

proptest! {
	#[test]
	fn funding_rate_query_fails_if_oracle_twap_fails(market in any_market()) {
		ExtBuilder { oracle_twap: None, ..Default::default() }.build().execute_with(|| {
			assert_noop!(
				<TestPallet as Instruments>::funding_rate(&market),
				mock::oracle::Error::<Runtime>::CantComputeTwap
			);
		})
	}
}

proptest! {
	#[test]
	fn funding_rate_query_fails_if_vamm_twap_fails(market in any_market()) {
		ExtBuilder { vamm_twap: None, ..Default::default() }.build().execute_with(|| {
			assert_noop!(
				<TestPallet as Instruments>::funding_rate(&market),
				mock::vamm::Error::<Runtime>::FailedToCalculateTwap
			);
		})
	}
}

proptest! {
	#[test]
	fn unrealized_funding_query_leaves_storage_intact(
		market in any_market(), position in any_position()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_storage_noop!(
				assert_ok!(<TestPallet as Instruments>::unrealized_funding(&market, &position))
			);
		})
	}
}

proptest! {
	#[test]
	fn unrealized_funding_is_nonzero_iff_cum_rates_not_equal(
		market in any_market(),
		market_id in any::<MarketId>(),
		base_asset_amount in bounded_decimal(),
		quote_asset_notional_amount in bounded_decimal(),
		cum_funding_delta in bounded_decimal(),
	) {
		ExtBuilder::default().build().execute_with(|| {
			let position = Position {
				market_id,
				base_asset_amount,
				quote_asset_notional_amount,
				last_cum_funding: market.cum_funding_rate + cum_funding_delta
			};

			let result = <TestPallet as Instruments>::unrealized_funding(&market, &position).unwrap();

			assert_eq!(cum_funding_delta.is_zero(), result.is_zero());
		})
	}
}
