use crate::{
	mock::{
		accounts::ALICE,
		assets::{AssetId, DOT, PICA, USDC},
		oracle as mock_oracle,
		runtime::{
			Balance, ExtBuilder, MarketId, Origin, Runtime, System, TestPallet, Timestamp, VammId,
		},
		vamm as mock_vamm,
	},
	pallet::*,
};
use composable_traits::{
	clearing_house::{ClearingHouse, Instruments},
	oracle::Oracle,
	time::{DurationSeconds, ONE_HOUR},
	vamm::Vamm,
};
use frame_support::{
	assert_err, assert_noop, assert_ok, assert_storage_noop, pallet_prelude::Hooks,
	traits::UnixTime,
};
use orml_tokens::Error as TokenError;
use proptest::{
	num::f64::{NEGATIVE, POSITIVE, ZERO},
	prelude::*,
};
use sp_runtime::{traits::Zero, FixedI128};

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

type MarketConfig = <TestPallet as ClearingHouse>::MarketConfig;
type Market = <TestPallet as Instruments>::Market;
type Position = <TestPallet as Instruments>::Position;
type VammParams = mock_vamm::VammParams;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_types: vec![USDC],
			vamm_id: Some(0u64),
			vamm_twap: Some(FixedI128::from_float(100.0)),
			oracle_asset_support: Some(true),
			oracle_twap: Some(10_000u64),
		}
	}
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			Timestamp::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = Timestamp::set(Origin::none(), System::block_number() * 1000);
		System::on_initialize(System::block_number());
		Timestamp::on_initialize(System::block_number());
	}
}

// ----------------------------------------------------------------------------------------------------
//                                          Valid Inputs
// ----------------------------------------------------------------------------------------------------

fn valid_vamm_params() -> VammParams {
	VammParams {}
}

fn valid_market_config() -> MarketConfig {
	MarketConfig {
		asset: DOT,
		vamm_params: valid_vamm_params(),
		// 10x max leverage to open a position
		margin_ratio_initial: FixedI128::from_float(0.1),
		// liquidate when above 50x leverage
		margin_ratio_maintenance: FixedI128::from_float(0.02),
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR * 24,
	}
}

// ----------------------------------------------------------------------------------------------------
//                                           Initializers
// ----------------------------------------------------------------------------------------------------

trait MarketInitializer {
	fn init_market(self) -> Self;
}

