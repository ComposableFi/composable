use crate::{
	mock::{
		Balance, ExtBuilder, MockRuntime, Moment, Origin, System as SystemPallet, TestPallet,
		Timestamp as TimestampPallet,
	},
	pallet::VammMap,
	tests::types::{Decimal, TestSwapConfig, TestVammConfig, Timestamp, VammId},
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, Vamm as VammTrait, VammConfig};
use frame_support::{assert_ok, pallet_prelude::Hooks};
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedPointNumber};
use std::ops::RangeInclusive;

// ----------------------------------------------------------------------------------------------------
//                                       General Helper Functions
// ----------------------------------------------------------------------------------------------------

pub fn run_to_block(n: u64) {
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

pub fn run_for_seconds(seconds: u64) {
	// Not using an equivalent run_to_block call here because it causes the
	// tests to slow down drastically
	if SystemPallet::block_number() > 0 {
		TimestampPallet::on_finalize(SystemPallet::block_number());
		SystemPallet::on_finalize(SystemPallet::block_number());
	}
	SystemPallet::set_block_number(SystemPallet::block_number() + 1);
	// Time is set in milliseconds, so we multiply the seconds by 1_000
	let _ = TimestampPallet::set(
		Origin::none(),
		TimestampPallet::now().saturating_add(seconds.saturating_mul(1_000)),
	);
	SystemPallet::on_initialize(SystemPallet::block_number());
	TimestampPallet::on_initialize(SystemPallet::block_number());
}

pub fn as_decimal(x: u128) -> Decimal {
	Decimal::from_inner(x.saturating_mul(Decimal::DIV))
}

pub const fn as_decimal_inner(x: u128) -> Balance {
	x * Decimal::DIV
}

pub fn as_decimal_from_fraction(n: u128, d: u128) -> Decimal {
	let n = as_decimal(n);
	let d = as_decimal(d);
	n / d
}

pub fn one_up_to_(x: Balance) -> RangeInclusive<Balance> {
	1..=x
}

// ----------------------------------------------------------------------------------------------------
//                                    Vamm Specific Helper Functions
// ----------------------------------------------------------------------------------------------------

pub fn create_vamm(vamm_config: &VammConfig<Balance, Moment>) {
	assert_ok!(TestPallet::create(vamm_config));
}

pub fn any_vamm_id() -> RangeInclusive<VammId> {
	VammId::MIN..=VammId::MAX
}

pub fn with_existing_vamm_context(
	vamm_config: TestVammConfig<Balance, Moment>,
	execute: impl FnOnce(),
) {
	ExtBuilder::default().build().execute_with(|| {
		create_vamm(&vamm_config.into());
		execute();
	});
}

// ----------------------------------------------------------------------------------------------------
//                                               Balance
// ----------------------------------------------------------------------------------------------------

fn min_sane_balance() -> u128 {
	10_u128.pow(14)
}

fn max_sane_balance() -> u128 {
	10_u128.pow(30)
}

pub fn any_sane_asset_amount() -> RangeInclusive<u128> {
	// From 0.0001 to 1 trilion.
	min_sane_balance()..=max_sane_balance()
}

// ----------------------------------------------------------------------------------------------------
//                                                 Time
// ----------------------------------------------------------------------------------------------------

pub fn any_time() -> RangeInclusive<Timestamp> {
	Timestamp::MIN..=Timestamp::MAX.saturating_div(10000)
}

// ----------------------------------------------------------------------------------------------------
//                                                 Swap
// ----------------------------------------------------------------------------------------------------

pub fn swap_config() -> BoxedStrategy<SwapConfig<VammId, Balance>> {
	(
		Just(0_u128),
		prop_oneof![Just(AssetType::Base), Just(AssetType::Quote)],
		min_sane_balance()..=max_sane_balance(),
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

pub fn multiple_swap_configs(max_swaps: usize) -> Vec<BoxedStrategy<SwapConfig<VammId, Balance>>> {
	let mut swaps = Vec::with_capacity(max_swaps);
	for _ in 0..max_swaps {
		swaps.push(swap_config());
	}
	swaps
}

pub fn with_swap_context(
	vamm_config: TestVammConfig<Balance, Moment>,
	swap_config: TestSwapConfig<VammId, Balance>,
	execute: impl FnOnce(SwapConfig<VammId, Balance>),
) {
	ExtBuilder::default().build().execute_with(|| {
		create_vamm(&vamm_config.into());
		execute(swap_config.into());
	});
}

pub fn with_swap_context_checking_limit(
	vamm_config: VammConfig<Balance, Moment>,
	swap_config: SwapConfig<VammId, Balance>,
	output_amount_limit: Balance,
	execute: impl FnOnce(SwapConfig<VammId, Balance>),
) {
	ExtBuilder::default().build().execute_with(|| {
		VammMap::<MockRuntime>::mutate(TestPallet::create(&vamm_config).unwrap(), |vamm_state| {
			match vamm_state {
				Some(v) => v.closed = None,
				None => (),
			}
		});
		execute(SwapConfig {
			vamm_id: VammId::zero(),
			output_amount_limit: Some(output_amount_limit),
			..swap_config
		});
	})
}

// ----------------------------------------------------------------------------------------------------
//                                                 TWAP
// ----------------------------------------------------------------------------------------------------

pub fn twap_update_delay(vamm_id: VammId) -> Moment {
	let vamm_state = TestPallet::get_vamm(vamm_id).unwrap();
	vamm_state
		.twap_period
		.saturating_add(vamm_state.twap_timestamp)
		.saturating_add(1)
}
