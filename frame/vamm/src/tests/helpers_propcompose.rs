use crate::{
	mock::{Balance, TestPallet},
	tests::{
		constants::{MAXIMUM_RESERVE, MINIMUM_RESERVE, RUN_CASES, ZERO_RESERVE},
		helpers::{
			any_sane_asset_amount, any_time, any_vamm_id, multiple_swap_configs, one_up_to_,
		},
		types::{Decimal, Timestamp, VammId},
	},
	types::VammState,
};
use composable_traits::vamm::{
	AssetType, Direction, MovePriceConfig, SwapConfig, VammConfig, MINIMUM_TWAP_PERIOD,
};
use proptest::prelude::*;
use sp_runtime::{traits::One, FixedPointNumber};

// ----------------------------------------------------------------------------------------------------
//                                     General Helper Propcomposes
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn loop_times()(
		loop_times in MINIMUM_RESERVE..=10,
	) -> Balance {
		loop_times
	}
}

// ----------------------------------------------------------------------------------------------------
//                                                 Time
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn then_and_now()(then in u64::MIN..1000)(
		then in Just(then),
		now in (then+1)..=1000,
	) -> (u64, u64) {
		(then, now)
	}
}

// ----------------------------------------------------------------------------------------------------
//                                               Balance
// ----------------------------------------------------------------------------------------------------

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
	pub fn balance_range()(
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
	pub fn balance_range_lower_half()(
		range in MINIMUM_RESERVE..MAXIMUM_RESERVE/2
	) -> Balance {
		range
	}
}

prop_compose! {
	pub fn balance_range_upper_half()(
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
	fn any_sane_base_quote_peg()
	(
		base in any_sane_asset_amount(),
		quote in any_sane_asset_amount(),
		peg in any_sane_asset_amount(),
	) -> (u128, u128, u128) {
		(base, quote, peg)
	}
}

// ----------------------------------------------------------------------------------------------------
//                                                 TWAP
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn valid_twap_period()(
		twap_period in (MINIMUM_TWAP_PERIOD+1).into()..=Timestamp::MAX
	) -> Timestamp {
		twap_period
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Vamm Config
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	fn limited_quote_peg() (
		x in 1..=(Balance::MAX/Decimal::DIV),
	) (
		y in one_up_to_(x),
		x in Just(x),
		first_is_quote in any::<bool>()
	) -> (Balance, Balance) {
		if first_is_quote {
			(x, y)
		} else {
			(y, x)
		}
	}
}

prop_compose! {
	pub fn any_valid_vammconfig() (
		base_asset_reserves in any_sane_asset_amount(),
		quote_asset_reserves in any_sane_asset_amount(),
		peg_multiplier in 1..=100_000_u128,
		twap_period in valid_twap_period()
	) -> VammConfig<Balance, Timestamp> {
		VammConfig {
			base_asset_reserves,
			quote_asset_reserves,
			peg_multiplier,
			twap_period
		}
	}
}

// ----------------------------------------------------------------------------------------------------
//                                              Vamm State
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn any_vamm_state()(
		base_asset_reserves in any_sane_asset_amount(),
		quote_asset_reserves in any_sane_asset_amount(),
		peg_multiplier in 1..=100_000_u128,
		closed in prop_oneof![any_time().prop_map(Some), Just(None)],
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
			closed,
			twap_period,
		}
	}
}

// ----------------------------------------------------------------------------------------------------
//                                                 Swap
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn multiple_swaps()(
		swaps_count in One::one()..RUN_CASES.saturating_pow(2) as usize
	) (
		swaps in multiple_swap_configs(swaps_count)
	) -> Vec<SwapConfig<VammId, Balance>> {
		swaps
	}
}

prop_compose! {
	pub fn any_swap_config()(
		vamm_id in balance_range(),
		asset in prop_oneof![Just(AssetType::Base), Just(AssetType::Quote)],
		input_amount in balance_range(),
		direction in prop_oneof![Just(Direction::Add), Just(Direction::Remove)],
		output_amount_limit in balance_range(),
	) -> SwapConfig<VammId, Balance> {
		SwapConfig {
			vamm_id,
			asset,
			input_amount,
			direction,
			output_amount_limit: Some(output_amount_limit),
		}
	}
}

// ----------------------------------------------------------------------------------------------------
//                                              Move Price
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn any_move_price_config()(
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
