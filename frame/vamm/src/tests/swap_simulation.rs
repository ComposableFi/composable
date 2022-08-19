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
		types::{Balance, TestSwapConfig, TestVammConfig},
	},
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, SwapOutput, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok, assert_storage_noop};
use proptest::prelude::*;
use rstest::rstest;
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

#[rstest]
#[case(AssetType::Base, Direction::Add)]
#[case(AssetType::Base, Direction::Remove)]
#[case(AssetType::Quote, Direction::Add)]
#[case(AssetType::Quote, Direction::Remove)]
fn should_not_modify_runtime_storage(#[case] asset: AssetType, #[case] direction: Direction) {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_storage_noop!(TestPallet::swap_simulation(&SwapConfig {
			asset,
			direction,
			..swap_config
		}));
	});
}

#[rstest]
#[case(AssetType::Base, Direction::Add, SwapOutput { output: QUOTE_RETURNED_AFTER_ADDING_BASE, negative: false })]
#[case(AssetType::Base, Direction::Remove, SwapOutput { output: QUOTE_REQUIRED_FOR_REMOVING_BASE, negative: true })]
#[case(AssetType::Quote, Direction::Add, SwapOutput { output: BASE_RETURNED_AFTER_ADDING_QUOTE, negative: false })]
#[case(AssetType::Quote, Direction::Remove, SwapOutput { output: BASE_REQUIRED_FOR_REMOVING_QUOTE, negative: true })]
fn should_return_correct_value(
	#[case] asset: AssetType,
	#[case] direction: Direction,
	#[case] expected: SwapOutput<Balance>,
) {
	with_swap_context(TestVammConfig::default(), TestSwapConfig::default(), |_, swap_config| {
		assert_ok!(
			TestPallet::swap_simulation(&SwapConfig { asset, direction, ..swap_config }),
			expected
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
