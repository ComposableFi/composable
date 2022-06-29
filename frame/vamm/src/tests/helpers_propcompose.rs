use crate::{
	mock::{Balance, TestPallet},
	pallet::VammState,
	tests::{
		helpers::{
			any_sane_asset_amount, any_time, any_vamm_id, limited_peg, multiple_swap_configs,
		},
		Decimal, TestSwapConfig, TestVammState, Timestamp, VammId, MAXIMUM_RESERVE,
		MINIMUM_RESERVE, ZERO_RESERVE,
	},
};
use composable_traits::vamm::{
	AssetType, Direction, MovePriceConfig, SwapConfig, MINIMUM_TWAP_PERIOD,
};
use proptest::prelude::*;

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

prop_compose! {
	fn timestamp()(
		t in Timestamp::MIN..=Timestamp::MAX
	) -> Timestamp {
		t
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
//                                              Vamm State
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn get_vamm_state(config: TestVammState<Balance, Timestamp>)(
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
	pub fn any_vamm_state()(
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

// ----------------------------------------------------------------------------------------------------
//                                                 Swap
// ----------------------------------------------------------------------------------------------------

prop_compose! {
	pub fn multiple_swaps()(
		swaps_count in 1_000..100_000_usize
	) (
		swaps in multiple_swap_configs(swaps_count)
	) -> Vec<SwapConfig<VammId, Balance>> {
		swaps
	}
}

prop_compose! {
	pub fn get_swap_config(config: TestSwapConfig<VammId, Balance>)(
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
