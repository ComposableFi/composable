#![allow(clippy::disallowed_methods)] // Allow use of .unwrap() in tests

use crate::{
	math::FromBalance,
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
use sp_runtime::{traits::Zero, FixedI128, FixedPointNumber, FixedU128};

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
			vamm_twap: Some(100.into()),
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

/// Return the balance representation of the input value, according to the precision of the fixed
/// point implementation
fn as_balance<T: Into<FixedI128>>(value: T) -> u128 {
	as_inner(value) as u128
}

/// Return the inner integer of the fixed point representation of the input value
fn as_inner<T: Into<FixedI128>>(value: T) -> i128 {
	let f: FixedI128 = value.into();
	f.into_inner()
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
	as_balance(100)
}

fn valid_base_asset_amount_limit() -> Balance {
	as_balance(10)
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
		inner in as_inner(-1_000_000_000)..as_inner(1_000_000_000)
	) -> FixedI128 {
		FixedI128::from_inner(inner)
	}
}

prop_compose! {
	fn any_minimum_trade_size()(inner in 0..as_inner(1_000_000_000)) -> FixedI128 {
		FixedI128::from_inner(inner)
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

fn any_direction() -> impl Strategy<Value = Direction> {
	prop_oneof![Just(Direction::Long), Just(Direction::Short)]
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
	fn min_trade_size_and_eps(min_size: u128)(
		eps in -(min_size as i128)..=(min_size as i128)
	) -> (FixedI128, i128) {
		// Couldn't find a better way to ensure that min_size is positive, so this will trigger a
		// test error otherwise
		assert!(min_size > 0);
		(FixedI128::from_inner(min_size as i128), eps)
	}
}

prop_compose! {
	fn any_price()(inner in 1..=as_balance(1_000_000_000)) -> FixedU128 {
		FixedU128::from_inner(inner)
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
	fn can_set_price_for_mock_vamm_swap(
		price in prop_oneof![any_price().prop_map(Some), Just(None)],
		config in any_swap_config()
	) {
		ExtBuilder::default()
			.build()
			.execute_with(|| {
				VammPallet::set_price(price);

				let output = VammPallet::swap(&config);
				match price {
					Some(_) => {
						assert_ok!(output);
					},
					None => {
						assert_err!(output, mock_vamm::Error::<Runtime>::FailedToExecuteSwap);
					}
				};
			})
	}
}

proptest! {
	#[test]
	fn can_set_price_for_mock_vamm_swap_simulation(
		price in prop_oneof![any_price().prop_map(Some), Just(None)],
		config in any_swap_simulation_config()
	) {
		ExtBuilder::default()
			.build()
			.execute_with(|| {
				VammPallet::set_price(price);

				let output = VammPallet::swap_simulation(&config);
				match price {
					Some(_) => {
						assert_ok!(output);
					},
					None => {
						assert_err!(output, mock_vamm::Error::<Runtime>::FailedToSimulateSwap);
					}
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
		inner in as_inner(-1_000_000_000)..0
	) {
		ExtBuilder::default().build().execute_with(|| {
			let mut config = valid_market_config();
			config.minimum_trade_size = FixedI128::from_inner(inner);
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
		let mut config = valid_market_config();
		config.minimum_trade_size = 0.into();
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));
	})
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
			// Current price = quote_amount / base_amount_limit
			VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

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

proptest! {
	#[test]
	fn open_position_in_new_market_succeeds(
		direction in any_direction()
	) {
		let mut market_id: MarketId = 0;
		let quote_amount = valid_quote_asset_amount();
		let base_amount = valid_base_asset_amount_limit();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, None)
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				// Current price = quote_amount / base_amount
				VammPallet::set_price(Some((quote_amount, base_amount).into()));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					direction,
					quote_amount,
					base_amount,
				));

				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);
				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				assert!(match direction {
					Direction::Long => position.base_asset_amount.is_positive(),
					Direction::Short => position.base_asset_amount.is_negative()
				});
				assert!(match direction {
					Direction::Long => position.quote_asset_notional_amount.is_positive(),
					Direction::Short => position.quote_asset_notional_amount.is_negative()
				});

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction,
						quote: quote_amount,
						base: base_amount,
					}.into()
				);
			})
	}
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
			// Current price = quote_amount / base_amount_limit
			VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));

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
	fn fails_to_open_position_if_trade_size_too_small(
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = eps.unsigned_abs();
		let direction = match eps.is_positive() {
			true => Direction::Long,
			false => Direction::Short,
		};
		let base_asset_amount_limit = eps; // Arbitrary (price = 1 in this case)

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				VammPallet::set_price(Some((quote_amount, base_asset_amount_limit).into()));
				assert_noop!(
					TestPallet::open_position(
						Origin::signed(ALICE),
						market_id,
						direction,
						quote_amount,
						base_asset_amount_limit.unsigned_abs()
					),
					Error::<Runtime>::TradeSizeTooSmall
				);
			})
	}
}

