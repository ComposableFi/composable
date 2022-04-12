use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, Origin, System, Timestamp, Vamm},
	pallet,
	pallet::{Error, VammMap, VammState},
};

use composable_traits::vamm::{Vamm as VammTrait, VammConfig};

use proptest::prelude::*;

use frame_support::{assert_noop, assert_ok, pallet_prelude::Hooks};

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

// ----------------------------------------------------------------------------------------------------
//                                             Prop_compose
// ----------------------------------------------------------------------------------------------------

#[allow(dead_code)]
const ZERO_RESERVE: Balance = Balance::MIN;

#[allow(dead_code)]
const MINIMUM_RESERVE: Balance = ZERO_RESERVE + 1;

#[allow(dead_code)]
const MAXIMUM_RESERVE: Balance = Balance::MAX;

#[allow(dead_code)]
const RUN_CASES: u32 = 100;

// ----------------------------------------------------------------------------------------------------
//                                             Create Vamm
// ----------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[allow(clippy::disallowed_methods)]
	fn create_vamm(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) {
		ExtBuilder::default().build().execute_with(|| {
			let vamm_counter = Vamm::vamm_count();

			let vamm_expected = VammState::<Balance, <MockRuntime as pallet::Config>::Timestamp> {
					base_asset_reserves,
					quote_asset_reserves,
					peg_multiplier,
					closed: Default::default(),
			};

			let vamm_created_ok = Vamm::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier});
			let vamm_created_some = Vamm::get_vamm(vamm_created_ok.unwrap());

			assert_ok!(vamm_created_ok);
			assert_eq!(vamm_created_some, Some(vamm_expected));

			assert_eq!(Vamm::vamm_count(), vamm_counter+1);
		});
	}


	fn create_vamm_succeeds(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(Vamm::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier}));
		});
	}


	fn create_vamm_zero_base_asset_reserves_error(
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
				Vamm::create(
					&VammConfig{base_asset_reserves: ZERO_RESERVE,
							   quote_asset_reserves,
							   peg_multiplier}),
				Error::<MockRuntime>::BaseAssetReserveIsZero);
		})
	}


	fn create_vamm_zero_quote_asset_reserves_error(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
				Vamm::create(
					&VammConfig{base_asset_reserves,
							quote_asset_reserves: ZERO_RESERVE,
							peg_multiplier}),
				Error::<MockRuntime>::QuoteAssetReserveIsZero);
		})
	}


	fn create_vamm_zero_peg_multiplier_error(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_noop!(
				Vamm::create(
					&VammConfig{base_asset_reserves,
							   quote_asset_reserves,
							   peg_multiplier: ZERO_RESERVE}),
				Error::<MockRuntime>::PegMultiplierIsZero);
		})
	}


	fn create_vamm_update_counter_succeeds(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		loop_times in MINIMUM_RESERVE..=100
	) {
		ExtBuilder::default().build().execute_with(|| {
			let markets = Vamm::vamm_count();

			for _ in 0..loop_times {
				assert_ok!(Vamm::create(
					&VammConfig{base_asset_reserves,
							   quote_asset_reserves,
							   peg_multiplier}));
			}

			assert_eq!(Vamm::vamm_count(), markets + loop_times);
		});
	}


	#[allow(clippy::disallowed_methods)]
	fn create_vamm_emits_event_succeeds(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) {
		ExtBuilder::default().build().execute_with(|| {
			System::set_block_number(1);

			let vamm_created_ok = Vamm::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier});
			let vamm_created = Vamm::get_vamm(vamm_created_ok.unwrap()).unwrap();
			assert_ok!(vamm_created_ok);

			System::assert_last_event(Event::Vamm(
				pallet::Event::Created { vamm_id: 0_u128, state: vamm_created}
			))
		});
	}


	fn create_vamm_updates_storage_map(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert!(!VammMap::<MockRuntime>::contains_key(0_u128));

			let vamm_created_ok = Vamm::create(
				&VammConfig{base_asset_reserves,
						   quote_asset_reserves,
						   peg_multiplier});
			assert_ok!(vamm_created_ok);

			assert!(VammMap::<MockRuntime>::contains_key(0_u128));
		});
	}
}
