// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use)]

use crate::{
	mock::{
		self as mock,
		accounts::{AccountId, ALICE},
		assets::{AssetId, DOT, USDC},
		runtime::{
			Assets as AssetsPallet, Balance, Decimal, ExtBuilder, MarketId, Oracle as OraclePallet,
			Origin, Runtime, System as SystemPallet, TestPallet, Timestamp as TimestampPallet,
			Vamm as VammPallet, VammId,
		},
		vamm::VammConfig,
	},
	Direction, Market as MarketGeneric, MarketConfig as MarketConfigGeneric, Markets,
	MaxPriceDivergence, MaxTwapDivergence,
};
use composable_traits::{
	clearing_house::{ClearingHouse, Instruments},
	oracle::Oracle,
	time::{DurationSeconds, ONE_HOUR},
	vamm::{AssetType, Direction as VammDirection, Vamm},
};
use frame_support::{
	assert_err, assert_ok,
	pallet_prelude::Hooks,
	traits::fungibles::{Inspect, Unbalanced},
};
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedI128, FixedPointNumber, FixedU128};

pub mod close_position;
pub mod comp;
pub mod create_market;
pub mod deposit_collateral;
pub mod instruments;
pub mod liquidate;
pub mod math;
pub mod open_position;
pub mod update_funding;
pub mod withdraw_collateral;

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

pub const BALANCE_LOWER_BOUND: Balance = FixedU128::DIV / 10_u128.pow(12); // 1 / (1 trillion)
pub const BALANCE_UPPER_BOUND: Balance = 10_u128.pow(12) * FixedU128::DIV; // 1 trillion

type Market = <TestPallet as Instruments>::Market;
type MarketConfig = <TestPallet as ClearingHouse>::MarketConfig;
type Position = <TestPallet as Instruments>::Position;
type SwapConfig = <VammPallet as Vamm>::SwapConfig;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_type: Some(USDC),
			vamm_id: Some(0_u64),
			vamm_twap: Some(100.into()),
			oracle_asset_support: Some(true),
			oracle_price: Some(10_000),
			oracle_twap: Some(10_000),
			max_price_divergence: FixedI128::from_inner(i128::MAX),
		}
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Helpers
// ----------------------------------------------------------------------------------------------------

fn run_to_block(n: u64) {
	while SystemPallet::block_number() < n {
		if SystemPallet::block_number() > 0 {
			TimestampPallet::on_finalize(SystemPallet::block_number());
			SystemPallet::on_finalize(SystemPallet::block_number());
		}
		SystemPallet::set_block_number(SystemPallet::block_number() + 1);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = TimestampPallet::set(Origin::none(), (SystemPallet::block_number() - 1) * 1000);
		SystemPallet::on_initialize(SystemPallet::block_number());
		TimestampPallet::on_initialize(SystemPallet::block_number());
	}
}

fn run_for_seconds(seconds: DurationSeconds) {
	// Not using an equivalent run_to_block call here because it causes the tests to slow down
	// drastically
	if SystemPallet::block_number() > 0 {
		TimestampPallet::on_finalize(SystemPallet::block_number());
		SystemPallet::on_finalize(SystemPallet::block_number());
	}
	SystemPallet::set_block_number(SystemPallet::block_number() + 1);
	// Time is set in milliseconds, so we multiply the seconds by 1_000
	let _ = TimestampPallet::set(Origin::none(), TimestampPallet::now() + 1_000 * seconds);
	SystemPallet::on_initialize(SystemPallet::block_number());
	TimestampPallet::on_initialize(SystemPallet::block_number());
}

