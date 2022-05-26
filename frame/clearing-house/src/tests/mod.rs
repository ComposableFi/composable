// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use)]

use crate::{
	mock::{
		self as mock,
		accounts::{AccountId, ALICE},
		assets::{AssetId, DOT, USDC},
		runtime::{
			Balance, Decimal, ExtBuilder, MarketId, Oracle as OraclePallet, Origin, Runtime,
			System as SystemPallet, TestPallet, Timestamp as TimestampPallet, Vamm as VammPallet,
			VammId,
		},
		vamm::VammConfig,
	},
	Config, Market as MarketGeneric, MarketConfig as MarketConfigGeneric, Markets,
};
use composable_traits::{
	clearing_house::{ClearingHouse, Instruments},
	oracle::Oracle,
	time::{DurationSeconds, ONE_HOUR},
	vamm::{AssetType, Direction as VammDirection, Vamm},
};
use frame_support::{assert_err, assert_ok, pallet_prelude::Hooks};
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedI128, FixedPointNumber, FixedU128};

pub mod comp;
pub mod create_market;
pub mod deposit_collateral;
pub mod instruments;
pub mod liquidate;
pub mod open_position;
pub mod update_funding;

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

pub const BALANCE_LOWER_BOUND: Balance = FixedU128::DIV / 10_u128.pow(12); // 1 / (1 trillion)
pub const BALANCE_UPPER_BOUND: Balance = 10_u128.pow(12) * FixedU128::DIV; // 1 trillion

type Market = <TestPallet as Instruments>::Market;
type MarketConfig = <TestPallet as ClearingHouse>::MarketConfig;
type Position = <TestPallet as Instruments>::Position;
type SwapConfig = <VammPallet as Vamm>::SwapConfig;
type SwapSimulationConfig = <VammPallet as Vamm>::SwapSimulationConfig;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_type: Some(USDC),
			vamm_id: Some(0_u64),
			vamm_twap: Some(100.into()),
			oracle_asset_support: Some(true),
			oracle_twap: Some(10_000),
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
		let _ = TimestampPallet::set(Origin::none(), SystemPallet::block_number() * 1000);
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

pub fn set_fee_pool_depth(market_id: &MarketId, depth: Balance) {
	fn set_depth(market: &mut Option<Market>, d: Balance) -> Result<(), ()> {
		if let Some(m) = market {
			m.fee_pool = d;
			Ok(())
		} else {
			Err(())
		}
	}

	Markets::<Runtime>::try_mutate(market_id, |m| set_depth(m, depth)).unwrap();
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
		run_to_block(1);
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
			// liquidate when above 50x leverage
			margin_ratio_maintenance: FixedI128::from_float(0.02),
			// 'One cent' of the quote asset
			minimum_trade_size: FixedI128::from_float(0.01),
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR * 24,
			taker_fee: 10, // 0.1%
		}
	}
}

impl<T: Config> Default for MarketGeneric<T> {
	fn default() -> Self {
		Self {
			vamm_id: Zero::zero(),
			asset_id: Default::default(),
			margin_ratio_initial: Default::default(),
			margin_ratio_maintenance: Default::default(),
			minimum_trade_size: Default::default(),
			base_asset_amount_long: Default::default(),
			base_asset_amount_short: Default::default(),
			cum_funding_rate_long: Default::default(),
			cum_funding_rate_short: Default::default(),
			fee_pool: Default::default(),
			funding_rate_ts: Default::default(),
			funding_frequency: Default::default(),
			funding_period: Default::default(),
			taker_fee: Default::default(),
		}
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
		<TestPallet as ClearingHouse>::create_market(&config.unwrap_or_default()).unwrap()
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
