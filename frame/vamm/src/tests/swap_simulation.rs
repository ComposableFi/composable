use crate::{
	mock::{ExtBuilder, MockRuntime, TestPallet},
	pallet::{Error, VammMap},
	tests::{
		constants::{
			DEFAULT_BASE_REQUIRED_FOR_REMOVING_QUOTE, DEFAULT_BASE_RETURNED_AFTER_ADDING_QUOTE,
			DEFAULT_QUOTE_REQUIRED_FOR_REMOVING_BASE, DEFAULT_QUOTE_RETURNED_AFTER_ADDING_BASE,
			RUN_CASES,
		},
		helpers::{run_for_seconds, with_swap_context},
		helpers_propcompose::any_swap_config,
		types::{TestSwapConfig, TestVammConfig},
	},
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, SwapOutput, Vamm as VammTrait};
use frame_support::{assert_noop, assert_storage_noop};
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
		|swap_config| {
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
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Base,
			direction: Direction::Add,
			..swap_config
		}));
	});
}

#[test]
fn should_not_modify_runtime_storage_remove_base() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Base,
			direction: Direction::Remove,
			..swap_config
		}));
	});
}

#[test]
fn should_not_modify_runtime_storage_add_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Add,
			..swap_config
		}));
	});
}

#[test]
fn should_not_modify_runtime_storage_remove_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset: AssetType::Quote,
			direction: Direction::Remove,
			..swap_config
		}));
	});
}

#[test]
fn should_return_correct_value_add_base() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_eq!(
			SwapOutput { output: DEFAULT_QUOTE_RETURNED_AFTER_ADDING_BASE, negative: false },
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Quote,
				direction: Direction::Add,
				..swap_config
			})
			.unwrap()
		);
	});
}

#[test]
fn should_return_correct_value_remove_base() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_eq!(
			SwapOutput { output: DEFAULT_QUOTE_REQUIRED_FOR_REMOVING_BASE, negative: false },
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Base,
				direction: Direction::Remove,
				..swap_config
			})
			.unwrap()
		);
	});
}

#[test]
fn should_return_correct_value_add_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_eq!(
			SwapOutput { output: DEFAULT_BASE_RETURNED_AFTER_ADDING_QUOTE, negative: false },
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Quote,
				direction: Direction::Add,
				..swap_config
			})
			.unwrap()
		);
	});
}

#[test]
fn should_return_correct_value_remove_quote() {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
		assert_eq!(
			SwapOutput { output: DEFAULT_BASE_REQUIRED_FOR_REMOVING_QUOTE, negative: true },
			TestPallet::swap_simulation(&SwapConfig {
				asset: AssetType::Quote,
				direction: Direction::Remove,
				..swap_config
			})
			.unwrap()
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
		mut swap_config in any_swap_config()
	) {
		// Ensure we always perform operation on an existing vamm.
		swap_config.vamm_id = Zero::zero();

		with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |swap_config| {
			let vamm_state_before = TestPallet::get_vamm(swap_config.vamm_id).unwrap();
			TestPallet::swap_simulation(&swap_config);
			let vamm_state_after = TestPallet::get_vamm(swap_config.vamm_id).unwrap();
			assert_eq!(vamm_state_before, vamm_state_after);
		});
	}
}
