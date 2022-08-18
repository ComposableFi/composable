use crate::{
	mock::{Balance, ExtBuilder, MockRuntime, System, TestPallet, VammId},
	pallet::{Error, Event, VammMap},
	tests::{
		constants::{DEFAULT_OUTPUT_ADDING_BASE, DEFAULT_OUTPUT_REMOVING_BASE, RUN_CASES},
		helpers::{
			create_vamm, run_for_seconds, run_to_block, swap_config,
			with_swap_context_checking_limit,
		},
		helpers_propcompose::{
			any_swap_config, any_vamm_state, balance_range_lower_half, balance_range_upper_half,
			multiple_swaps, then_and_now,
		},
		types::{TestSwapConfig, TestVammConfig},
	},
};
use composable_traits::vamm::{
	AssetType, Direction, SwapConfig, SwapOutput, Vamm as VammTrait, MINIMUM_TWAP_PERIOD,
};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_core::U256;
use sp_runtime::traits::Zero;

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_add_base() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig::default();
		create_vamm(&vamm_config.into());

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config.into());
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 16666666666666666667, negative: false });
		assert_eq!(vamm_after_swap.base_asset_reserves, 3000000000000000000);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 33333333333333333333);
		assert_eq!(
			vamm_after_swap.base_asset_reserves,
			vamm_config.base_asset_reserves + swap_config.input_amount
		);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves + swap.unwrap().output,
			vamm_config.quote_asset_reserves
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
	});
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_add_quote() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Add,
			..Default::default()
		};
		create_vamm(&vamm_config.into());

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config.into());
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 39215686274509804, negative: false });
		assert_eq!(vamm_after_swap.base_asset_reserves, 1960784313725490196);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 51000000000000000000);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves,
			vamm_config.quote_asset_reserves + swap_config.input_amount
		);
		assert_eq!(
			vamm_after_swap.base_asset_reserves + swap.unwrap().output,
			vamm_config.base_asset_reserves
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
	});
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_remove_base() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig {
			asset: AssetType::Base,
			direction: Direction::Remove,
			..Default::default()
		};
		create_vamm(&vamm_config.into());

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config.into());
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 50000000000000000000, negative: false });
		assert_eq!(vamm_after_swap.base_asset_reserves, 1000000000000000000);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 100000000000000000000);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves,
			vamm_config.quote_asset_reserves + swap.unwrap().output
		);
		assert_eq!(
			vamm_after_swap.base_asset_reserves,
			vamm_config.base_asset_reserves - swap_config.input_amount
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
	});
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_remove_quote() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Remove,
			..Default::default()
		};
		create_vamm(&vamm_config.into());

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config.into());
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 40816326530612244, negative: true });
		assert_eq!(vamm_after_swap.base_asset_reserves, 2040816326530612244);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 49000000000000000000);
		assert_eq!(
			vamm_after_swap.base_asset_reserves,
			vamm_config.base_asset_reserves + swap.unwrap().output
		);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves,
			vamm_config.quote_asset_reserves - swap_config.input_amount
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
	});
}

#[test]
fn should_update_twap_when_adding_base_asset() {
	ExtBuilder::default().build().execute_with(|| {
		// For event emission
		run_to_block(1);

		// Create Vamm
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig {
			asset: AssetType::Base,
			direction: Direction::Add,
			..Default::default()
		};
		create_vamm(&vamm_config.into());

		// Get Initial Vamm State
		let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

		// Perform swap
		run_for_seconds(vamm_state_initial.twap_period);
		assert_ok!(TestPallet::swap(&swap_config.into()));

		// Ensure event for update twap was emitted
		let vamm_state = TestPallet::get_vamm(0).unwrap();
		System::assert_has_event(
			Event::UpdatedTwap { vamm_id: 0, base_twap: vamm_state.base_asset_twap }.into(),
		);
	});
}