impl MarketInitializer for sp_io::TestExternalities {
	fn init_market(mut self) -> Self {
		self.execute_with(|| {
			<TestPallet as ClearingHouse>::create_market(&valid_market_config()).unwrap();
		});

		self
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn float_ge_one()(float in ZERO | POSITIVE) -> f64 {
		1.0 + float
	}
}

prop_compose! {
	fn float_le_zero()(float in ZERO | NEGATIVE) -> f64 {
		float
	}
}

prop_compose! {
	fn zero_to_one_open_interval()(
		float in (0.0..1.0f64).prop_filter("Zero not included in (0, 1)", |num| num > &0.0)
	) -> f64 {
		float
	}
}

prop_compose! {
	fn invalid_margin_ratio_req()(
		float in prop_oneof![float_le_zero(), float_ge_one()]
	) -> FixedI128 {
		FixedI128::from_float(float)
	}
}

prop_compose! {
	fn valid_margin_ratio_req()(float in zero_to_one_open_interval()) -> FixedI128 {
		FixedI128::from_float(float)
	}
}

prop_compose! {
	fn initial_le_maintenance_margin_ratio()(
		(maintenance, decrement) in zero_to_one_open_interval()
			.prop_flat_map(|num| (Just(num), 0.0..num))
	) -> (FixedI128, FixedI128) {
		(FixedI128::from_float(maintenance - decrement), FixedI128::from_float(maintenance))
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
	fn any_decimal()(float in any::<f64>()) -> FixedI128 {
		FixedI128::from_float(float)
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
	fn bounded_decimal()(float in -1e9..1e9f64) -> FixedI128 {
		FixedI128::from_float(float)
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
		cum_funding_rate in bounded_decimal(),
		funding_rate_ts in any_duration(),
		(funding_frequency, funding_period) in funding_params()
	) -> Market {
		Market {
			vamm_id,
			asset_id,
			margin_ratio_initial,
			margin_ratio_maintenance,
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
//                                           Mocked Pallets Tests
// ----------------------------------------------------------------------------------------------------

proptest! {
	// Can we guarantee that any::<Option<Value>> will generate at least one of `Some` and `None`?
	#[test]
	fn mock_oracle_asset_support_reflects_genesis_config(oracle_asset_support in any::<Option<bool>>()) {
		ExtBuilder { oracle_asset_support, ..Default::default() }.build().execute_with(|| {
			let is_supported = <Runtime as Config>::Oracle::is_supported(DOT);
			match oracle_asset_support {
				Some(support) => assert_ok!(is_supported, support),
				None => {
					assert_err!(is_supported, mock_oracle::Error::<Runtime>::CantCheckAssetSupport)
				},
			}
		})
	}
}

proptest! {
	// Can we guarantee that any::<Option<Value>> will generate at least one of `Some` and `None`?
	#[test]
	fn mock_vamm_created_id_reflects_genesis_config(vamm_id in any::<Option<VammId>>()) {
		ExtBuilder { vamm_id , ..Default::default() }.build().execute_with(|| {
			let created = <Runtime as Config>::Vamm::create(&valid_vamm_params());
			match vamm_id {
				Some(id) => assert_ok!(created, id),
				None => assert_err!(created, mock_vamm::Error::<Runtime>::FailedToCreateVamm),
			}
		})
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Add Margin
// ----------------------------------------------------------------------------------------------------

#[test]
fn add_margin_returns_transfer_error() {
	ExtBuilder::default().build().execute_with(|| {
		let origin = Origin::signed(ALICE);
		assert_noop!(
			TestPallet::add_margin(origin, USDC, 1_000u32.into()),
			TokenError::<Runtime>::BalanceTooLow
		);
	});
}

#[test]
fn deposit_unsupported_collateral_returns_error() {
	ExtBuilder { balances: vec![(ALICE, PICA, 1_000_000)], ..Default::default() }
		.build()
		.execute_with(|| {
			let origin = Origin::signed(ALICE);
			assert_noop!(
				TestPallet::add_margin(origin, PICA, 1_000u32.into()),
				Error::<Runtime>::UnsupportedCollateralType
			);
		});
}

#[test]
fn deposit_supported_collateral_succeeds() {
	ExtBuilder { balances: vec![(ALICE, USDC, 1_000_000)], ..Default::default() }
		.build()
		.execute_with(|| {
			run_to_block(1);
			let account = ALICE;
			let asset = USDC;
			let amount: Balance = 1_000u32.into();

			let before = AccountsMargin::<Runtime>::get(&account).unwrap_or_default();
			assert_ok!(TestPallet::add_margin(Origin::signed(account), asset, amount));

			System::assert_last_event(Event::MarginAdded { account, asset, amount }.into());

			let after = AccountsMargin::<Runtime>::get(&account).unwrap_or_default();
			assert_eq!(after - before, amount);
		})
}

// ----------------------------------------------------------------------------------------------------
//                                             Create Market
// ----------------------------------------------------------------------------------------------------

#[allow(clippy::disallowed_methods)]
#[test]
fn create_first_market_succeeds() {
	ExtBuilder::default().build().execute_with(|| {
		run_to_block(10); // Timestamp unix time does not work properly at genesis
		let old_count = TestPallet::market_count();
		let block_time_now = <Timestamp as UnixTime>::now().as_secs();

		let config = valid_market_config();
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config.clone()));

		// Ensure first market id is 0 (we know its type since it's defined in the mock runtime)
		System::assert_last_event(
			Event::MarketCreated { market: 0u64, asset: config.asset }.into(),
		);
		assert!(Markets::<Runtime>::contains_key(0u64));

		// Ensure market count is increased by 1
		assert_eq!(TestPallet::market_count(), old_count + 1);

		// Ensure new market matches creation parameters
		let market = TestPallet::get_market(0u64).unwrap();
		assert_eq!(market.asset_id, config.asset);
		assert_eq!(market.margin_ratio_initial, config.margin_ratio_initial);
		assert_eq!(market.margin_ratio_maintenance, config.margin_ratio_maintenance);
		assert_eq!(market.funding_frequency, config.funding_frequency);
		assert_eq!(market.funding_period, config.funding_period);

		// Ensure last funding rate timestamp is the same as this block's time
		assert_eq!(market.funding_rate_ts, block_time_now);
	})
}

#[test]
fn fails_to_create_market_for_unsupported_asset_by_oracle() {
	ExtBuilder { oracle_asset_support: Some(false), ..Default::default() }
		.build()
		.execute_with(|| {
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), valid_market_config()),
				Error::<Runtime>::NoPriceFeedForAsset
			);
		})
}

#[test]
fn fails_to_create_market_if_fails_to_create_vamm() {
	ExtBuilder { vamm_id: None, ..Default::default() }.build().execute_with(|| {
		assert_noop!(
			TestPallet::create_market(Origin::signed(ALICE), valid_market_config()),
			mock_vamm::Error::<Runtime>::FailedToCreateVamm
		);
	})
}

proptest! {
	#[test]
	fn fails_to_create_market_if_funding_period_is_not_multiple_of_frequency(rem in 1..ONE_HOUR) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.funding_frequency = ONE_HOUR;
			config.funding_period = ONE_HOUR * 2 + rem;
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
			let mut config = valid_market_config();
			config.funding_frequency = funding_frequency;
			config.funding_period = funding_period;
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::ZeroLengthFundingPeriodOrFrequency
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_margin_ratios_not_between_zero_and_one(
		(margin_ratio_initial, margin_ratio_maintenance) in prop_oneof![
			(valid_margin_ratio_req(), invalid_margin_ratio_req()),
			(invalid_margin_ratio_req(), valid_margin_ratio_req()),
			(invalid_margin_ratio_req(), invalid_margin_ratio_req())
		]
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.margin_ratio_initial = margin_ratio_initial;
			config.margin_ratio_maintenance = margin_ratio_maintenance;
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::InvalidMarginRatioRequirement
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_create_market_if_initial_margin_ratio_le_maintenance(
		(margin_ratio_initial, margin_ratio_maintenance) in initial_le_maintenance_margin_ratio()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.margin_ratio_initial = margin_ratio_initial;
			config.margin_ratio_maintenance = margin_ratio_maintenance;
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::InitialMarginRatioLessThanMaintenance
			);
		})
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
				mock_oracle::Error::<Runtime>::CantComputeTwap
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
				mock_vamm::Error::<Runtime>::FailedToCalculateTwap
			);
		})
	}
}

proptest! {
	#[test]
	fn funding_owed_query_leaves_storage_intact(
		market in any_market(), position in any_position()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_storage_noop!(
				assert_ok!(<TestPallet as Instruments>::funding_owed(&market, &position))
			);
		})
	}
}

proptest! {
	#[test]
	fn funding_owed_is_nonzero_iff_cum_rates_not_equal(
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

			let result = <TestPallet as Instruments>::funding_owed(&market, &position).unwrap();

			assert_eq!(cum_funding_delta.is_zero(), result.is_zero());
		})
	}
}