proptest! {
	#[test]
	fn short_trade_can_close_long_position_within_tolerance(
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = valid_quote_asset_amount();
		let base_amount_limit = valid_base_asset_amount_limit();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				// price * base_amount_limit = quote_amount
				VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit,
				));

				// price' * base_amount_limit = (quote_amount + eps)
				VammPallet::set_price(Some(
					((quote_amount as i128 + eps).unsigned_abs(), base_amount_limit).into()
				));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Short,
					quote_amount,
					base_amount_limit,
				));

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: quote_amount,
						base: base_amount_limit,
					}.into()
				);
		})
	}
}

proptest! {
	#[test]
	fn long_trade_can_close_short_position_within_tolerance(
		(minimum_trade_size, eps) in min_trade_size_and_eps(as_balance((1, 100)))
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = minimum_trade_size;

		let quote_amount = valid_quote_asset_amount();
		let base_amount_limit = valid_base_asset_amount_limit();

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				// price * base_amount_limit = quote_amount
				VammPallet::set_price(Some((quote_amount, base_amount_limit).into()));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Short,
					quote_amount,
					base_amount_limit,
				));

				// price' * base_amount_limit = (quote_amount + eps)
				VammPallet::set_price(Some(
					((quote_amount as i128 + eps).unsigned_abs(), base_amount_limit).into()
				));
				assert_ok!(TestPallet::open_position(
					Origin::signed(ALICE),
					market_id,
					Direction::Long,
					quote_amount,
					base_amount_limit,
				));

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: quote_amount,
						base: base_amount_limit,
					}.into()
				);
			})
	}
}

proptest! {
	#[test]
	fn closing_long_position_with_trade_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(valid_market_config()))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount_limit = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_amount,
						base_amount_limit,
					),
					base_amount_limit,
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount_limit);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						new_base_value,
						base_amount_limit,
					),
					base_amount_limit
				);

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
				let pnl = new_base_value as i128 - margin;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: new_base_value,
						base: base_amount_limit,
					}.into()
				);
		})
	}
}

proptest! {
	#[test]
	fn closing_short_position_with_trade_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;

		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						new_base_value,
						base_amount,
					),
					base_amount
				);

				assert_eq!(TestPallet::get_positions(&ALICE).len(), positions_before);
				let pnl = margin - new_base_value as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl).max(0) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: new_base_value,
						base: base_amount,
					}.into()
				);
		})
	}
}

proptest! {
	#[test]
	fn reducing_long_position_partially_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_amount,
						base_amount,
					),
					base_amount
				);


				VammPallet::set_price(Some(new_price));
				let base_value_to_close =
					(new_price * FixedU128::from_inner(base_amount / 2)).into_inner();
				// Reduce (close) position by 50%
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						base_value_to_close,
						base_amount / 2,
					),
					base_amount / 2,
				);

				let positions = TestPallet::get_positions(&ALICE);
				// Positions remains open
				assert_eq!(positions.len(), positions_before + 1);

				// 50% of the PnL is realized
				let pnl = base_value_to_close as i128 - (quote_amount / 2) as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl) as u128
				);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				// Position base asset and quote asset notional are cut in half
				assert_eq!(position.base_asset_amount.into_inner(), (base_amount / 2) as i128);
				assert_eq!(
					position.quote_asset_notional_amount.into_inner(),
					(quote_amount / 2) as i128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: base_value_to_close,
						base: base_amount / 2,
					}.into()
				);
			})
	}
}

proptest! {
	#[test]
	fn reducing_short_position_partially_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				// Initial price = 10
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				// Reduce (close) position by 50%
				let base_value_to_close =
					new_price.saturating_mul_int(base_amount / 2);
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						base_value_to_close,
						base_amount / 2,
					),
					base_amount / 2
				);

				// Positions remains open
				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);

				// 50% of the PnL is realized
				let pnl = margin / 2 - base_value_to_close as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl).max(0) as u128
				);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				// Position base asset and quote asset notional are cut in half
				assert_eq!(position.base_asset_amount.into_inner(), -(base_amount as i128) / 2);
				assert_eq!(
					position.quote_asset_notional_amount.into_inner(),
					-(quote_amount as i128) / 2
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: base_value_to_close,
						base: base_amount / 2,
					}.into()
				);
			})
	}
}

