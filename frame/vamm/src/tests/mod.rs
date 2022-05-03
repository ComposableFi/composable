#![allow(clippy::disallowed_methods)] // Allow use of .unwrap() in tests

use crate::{
	mock::{Balance, MockRuntime, Origin, System, TestPallet, Timestamp},
	pallet::{self, VammState},
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, Vamm as VammTrait};
use frame_support::pallet_prelude::Hooks;
use proptest::prelude::*;

pub mod create_vamm;
pub mod get_price;
pub mod swap_asset;
pub mod swap_base_asset;
pub mod swap_quote_asset;

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

#[allow(dead_code)]
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

type Decimal = <MockRuntime as pallet::Config>::Decimal;
type VammTimestamp = <MockRuntime as pallet::Config>::Moment;
type VammId = <TestPallet as VammTrait>::VammId;

#[derive(Default)]
struct TestVammState<Balance, VammTimestamp> {
	base_asset_reserves: Option<Balance>,
	quote_asset_reserves: Option<Balance>,
	peg_multiplier: Option<Balance>,
	closed: Option<Option<VammTimestamp>>,
}

#[derive(Default)]
struct TestSwapConfig<VammId, Balance> {
	vamm_id: Option<VammId>,
	asset: Option<AssetType>,
	input_amount: Option<Balance>,
	direction: Option<Direction>,
	output_amount_limit: Option<Balance>,
}

#[allow(dead_code)]
const ZERO_RESERVE: Balance = Balance::MIN;

#[allow(dead_code)]
const MINIMUM_RESERVE: Balance = ZERO_RESERVE + 1;

#[allow(dead_code)]
const MAXIMUM_RESERVE: Balance = Balance::MAX;

#[allow(dead_code)]
const RUN_CASES: u32 = 1000;

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
		t in VammTimestamp::MIN..=VammTimestamp::MAX
	) -> VammTimestamp {
		t
	}
}

prop_compose! {
	fn get_vamm_state(config: TestVammState<Balance, VammTimestamp>)(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
		closed in prop_oneof![timestamp().prop_map(Some), Just(None)]

	) -> VammState<Balance, VammTimestamp> {
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
			closed: config
				.closed
				.unwrap_or(closed),
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
			output_amount_limit: config
				.output_amount_limit
				.unwrap_or(output_amount_limit),
		}
	}
}