fn run_to_time(seconds: DurationSeconds) {
	// It's up to the caller to choose a time that is greater than the current one
	if SystemPallet::block_number() > 0 {
		TimestampPallet::on_finalize(SystemPallet::block_number());
		SystemPallet::on_finalize(SystemPallet::block_number());
	}
	SystemPallet::set_block_number(SystemPallet::block_number() + 1);
	// Time is set in milliseconds, so we multiply the seconds by 1_000
	let _ = TimestampPallet::set(Origin::none(), 1_000 * seconds);
	SystemPallet::on_initialize(SystemPallet::block_number());
	TimestampPallet::on_initialize(SystemPallet::block_number());
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

fn get_collateral(account_id: AccountId) -> Balance {
	TestPallet::get_collateral(&account_id).unwrap()
}

fn get_position(account_id: &AccountId, market_id: &MarketId) -> Option<Position> {
	let positions = TestPallet::get_positions(account_id);
	positions.into_iter().find(|p| p.market_id == *market_id)
}

fn get_outstanding_gains(account_id: AccountId, market_id: &MarketId) -> Balance {
	TestPallet::outstanding_profits(&account_id, market_id).unwrap_or_else(Zero::zero)
}

fn get_market(market_id: &MarketId) -> Market {
	TestPallet::get_market(market_id).unwrap()
}

fn get_market_fee_pool(market_id: &MarketId) -> Balance {
	AssetsPallet::balance(USDC, &TestPallet::get_fee_pool_account(*market_id))
}

fn set_fee_pool_depth(market_id: &MarketId, depth: Balance) {
	AssetsPallet::set_balance(USDC, &TestPallet::get_fee_pool_account(*market_id), depth);
}

fn set_maximum_oracle_mark_divergence(fraction: FixedI128) {
	MaxPriceDivergence::<Runtime>::set(fraction);
}

fn set_maximum_twap_divergence(fraction: FixedI128) {
	MaxTwapDivergence::<Runtime>::set(Some(fraction));
}

fn set_oracle_price(_market_id: &MarketId, price: FixedU128) {
	// let market = get_market(market_id);
	OraclePallet::set_price(Some(price.saturating_mul_int(100)));
}

fn set_oracle_twap(market_id: &MarketId, twap: FixedI128) {
	// Prepare everything so that even if the TWAP is updated via an EMA, it stays the same
	OraclePallet::set_price(Some(twap.saturating_mul_int(100)));
	Markets::<Runtime>::try_mutate(market_id, |m| {
		if let Some(m) = m {
			m.last_oracle_price = twap;
			m.last_oracle_twap = twap;
			Ok(())
		} else {
			Err(())
		}
	})
	.unwrap();
}

// ----------------------------------------------------------------------------------------------------
//                                        Execution Contexts
// ----------------------------------------------------------------------------------------------------

fn with_market_context<R>(
	ext_builder: ExtBuilder,
	config: MarketConfig,
	execute: impl FnOnce(MarketId) -> R,
) -> R {
	let configs = vec![config];
	with_markets_context(ext_builder, configs, |market_ids| execute(market_ids[0]))
}

fn with_markets_context<R>(
	ext_builder: ExtBuilder,
	configs: Vec<MarketConfig>,
	execute: impl FnOnce(Vec<MarketId>) -> R,
) -> R {
	let mut ext = ext_builder.build();

	ext.execute_with(|| {
		run_to_time(0);
		let ids: Vec<_> = configs
			.into_iter()
			.map(|c| <sp_io::TestExternalities as MarketInitializer>::create_market_helper(Some(c)))
			.collect();

		execute(ids)
	})
}

fn with_trading_context<R>(
	config: MarketConfig,
	margin: Balance,
	execute: impl FnOnce(MarketId) -> R,
) -> R {
	let configs = vec![config];
	let margins = vec![(ALICE, margin)];
	multi_market_and_trader_context(configs, margins, |market_ids| execute(market_ids[0]))
}

fn traders_in_one_market_context<R>(
	config: MarketConfig,
	margins: Vec<(AccountId, Balance)>,
	execute: impl FnOnce(MarketId) -> R,
) -> R {
	let configs = vec![config];
	multi_market_and_trader_context(configs, margins, |market_ids| execute(market_ids[0]))
}

fn multi_market_and_trader_context<R>(
	configs: Vec<MarketConfig>,
	margins: Vec<(AccountId, Balance)>,
	execute: impl FnOnce(Vec<MarketId>) -> R,
) -> R {
	let balances: Vec<(AccountId, AssetId, Balance)> =
		margins.iter().map(|&(a, m)| (a, USDC, m)).collect();
	let ext_builder = ExtBuilder { balances, ..Default::default() };

	with_markets_context(ext_builder, configs, |market_ids| {
		for (acc, margin) in margins {
			TestPallet::deposit_collateral(Origin::signed(acc), USDC, margin);
		}

		execute(market_ids)
	})
}

// ----------------------------------------------------------------------------------------------------
//                                          Default Inputs
// ----------------------------------------------------------------------------------------------------

impl Default for MarketConfigGeneric<AssetId, Balance, Decimal, VammConfig> {
	fn default() -> Self {
		Self {
			asset: DOT,
			vamm_config: Default::default(),
			// 10x max leverage to open a position
			margin_ratio_initial: FixedI128::from_float(0.1),
			// fully liquidate when above 50x leverage
			margin_ratio_maintenance: FixedI128::from_float(0.02),
			// partially liquidate when above 25x leverage
			margin_ratio_partial: FixedI128::from_float(0.04),
			minimum_trade_size: 0.into(),
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR * 24,
			taker_fee: 0,
			twap_period: ONE_HOUR,
		}
	}
}

impl Default for MarketGeneric<Runtime> {
	fn default() -> Self {
		Self::new(MarketConfigGeneric::<AssetId, Balance, Decimal, VammConfig>::default()).unwrap()
	}
}

// ----------------------------------------------------------------------------------------------------
//                                           Initializers
// ----------------------------------------------------------------------------------------------------

trait RunToBlock {
	fn run_to_block(self, n: u64) -> Self;
}

impl RunToBlock for sp_io::TestExternalities {
	fn run_to_block(mut self, n: u64) -> Self {
		self.execute_with(|| {
			run_to_block(n);
		});

		self
	}
}

trait CollateralInitializer {
	fn deposit_collateral(self, account_id: &AccountId, asset_id: AssetId, amount: Balance)
		-> Self;
}

impl CollateralInitializer for sp_io::TestExternalities {
	fn deposit_collateral(
		mut self,
		account_id: &AccountId,
		asset_id: AssetId,
		amount: Balance,
	) -> Self {
		self.execute_with(|| {
			assert_ok!(<TestPallet as ClearingHouse>::deposit_collateral(
				account_id, asset_id, amount
			));
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
		<TestPallet as ClearingHouse>::create_market(config.unwrap_or_default()).unwrap()
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
	fn bounded_decimal()(
		inner in as_inner(-1_000_000_000)..as_inner(1_000_000_000)
	) -> FixedI128 {
		FixedI128::from_inner(inner)
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
	// Anywhere from 1 / (1 trillion) to 1 trillion
	fn any_balance()(balance in BALANCE_LOWER_BOUND..=BALANCE_UPPER_BOUND) -> Balance {
		balance
	}
}

fn any_direction() -> impl Strategy<Value = Direction> {
	prop_oneof![Just(Direction::Long), Just(Direction::Short)]
}

prop_compose! {
	// Anywhere from 1 / (1 trillion) to 1 trillion
	fn any_price()(inner in any_balance()) -> FixedU128 {
		FixedU128::from_inner(inner)
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
		output_amount_limit in any::<Option<Balance>>(),
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
		output_amount_limit in Just(None),
	) -> SwapConfig {
		SwapConfig { vamm_id, asset, input_amount, direction, output_amount_limit }
	}
}

// ----------------------------------------------------------------------------------------------------
//                                           Mocked Pallets Tests
// ----------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn can_set_global_slippage_for_mock_vamm(vamm_id in any::<VammId>()) {
		ExtBuilder::default().build().execute_with(|| {
			// Set arbitrary price
			VammPallet::set_price(Some(1.into()));

			// Ensure that the global slippage is set to 0.0 by default
			let output = VammPallet::swap(&SwapConfig {
				vamm_id,
				asset: AssetType::Quote,
				input_amount: as_balance(100),
				direction: VammDirection::Add,
				output_amount_limit: Some(as_balance(100)),
			})
			.unwrap();
			assert_eq!(output.output, as_balance(100));

			// Set global slippage to 0.1
			VammPallet::set_slippage(Some((1, 10).into()));
			let output = VammPallet::swap(&SwapConfig {
				vamm_id,
				asset: AssetType::Quote,
				input_amount: as_balance(100),
				direction: VammDirection::Add,
				output_amount_limit: Some(as_balance(90)),
			})
			.unwrap();
			// Buying base asset with slippage gets you less
			assert_eq!(output.output, as_balance(90));

			// Keep global slippage to 0.1
			let output = VammPallet::swap(&SwapConfig {
				vamm_id,
				asset: AssetType::Base,
				input_amount: as_balance(100),
				direction: VammDirection::Add,
				output_amount_limit: Some(as_balance(90)),
			})
			.unwrap();
			// Selling base asset with slippage gets you less
			assert_eq!(output.output, as_balance(90));

			// Keep global slippage to 0.1
			let output = VammPallet::swap(&SwapConfig {
				vamm_id,
				asset: AssetType::Quote,
				input_amount: as_balance(100),
				direction: VammDirection::Remove,
				output_amount_limit: Some(as_balance(90)),
			})
			.unwrap();
			// Shorting base asset with slippage gets you less
			assert_eq!(output.output, as_balance(90));
		})
	}
}

proptest! {
	// Can we guarantee that any::<Option<Value>> will generate at least one of `Some` and `None`?
	#[test]
	fn mock_oracle_asset_support_reflects_genesis_config(oracle_asset_support in any::<Option<bool>>()) {
		ExtBuilder { oracle_asset_support, ..Default::default() }.build().execute_with(|| {
			let is_supported = OraclePallet::is_supported(DOT);
			match oracle_asset_support {
				Some(support) => assert_ok!(is_supported, support),
				None => {
					assert_err!(is_supported, mock::oracle::Error::<Runtime>::CantCheckAssetSupport)
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
			let created = VammPallet::create(&VammConfig::default());
			match vamm_id {
				Some(id) => assert_ok!(created, id),
				None => assert_err!(created, mock::vamm::Error::<Runtime>::FailedToCreateVamm),
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
						assert_err!(output, mock::vamm::Error::<Runtime>::FailedToExecuteSwap);
					}
				};
			})
	}
}

proptest! {
	#[test]
	fn can_set_price_impact_for_mock_vamm_swap(vamm_id in any::<VammId>(), factor in any_price()) {
		ExtBuilder::default()
			.build()
			.execute_with(|| {
				VammPallet::set_price_of(&vamm_id, Some(100.into()));
				VammPallet::set_price_impact_of(&vamm_id, Some(factor));

				assert_ok!(VammPallet::swap(&SwapConfig {
					vamm_id,
					asset: AssetType::Quote,
					input_amount: as_balance(100),
					direction: VammDirection::Add,
					output_amount_limit: Some(as_balance(1)),
				}));
				assert_ok!(VammPallet::get_price(vamm_id, AssetType::Base), factor * 100.into());
			});
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
						assert_err!(output, mock::vamm::Error::<Runtime>::FailedToSimulateSwap);
					}
				};
			})
	}
}