#[test]
fn should_update_twap_when_removing_base_asset() {
	ExtBuilder::default().build().execute_with(|| {
		// For event emission
		run_to_block(1);

		// Create Vamm
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig {
			asset: AssetType::Base,
			direction: Direction::Remove,
			..Default::default()
		};
		create_vamm(&vamm_config.into());

		// Get Initial Vamm State
		let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

		// Perform swap
		run_for_seconds(vamm_state_initial.twap_period);
		assert_ok!(TestPallet::swap(&swap_config.into()));

		// Ensure event for update twap was emitted
		let vamm_state = TestPallet::get_vamm(0).unwrap();
		System::assert_has_event(
			Event::UpdatedTwap { vamm_id: 0, base_twap: vamm_state.base_asset_twap }.into(),
		);
	});
}

#[test]
fn should_update_twap_when_adding_quote_asset() {
	ExtBuilder::default().build().execute_with(|| {
		// For event emission
		run_to_block(1);

		// Create Vamm
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Add,
			..Default::default()
		};
		create_vamm(&vamm_config.into());

		// Get Initial Vamm State
		let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

		// Perform swap
		run_for_seconds(vamm_state_initial.twap_period);
		assert_ok!(TestPallet::swap(&swap_config.into()));

		// Ensure event for update twap was emitted
		let vamm_state = TestPallet::get_vamm(0).unwrap();
		System::assert_has_event(
			Event::UpdatedTwap { vamm_id: 0, base_twap: vamm_state.base_asset_twap }.into(),
		);
	});
}

#[test]
fn should_update_twap_when_removing_quote_asset() {
	ExtBuilder::default().build().execute_with(|| {
		// For event emission
		run_to_block(1);

		// Create Vamm
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Remove,
			..Default::default()
		};
		create_vamm(&vamm_config.into());

		// Get Initial Vamm State
		let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

		// Perform swap
		run_for_seconds(vamm_state_initial.twap_period);
		assert_ok!(TestPallet::swap(&swap_config.into()));

		// Ensure event for update twap was emitted
		let vamm_state = TestPallet::get_vamm(0).unwrap();
		System::assert_has_event(
			Event::UpdatedTwap { vamm_id: 0, base_twap: vamm_state.base_asset_twap }.into(),
		);
	});
}

#[test]
fn should_fail_if_output_is_less_than_minimum_limit_when_adding_asset() {
	with_swap_context_checking_limit(
		TestVammConfig::default().into(),
		SwapConfig { direction: Direction::Add, ..TestSwapConfig::default().into() },
		DEFAULT_OUTPUT_ADDING_BASE + 1,
		|swap_config| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::SwappedAmountLessThanMinimumLimit
			);
		},
	)
}

#[test]
fn should_fail_if_output_is_more_than_maximum_limit_when_removing_asset() {
	with_swap_context_checking_limit(
		TestVammConfig::default().into(),
		SwapConfig { direction: Direction::Remove, ..TestSwapConfig::default().into() },
		DEFAULT_OUTPUT_REMOVING_BASE - 1,
		|swap_config| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::SwappedAmountMoreThanMaximumLimit
			);
		},
	)
}

