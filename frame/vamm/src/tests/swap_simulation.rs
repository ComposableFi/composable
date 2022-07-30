use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::{Error, VammMap},
	tests::{
		constants::{
			BASE_REQUIRED_FOR_REMOVING_QUOTE, BASE_RETURNED_AFTER_ADDING_QUOTE,
			QUOTE_REQUIRED_FOR_REMOVING_BASE, QUOTE_RETURNED_AFTER_ADDING_BASE, RUN_CASES,
		},
		helpers::{run_for_seconds, with_swap_context},
		helpers_propcompose::{any_swap_config, any_vamm_state},
		types::{TestSwapConfig, TestVammConfig},
	},
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, SwapOutput, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;
use sp_runtime::traits::{One, Zero};

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_vamm_does_not_exist() {
	with_swap_context(
		TestVammConfig::default(),
		TestSwapConfig { vamm_id: One::one(), ..Default::default() },
		|_, swap_config| {
			assert_noop!(
				TestPallet::swap_simulation(&swap_config),
				Error::<MockRuntime>::VammDoesNotExist
			);
		},
	);
}

#[test]
fn should_fail_if_vamm_is_closed() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = TestVammConfig::default();
		let swap_config = TestSwapConfig::default();
		VammMap::<MockRuntime>::mutate(
			TestPallet::create(&vamm_config.into()).unwrap(),
			|vamm_state| match vamm_state {
				Some(v) => v.closed = Some(Zero::zero()),
				None => (),
			},
		);
		run_for_seconds(One::one());
		assert_noop!(
			TestPallet::swap_simulation(&swap_config.into()),
			Error::<MockRuntime>::VammIsClosed
		);
	});
}

#[test]
fn should_not_modify_runtime_storage_add_base() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Base,
			direction: Direction::Add,
			..swap_config
		}));
	});
}

#[test]
fn should_not_modify_runtime_storage_remove_base() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Base,
			direction: Direction::Remove,
			..swap_config
		}));
	});
}

#[test]
fn should_not_modify_runtime_storage_add_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Add,
			..swap_config
		}));
	});
}

#[test]
fn should_not_modify_runtime_storage_remove_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Remove,
			..swap_config
		}));
	});
}

#[test]
fn should_return_correct_value_add_base() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_ok!(
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Base,
				direction: Direction::Add,
				..swap_config
			}),
			SwapOutput { output: QUOTE_RETURNED_AFTER_ADDING_BASE, negative: false }
		);
	});
}

#[test]
fn should_return_correct_value_remove_base() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_ok!(
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Base,
				direction: Direction::Remove,
				..swap_config
			}),
			SwapOutput { output: QUOTE_REQUIRED_FOR_REMOVING_BASE, negative: true }
		);
	});
}

#[test]
fn should_return_correct_value_add_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_ok!(
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Quote,
				direction: Direction::Add,
				..swap_config
			}),
			SwapOutput { output: BASE_RETURNED_AFTER_ADDING_QUOTE, negative: false }
		);
	});
}

#[test]
fn should_return_correct_value_remove_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_ok!(
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Quote,
				direction: Direction::Remove,
				..swap_config
			}),
			SwapOutput { output: BASE_REQUIRED_FOR_REMOVING_QUOTE, negative: true }
		);
	});
}

// -------------------------------------------------------------------------------------------------
//                                             Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn should_not_update_runtime_storage(
		mut vamm_state in any_vamm_state(),
		mut swap_config in any_swap_config()
	) {
		// Ensure vamm is always open.
		vamm_state.closed = None;
		// Ensure we always perform operation on an existing vamm.
		swap_config.vamm_id = Zero::zero();

		with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
			let vamm_state_before = TestPallet::get_vamm(swap_config.vamm_id).unwrap();
			assert_storage_noop!(TestPallet::swap_simulation(&swap_config));
			let vamm_state_after = TestPallet::get_vamm(swap_config.vamm_id).unwrap();
			assert_eq!(vamm_state_before, vamm_state_after);
		});
	}
}