proptest! {
	#[test]
	fn reversing_long_position_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount);
				// We want to end up with the reverse of the position (in base tokens)
				// Now:
				// base = new_base_value
				// Goal:
				// -base = -new_base_value
				// Delta:
				// base * 2 = new_base_value * 2
				let base_delta = base_amount * 2;
				let quote_delta = new_base_value * 2;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_delta,
						base_delta,
					),
					base_delta
				);

				// Position remains open
				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				assert_eq!(
					position.base_asset_amount,
					-FixedI128::from_balance(base_amount).unwrap());
				assert_eq!(
					position.quote_asset_notional_amount,
					-FixedI128::from_balance(new_base_value).unwrap()
				);

				// Full PnL is realized
				let pnl = new_base_value as i128 - margin;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Short,
						quote: quote_delta,
						base: base_delta,
					}.into()
				);
			})
	}
}

proptest! {
	#[test]
	fn reversing_short_position_realizes_pnl(new_price in any_price()) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.minimum_trade_size = 0.into();

		let quote_amount = as_balance(100);
		let margin = quote_amount as i128;
		ExtBuilder { balances: vec![(ALICE, USDC, quote_amount)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, quote_amount)
			.execute_with(|| {
				// For event emission
				run_to_block(1);

				let positions_before = TestPallet::get_positions(&ALICE).len();

				VammPallet::set_price(Some(10.into()));
				let base_amount = quote_amount / 10;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Short,
						quote_amount,
						base_amount,
					),
					base_amount
				);

				VammPallet::set_price(Some(new_price));
				let new_base_value = new_price.saturating_mul_int(base_amount);
				// We want to end up with the reverse of the position (in base tokens)
				// Now:
				// -base = -new_base_value
				// Goal:
				// base = new_base_value
				// Delta:
				// -base * 2 = -new_base_value * 2
				let base_delta = base_amount * 2;
				let quote_delta = new_base_value * 2;
				assert_ok!(
					<TestPallet as ClearingHouse>::open_position(
						&ALICE,
						&market_id,
						Direction::Long,
						quote_delta,
						base_delta,
					),
					base_delta
				);

				// Position remains open
				let positions = TestPallet::get_positions(&ALICE);
				assert_eq!(positions.len(), positions_before + 1);

				let position = positions.iter().find(|p| p.market_id == market_id).unwrap();
				assert_eq!(
					position.base_asset_amount,
					FixedI128::from_balance(base_amount).unwrap()
				);
				assert_eq!(
					position.quote_asset_notional_amount,
					FixedI128::from_balance(new_base_value).unwrap()
				);

				// Full PnL is realized
				let pnl = margin - new_base_value as i128;
				assert_eq!(
					TestPallet::get_margin(&ALICE).unwrap(),
					(margin + pnl).max(0) as u128
				);

				SystemPallet::assert_last_event(
					Event::TradeExecuted {
						market: market_id,
						direction: Direction::Long,
						quote: quote_delta,
						base: base_delta,
					}.into()
				);
			})
	}
}

proptest! {
	#[test]
	fn fails_to_increase_position_if_not_enough_margin(
		direction in any_direction(),
		excess in 1..as_balance(1_000_000),
	) {
		let mut market_id: MarketId = 0;
		let mut market_config = valid_market_config();
		market_config.margin_ratio_initial = (1, 10).into();  // 1/10 IMR, or 10x leverage

		let margin = as_balance(10);
		let quote_amount = as_balance(100) + excess; // Over 10x margin

		ExtBuilder { balances: vec![(ALICE, USDC, margin)], ..Default::default() }
			.build()
			.init_market(&mut market_id, Some(market_config))
			.add_margin(&ALICE, USDC, margin)
			.execute_with(|| {
				VammPallet::set_price(Some(10.into()));
				let base_amount_limit = quote_amount / 10;
				assert_noop!(
					TestPallet::open_position(
						Origin::signed(ALICE),
						market_id,
						direction,
						quote_amount,
						base_amount_limit,
					),
					Error::<Runtime>::InsufficientCollateral,
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
