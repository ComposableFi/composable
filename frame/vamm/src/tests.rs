use crate::{
	mock::{Balance, Event, ExtBuilder, MockRuntime, Origin, System, TestPallet, Timestamp},
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

type Decimal = <MockRuntime as pallet::Config>::Decimal;
type VammTimestamp = <MockRuntime as pallet::Config>::Timestamp;
type VammId = <TestPallet as VammTrait>::VammId;

#[derive(Default)]
struct TestVammState<Balance, VammTimestamp> {
	base_asset_reserves: Option<Balance>,
	quote_asset_reserves: Option<Balance>,
	peg_multiplier: Option<Balance>,
	closed: Option<Option<VammTimestamp>>,
}

#[derive(Default)]
struct TestSwapConfig<VammId, Balance> {
	vamm_id: Option<VammId>,
	asset: Option<AssetType>,
	input_amount: Option<Balance>,
	direction: Option<Direction>,
	output_amount_limit: Option<Balance>,
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
	fn balance_range()(
		range in MINIMUM_RESERVE..=MAXIMUM_RESERVE
	) -> Balance {
		range
	}
}

prop_compose! {
	fn balance_range_lower_half()(
		range in MINIMUM_RESERVE..MAXIMUM_RESERVE/2
	) -> Balance {
		range
	}
}

prop_compose! {
	fn balance_range_upper_half()(
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
	fn loop_times()(
		loop_times in MINIMUM_RESERVE..=500,
	) -> Balance {
		loop_times
	}
}

prop_compose! {
	fn timestamp()(
		t in VammTimestamp::MIN..=VammTimestamp::MAX
	) -> VammTimestamp {
		t
	}
}

prop_compose! {
	fn get_vamm_state(config: TestVammState<Balance, VammTimestamp>)(
		(base_asset_reserves, quote_asset_reserves, peg_multiplier) in min_max_reserve(),
		closed in prop_oneof![timestamp().prop_map(|t| Some(t)), Just(None)]

	) -> VammState<Balance, VammTimestamp> {
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
			closed: config
				.closed
				.unwrap_or(closed),
		}
	}
}

prop_compose! {
	fn get_swap_config(config: TestSwapConfig<VammId, Balance>)(
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
			let vamm_counter = TestPallet::vamm_count();

			let vamm_expected = VammState::<Balance, VammTimestamp> {
					base_asset_reserves,
					quote_asset_reserves,
					peg_multiplier,
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
					TestPallet::get_price(0, AssetType::Base),
					Err(DispatchError::Arithmetic(ArithmeticError::Overflow)))
			} else if quote_peg.unwrap().checked_div(base_asset_reserves).is_none() {
				assert_eq!(
					TestPallet::get_price(0, AssetType::Base),
					Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero)))
			} else {
				assert_eq!(
					TestPallet::get_price(0, AssetType::Base),
					Ok(Decimal::from_inner(quote_peg.unwrap().checked_div(base_asset_reserves).unwrap())))
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
					TestPallet::get_price(0, AssetType::Quote),
					Err(DispatchError::Arithmetic(ArithmeticError::Overflow)))
			} else if quote_peg.unwrap().checked_div(base_asset_reserves).is_none() {
				assert_eq!(
					TestPallet::get_price(0, AssetType::Quote),
					Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero)))
			} else {
				assert_eq!(
					TestPallet::get_price(0, AssetType::Quote),
					Ok(Decimal::from_inner(quote_peg.unwrap().checked_div(base_asset_reserves).unwrap())))
			}
		})
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Swap Asset
// ----------------------------------------------------------------------------------------------------

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
			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::VammDoesNotExist);
		})
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Swap Base Asset
// ----------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_base_remove_insufficient_funds_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_lower_half(),
	) {
		prop_assume!(input_amount > base_asset_reserves);
		prop_assume!(swap_config.direction == Direction::Remove);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			swap_config.input_amount = input_amount;
			vamm_state.base_asset_reserves = base_asset_reserves;
			VammMap::<MockRuntime>::mutate(0, |vamm| {
				*vamm = Some(vamm_state);
			});

			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::InsufficientFundsForTrade);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	#[ignore = "to be implemented"]
	// TODO(Cardosaum): Implement test correctly
	fn swap_base_remove_succeeds(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
		input_amount in balance_range_lower_half(),
		base_asset_reserves in balance_range_upper_half(),
	) {
		prop_assume!(input_amount <= base_asset_reserves);
		prop_assume!(swap_config.direction == Direction::Remove);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			swap_config.input_amount = input_amount;
			vamm_state.base_asset_reserves = base_asset_reserves;
			VammMap::<MockRuntime>::mutate(0, |vamm| {
				*vamm = Some(vamm_state);
			});

			let swap = TestPallet::swap(&swap_config);
			assert_ok!(swap);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_base_add_trade_extrapoles_maximum_supported_amount_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Add),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_upper_half(),
	) {
		prop_assume!(swap_config.direction == Direction::Add);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			swap_config.input_amount = input_amount;
			vamm_state.base_asset_reserves = base_asset_reserves;
			VammMap::<MockRuntime>::mutate(0, |vamm| {
				*vamm = Some(vamm_state);
			});

			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount);
		})
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Swap Quote Asset
// ----------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_quote_remove_insufficient_funds_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Quote),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_lower_half(),
	) {
		prop_assume!(input_amount > quote_asset_reserves);
		prop_assume!(swap_config.direction == Direction::Remove);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			swap_config.input_amount = input_amount;
			vamm_state.quote_asset_reserves = quote_asset_reserves;
			VammMap::<MockRuntime>::mutate(0, |vamm| {
				*vamm = Some(vamm_state);
			});

			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::InsufficientFundsForTrade);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn swap_quote_add_trade_extrapoles_maximum_supported_amount_error(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Add),
				vamm_id: Some(0),
				asset: Some(AssetType::Quote),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_upper_half(),
	) {
		prop_assume!(swap_config.direction == Direction::Add);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			swap_config.input_amount = input_amount;
			vamm_state.quote_asset_reserves = quote_asset_reserves;
			VammMap::<MockRuntime>::mutate(0, |vamm| {
				*vamm = Some(vamm_state);
			});

			// dbg!(Timestamp::now());
			let swap = TestPallet::swap(&swap_config);
			assert_err!(swap, Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount);
		})
	}
}
