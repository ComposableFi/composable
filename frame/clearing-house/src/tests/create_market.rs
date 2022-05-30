use crate::{
	mock::{
		self as mock,
		accounts::ALICE,
		runtime::{
			ExtBuilder, Origin, Runtime, System as SystemPallet, TestPallet,
			Timestamp as TimestampPallet,
		},
	},
	pallet::{Error, Event, Markets},
	tests::{as_inner, run_to_block, MarketConfig},
};
use composable_traits::time::{DurationSeconds, ONE_HOUR};
use frame_support::{assert_noop, assert_ok, traits::UnixTime};
use proptest::prelude::*;
use sp_runtime::{FixedI128, FixedPointNumber};

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn decimal_gt_zero_lt_one()(inner in 1..as_inner(1)) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn decimal_gt_zero_le_one()(inner in 1..=as_inner(1)) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn decimal_le_zero()(inner in i128::MIN..=0) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn decimal_ge_one()(inner in as_inner(1)..=i128::MAX) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn decimal_gt_one()(inner in (as_inner(1) + 1)..=i128::MAX) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn invalid_imr()(
		decimal in prop_oneof![decimal_le_zero(), decimal_gt_one()]
	) -> FixedI128 {
		decimal
	}
}

prop_compose! {
	fn invalid_mmr()(
		decimal in prop_oneof![decimal_le_zero(), decimal_ge_one()]
	) -> FixedI128 {
		decimal
	}
}

prop_compose! {
	fn invalid_pmr()(
		decimal in prop_oneof![decimal_le_zero(), decimal_ge_one()]
	) -> FixedI128 {
		decimal
	}
}

prop_compose! {
	// 0 < _ <= input
	fn decimal_gt_zero_le_input(input: FixedI128)(
		inner in 1..=input.into_inner()
	) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	// lower < _ < upper
	fn decimal_in_exclusive_range(lower: FixedI128, upper: FixedI128)(
		inner in (lower.into_inner() + 1)..upper.into_inner()
	) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	// lower < smaller < bigger < upper
	fn distinct_decimal_pair_in_exclusive_range(lower: FixedI128, upper: FixedI128)(
		// leave at least one value for smaller decimal
		bigger in decimal_in_exclusive_range(FixedI128::from_inner(lower.into_inner() + 1), upper)
	)(
		bigger in Just(bigger),
		smaller in decimal_in_exclusive_range(lower, bigger),
	) -> (FixedI128, FixedI128) {
		(smaller, bigger)
	}
}

prop_compose! {
	fn initial_le_partial_margin_ratio()(
		(maint, partial) in distinct_decimal_pair_in_exclusive_range(0.into(), 1.into())
	)(
		initial in decimal_gt_zero_le_input(partial),
		(maint, partial) in Just((maint, partial)),
	) -> (FixedI128, FixedI128, FixedI128) {
		(initial, maint, partial)
	}
}

prop_compose! {
	// lower < _ <= upper
	fn decimal_in_half_inclusive_range(lower: FixedI128, upper: FixedI128)(
		inner in (lower.into_inner() + 1)..=upper.into_inner()
	) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	// lower < smaller < bigger <= upper
	fn distinct_decimal_pair_in_half_inclusive_range(lower: FixedI128, upper: FixedI128)(
		// leave at least one value for smaller decimal
		bigger in decimal_in_half_inclusive_range(
			FixedI128::from_inner(lower.into_inner() + 1),
			upper
		)
	)(
		bigger in Just(bigger),
		smaller in decimal_in_exclusive_range(lower, bigger),
	) -> (FixedI128, FixedI128) {
		(smaller, bigger)
	}
}

prop_compose! {
	fn partial_le_maintenance_margin_ratio()(
		(maint, initial) in distinct_decimal_pair_in_half_inclusive_range(0.into(), 1.into())
	)(
		(initial, maint) in Just((initial, maint)),
		partial in decimal_in_half_inclusive_range(0.into(), maint)
	) -> (FixedI128, FixedI128, FixedI128) {
		(initial, maint, partial)
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Create Market
// ----------------------------------------------------------------------------------------------------

#[test]
fn create_first_market_succeeds() {
	ExtBuilder::default().build().execute_with(|| {
		run_to_block(10); // TimestampPallet unix time does not work properly at genesis
		let old_count = TestPallet::market_count();
		let block_time_now = <TimestampPallet as UnixTime>::now().as_secs();

		let config = MarketConfig::default();
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config.clone()));

		// Ensure first market id is 0 (we know its type since it's defined in the mock runtime)
		SystemPallet::assert_last_event(
			Event::MarketCreated { market: 0_u64, asset: config.asset }.into(),
		);
		assert!(Markets::<Runtime>::contains_key(0_u64));

		// Ensure market count is increased by 1
		assert_eq!(TestPallet::market_count(), old_count + 1);

		// Ensure new market matches creation parameters
		let market = TestPallet::get_market(0_u64).unwrap();
		assert_eq!(market.asset_id, config.asset);
		assert_eq!(market.margin_ratio_initial, config.margin_ratio_initial);
		assert_eq!(market.margin_ratio_maintenance, config.margin_ratio_maintenance);
		assert_eq!(market.minimum_trade_size, config.minimum_trade_size);
		assert_eq!(market.funding_frequency, config.funding_frequency);
		assert_eq!(market.funding_period, config.funding_period);
		assert_eq!(market.taker_fee, config.taker_fee);

		assert_eq!(market.base_asset_amount_long, 0.into());
		assert_eq!(market.base_asset_amount_short, 0.into());
		assert_eq!(market.cum_funding_rate_long, 0.into());
		assert_eq!(market.cum_funding_rate_short, 0.into());
		assert_eq!(market.fee_pool, 0);
		// Ensure last funding rate timestamp is the same as this block's time
		assert_eq!(market.funding_rate_ts, block_time_now);
	})
}

