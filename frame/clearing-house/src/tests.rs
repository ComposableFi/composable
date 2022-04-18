use crate::{
	mock::{
		accounts::{AccountId, ALICE},
		assets::{AssetId, DOT, PICA, USDC},
		oracle as mock_oracle,
		runtime::{
			Balance, ExtBuilder, MarketId, Oracle as OraclePallet, Origin, Runtime,
			System as SystemPallet, TestPallet, Timestamp as TimestampPallet, Vamm as VammPallet,
			VammId,
		},
		vamm as mock_vamm,
	},
	pallet::*,
};
use composable_traits::{
	clearing_house::{ClearingHouse, Instruments},
	oracle::Oracle,
	time::{DurationSeconds, ONE_HOUR},
	vamm::{AssetType, Direction as VammDirection, Vamm},
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
use sp_runtime::{traits::Zero, FixedI128, FixedPointNumber};

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

type Market = <TestPallet as Instruments>::Market;
type MarketConfig = <TestPallet as ClearingHouse>::MarketConfig;
type Position = <TestPallet as Instruments>::Position;
type SwapConfig = <VammPallet as Vamm>::SwapConfig;
type SwapSimulationConfig = <VammPallet as Vamm>::SwapSimulationConfig;
type VammConfig = mock_vamm::VammConfig;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_types: vec![USDC],
			vamm_id: Some(0_u64),
			vamm_twap: Some(FixedI128::from_float(100.0)),
			oracle_asset_support: Some(true),
			oracle_twap: Some(10_000_u64),
		}
	}
}

fn run_to_block(n: u64) {
	while SystemPallet::block_number() < n {
		if SystemPallet::block_number() > 0 {
			TimestampPallet::on_finalize(SystemPallet::block_number());
			SystemPallet::on_finalize(SystemPallet::block_number());
		}
		SystemPallet::set_block_number(SystemPallet::block_number() + 1);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = TimestampPallet::set(Origin::none(), SystemPallet::block_number() * 1000);
		SystemPallet::on_initialize(SystemPallet::block_number());
		TimestampPallet::on_initialize(SystemPallet::block_number());
	}
}

// ----------------------------------------------------------------------------------------------------
//                                          Valid Inputs
// ----------------------------------------------------------------------------------------------------

fn valid_vamm_config() -> VammConfig {
	VammConfig {}
}

fn valid_market_config() -> MarketConfig {
	MarketConfig {
		asset: DOT,
		vamm_config: valid_vamm_config(),
		// 10x max leverage to open a position
		margin_ratio_initial: FixedI128::from_float(0.1),
		// liquidate when above 50x leverage
		margin_ratio_maintenance: FixedI128::from_float(0.02),
		// 'One cent' of the quote asset
		minimum_trade_size: FixedI128::from_float(0.01),
		funding_frequency: ONE_HOUR,
		funding_period: ONE_HOUR * 24,
	}
}

fn valid_quote_asset_amount() -> Balance {
	FixedI128::checked_from_integer(100).unwrap().into_inner().unsigned_abs()
}

fn valid_base_asset_amount_limit() -> Balance {
	FixedI128::checked_from_integer(10).unwrap().into_inner().unsigned_abs()
}

// ----------------------------------------------------------------------------------------------------
//                                           Initializers
// ----------------------------------------------------------------------------------------------------

trait MarginInitializer {
	fn add_margin(self, account_id: &AccountId, asset_id: AssetId, amount: Balance) -> Self;
}

impl MarginInitializer for sp_io::TestExternalities {
	fn add_margin(mut self, account_id: &AccountId, asset_id: AssetId, amount: Balance) -> Self {
		self.execute_with(|| {
			assert_ok!(<TestPallet as ClearingHouse>::add_margin(account_id, asset_id, amount));
		});

		self
	}
}

trait MarketInitializer {
	fn init_market(self, market_id: &mut MarketId, config: Option<MarketConfig>) -> Self;
	fn init_markets<T>(self, market_ids: &mut Vec<MarketId>, configs: T) -> Self
	where
		T: Iterator<Item = Option<MarketConfig>>;

	fn create_market_helper(config: Option<MarketConfig>) -> MarketId {
		<TestPallet as ClearingHouse>::create_market(&match config {
			Some(c) => c,
			None => valid_market_config(),
		})
		.unwrap()
	}
}

impl MarketInitializer for sp_io::TestExternalities {
	fn init_market(mut self, market_id: &mut MarketId, config: Option<MarketConfig>) -> Self {
		self.execute_with(|| {
			*market_id = Self::create_market_helper(config);
		});

		self
	}

