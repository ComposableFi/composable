use crate::{
	mock::{Balance, ExtBuilder, MockRuntime, System, TestPallet, VammId},
	pallet::{Error, Event, VammMap},
	tests::{
		constants::{
			BASE_ASSET_RESERVES_AFTER_ADDING_BASE, BASE_ASSET_RESERVES_AFTER_ADDING_QUOTE,
			BASE_ASSET_RESERVES_AFTER_REMOVING_BASE, BASE_ASSET_RESERVES_AFTER_REMOVING_QUOTE,
			BASE_REQUIRED_FOR_REMOVING_QUOTE, BASE_RETURNED_AFTER_ADDING_QUOTE,
			QUOTE_ASSET_RESERVES_AFTER_ADDING_BASE, QUOTE_ASSET_RESERVES_AFTER_ADDING_QUOTE,
			QUOTE_ASSET_RESERVES_AFTER_REMOVING_BASE, QUOTE_ASSET_RESERVES_AFTER_REMOVING_QUOTE,
			QUOTE_REQUIRED_FOR_REMOVING_BASE, QUOTE_RETURNED_AFTER_ADDING_BASE, RUN_CASES,
		},
		helpers::{
			run_for_seconds, run_to_block, swap_config, with_existent_vamm_swap_contex,
			with_swap_context,
		},
		helpers_propcompose::{
			any_swap_config, any_vamm_state, balance_range_lower_half, balance_range_upper_half,
			multiple_swaps, then_and_now,
		},
		types::{TestSwapConfig, TestVammConfig, Timestamp},
	},
};
use composable_traits::vamm::{
	AssetType, Direction, SwapOutput, Vamm as VammTrait, MINIMUM_TWAP_PERIOD,
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
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig::default(),
		|vamm_config, swap_config| {
			// For event emission
			run_to_block(1);

			let swap = TestPallet::swap(&swap_config);
			let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
			assert_ok!(
				swap,
				SwapOutput { output: QUOTE_RETURNED_AFTER_ADDING_BASE, negative: false }
			);
			assert_eq!(vamm_after_swap.base_asset_reserves, BASE_ASSET_RESERVES_AFTER_ADDING_BASE);
			assert_eq!(
				vamm_after_swap.quote_asset_reserves,
				QUOTE_ASSET_RESERVES_AFTER_ADDING_BASE
			);
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
		},
	);
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_add_quote() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig { asset: AssetType::Quote, direction: Direction::Add, ..Default::default() },
		|vamm_config, swap_config| {
			// For event emission
			run_to_block(1);

			let swap = TestPallet::swap(&swap_config);
			let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
			assert_ok!(
				swap,
				SwapOutput { output: BASE_RETURNED_AFTER_ADDING_QUOTE, negative: false }
			);
			assert_eq!(vamm_after_swap.base_asset_reserves, BASE_ASSET_RESERVES_AFTER_ADDING_QUOTE);
			assert_eq!(
				vamm_after_swap.quote_asset_reserves,
				QUOTE_ASSET_RESERVES_AFTER_ADDING_QUOTE
			);
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
		},
	);
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_remove_base() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig {
			asset: AssetType::Base,
			direction: Direction::Remove,
			..Default::default()
		},
		|vamm_config, swap_config| {
			// For event emission
			run_to_block(1);

			let swap = TestPallet::swap(&swap_config);
			let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
			assert_ok!(
				swap,
				SwapOutput { output: QUOTE_REQUIRED_FOR_REMOVING_BASE, negative: true }
			);
			assert_eq!(
				vamm_after_swap.base_asset_reserves,
				BASE_ASSET_RESERVES_AFTER_REMOVING_BASE
			);
			assert_eq!(
				vamm_after_swap.quote_asset_reserves,
				QUOTE_ASSET_RESERVES_AFTER_REMOVING_BASE
			);
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
		},
	);
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_remove_quote() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Remove,
			..Default::default()
		},
		|vamm_config, swap_config| {
			// For event emission
			run_to_block(1);

			let swap = TestPallet::swap(&swap_config);
			let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
			assert_ok!(
				swap,
				SwapOutput { output: BASE_REQUIRED_FOR_REMOVING_QUOTE, negative: true }
			);
			assert_eq!(
				vamm_after_swap.base_asset_reserves,
				BASE_ASSET_RESERVES_AFTER_REMOVING_QUOTE
			);
			assert_eq!(
				vamm_after_swap.quote_asset_reserves,
				QUOTE_ASSET_RESERVES_AFTER_REMOVING_QUOTE
			);
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
		},
	);
}

#[test]
fn should_update_twap_when_adding_base_asset() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig { asset: AssetType::Base, direction: Direction::Add, ..Default::default() },
		|_, swap_config| {
			// For event emission
			run_to_block(1);

			// Get Initial Vamm State
			let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

			// Perform swap
			run_for_seconds(vamm_state_initial.twap_period);
			assert_ok!(TestPallet::swap(&swap_config));

			// Ensure twap was updated
			let vamm_state = TestPallet::get_vamm(0).unwrap();
			assert_ne!(vamm_state_initial.twap_timestamp, vamm_state.twap_timestamp);
		},
	);
}