// -------------------------------------------------------------------------------------------------
//                                             Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn should_fail_if_vamm_does_not_exist(
		vamm_state in any_vamm_state(),
		swap_config in any_swap_config(),
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
		});
	}

	#[test]
	fn should_fail_if_vamm_is_closed(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config(),
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
		});
	}

	#[test]
	fn should_succeed_emitting_swap_event(
		mut vamm_state in any_vamm_state(),
		mut swap_config in swap_config(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Disable output limit check.
		swap_config.output_amount_limit = Some(Balance::zero());

		// Ensure always the correct vamm.
		swap_config.vamm_id = VammId::zero();

		// Ensure funds will be enough.
		if swap_config.direction == Direction::Remove {
			match swap_config.asset {
				AssetType::Base => {
					swap_config.input_amount = swap_config.input_amount.min(vamm_state.base_asset_reserves) - 1;
				}
				AssetType::Quote => {
					swap_config.input_amount = swap_config.input_amount.min(vamm_state.quote_asset_reserves) - 1;
				}
			};
		};

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// For event emission
			run_for_seconds(1);
			let swap = TestPallet::swap(&swap_config);
			if let Ok(swap) = swap {
				System::assert_last_event(
					Event::Swapped {
						vamm_id: swap_config.vamm_id,
						input_amount: swap_config.input_amount,
						output_amount: swap,
						input_asset_type: swap_config.asset,
						direction: swap_config.direction,
					}.into()
				);
			}
		});
	}

	#[test]
	fn should_succeed_updating_twap_and_emitting_update_twap_event(
		mut vamm_state in any_vamm_state(),
		mut swap_config in swap_config(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Ensure twap timestamp is in the past and that twap period is valid
		// for twap updates.
		vamm_state.twap_timestamp = 0;
		vamm_state.twap_period = MINIMUM_TWAP_PERIOD.into();

		// Disable output limit check.
		swap_config.output_amount_limit = None;

		swap_config.vamm_id = VammId::zero();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// For event emission.
			run_for_seconds(vamm_state.twap_period);
			if TestPallet::swap(&swap_config).is_ok() {
				let new_vamm_state = TestPallet::get_vamm(0).unwrap();
				System::assert_has_event(
					Event::UpdatedTwap { vamm_id: 0, base_twap: new_vamm_state.base_asset_twap }.into(),
				);
			}
		});
	}

	#[test]
	fn should_fail_if_insufficient_funds_base(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config(),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_lower_half(),
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Set correct values for test.
		swap_config.direction = Direction::Remove;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Base;
		swap_config.input_amount = input_amount;
		vamm_state.base_asset_reserves = base_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::InsufficientFundsForTrade
			);
		});
	}

	#[test]
	fn should_fail_if_insufficient_funds_quote(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config(),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_lower_half(),
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Set correct values for test.
		swap_config.direction = Direction::Remove;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Quote;
		swap_config.input_amount = input_amount;
		vamm_state.quote_asset_reserves = quote_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::InsufficientFundsForTrade
			);
		});
	}

	#[test]
	fn should_succeed_removing_base(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config()
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		// Disable output limit check
		swap_config.output_amount_limit = Some(0);

		// Set correct values for test.
		swap_config.direction = Direction::Remove;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Base;

		// Set correct values for test.
		swap_config.direction = Direction::Remove;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Base;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::swap(&swap_config));
		});
	}

	#[test]
	fn should_succeed_removing_quote(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config()
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		// Disable output limit check
		swap_config.output_amount_limit = Some(0);

		// Set correct values for test.
		swap_config.direction = Direction::Remove;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Quote;

		// Set correct values for test.
		swap_config.direction = Direction::Remove;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Quote;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::swap(&swap_config));
		});
	}

	#[test]
	fn should_fail_if_trade_extrapolates_maximum_supported_amount_base(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config(),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_upper_half(),
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Set correct values for test.
		swap_config.direction = Direction::Add;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Base;
		swap_config.input_amount = input_amount;
		vamm_state.base_asset_reserves = base_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount
			);
		});
	}

	#[test]
	fn should_fail_if_trade_extrapolates_maximum_supported_amount_quote(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config(),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_upper_half(),
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Set correct values for test.
		swap_config.direction = Direction::Add;
		swap_config.vamm_id = 0;
		swap_config.asset = AssetType::Quote;
		swap_config.input_amount = input_amount;
		vamm_state.quote_asset_reserves = quote_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount
			);
		});
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1))]
	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant(
		mut vamm_state in any_vamm_state(),
		mut swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open.
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for x in swap_config.iter_mut() {
				// Ensure we always perform operation on an existing vamm.
				x.vamm_id = Zero::zero();
				TestPallet::swap(x);
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

	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant_only_base(
		mut vamm_state in any_vamm_state(),
		mut swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open.
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for mut x in swap_config.iter_mut() {
				// Ensure we always perform operation on an existing vamm.
				x.vamm_id = Zero::zero();
				// Make swaps only for base asset
				x.asset = AssetType::Base;
				TestPallet::swap(x);
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
				// Ensure we always perform operation on an existing vamm.
				x.vamm_id = Zero::zero();
				// Make swaps only for quote asset
				x.asset = AssetType::Quote;
				TestPallet::swap(x);
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