	fn init_markets<T>(mut self, market_ids: &mut Vec<MarketId>, configs: T) -> Self
	where
		T: Iterator<Item = Option<MarketConfig>>,
	{
		self.execute_with(|| {
			for config in configs {
				market_ids.push(Self::create_market_helper(config));
			}
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
		float in (0.0..1.0_f64).prop_filter("Zero not included in (0, 1)", |num| num > &0.0)
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
	fn any_decimal()(inner in any::<i128>()) -> FixedI128 {
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
	fn bounded_decimal()(
		inner in (-1_000_000_000 * FixedI128::DIV)..(1_000_000_000 * FixedI128::DIV)
	) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn any_minimum_trade_size()(float in 0.0..1e9_f64) -> FixedI128 {
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

fn any_asset_type() -> impl Strategy<Value = AssetType> {
	prop_oneof![Just(AssetType::Base), Just(AssetType::Quote)]
}

fn any_vamm_direction() -> impl Strategy<Value = VammDirection> {
	prop_oneof![Just(VammDirection::Add), Just(VammDirection::Remove)]
}

prop_compose! {
	fn any_swap_config()(
		vamm_id in any::<VammId>(),
		asset in any_asset_type(),
		input_amount in any::<Balance>(),
		direction in any_vamm_direction(),
		output_amount_limit in any::<Balance>(),
	) -> SwapConfig {
		SwapConfig {
			vamm_id, asset, input_amount, direction, output_amount_limit
		}
	}
}

prop_compose! {
	fn any_swap_simulation_config()(
		vamm_id in any::<VammId>(),
		asset in any_asset_type(),
		input_amount in any::<Balance>(),
		direction in any_vamm_direction(),
	) -> SwapSimulationConfig {
		SwapSimulationConfig { vamm_id, asset, input_amount, direction }
	}
}

prop_compose! {
	// Assumes min_fixed is positive and nonzero
	fn min_trade_size_and_eps(min_fixed: FixedI128)(
		eps in (-min_fixed.into_inner())..=min_fixed.into_inner()
	) -> (FixedI128, i128) {
		(min_fixed, eps)
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
			let is_supported = OraclePallet::is_supported(DOT);
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
			let created = VammPallet::create(&valid_vamm_config());
			match vamm_id {
				Some(id) => assert_ok!(created, id),
				None => assert_err!(created, mock_vamm::Error::<Runtime>::FailedToCreateVamm),
			}
		})
	}
}

proptest! {
	#[test]
	fn can_set_swap_output_for_mock_vamm(
		integer in any::<Option<i128>>(), config in any_swap_config()
	) {
		ExtBuilder::default()
			.build()
			.execute_with(|| {
				VammPallet::set_swap_output(integer);

				let output = VammPallet::swap(&config);
				match integer {
					Some(i) => assert_ok!(output, i),
					None => assert_err!(output, mock_vamm::Error::<Runtime>::FailedToExecuteSwap)
				};
			})
	}
}

proptest! {
	#[test]
	fn cat_set_swap_simulation_output_for_mock_vamm(
		integer in any::<Option<i128>>(), config in any_swap_simulation_config()
	) {
		ExtBuilder::default()
			.build()
			.execute_with(|| {
				VammPallet::set_swap_simulation_output(integer);

				let output = VammPallet::swap_simulation(&config);
				match integer {
					Some(i) => assert_ok!(output, i),
					None => assert_err!(output, mock_vamm::Error::<Runtime>::FailedToSimulateSwap)
				};
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
			TestPallet::add_margin(origin, USDC, 1_000_u32.into()),
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
				TestPallet::add_margin(origin, PICA, 1_000_u32.into()),
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
			let amount: Balance = 1_000_u32.into();

			let before = AccountsMargin::<Runtime>::get(&account).unwrap_or_default();
			assert_ok!(TestPallet::add_margin(Origin::signed(account), asset, amount));

			SystemPallet::assert_last_event(Event::MarginAdded { account, asset, amount }.into());

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
		run_to_block(10); // TimestampPallet unix time does not work properly at genesis
		let old_count = TestPallet::market_count();
		let block_time_now = <TimestampPallet as UnixTime>::now().as_secs();

		let config = valid_market_config();
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
		assert_eq!(market.funding_frequency, config.funding_frequency);
		assert_eq!(market.funding_period, config.funding_period);

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
			assert_ok!(TestPallet::create_market(Origin::signed(ALICE), valid_market_config()));

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

proptest! {
	#[test]
	fn fails_to_create_market_if_minimum_trade_size_is_negative(
		float in (-1e9_f64)..0.0
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.minimum_trade_size = FixedI128::from_float(float);
			assert_noop!(
				TestPallet::create_market(Origin::signed(ALICE), config),
				Error::<Runtime>::NegativeMinimumTradeSize
			);
		})
	}
}

// ----------------------------------------------------------------------------------------------------
//                                            Open Position
// ----------------------------------------------------------------------------------------------------

#[test]
fn fails_to_open_position_if_market_id_invalid() {
	let mut market_id: MarketId = 0;
	let quote_amount = valid_quote_asset_amount();
	let base_amount_limit = valid_base_asset_amount_limit();

	ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
		.build()
		.init_market(&mut market_id, None)
		.add_margin(&ALICE, USDC, quote_amount)
		.execute_with(|| {
			VammPallet::set_swap_output(Some(base_amount_limit.try_into().unwrap()));

			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_id + 1,
					Direction::Long,
					quote_amount,
					base_amount_limit
				),
				Error::<Runtime>::MarketIdNotFound,
			);
		})
}

#[test]
fn open_position_in_new_market_increases_number_of_positions() {
	let mut market_id: MarketId = 0;
	let quote_amount = valid_quote_asset_amount();
	let base_amount_limit = valid_base_asset_amount_limit();

	ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
		.build()
		.init_market(&mut market_id, None)
		.add_margin(&ALICE, USDC, quote_amount)
		.execute_with(|| {
			VammPallet::set_swap_output(Some(base_amount_limit.try_into().unwrap()));

			let positions_before = TestPallet::get_positions(&ALICE).len();
			assert_ok!(TestPallet::open_position(
				Origin::signed(ALICE),
				market_id,
				Direction::Long,
				quote_amount,
				base_amount_limit,
			));
			assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before + 1);
		})
}

#[test]
fn fails_to_create_new_position_if_violates_maximum_positions_num() {
	let max_positions = <Runtime as Config>::MaxPositions::get() as usize;
	let mut market_ids = Vec::<_>::new();
	let orders = max_positions + 1;
	let configs = vec![None; orders];

	let quote_amount_total = valid_quote_asset_amount();
	let quote_amount: Balance = quote_amount_total / (orders as u128);
	let base_amount_limit: Balance = valid_base_asset_amount_limit() / (orders as u128);

	ExtBuilder { balances: vec![(ALICE, USDC, quote_amount_total)], ..Default::default() }
		.build()
		.init_markets(&mut market_ids, configs.into_iter())
		.add_margin(&ALICE, USDC, quote_amount_total)
		.execute_with(|| {
			VammPallet::set_swap_output(Some(base_amount_limit.try_into().unwrap()));

			for market_id in market_ids.iter().take(max_positions) {
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					*market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit,
				));
			}
			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_ids[max_positions],
					Direction::Long,
					quote_amount,
					base_amount_limit,
				),
				Error::<Runtime>::MaxPositionsExceeded
			);
		})
}

