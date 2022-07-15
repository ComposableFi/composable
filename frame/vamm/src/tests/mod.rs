// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use, dead_code)]

use std::ops::RangeInclusive;

use crate::{
	mock::{
		Balance, MockRuntime, Moment, Origin, System as SystemPallet, TestPallet,
		Timestamp as TimestampPallet,
	},
	pallet::{self, VammState},
};
use composable_traits::vamm::{
	AssetType, Direction, MovePriceConfig, SwapConfig, Vamm as VammTrait, VammConfig,
	MINIMUM_TWAP_PERIOD,
};
use frame_support::{assert_ok, pallet_prelude::Hooks};
use proptest::prelude::*;

pub mod compute_invariant;
pub mod create_vamm;
pub mod get_price;
pub mod get_twap;
pub mod move_price;
pub mod swap_asset;
pub mod update_twap;

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

type Decimal = <MockRuntime as pallet::Config>::Decimal;
type Timestamp = <MockRuntime as pallet::Config>::Moment;
type VammId = <TestPallet as VammTrait>::VammId;

#[derive(Default)]
struct TestVammState<Balance, Timestamp> {
	base_asset_reserves: Option<Balance>,
	quote_asset_reserves: Option<Balance>,
	peg_multiplier: Option<Balance>,
	closed: Option<Option<Timestamp>>,
}

#[derive(Default)]
struct TestSwapConfig<VammId, Balance> {
	vamm_id: Option<VammId>,
	asset: Option<AssetType>,
	input_amount: Option<Balance>,
	direction: Option<Direction>,
	output_amount_limit: Option<Balance>,
}

const ZERO_RESERVE: Balance = Balance::MIN;
const MINIMUM_RESERVE: Balance = ZERO_RESERVE + 1;
const MAXIMUM_RESERVE: Balance = Balance::MAX;
const RUN_CASES: u32 = 1000;

// ----------------------------------------------------------------------------------------------------
//                                             Helper Functions
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

fn run_for_seconds(seconds: u64) {
	// Not using an equivalent run_to_block call here because it causes the
	// tests to slow down drastically
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

fn default_vamm_config() -> VammConfig<Balance, Moment> {
	VammConfig {
		base_asset_reserves: 10_u128.pow(18) * 2,
		quote_asset_reserves: 10_u128.pow(18) * 50,
		peg_multiplier: 1,
		twap_period: 3600,
	}
}

fn default_swap_config(asset: AssetType, direction: Direction) -> SwapConfig<VammId, Balance> {
	SwapConfig {
		vamm_id: 0,
		asset,
		input_amount: 10_u128.pow(18),
		direction,
		output_amount_limit: Some(0),
	}
}

fn create_vamm(vamm_config: &VammConfig<Balance, Moment>) {
	assert_ok!(TestPallet::create(vamm_config));
}

// ----------------------------------------------------------------------------------------------------
//                                             Prop_compose
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn then_and_now()(then in u64::MIN..1000)(
		then in Just(then),
		now in (then+1)..=1000,
	) -> (u64, u64) {
		(then, now)
	}
}

prop_compose! {
	fn u64_range_lower_half()(
		x in u64::MIN..u64::MAX/2
	) -> u64 {
		x
	}
}

prop_compose! {
	fn u64_range_upper_half()(
		x in u64::MAX/2..=u64::MAX
	) -> u64 {
		x
	}
}

prop_compose! {
	fn balance_range()(
		range in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) -> Balance {
		range
	}
}

prop_compose! {
	fn balance_range_low()(
		range in MINIMUM_RESERVE..1_000_000_000_000
	) -> Balance {
		range
	}
}

prop_compose! {
	fn balance_range_lower_half()(
		range in MINIMUM_RESERVE..MAXIMUM_RESERVE/2
	) -> Balance {
		range
	}
}

prop_compose! {
	fn balance_range_upper_half()(
		range in MAXIMUM_RESERVE/2..=MAXIMUM_RESERVE
	) -> Balance {
		range
	}
}

prop_compose! {
	fn min_max_reserve()(
		base_asset_reserves in balance_range(),
		quote_asset_reserves in balance_range(),
		peg_multiplier in balance_range()
	) -> (Balance, Balance, Balance) {
		(base_asset_reserves, quote_asset_reserves, peg_multiplier)
	}
}

prop_compose! {
	fn valid_twap_period()(
		twap_period in (MINIMUM_TWAP_PERIOD+1).into()..=Timestamp::MAX
	) -> Timestamp {
		twap_period
	}
}

prop_compose! {
	fn zero_reserve()(
		zero_reserve in ZERO_RESERVE..=ZERO_RESERVE,
	) -> Balance {
		zero_reserve
	}
}

prop_compose! {
	fn loop_times()(
		loop_times in MINIMUM_RESERVE..=10,
	) -> Balance {
		loop_times
	}
}

prop_compose! {
	fn timestamp()(
		t in Timestamp::MIN..=Timestamp::MAX
	) -> Timestamp {
		t
	}
}

fn min_sane_balance() -> u128 {
	10_u128.pow(18)
}

fn max_sane_balance() -> u128 {
	10_u128.pow(30)
}

fn any_sane_asset_amount() -> RangeInclusive<u128> {
	// From 1 to 1 trilion.
	min_sane_balance()..=max_sane_balance()
}

#[allow(dead_code)]
fn limited_peg(x: u128) -> RangeInclusive<u128> {
	1..=(u128::MAX / x)
}