#[test]
fn should_update_twap_when_removing_base_asset() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig {
			asset: AssetType::Base,
			direction: Direction::Remove,
			..Default::default()
		},
		|_, swap_config| {
			// For event emission
			run_to_block(1);

			// Get Initial Vamm State
			let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

			// Perform swap
			run_for_seconds(vamm_state_initial.twap_period);
			assert_ok!(TestPallet::swap(&swap_config));

			// Ensure twap was updated
			let vamm_state = TestPallet::get_vamm(0).unwrap();
			assert_ne!(vamm_state_initial.twap_timestamp, vamm_state.twap_timestamp);
		},
	);
}

#[test]
fn should_update_twap_when_adding_quote_asset() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig { asset: AssetType::Quote, direction: Direction::Add, ..Default::default() },
		|_, swap_config| {
			// For event emission
			run_to_block(1);

			// Get Initial Vamm State
			let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

			// Perform swap
			run_for_seconds(vamm_state_initial.twap_period);
			assert_ok!(TestPallet::swap(&swap_config));

			// Ensure twap was updated
			let vamm_state = TestPallet::get_vamm(0).unwrap();
			assert_ne!(vamm_state_initial.twap_timestamp, vamm_state.twap_timestamp);
		},
	);
}

#[test]
fn should_update_twap_when_removing_quote_asset() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Remove,
			..Default::default()
		},
		|_, swap_config| {
			// For event emission
			run_to_block(1);

			// Get Initial Vamm State
			let vamm_state_initial = TestPallet::get_vamm(0).unwrap();

			// Perform swap
			run_for_seconds(vamm_state_initial.twap_period);
			assert_ok!(TestPallet::swap(&swap_config));

			// Ensure twap was updated
			let vamm_state = TestPallet::get_vamm(0).unwrap();
			assert_ne!(vamm_state_initial.twap_timestamp, vamm_state.twap_timestamp);
		},
	);
}

#[test]
fn should_not_update_twap_if_current_twap_timestamp_is_more_recent() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		let vamm_state_t0 = TestPallet::get_vamm(0).unwrap();

		// In the first swap the current time was more recent then the time when
		// the twap update happened, so we must ensure we updated the twap
		// timestamp.
		let delay = vamm_state_t0.twap_period - 1;
		run_for_seconds(delay);
		assert_ok!(TestPallet::swap(&swap_config));
		let vamm_state_t1 = TestPallet::get_vamm(0).unwrap();
		assert_eq!(vamm_state_t1.twap_timestamp, vamm_state_t0.twap_timestamp + delay);
		assert_ne!(vamm_state_t0.twap_timestamp, vamm_state_t1.twap_timestamp);

		// In the second swap the time didn't pass between operations, so we
		// can't allow the twap timestamp nor the twap value to be updated
		// again.
		assert_ok!(TestPallet::swap(&swap_config));
		let vamm_state_t2 = TestPallet::get_vamm(0).unwrap();
		assert_eq!(vamm_state_t1.twap_timestamp, vamm_state_t2.twap_timestamp);
		assert_eq!(vamm_state_t1.base_asset_twap, vamm_state_t2.base_asset_twap);
	});
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_noop!(
					TestPallet::swap(&swap_config),
					Error::<MockRuntime>::VammDoesNotExist
				);
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				run_to_block(now);
				assert_noop!(
					TestPallet::swap(&swap_config),
					Error::<MockRuntime>::VammIsClosed
				);
			}
		)
	}

	#[test]
	fn should_fail_if_output_is_less_than_minimum_limit(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config(),
		limit in balance_range_upper_half(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		swap_config.output_amount_limit = Some(limit);
		swap_config.vamm_id = VammId::zero();

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_noop!(
					TestPallet::swap(&swap_config),
					Error::<MockRuntime>::SwappedAmountLessThanMinimumLimit
				);
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
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
			}
		)
	}

	#[test]
	fn should_succeed_updating_twap_when_performing_swap(
		mut vamm_state in any_vamm_state(),
		mut swap_config in swap_config(),
		delta in Timestamp::MIN+1..=Timestamp::MAX,
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				run_for_seconds(delta);
				match TestPallet::swap(&swap_config) {
					Ok(_) => {
						let vamm_state_after = TestPallet::get_vamm(0).unwrap();
						assert_ne!(vamm_state.twap_timestamp, vamm_state_after.twap_timestamp);
					},
					_ => {
						let vamm_state_after = TestPallet::get_vamm(0).unwrap();
						assert_eq!(vamm_state.twap_timestamp, vamm_state_after.twap_timestamp);
					}
				}
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_noop!(
					TestPallet::swap(&swap_config),
					Error::<MockRuntime>::InsufficientFundsForTrade
				);
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_noop!(
					TestPallet::swap(&swap_config),
					Error::<MockRuntime>::InsufficientFundsForTrade
				);
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_ok!(TestPallet::swap(&swap_config));
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_ok!(TestPallet::swap(&swap_config));
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_noop!(
					TestPallet::swap(&swap_config),
					Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount
				);
			}
		)
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

		with_existent_vamm_swap_contex(
			vamm_state,
			swap_config,
			|swap_config| {
				assert_noop!(
					TestPallet::swap(&swap_config),
					Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount
				);
			}
		)
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
