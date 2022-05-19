use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, System, TestPallet},
	pallet::{self, Error, VammMap, VammState},
	tests::{loop_times, min_max_reserve, zero_reserve, VammTimestamp, RUN_CASES},
};
use composable_traits::vamm::{Vamm as VammTrait, VammConfig};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[allow(clippy::disallowed_methods)]
	fn create_vamm_correctly_returns_vamm_state(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let vamm_counter = TestPallet::vamm_count();

			let invariant = TestPallet::compute_invariant(
				base_asset_reserves,
				quote_asset_reserves
			).unwrap();

			let vamm_expected = VammState::<Balance, VammTimestamp> {
					base_asset_reserves,
					quote_asset_reserves,
					peg_multiplier,
					invariant,
					closed: Default::default(),
			};

			let vamm_created_ok = TestPallet::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier});
			let vamm_created_some = TestPallet::get_vamm(vamm_created_ok.unwrap());

			assert_ok!(vamm_created_ok);
			assert_eq!(vamm_created_some, Some(vamm_expected));

			assert_eq!(TestPallet::vamm_count(), vamm_counter+1);
		});
	}

	#[test]
	fn create_vamm_succeeds(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(TestPallet::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier}));
		});
	}

	#[test]
	fn create_vamm_zero_base_asset_reserves_error(
		base_asset_reserves in zero_reserve(),
		(_, quote_asset_reserves, peg_multiplier) in min_max_reserve()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
				TestPallet::create(
					&VammConfig{base_asset_reserves,
							   quote_asset_reserves,
							   peg_multiplier}),
				Error::<MockRuntime>::BaseAssetReserveIsZero);
		})
	}

	#[test]
	fn create_vamm_zero_quote_asset_reserves_error(
		quote_asset_reserves in zero_reserve(),
		(base_asset_reserves, _, peg_multiplier) in min_max_reserve()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
				TestPallet::create(
					&VammConfig{base_asset_reserves,
							quote_asset_reserves,
							peg_multiplier}),
				Error::<MockRuntime>::QuoteAssetReserveIsZero);
		})
	}

	#[test]
	fn create_vamm_zero_peg_multiplier_error(
		peg_multiplier in zero_reserve(),
		(base_asset_reserves, quote_asset_reserves, _) in min_max_reserve()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
				TestPallet::create(
					&VammConfig{base_asset_reserves,
							   quote_asset_reserves,
							   peg_multiplier}),
				Error::<MockRuntime>::PegMultiplierIsZero);
		})
	}

	#[test]
	fn create_vamm_update_counter_succeeds(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
		loop_times in loop_times()
	) {
		ExtBuilder::default().build().execute_with(|| {
			let markets = TestPallet::vamm_count();

			for _ in 0..loop_times {
				assert_ok!(TestPallet::create(
					&VammConfig{base_asset_reserves,
							   quote_asset_reserves,
							   peg_multiplier}));
			}

			assert_eq!(TestPallet::vamm_count(), markets + loop_times);
		});
	}

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn create_vamm_emits_event_succeeds(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
	) {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);

			let vamm_created_ok = TestPallet::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier});
			let vamm_created = TestPallet::get_vamm(vamm_created_ok.unwrap()).unwrap();
			assert_ok!(vamm_created_ok);

			System::assert_last_event(Event::TestPallet(
				pallet::Event::Created { vamm_id: 0_u128, state: vamm_created}
			))
		});
	}

	#[test]
	fn create_vamm_updates_storage_map(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert!(!VammMap::<MockRuntime>::contains_key(0_u128));

			let vamm_created_ok = TestPallet::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier});
			assert_ok!(vamm_created_ok);

			assert!(VammMap::<MockRuntime>::contains_key(0_u128));
		});
	}
}