#[test]
fn can_create_two_markets_with_same_config() {
	ExtBuilder::default().build().execute_with(|| {
		run_to_block(2);
		let mut count = TestPallet::market_count();
		let block_time_now = <TimestampPallet as UnixTime>::now().as_secs();

		for _ in 0..2 {
			assert_ok!(TestPallet::create_market(Origin::signed(ALICE), Default::default()));

			assert_eq!(TestPallet::get_market(count).unwrap().funding_rate_ts, block_time_now);
			count += 1;
		}
	})
}

#[test]
fn fails_to_create_market_for_unsupported_asset_by_oracle() {
	ExtBuilder { oracle_asset_support: Some(false), ..Default::default() }
		.build()
		.execute_with(|| {
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), Default::default()),
				Error::<Runtime>::NoPriceFeedForAsset
			);
		})
}

#[test]
fn fails_to_create_market_if_fails_to_create_vamm() {
	ExtBuilder { vamm_id: None, ..Default::default() }.build().execute_with(|| {
		assert_noop!(
			TestPallet::create_market(Origin::signed(ALICE), Default::default()),
			mock::vamm::Error::<Runtime>::FailedToCreateVamm
		);
	})
}

proptest! {
	#[test]
	fn fails_to_create_market_if_funding_period_is_not_multiple_of_frequency(rem in 1..ONE_HOUR) {
		ExtBuilder::default().build().execute_with(|| {
			let config = MarketConfig {
				funding_frequency: ONE_HOUR,
				funding_period: ONE_HOUR * 2 + rem,
				..Default::default()
			};
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::FundingPeriodNotMultipleOfFrequency
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_either_funding_period_or_frequency_are_zero(
		(funding_period, funding_frequency) in prop_oneof![
			(Just(0), any::<DurationSeconds>()),
			(any::<DurationSeconds>(), Just(0)),
			Just((0, 0))
		]
	) {
		ExtBuilder::default().build().execute_with(|| {
			let config = MarketConfig { funding_frequency, funding_period, ..Default::default() };
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::ZeroLengthFundingPeriodOrFrequency
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_margin_ratios_not_in_valid_range(
		(margin_ratio_initial, margin_ratio_maintenance, margin_ratio_partial) in prop_oneof![
			(decimal_gt_zero_le_one(), decimal_gt_zero_lt_one(), invalid_pmr()),
			(decimal_gt_zero_le_one(), invalid_mmr(),            decimal_gt_zero_lt_one()),
			(decimal_gt_zero_le_one(), invalid_mmr(),            invalid_pmr()),
			(invalid_mmr(),            decimal_gt_zero_lt_one(), decimal_gt_zero_lt_one()),
			(invalid_mmr(),            decimal_gt_zero_lt_one(), invalid_pmr()),
			(invalid_mmr(),            invalid_mmr(),            decimal_gt_zero_lt_one()),
			(invalid_mmr(),            invalid_mmr(),            invalid_pmr()),
		]
	) {
		ExtBuilder::default().build().execute_with(|| {
			let config = MarketConfig {
				margin_ratio_initial,
				margin_ratio_maintenance,
				margin_ratio_partial,
				..Default::default()
			};
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::InvalidMarginRatioRequirement
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_initial_margin_ratio_le_partial(
		(margin_ratio_initial, margin_ratio_maintenance, margin_ratio_partial)
			in initial_le_partial_margin_ratio()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let config = MarketConfig {
				margin_ratio_initial,
				margin_ratio_maintenance,
				margin_ratio_partial,
				..Default::default()
			};
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::InvalidMarginRatioOrdering
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_partial_margin_ratio_le_maintenance(
		(margin_ratio_initial, margin_ratio_maintenance, margin_ratio_partial)
			in partial_le_maintenance_margin_ratio()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let config = MarketConfig {
				margin_ratio_initial,
				margin_ratio_maintenance,
				margin_ratio_partial,
				..Default::default()
			};
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::InvalidMarginRatioOrdering
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_minimum_trade_size_is_negative(
		inner in as_inner(-1_000_000_000)..0
	) {
		ExtBuilder::default().build().execute_with(|| {
			let config = MarketConfig {
				minimum_trade_size: FixedI128::from_inner(inner),
				..Default::default()
			};
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::NegativeMinimumTradeSize
			);
		})
	}
}

#[test]
fn can_create_market_with_zero_minimum_trade_size() {
	ExtBuilder::default().build().execute_with(|| {
		let config = MarketConfig { minimum_trade_size: 0.into(), ..Default::default() };
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));
	})
}

#[test]
fn can_create_market_with_zero_taker_fees() {
	ExtBuilder::default().build().execute_with(|| {
		let config = MarketConfig { taker_fee: 0, ..Default::default() };
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));
	})
}