proptest! {
	#[test]
	fn short_trade_can_close_long_position(
		(minimum_trade_size, eps) in min_trade_size_and_eps(FixedI128::from_float(0.01))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = valid_quote_asset_amount();
		let base_amount_limit: i128 = valid_base_asset_amount_limit().try_into().unwrap();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount * 2)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_swap_output(Some(base_amount_limit));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit.unsigned_abs(),
				));

				VammPallet::set_swap_output(Some(-base_amount_limit));
				VammPallet::set_swap_simulation_output(Some(
					i128::try_from(quote_amount).unwrap() + eps
				));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Short,
					quote_amount,
					base_amount_limit.unsigned_abs(),
				));

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
		})
	}
}

proptest! {
	#[test]
	fn long_trade_can_close_long_position(
		(minimum_trade_size, eps) in min_trade_size_and_eps(FixedI128::from_float(0.01))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = valid_quote_asset_amount();
		let base_amount_limit: i128 = valid_base_asset_amount_limit().try_into().unwrap();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount * 2)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_swap_output(Some(-base_amount_limit));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Short,
					quote_amount,
					base_amount_limit.unsigned_abs(),
				));

				VammPallet::set_swap_output(Some(base_amount_limit));
				VammPallet::set_swap_simulation_output(Some(
					-(i128::try_from(quote_amount).unwrap() + eps)
				));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit.unsigned_abs(),
				));

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
			})
	}
}

proptest! {
	#[test]
	fn closing_position_with_trade_realizes_pnl(pnl_decimal in bounded_decimal()) {
		let mut market_id: MarketId = 0;

		let quote_amount = valid_quote_asset_amount() as i128;
		let quote_amount_abs = quote_amount.unsigned_abs();
		let base_amount_limit = valid_base_asset_amount_limit() as i128;
		let margin = quote_amount;
		let pnl = pnl_decimal.into_inner();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount_abs * 2)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(valid_market_config()))
			.add_margin(&ALICE, USDC, quote_amount_abs)
			.execute_with(|| {
				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_swap_output(Some(base_amount_limit));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					quote_amount_abs,
					base_amount_limit.unsigned_abs(),
				));

				// Set price of base so that it should give the desired PnL in quote
				VammPallet::set_swap_output(Some(quote_amount + pnl));
				VammPallet::set_swap_simulation_output(Some(quote_amount + pnl));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Short,
					(quote_amount + pnl).unsigned_abs(),
					base_amount_limit.unsigned_abs(),
				));

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap() as i128,
					(margin + pnl).max(0)
				)
		})
	}
}

#[test]
#[ignore = "to be implemented"]
fn fails_to_increase_position_if_not_enough_margin() {
	let mut market_id: MarketId = 0;
	let base_amount_limit = valid_base_asset_amount_limit().try_into().unwrap();

	ExtBuilder::default()
		.build()
		.init_market(&mut market_id, None)
		.execute_with(|| {
			VammPallet::set_swap_output(Some(base_amount_limit));
			assert_noop!(
				TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					valid_quote_asset_amount(),
					base_amount_limit.unsigned_abs(),
				),
				Error::<Runtime>::InsufficientCollateral,
			);
		})
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