prop_compose! {
	fn asset_times_peg_dont_overflow()(
		asset in any_sane_asset_amount()
	)(peg in limited_peg(asset), asset in Just(asset)) -> (u128, u128) {
		(asset, peg)
	}
}

prop_compose! {
	fn any_sane_base_quote_peg()(
		(asset1, peg) in asset_times_peg_dont_overflow()
	) (
		peg in Just(peg),
		asset1 in Just(asset1),
		// asset2 in Just(peg),
		asset2 in limited_peg(asset1),
		first_asset_is_base in any::<bool>()
	) -> (u128, u128, u128) {
		if first_asset_is_base {
			(asset1, asset2, peg)
		} else {
			(asset2, asset1, peg)
		}
	}
}

fn any_vamm_id() -> RangeInclusive<VammId> {
	VammId::MIN..=VammId::MAX
}

fn any_time() -> RangeInclusive<Timestamp> {
	Timestamp::MIN..=Timestamp::MAX
}

prop_compose! {
	fn any_vamm_state()(
		base_asset_reserves in any_sane_asset_amount(),
		quote_asset_reserves in any_sane_asset_amount(),
		peg_multiplier in 1..=100_000_u128,
		closed in prop_oneof![timestamp().prop_map(Some), Just(None)],
		twap_timestamp in any_time(),
		twap_period in any_time()
	) -> VammState<Balance, Timestamp, Decimal> {
		VammState {
			base_asset_reserves,
			quote_asset_reserves,
			peg_multiplier,
			invariant: TestPallet::compute_invariant(
				base_asset_reserves, quote_asset_reserves
			).unwrap(),
			twap_timestamp,
			base_asset_twap: Decimal::from_inner(base_asset_reserves),
			quote_asset_twap: Decimal::from_inner(quote_asset_reserves),
			closed,
			twap_period,
		}
	}
}

prop_compose! {
	fn any_move_price_config()(
		vamm_id in any_vamm_id(),
		base_asset_reserves in any_sane_asset_amount(),
		quote_asset_reserves in any_sane_asset_amount(),
	) -> MovePriceConfig<VammId, Balance> {
		MovePriceConfig {
			vamm_id,
			base_asset_reserves,
			quote_asset_reserves,
		}
	}
}

prop_compose! {
	fn get_vamm_state(config: TestVammState<Balance, Timestamp>)(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in any_sane_base_quote_peg(),
		closed in prop_oneof![timestamp().prop_map(Some), Just(None)],
		base_asset_twap in balance_range(),
		quote_asset_twap in balance_range(),
		twap_timestamp in timestamp(),
	) -> VammState<Balance, Timestamp, Decimal> {
		let invariant = match (
			config.base_asset_reserves,
			config.quote_asset_reserves
		) {
			(Some(base), Some(quote)) => TestPallet::compute_invariant(base, quote),
			_ => TestPallet::compute_invariant(base_asset_reserves, quote_asset_reserves)
		}.unwrap();

		let base_asset_twap = Decimal::from_inner(base_asset_twap);
		let quote_asset_twap = Decimal::from_inner(quote_asset_twap);

		VammState {
			base_asset_reserves: config
				.base_asset_reserves
				.unwrap_or(base_asset_reserves),
			quote_asset_reserves: config
				.quote_asset_reserves
				.unwrap_or(quote_asset_reserves),
			peg_multiplier: config
				.peg_multiplier
				.unwrap_or(peg_multiplier),
			invariant,
			closed: config
				.closed
				.unwrap_or(closed),
			base_asset_twap,
			quote_asset_twap,
			twap_timestamp,
			..Default::default()
		}
	}
}

prop_compose! {
	fn get_swap_config(config: TestSwapConfig<VammId, Balance>)(
		vamm_id in balance_range(),
		asset in prop_oneof![Just(AssetType::Base), Just(AssetType::Quote)],
		input_amount in balance_range(),
		direction in prop_oneof![Just(Direction::Add), Just(Direction::Remove)],
		output_amount_limit in balance_range(),
	) -> SwapConfig<VammId, Balance> {
		SwapConfig {
			vamm_id: config
				.vamm_id
				.unwrap_or(vamm_id),
			asset: config
				.asset
				.unwrap_or(asset),
			input_amount: config
				.input_amount
				.unwrap_or(input_amount),
			direction: config
				.direction
				.unwrap_or(direction),
			output_amount_limit: Some(config
				.output_amount_limit
				.unwrap_or(output_amount_limit)),
		}
	}
}

fn swap_config() -> BoxedStrategy<SwapConfig<VammId, Balance>> {
	(
		Just(0_u128),
		prop_oneof![Just(AssetType::Base), Just(AssetType::Quote)],
		1_000_000_000..=1_000_000_000_000_000_u128,
		prop_oneof![Just(Direction::Add), Just(Direction::Remove)],
		Just(0_u128),
	)
		.prop_map(|(vamm_id, asset, input_amount, direction, output_amount_limit)| SwapConfig {
			vamm_id,
			asset,
			input_amount,
			direction,
			output_amount_limit: Some(output_amount_limit),
		})
		.boxed()
}

fn multiple_swap_configs(max_swaps: usize) -> Vec<BoxedStrategy<SwapConfig<VammId, Balance>>> {
	let mut swaps = Vec::with_capacity(max_swaps);
	for _ in 0..max_swaps {
		swaps.push(swap_config());
	}
	swaps
}

prop_compose! {
	fn multiple_swaps()(
		swaps_count in 1_000..100_000_usize
	) (
		swaps in multiple_swap_configs(swaps_count)
	) -> Vec<SwapConfig<VammId, Balance>> {
		swaps
	}
}
