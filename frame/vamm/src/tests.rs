use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, Origin, System, Timestamp, Vamm},
	pallet,
	pallet::{Error, VammMap, VammState},
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, Vamm as VammTrait, VammConfig};
use frame_support::{assert_err, assert_noop, assert_ok, pallet_prelude::Hooks};
use proptest::prelude::*;
use sp_runtime::{ArithmeticError, DispatchError};

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
const RUN_CASES: u32 = 1000;

prop_compose! {
	fn min_max_reserve()(
		base_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		quote_asset_reserves in MINIMUM_RESERVE..=MAXIMUM_RESERVE,
		peg_multiplier in MINIMUM_RESERVE..=MAXIMUM_RESERVE
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
	fn loop_times()(
		loop_times in MINIMUM_RESERVE..=500,
	) -> Balance {
		loop_times
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Create Vamm
// ----------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[allow(clippy::disallowed_methods)]
	fn create_vamm(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve()
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

	#[test]
	fn create_vamm_succeeds(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve()
	) {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(Vamm::create(
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
				Vamm::create(
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
				Vamm::create(
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
				Vamm::create(
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

	#[test]
	#[allow(clippy::disallowed_methods)]
	fn create_vamm_emits_event_succeeds(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
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

	#[test]
	fn create_vamm_updates_storage_map(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
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

// ----------------------------------------------------------------------------------------------------
//                                             Get Price
// ----------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_base_asset(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![
				(0,
				 VammState{
					 base_asset_reserves,
					 quote_asset_reserves,
					 peg_multiplier,
					 closed: None})]
		}.build().execute_with(|| {
			let quote_peg = quote_asset_reserves.checked_mul(peg_multiplier);
			if quote_peg.is_none() {
				assert_eq!(
					Vamm::get_price(0, AssetType::Base),
					Err(DispatchError::Arithmetic(ArithmeticError::Overflow)))
			} else if quote_peg.unwrap().checked_div(base_asset_reserves).is_none() {
				assert_eq!(
					Vamm::get_price(0, AssetType::Base),
					Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero)))
			} else {
				assert_eq!(
					Vamm::get_price(0, AssetType::Base),
					Ok(quote_peg.unwrap().checked_div(base_asset_reserves).unwrap()))
			}
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[allow(clippy::disallowed_methods)]
	fn get_price_quote_asset(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
	) {
		ExtBuilder {
			vamm_count: 1,
			vamms: vec![
				(0,
				 VammState{
					 base_asset_reserves,
					 quote_asset_reserves,
					 peg_multiplier,
					 closed: None})]
		}.build().execute_with(|| {
			let quote_peg = quote_asset_reserves.checked_mul(peg_multiplier);
			if quote_peg.is_none() {
				assert_eq!(
					Vamm::get_price(0, AssetType::Quote),
					Err(DispatchError::Arithmetic(ArithmeticError::Overflow)))
			} else if quote_peg.unwrap().checked_div(base_asset_reserves).is_none() {
				assert_eq!(
					Vamm::get_price(0, AssetType::Quote),
					Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero)))
			} else {
				assert_eq!(
					Vamm::get_price(0, AssetType::Quote),
					Ok(quote_peg.unwrap().checked_div(base_asset_reserves).unwrap()))
			}
		})
	}
}
