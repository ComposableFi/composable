use crate::{
	mock::{Balance, ExtBuilder, MockRuntime, System, TestPallet, VammId},
	pallet::{Error, Event, VammMap},
	tests::{
		any_vamm_state, balance_range_upper_half, get_swap_config, get_vamm_state, multiple_swaps,
		run_to_block, swap_config, then_and_now, RUN_CASES,
	},
	VammState,
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, SwapOutput, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_core::U256;
use sp_runtime::traits::Zero;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_invalid_vamm_error(
		vamm_state in get_vamm_state(Default::default()),
		swap_config in get_swap_config(Default::default()),
	) {
		prop_assume!(swap_config.vamm_id != 0);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
	}
}

proptest! {
	#[test]
	fn fails_to_swap_assets_if_vamm_is_closed(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(Default::default()),
		(close, now) in then_and_now()
	) {
		// Make the current time be greater than the time when the vamm is
		// set to close, doing this we ensure we can't make swaps due to the
		// vamm be closed.
		vamm_state.closed = Some(close);
		swap_config.vamm_id = VammId::zero();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			run_to_block(now);

			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::VammIsClosed
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn fails_to_swap_assets_if_output_is_less_than_minimum_limit(
		mut vamm_state in any_vamm_state(),
		mut swap_config in get_swap_config(Default::default()),
		limit in balance_range_upper_half(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		swap_config.output_amount_limit = limit;
		swap_config.vamm_id = VammId::zero();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::SwappedAmountLessThanMinimumLimit
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_asset_suceeds_emitting_event(
		mut vamm_state in any_vamm_state(),
		mut swap_config in swap_config(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Disable output limit check
		swap_config.output_amount_limit = Balance::zero();

		swap_config.vamm_id = VammId::zero();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// For event emission
			run_to_block(1);

			let swap = TestPallet::swap(&swap_config);
			assert_ok!(swap);

			System::assert_last_event(
				Event::Swapped {
					vamm_id: swap_config.vamm_id,
					input_amount: swap_config.input_amount,
					output_amount: swap.unwrap(),
					input_asset_type: swap_config.asset,
					direction: swap_config.direction,
				}.into()
			);
		})
	}
}

#[test]
fn swap_add_base() {
	let swap_config = SwapConfig {
		vamm_id: 0,
		asset: AssetType::Base,
		input_amount: 1_000_000_000_000,
		direction: Direction::Add,
		output_amount_limit: 0,
	};

	let base_u256 = U256::from(2_u128) * U256::exp10(12);
	let quote_u256 = U256::from(50_u128) * U256::exp10(12);
	let invariant = base_u256 * quote_u256;

	ExtBuilder {
		vamm_count: 1,
		vamms: vec![(
			0,
			VammState {
				base_asset_reserves: base_u256.as_u128(),
				quote_asset_reserves: quote_u256.as_u128(),
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			},
		)],
	}
	.build()
	.execute_with(|| {
		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(0);

		assert_ok!(swap, SwapOutput { output: 16_666_666_666_667, negative: false });
		assert_eq!(
			vamm_after_swap.unwrap(),
			VammState {
				base_asset_reserves: 3_000_000_000_000,
				quote_asset_reserves: 33_333_333_333_333,
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			}
		);

		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

#[test]
fn swap_remove_base() {
	let swap_config = SwapConfig {
		vamm_id: 0,
		asset: AssetType::Base,
		input_amount: 1_000_000_000_000,
		direction: Direction::Remove,
		output_amount_limit: 0,
	};

	let base_u256 = U256::from(2_u128) * U256::exp10(12);
	let quote_u256 = U256::from(50_u128) * U256::exp10(12);
	let invariant = base_u256 * quote_u256;

	ExtBuilder {
		vamm_count: 1,
		vamms: vec![(
			0,
			VammState {
				base_asset_reserves: base_u256.as_u128(),
				quote_asset_reserves: quote_u256.as_u128(),
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			},
		)],
	}
	.build()
	.execute_with(|| {
		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(0);

		assert_ok!(swap, SwapOutput { output: 50_000_000_000_000, negative: false });
		assert_eq!(
			vamm_after_swap.unwrap(),
			VammState {
				base_asset_reserves: 1_000_000_000_000,
				quote_asset_reserves: 100_000_000_000_000,
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			}
		);

		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

#[test]
fn swap_add_quote() {
	let swap_config = SwapConfig {
		vamm_id: 0,
		asset: AssetType::Quote,
		input_amount: 1_000_000_000_000,
		direction: Direction::Add,
		output_amount_limit: 0,
	};

	let base_u256 = U256::from(2_u128) * U256::exp10(12);
	let quote_u256 = U256::from(50_u128) * U256::exp10(12);
	let invariant = base_u256 * quote_u256;

	ExtBuilder {
		vamm_count: 1,
		vamms: vec![(
			0,
			VammState {
				base_asset_reserves: base_u256.as_u128(),
				quote_asset_reserves: quote_u256.as_u128(),
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			},
		)],
	}
	.build()
	.execute_with(|| {
		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(0);

		assert_ok!(swap, SwapOutput { output: 39_215_686_275, negative: false });
		assert_eq!(
			vamm_after_swap.unwrap(),
			VammState {
				base_asset_reserves: 1_960_784_313_725,
				quote_asset_reserves: 51_000_000_000_000,
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			}
		);

		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

#[test]
fn swap_remove_quote() {
	let base_u256 = U256::from(2_u128) * U256::exp10(12);
	let quote_u256 = U256::from(50_u128) * U256::exp10(12);
	let invariant = base_u256 * quote_u256;

	let swap_config = SwapConfig {
		vamm_id: 0,
		asset: AssetType::Quote,
		input_amount: base_u256.as_u128() / 100_u128,
		direction: Direction::Remove,
		output_amount_limit: 0,
	};

	ExtBuilder {
		vamm_count: 1,
		vamms: vec![(
			0,
			VammState {
				base_asset_reserves: base_u256.as_u128(),
				quote_asset_reserves: quote_u256.as_u128(),
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			},
		)],
	}
	.build()
	.execute_with(|| {
		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(0);

		assert_ok!(swap, SwapOutput { output: 800_320_128, negative: true });
		assert_eq!(
			vamm_after_swap.unwrap(),
			VammState {
				base_asset_reserves: 2_000_800_320_128,
				quote_asset_reserves: 49_980_000_000_000,
				peg_multiplier: 1,
				invariant,
				closed: None,
				..Default::default()
			}
		);

		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1))]
	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant(
		mut vamm_state in any_vamm_state(),
		swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for x in swap_config.iter() {
				assert_ok!(TestPallet::swap(x));
			}
			let vamm_after_swap = VammMap::<MockRuntime>::get(0);

			let invariant_before = TestPallet::compute_invariant(
				vamm_before_swap.unwrap().base_asset_reserves,
				vamm_before_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_after = TestPallet::compute_invariant(
				vamm_after_swap.unwrap().base_asset_reserves,
				vamm_after_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_epsilon = invariant_before / U256::exp10(8);
			let invariant_delta = invariant_before.max(invariant_after)
				- invariant_before.min(invariant_after);

			assert!(invariant_delta <= invariant_epsilon);
		});
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1))]
	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant_only_base(
		mut vamm_state in any_vamm_state(),
		mut swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for mut x in swap_config.iter_mut() {
				// Make swaps only for base asset
				x.asset = AssetType::Base;
				assert_ok!(TestPallet::swap(x));
			}
			let vamm_after_swap = VammMap::<MockRuntime>::get(0);

			let invariant_before = TestPallet::compute_invariant(
				vamm_before_swap.unwrap().base_asset_reserves,
				vamm_before_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_after = TestPallet::compute_invariant(
				vamm_after_swap.unwrap().base_asset_reserves,
				vamm_after_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_epsilon = invariant_before / U256::exp10(8);
			let invariant_delta = invariant_before.max(invariant_after)
				- invariant_before.min(invariant_after);

			assert!(invariant_delta <= invariant_epsilon);
		});

	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1))]
	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant_only_quote(
		mut vamm_state in any_vamm_state(),
		mut swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for mut x in swap_config.iter_mut() {
				// Make swaps only for quote asset
				x.asset = AssetType::Quote;
				assert_ok!(TestPallet::swap(x));
			}
			let vamm_after_swap = VammMap::<MockRuntime>::get(0);

			let invariant_before = TestPallet::compute_invariant(
				vamm_before_swap.unwrap().base_asset_reserves,
				vamm_before_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_after = TestPallet::compute_invariant(
				vamm_after_swap.unwrap().base_asset_reserves,
				vamm_after_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_epsilon = invariant_before / U256::exp10(8);
			let invariant_delta = invariant_before.max(invariant_after)
				- invariant_before.min(invariant_after);

			assert!(invariant_delta <= invariant_epsilon);
		});
	}
}
