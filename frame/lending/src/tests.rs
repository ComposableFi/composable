#![allow(unused_imports)]

//! Test for Lending. Runtime is almost real.
//! TODO: cover testing events - so make sure that each even is handled at least once
//! (events can be obtained from System pallet as in banchmarking.rs before this commit)
//! TODO: OCW of liquidations (like in Oracle)
//! TODO: test on small numbers via proptests - detect edge case what is minimal amounts it starts
//! to accure(and miminal block delta), and maximal amounts when it overflows

use composable_traits::lending::{Lending as LendingTrait, RepayStrategy, TotalDebtWithInterest};
use frame_benchmarking::Zero;
use std::ops::{Div, Mul};

use crate::{
	self as pallet_lending, accrue_interest_internal, currency::*, mocks::*,
	models::borrower_data::BorrowerData, setup::assert_last_event, AccruedInterest, Error,
	MarketIndex,
};
use codec::{Decode, Encode};
use composable_support::validation::{TryIntoValidated, Validated};
use composable_tests_helpers::{prop_assert_acceptable_computation_error, prop_assert_ok};
use composable_traits::{
	defi::{CurrencyPair, LiftedFixedBalance, MoreThanOneFixedU128, Rate, ZeroToOneFixedU128},
	lending::{self, math::*, CreateInput, UpdateInput, UpdateInputValid},
	oracle,
	time::SECONDS_PER_YEAR_NAIVE,
	vault::{self, Deposit, VaultConfig},
};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	dispatch::{DispatchErrorWithPostInfo, DispatchResultWithPostInfo, PostDispatchInfo},
	traits::fungibles::{Inspect, Mutate},
	weights::Pays,
};
use frame_system::{EventRecord, Phase};
use pallet_vault::models::VaultInfo;
use proptest::{prelude::*, test_runner::TestRunner};
use sp_arithmetic::assert_eq_error_rate;
use sp_core::{H256, U256};
use sp_runtime::{
	ArithmeticError, DispatchError, FixedPointNumber, FixedU128, ModuleError, Percent, Perquintill,
};

const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);
const DEFAULT_MARKET_VAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);
const DEFAULT_COLLATERAL_FACTOR: u128 = 2;
const DEFAULT_ACTUAL_BLOCKS_COUNT: u64 = 1020;

#[test]
fn accrue_interest_base_cases() {
	let (optimal, ref mut interest_rate_model) = new_jump_model();
	let stable_rate = interest_rate_model.get_borrow_rate(optimal).unwrap();
	assert_eq!(stable_rate, ZeroToOneFixedU128::saturating_from_rational(10_u128, 100_u128));
	let borrow_index = Rate::saturating_from_integer(1_u128);
	let delta_time = SECONDS_PER_YEAR_NAIVE;
	let total_issued = 100_000_000_000_000_000_000;
	let accrued_debt = 0;
	let total_borrows = total_issued - accrued_debt;
	let AccruedInterest { accrued_increment: accrued_increase, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			optimal,
			interest_rate_model,
			borrow_index,
			delta_time,
			total_borrows,
		)
		.unwrap();
	assert_eq!(accrued_increase, 10_000_000_000_000_000_000);

	let delta_time = MILLISECS_PER_BLOCK;
	let AccruedInterest { accrued_increment: accrued_increase, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			optimal,
			interest_rate_model,
			borrow_index,
			delta_time,
			total_borrows,
		)
		.unwrap();
	// small increments instead one year lead to some loss by design (until we lift calculation to
	// 256 bit)
	let error = 25;
	assert_eq!(
		accrued_increase,
		10_000_000_000_000_000_000 * MILLISECS_PER_BLOCK as u128 / SECONDS_PER_YEAR_NAIVE as u128 +
			error
	);
}

#[test]
fn apr_for_zero() {
	let (_, ref mut interest_rate_model) = new_jump_model();
	let utilization = Percent::from_percent(100);
	let borrow_index = Rate::saturating_from_integer(1_u128);

	let AccruedInterest { accrued_increment: accrued_increase, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			utilization,
			interest_rate_model,
			borrow_index,
			SECONDS_PER_YEAR_NAIVE,
			0,
		)
		.unwrap();
	assert_eq!(accrued_increase, 0);
}

#[test]
fn apr_for_year_for_max() {
	let (_, ref mut interest_rate_model) = new_jump_model();
	let utilization = Percent::from_percent(80);
	let borrow_index = Rate::saturating_from_integer(1_u128);
	let total_borrows = u128::MAX;
	let result = accrue_interest_internal::<Runtime, InterestRateModel>(
		utilization,
		interest_rate_model,
		borrow_index,
		SECONDS_PER_YEAR_NAIVE,
		total_borrows,
	);
	assert_err!(result, ArithmeticError::Overflow);
}

#[test]
fn accrue_interest_induction() {
	let borrow_index = Rate::saturating_from_integer(1_u128);
	let minimal: u128 = 100;
	let mut runner = TestRunner::default();
	let accrued_debt: u128 = 0;
	runner
		.run(
			&(
				0..=2 * SECONDS_PER_YEAR_NAIVE / MILLISECS_PER_BLOCK,
				(minimal..=minimal * 1_000_000_000),
			),
			|(slot, total_issued)| {
				let (optimal, ref mut interest_rate_model) = new_jump_model();

				let AccruedInterest {
					accrued_increment: accrued_increase_1,
					new_borrow_index: borrow_index_1,
				} = accrue_interest_internal::<Runtime, InterestRateModel>(
					optimal,
					interest_rate_model,
					borrow_index,
					slot * MILLISECS_PER_BLOCK,
					total_issued - accrued_debt,
				)
				.unwrap();

				let AccruedInterest {
					accrued_increment: accrued_increase_2,
					new_borrow_index: borrow_index_2,
				} = accrue_interest_internal::<Runtime, InterestRateModel>(
					optimal,
					interest_rate_model,
					borrow_index,
					(slot + 1) * MILLISECS_PER_BLOCK,
					total_issued - accrued_debt,
				)
				.unwrap();
				prop_assert!(accrued_increase_1 < accrued_increase_2);
				prop_assert!(borrow_index_1 < borrow_index_2);
				Ok(())
			},
		)
		.unwrap();
}

#[test]
fn accrue_interest_plotter() {
	let (optimal, ref mut interest_rate_model) = new_jump_model();
	let borrow_index = MoreThanOneFixedU128::checked_from_integer(1).unwrap();
	let total_issued = 10_000_000_000;
	let accrued_debt = 0;
	let total_borrows = total_issued - accrued_debt;
	// no sure how handle in rust previous + next (so map has access to previous result)
	let mut previous = 0;
	const TOTAL_BLOCKS: u64 = 1000;
	let _data: Vec<_> = (0..TOTAL_BLOCKS)
		.map(|x| {
			let AccruedInterest { accrued_increment, .. } =
				accrue_interest_internal::<Runtime, InterestRateModel>(
					optimal,
					interest_rate_model,
					borrow_index,
					MILLISECS_PER_BLOCK,
					total_borrows,
				)
				.unwrap();
			previous += accrued_increment;
			(x, previous)
		})
		.collect();

	let AccruedInterest { accrued_increment: total_accrued, .. } =
		accrue_interest_internal::<Runtime, InterestRateModel>(
			optimal,
			interest_rate_model,
			Rate::checked_from_integer(1).unwrap(),
			TOTAL_BLOCKS * MILLISECS_PER_BLOCK,
			total_borrows,
		)
		.unwrap();
	assert_eq_error_rate!(previous, total_accrued, 1_000);

	#[cfg(feature = "visualization")]
	{
		use plotters::prelude::*;
		let area =
			BitMapBackend::new("./accrue_interest_plotter.png", (1024, 768)).into_drawing_area();
		area.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&area)
			.set_label_area_size(LabelAreaPosition::Left, 80)
			.set_label_area_size(LabelAreaPosition::Bottom, 80)
			.build_cartesian_2d(
				0.0..1100.0,
				total_issued as f64..(total_issued as f64 + 1.1 * total_accrued as f64),
			)
			.unwrap();

		chart.configure_mesh().draw().unwrap();
		chart
			.draw_series(LineSeries::new(
				_data.iter().map(|(x, y)| (*x as f64, total_issued as f64 + *y as f64)),
				&RED,
			))
			.unwrap();
	}
}

// This is only the test where MarketUpdated event is used.
#[test]
fn can_update_market() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		// Create a market
		let ((market_id, _), _) = create_simple_vaulted_market(BTC::instance(), manager);
		// Get the market from the storage via market id
		let market = crate::Markets::<Runtime>::get(market_id).unwrap();

		let update_input = UpdateInput {
			collateral_factor: market.collateral_factor,
			under_collateralized_warn_percent: market.under_collateralized_warn_percent,
			liquidators: market.liquidators.clone(),
			max_price_age: market.max_price_age,
			interest_rate_model: InterestRateModel::Curve(
				CurveModel::new(CurveModel::MAX_BASE_RATE).unwrap(),
			),
		};
		let input = update_input.clone().try_into_validated().unwrap();
		let updated = Lending::update_market(origin, market_id, input);
		// check if the market was successfully updated
		assert_ok!(updated);
		let market_updated_event: crate::Event<Runtime> =
			crate::Event::MarketUpdated { market_id, input: update_input };
		// check if the event was emitted
		System::assert_has_event(Event::Lending(market_updated_event));

		// validation on input fails as it has collateral_factor less than one
		let update_input = UpdateInput {
			collateral_factor: FixedU128::from_float(0.5),
			under_collateralized_warn_percent: market.under_collateralized_warn_percent,
			liquidators: market.liquidators,
			max_price_age: market.max_price_age,
			interest_rate_model: InterestRateModel::Curve(
				CurveModel::new(CurveModel::MAX_BASE_RATE).unwrap(),
			),
		};
		assert_err!(
			update_input.try_into_validated::<UpdateInputValid>(),
			"collateral factor must be >= 1"
		);
	})
}

#[test]
/// Tests market creation and the associated event(s).
fn can_create_valid_market() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1); // ensure block is non-zero

		/// The amount of the borrow asset to mint into ALICE.
		const INITIAL_BORROW_ASSET_AMOUNT: u128 = 10_u128.pow(30);

		const BORROW_ASSET_ID: u128 = BTC::ID;
		const COLLATERAL_ASSET_ID: u128 = USDT::ID;
		const EXPECTED_AMOUNT_OF_BORROW_ASSET: u128 = 50_000 * USDT::ONE;

		let config = default_create_input(CurrencyPair::new(COLLATERAL_ASSET_ID, BORROW_ASSET_ID));

		set_price(BORROW_ASSET_ID, EXPECTED_AMOUNT_OF_BORROW_ASSET);
		set_price(COLLATERAL_ASSET_ID, USDT::ONE);

		let price = <Oracle as oracle::Oracle>::get_price(BORROW_ASSET_ID, BTC::ONE)
			.expect("impossible")
			.price;

		assert_eq!(price, EXPECTED_AMOUNT_OF_BORROW_ASSET);

		let should_have_failed = Lending::create_market(
			Origin::signed(*ALICE),
			config.clone().try_into_validated().unwrap(),
		);

		assert!(
			matches!(
				should_have_failed,
				Err(DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
					error: DispatchError::Module(ModuleError {
						index: _, // not important in mock runtime
						error: _, // not important in mock runtime
						message: Some(error)
					}),
				}) if Into::<&'static str>::into(orml_tokens::Error::<Runtime>::BalanceTooLow) == error
			),
			"Creating a market with insufficient funds should fail, with the error message being \"BalanceTooLow\".
			The other fields are also checked to make sure any changes are tested and accounted for, perhaps one of those fields changed?
			Market creation result was {:#?}",
			should_have_failed
		);

		Tokens::mint_into(BORROW_ASSET_ID, &*ALICE, INITIAL_BORROW_ASSET_AMOUNT).unwrap();
        let manager = *ALICE;
		let origin = Origin::signed(manager);
        let input = config.clone().try_into_validated().unwrap();

		let should_be_created = Lending::create_market(origin, input.clone());

		assert!(
			matches!(should_be_created, Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },)),
			"Market creation should have succeeded, since ALICE now has BTC.
			Market creation result was {:#?}",
			should_be_created,
		);

        //  Check if corresponded event was emitted
		let currency_pair = input.currency_pair;
        // Market id and vault id values are defined via previous logic.
        let market_id = pallet_lending::pallet::MarketIndex::new(1);
        let vault_id = 1;
	    let market_created_event = crate::Event::MarketCreated {market_id, vault_id, manager, currency_pair};
        System::assert_has_event(Event::Lending(market_created_event));

		let initial_pool_size = Lending::calculate_initial_pool_size(BORROW_ASSET_ID).unwrap();
		let alice_balance_after_market_creation = Tokens::balance(BORROW_ASSET_ID, &*ALICE);

		assert_eq!(
			alice_balance_after_market_creation,
			INITIAL_BORROW_ASSET_AMOUNT - initial_pool_size,
			"ALICE should have 'paid' the inital_pool_size into the market vault.
			alice_balance_after_market_creation: {alice_balance_after_market_creation}
			initial_pool_size: {initial_pool_size}",
			alice_balance_after_market_creation = alice_balance_after_market_creation,
			initial_pool_size = initial_pool_size,
		);

		let system_events = System::events();

		match &*system_events {
			[_, _, _, EventRecord {
				topics: event_topics,
				phase: Phase::Initialization,
				event:
					Event::Lending(crate::Event::MarketCreated {
						currency_pair:
							CurrencyPair { base: COLLATERAL_ASSET_ID, quote: BORROW_ASSET_ID },
						market_id: created_market_id @ MarketIndex(1),
						vault_id: created_vault_id @ 1,
						manager: event_manager,
					}),
			}] if event_manager == &*ALICE && event_topics.is_empty() => {
				assert_eq!(
					Lending::total_available_to_be_borrowed(&created_market_id).unwrap(),
					initial_pool_size,
					"The market should have {} in it.",
					initial_pool_size,
				);

				assert_eq!(
					<Vault as vault::Vault>::asset_id(&created_vault_id).unwrap(),
					BORROW_ASSET_ID,
					"The created market vault should be backed by the borrow asset"
				);

				// REVIEW: Review this test
				let alice_total_debt_with_interest = Tokens::balance(Lending::get_assets_for_market(&created_market_id).unwrap().debt_asset, &ALICE);
				assert_eq!(
					alice_total_debt_with_interest,
					0,
					"The borrowed balance of ALICE should be 0. Found {:#?}",
					alice_total_debt_with_interest
				);
			},
			_ => panic!(
				"Unexpected value for System::events(); found {:#?}",
				system_events
			),
		}
	});
}

#[test]
fn test_borrow_repay_in_same_block() {
	new_test_ext().execute_with(|| {
		let (market_id, vault) = create_simple_market();
		let initial_total_cash = Lending::total_available_to_be_borrowed(&market_id).unwrap();

		let collateral_amount = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral_amount));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral_amount),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral_amount,
				market_id,
			}),
		);

		let borrow_asset_deposit = USDT::units(1_000_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_asset_deposit));
		let mut total_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_asset_deposit) + initial_total_cash;

		process_and_progress_blocks(1);

		let limit_normalized = Lending::get_borrow_limit(&market_id, &ALICE).unwrap();
		assert_eq!(Lending::total_available_to_be_borrowed(&market_id), Ok(total_cash));
		process_and_progress_blocks(1); // <- ???

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market_id, limit_normalized / 4),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: limit_normalized / 4,
				market_id,
			}),
		);

		total_cash -= limit_normalized / 4;
		let total_borrows = limit_normalized / 4;
		assert_eq!(Lending::total_available_to_be_borrowed(&market_id), Ok(total_cash));
		assert_eq!(
			Lending::total_borrowed_from_market_excluding_interest(&market_id),
			Ok(total_borrows)
		);
		let alice_repay_amount =
			Lending::total_debt_with_interest(&market_id, &ALICE).unwrap().unwrap_or_zero();
		// MINT required BTC so that ALICE and BOB can repay the borrow.
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, alice_repay_amount - (limit_normalized / 4)));
		assert_noop!(
			Lending::repay_borrow(
				Origin::signed(*ALICE),
				market_id,
				*ALICE,
				RepayStrategy::PartialAmount(alice_repay_amount)
			),
			Error::<Runtime>::BorrowAndRepayInSameBlockIsNotSupported,
		);

		assert_no_event(Event::Lending(crate::Event::BorrowRepaid {
			sender: *ALICE,
			market_id,
			beneficiary: *ALICE,
			amount: alice_repay_amount,
		}));
	});
}

#[test]
fn test_calculate_utilization_ratio() {
	// 50% borrow
	assert_eq!(Lending::calculate_utilization_ratio(1, 1).unwrap(), Percent::from_percent(50));
	assert_eq!(Lending::calculate_utilization_ratio(100, 100).unwrap(), Percent::from_percent(50));
	// no borrow
	assert_eq!(Lending::calculate_utilization_ratio(1, 0).unwrap(), Percent::zero());
	// full borrow
	assert_eq!(Lending::calculate_utilization_ratio(0, 1).unwrap(), Percent::from_percent(100));
}

#[test]
fn test_borrow_math() {
	let borrower = BorrowerData::new(
		100_u128,
		0,
		MoreThanOneFixedU128::checked_from_rational(200_u8, 100_u8)
			.unwrap()
			.try_into_validated()
			.unwrap(),
		Percent::from_percent(10),
	);
	let borrow = borrower.get_borrow_limit().unwrap();
	assert_eq!(borrow, LiftedFixedBalance::from(50));
}

#[test]
fn old_price() {
	new_test_ext().execute_with(|| {
		// Create market

		const FIRST_PRICE: u128 = 10;
		const BORROW_AMOUNT: u128 = 30;
		const SECOND_PRICE: u128 = 3;

		let (market, vault) = create_market::<FIRST_PRICE>(
			USDT::instance(),
			BTC::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_integer(DEFAULT_COLLATERAL_FACTOR),
		);

		// Borrow amount
		let borrow_amount = USDT::units(BORROW_AMOUNT);

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, borrow_amount));
		assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault, borrow_amount * 2));

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(FIRST_PRICE));

		let collateral_amount = get_price(USDT::ID, borrow_amount) // get price of USDT
			.mul(BTC::ONE) // multiply to BTC ONE
			.div(get_price(BTC::ID, BTC::ONE)) // divide at one BTC price
			.mul(DEFAULT_COLLATERAL_FACTOR);

		// Mint BTC tokens for ALICE
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral_amount));

		// Deposit BTC on the market
		assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, collateral_amount));

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(SECOND_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// Try to borrow by SECOND_PRICE
		assert_noop!(
			Lending::borrow(Origin::signed(*ALICE), market, borrow_amount),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(FIRST_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// skip blocks
		process_and_progress_blocks(DEFAULT_ACTUAL_BLOCKS_COUNT as usize + 1);

		// Try to borrow by SECOND_PRICE
		assert_noop!(
			Lending::borrow(Origin::signed(*ALICE), market, borrow_amount),
			Error::<Runtime>::PriceTooOld
		);

		// Refresh price
		set_price(BTC::ID, BTC::ONE.mul(FIRST_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// Try to borrow by FIRST_PRICE
		assert_ok!(Lending::borrow(Origin::signed(*ALICE), market, borrow_amount),);

		// skip blocks
		process_and_progress_blocks(DEFAULT_ACTUAL_BLOCKS_COUNT as usize + 1);

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(SECOND_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// Try to borrow by SECOND_PRICE
		assert_noop!(
			Lending::borrow(Origin::signed(*ALICE), market, borrow_amount),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		// skip blocks
		process_and_progress_blocks(DEFAULT_ACTUAL_BLOCKS_COUNT as usize + 1);

		// Try to repay by SECOND_PRICE
		assert_ok!(Lending::repay_borrow(
			Origin::signed(*ALICE),
			market,
			*ALICE,
			RepayStrategy::PartialAmount(borrow_amount)
		),);
	});
}

#[test]
fn borrow_flow() {
	new_test_ext().execute_with(|| {
		let (market, vault) = create_simple_market();
		let initial_total_cash = Lending::total_available_to_be_borrowed(&market).unwrap();

		let borrow_amount = USDT::units(1_000_000);
		let collateral_amount = get_price(USDT::ID, borrow_amount)
			.mul(BTC::ONE)
			.div(get_price(BTC::ID, BTC::ONE));

		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral_amount));
		assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, collateral_amount));
		let event = Event::Lending(crate::Event::CollateralDeposited {
			sender: *ALICE,
			amount: collateral_amount,
			market_id: market,
		});
		System::assert_last_event(event);

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();

		assert_eq!(
			limit_normalized,
			get_price(USDT::ID, borrow_amount) / DEFAULT_COLLATERAL_FACTOR
		);

		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_amount));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_amount));

		process_and_progress_blocks(1);

		let expected_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_amount) + initial_total_cash;
		assert_eq!(Lending::total_available_to_be_borrowed(&market), Ok(expected_cash));

		let alice_borrow = borrow_amount / DEFAULT_COLLATERAL_FACTOR / 10;

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market, alice_borrow),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: alice_borrow,
				market_id: market,
			}),
		);

		assert_eq!(
			Lending::total_available_to_be_borrowed(&market),
			Ok(expected_cash - alice_borrow)
		);
		assert_eq!(
			Lending::total_borrowed_from_market_excluding_interest(&market),
			Ok(alice_borrow)
		);
		assert_eq!(Lending::total_interest(&market), Ok(0));

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let original_limit = limit_normalized * USDT::ONE / get_price(USDT::ID, USDT::ONE);

		assert_eq!(original_limit, borrow_amount / DEFAULT_COLLATERAL_FACTOR - alice_borrow);

		let borrow = Lending::total_debt_with_interest(&market, &ALICE).unwrap().unwrap_or_zero();
		assert_eq!(borrow, alice_borrow);
		let interest_before = Lending::total_interest(&market).unwrap();
		process_and_progress_blocks(49);
		let interest_after = Lending::total_interest(&market).unwrap();
		assert!(interest_before < interest_after);

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let new_limit = limit_normalized * USDT::ONE / get_price(USDT::ID, USDT::ONE);

		assert!(new_limit < original_limit);

		let borrow = Lending::total_debt_with_interest(&market, &ALICE).unwrap().unwrap_or_zero();

		assert!(borrow > alice_borrow);
		assert_noop!(
			Lending::borrow(Origin::signed(*ALICE), market, original_limit),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		assert_no_event(Event::Lending(crate::Event::Borrowed {
			sender: *ALICE,
			amount: original_limit,
			market_id: market,
		}));

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market, new_limit),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: new_limit,
				market_id: market,
			}),
		);

		assert_noop!(
			Lending::borrow(Origin::signed(*ALICE), market, USDT::ONE),
			Error::<Runtime>::InvalidTimestampOnBorrowRequest
		);

		assert_no_event(Event::Lending(crate::Event::Borrowed {
			sender: *ALICE,
			amount: USDT::ONE,
			market_id: market,
		}));

		process_and_progress_blocks(20);

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral_amount));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market, collateral_amount),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral_amount,
				market_id: market,
			}),
		);

		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		assert!(get_price(BTC::ID, collateral_amount) > alice_limit);

		assert_noop!(
			Lending::borrow(Origin::signed(*ALICE), market, alice_limit * 100),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		assert_no_event(Event::Lending(crate::Event::Borrowed {
			sender: *ALICE,
			amount: alice_limit * 100,
			market_id: market,
		}));

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market, 10),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: 10,
				market_id: market,
			}),
		);
	});
}

#[test]
fn vault_takes_part_of_borrow_so_cannot_withdraw() {
	new_test_ext().execute_with(|| {
		let (market_id, vault_id) = create_simple_market();
		let initial_total_cash = Lending::total_available_to_be_borrowed(&market_id).unwrap();
		let deposit_usdt = 1_000_000_000;
		let deposit_btc = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, deposit_usdt));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, deposit_btc));

		assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id, deposit_btc));
		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, deposit_usdt),
			Event::Lending(pallet_lending::Event::<Runtime>::CollateralDeposited {
				sender: *ALICE,
				market_id,
				amount: deposit_usdt,
			}),
		);
		assert_noop!(
			Lending::borrow(
				Origin::signed(*ALICE),
				market_id.clone(),
				deposit_btc + initial_total_cash
			),
			Error::<Runtime>::NotEnoughBorrowAsset
		);
		assert_no_event(Event::Lending(pallet_lending::Event::<Runtime>::Borrowed {
			sender: *ALICE,
			market_id,
			amount: deposit_btc + initial_total_cash,
		}));
	});
}

#[test]
fn test_vault_market_can_withdraw() {
	new_test_ext().execute_with(|| {
		let (market, vault_id) = create_simple_market();

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

		assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id, borrow));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market, collateral),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id: market,
			}),
		);

		process_and_progress_blocks(1);

		// We waited 1 block, the market should have withdraw the funds
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market, borrow - 1),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow - 1, // DEFAULT_MARKET_VAULT_RESERVE
				market_id: market,
			}),
		);
	});
}

#[test]
fn test_liquidate_multiple() {
	new_test_ext().execute_with(|| {
		let (market, _vault) = create_simple_market();

		mint_and_deposit_collateral::<Runtime>(&*ALICE, BTC::units(100), market, BTC::ID);
		mint_and_deposit_collateral::<Runtime>(&*BOB, BTC::units(100), market, BTC::ID);
		mint_and_deposit_collateral::<Runtime>(&*CHARLIE, BTC::units(100), market, BTC::ID);
		match Lending::liquidate(Origin::signed(*ALICE), market, vec![*ALICE, *BOB, *CHARLIE]) {
			Ok(_) => {
				println!("ok!")
			},
			Err(why) => {
				panic!("{:#?}", why)
			},
		};
		let event = Event::Lending(crate::Event::LiquidationInitiated {
			market_id: market,
			borrowers: vec![*ALICE, *BOB, *CHARLIE],
		});
		System::assert_last_event(event);
		Lending::should_liquidate(&market, &*ALICE).unwrap();

		let mut borrowers = vec![];
		let mut bytes = [0; 32];
		for i in 0..=<Runtime as crate::Config>::MaxLiquidationBatchSize::get() {
			let raw_account_id = U256::from(i);
			raw_account_id.to_little_endian(&mut bytes);
			let account_id = AccountId::from_raw(bytes);
			mint_and_deposit_collateral::<Runtime>(&account_id, BTC::units(100), market, BTC::ID);
			borrowers.push(account_id);
		}
		assert_noop!(
			Lending::liquidate(Origin::signed(*ALICE), market, borrowers),
			Error::<Runtime>::MaxLiquidationBatchSizeExceeded,
		);
	})
}

#[test]
fn test_repay_partial_amount() {
	new_test_ext().execute_with(|| {
		type COLLATERAL = BTC;
		type BORROW = USDT;

		// accounts have 1 unit of collateral
		let alice_balance = COLLATERAL::ONE;

		let (market_index, vault_id) = create_simple_market();

		mint_and_deposit_collateral::<Runtime>(
			&*ALICE,
			alice_balance,
			market_index,
			COLLATERAL::ID,
		);

		let borrow_asset_deposit = BORROW::units(1_000_000);
		assert_ok!(Tokens::mint_into(BORROW::ID, &CHARLIE, borrow_asset_deposit));
		assert_extrinsic_event::<Runtime>(
			Vault::deposit(Origin::signed(*CHARLIE), vault_id, borrow_asset_deposit),
			Event::Vault(pallet_vault::Event::<Runtime>::Deposited {
				account: *CHARLIE,
				asset_amount: borrow_asset_deposit,
				lp_amount: borrow_asset_deposit,
			}),
		);

		process_and_progress_blocks(1_000);

		let get_collateral_borrow_limit_for_account = |account| {
			// `limit_normalized` is the limit in USDT
			// `limit` is the limit in BTC
			// BTC is worth 50_000 times more than USDT (see `create_simple_market()`)

			// borrow_limit * COLLATERAL::ONE / price_of(COLLATERAL::ONE)
			// REVIEW: I'm still not sure if this makes sense
			let limit_normalized = Lending::get_borrow_limit(&market_index, &account).unwrap();
			let limit = limit_normalized
				.mul(COLLATERAL::ONE)
				.div(get_price(COLLATERAL::ID, COLLATERAL::ONE));
			limit
		};

		let alice_limit = get_collateral_borrow_limit_for_account(*ALICE);
		assert_extrinsic_event::<Runtime>(
			// partial borrow
			Lending::borrow(Origin::signed(*ALICE), market_index, alice_limit / 2),
			Event::Lending(crate::Event::<Runtime>::Borrowed {
				sender: *ALICE,
				market_id: market_index,
				amount: alice_limit / 2,
			}),
		);

		process_and_progress_blocks(1_000);

		// pay off a small amount
		assert_extrinsic_event::<Runtime>(
			Lending::repay_borrow(
				Origin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(BORROW::units(1) / 10_000),
			),
			Event::Lending(crate::Event::<Runtime>::BorrowRepaid {
				sender: *ALICE,
				market_id: market_index,
				beneficiary: *ALICE,
				amount: BORROW::units(1) / 10_000,
			}),
		);

		// wait a few blocks
		process_and_progress_blocks(3);

		// pay off a small amount
		assert_extrinsic_event::<Runtime>(
			Lending::repay_borrow(
				Origin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(BORROW::units(1) / 10_000),
			),
			Event::Lending(crate::Event::<Runtime>::BorrowRepaid {
				sender: *ALICE,
				market_id: market_index,
				beneficiary: *ALICE,
				amount: BORROW::units(1) / 10_000,
			}),
		);

		// wait a few blocks
		process_and_progress_blocks(10);

		let alice_total_debt_with_interest =
			Lending::total_debt_with_interest(&market_index, &ALICE)
				.unwrap()
				.unwrap_or_zero();

		dbg!(&alice_total_debt_with_interest);

		assert_ok!(Tokens::mint_into(BORROW::ID, &ALICE, alice_total_debt_with_interest));

		// can't repay more than is owed
		assert_err!(
			Lending::repay_borrow(
				Origin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(alice_total_debt_with_interest + 1)
			),
			DispatchErrorWithPostInfo {
				post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
				error: DispatchError::Module(ModuleError {
					index: 8,
					error: 34,
					message: Some(Error::<Runtime>::CannotRepayMoreThanTotalDebt.into(),),
				}),
			},
		);

		assert_no_event(Event::Lending(crate::Event::BorrowRepaid {
			sender: *ALICE,
			market_id: market_index,
			beneficiary: *ALICE,
			amount: alice_total_debt_with_interest + 1,
		}));

		assert_extrinsic_event::<Runtime>(
			Lending::repay_borrow(
				Origin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(alice_total_debt_with_interest),
			),
			Event::Lending(crate::Event::<Runtime>::BorrowRepaid {
				sender: *ALICE,
				market_id: market_index,
				beneficiary: *ALICE,
				amount: alice_total_debt_with_interest,
			}),
		);

		assert_eq!(Lending::collateral_of_account(&market_index, &*ALICE), Ok(alice_balance));
	});
}

#[test]
fn test_repay_total_debt() {
	new_test_ext().execute_with(|| {
		// accounts have 1 BTC of collateral
		let alice_original_btc_balance = BTC::ONE;
		let bob_original_btc_balance = BTC::ONE;

		let (market_index, vault_id) = create_simple_market();

		let deposit_collateral = |account, balance| {
			assert_ok!(Tokens::mint_into(BTC::ID, account, balance));
			assert_extrinsic_event::<Runtime>(
				Lending::deposit_collateral(Origin::signed(*account), market_index, balance),
				Event::Lending(crate::Event::<Runtime>::CollateralDeposited {
					market_id: market_index,
					amount: BTC::ONE,
					sender: *account,
				}),
			);
		};

		deposit_collateral(&*ALICE, alice_original_btc_balance);
		deposit_collateral(&*BOB, bob_original_btc_balance);

		// CHARLIE is the lender
		let borrow_asset_deposit = USDT::units(1_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault_id, borrow_asset_deposit));

		// processes one block
		process_and_progress_blocks(1);

		let get_btc_borrow_limit_for_account = |account| {
			// `limit_normalized` is the limit in USDT
			// `limit` is the limit in BTC
			// BTC is worth 50_000 times more than USDT (see `create_simple_market()`)

			// REVIEW: I'm still not sure if this makes sense
			let limit_normalized = Lending::get_borrow_limit(&market_index, &account).unwrap();
			let limit = limit_normalized.mul(BTC::ONE).div(get_price(BTC::ID, BTC::ONE));
			limit
		};

		let alice_borrow_limit = get_btc_borrow_limit_for_account(*ALICE);
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market_index, alice_borrow_limit),
			Event::Lending(crate::Event::<Runtime>::Borrowed {
				sender: *ALICE,
				market_id: market_index,
				amount: alice_borrow_limit,
			}),
		);

		process_and_progress_blocks(1000);

		let bob_limit_after_blocks = get_btc_borrow_limit_for_account(*BOB);
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*BOB), market_index, bob_limit_after_blocks),
			Event::Lending(crate::Event::<Runtime>::Borrowed {
				sender: *BOB,
				market_id: market_index,
				amount: bob_limit_after_blocks,
			}),
		);

		process_and_progress_blocks(100);

		let alice_total_debt_with_interest =
			Lending::total_debt_with_interest(&market_index, &ALICE)
				.unwrap()
				.unwrap_amount();
		let bob_total_debt_with_interest =
			Lending::total_debt_with_interest(&market_index, &BOB).unwrap().unwrap_amount();

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, alice_total_debt_with_interest));
		assert_ok!(Tokens::mint_into(USDT::ID, &BOB, bob_total_debt_with_interest));

		// repay ALICE and check state
		{
			assert_extrinsic_event::<Runtime>(
				Lending::repay_borrow(
					Origin::signed(*ALICE),
					market_index,
					*ALICE,
					RepayStrategy::TotalDebt,
				),
				Event::Lending(crate::Event::<Runtime>::BorrowRepaid {
					sender: *ALICE,
					market_id: market_index,
					beneficiary: *ALICE,
					amount: alice_total_debt_with_interest,
				}),
			);

			assert_eq!(
				Lending::total_debt_with_interest(&market_index, &ALICE).unwrap(),
				TotalDebtWithInterest::NoDebt
			);
		}

		// repay BOB and check state
		{
			assert_extrinsic_event::<Runtime>(
				Lending::repay_borrow(
					Origin::signed(*BOB),
					market_index,
					*BOB,
					RepayStrategy::TotalDebt,
				),
				Event::Lending(crate::Event::<Runtime>::BorrowRepaid {
					sender: *BOB,
					market_id: market_index,
					beneficiary: *BOB,
					amount: bob_total_debt_with_interest,
				}),
			);

			assert_eq!(
				Lending::total_debt_with_interest(&market_index, &BOB).unwrap(),
				TotalDebtWithInterest::NoDebt
			);
		}
	});
}

#[test]
fn liquidation() {
	new_test_ext().execute_with(|| {
		let (market_id, vault) = create_market::<50_000>(
			USDT::instance(),
			BTC::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_rational(2, 1),
		);

		let collateral = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id,
			}),
		);

		let usdt_amt = 2 * DEFAULT_COLLATERAL_FACTOR * USDT::ONE * get_price(BTC::ID, collateral) /
			get_price(NORMALIZED::ID, NORMALIZED::ONE);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, usdt_amt));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		process_and_progress_blocks(1);

		let borrow_limit = Lending::get_borrow_limit(&market_id, &ALICE).expect("impossible");
		assert!(borrow_limit > 0);

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market_id, borrow_limit),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow_limit,
				market_id,
			}),
		);

		process_and_progress_blocks(10_000);

		assert_extrinsic_event::<Runtime>(
			Lending::liquidate(Origin::signed(*ALICE), market_id.clone(), vec![*ALICE]),
			Event::Lending(crate::Event::LiquidationInitiated {
				market_id,
				borrowers: vec![*ALICE],
			}),
		);
	});
}

#[test]
fn test_warn_soon_under_collateralized() {
	new_test_ext().execute_with(|| {
		const NORMALIZED_UNITS: u128 = 50_000;
		let (market, vault) = create_market::<NORMALIZED_UNITS>(
			USDT::instance(),
			BTC::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_rational(2, 1),
		);

		// dbg!(&Vault::vault_info(vault));
		let two_btc_amount = BTC::units(2);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, two_btc_amount));
		assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, two_btc_amount));
		let event = Event::Lending(crate::Event::CollateralDeposited {
			sender: *ALICE,
			amount: two_btc_amount,
			market_id: market,
		});
		System::assert_last_event(event);

		let usdt_amt = USDT::units(100_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, usdt_amt));

		process_and_progress_blocks(1);

		assert_eq!(Lending::get_borrow_limit(&market, &ALICE), Ok(50_000_000_000_000_000));

		let borrow_amount = USDT::units(80);

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market, borrow_amount),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow_amount,
				market_id: market,
			}),
		);

		process_and_progress_blocks(10000);

		assert_eq!(Lending::soon_under_collateralized(&market, &ALICE), Ok(false));
		set_price(BTC::ID, NORMALIZED::units(85));
		assert_eq!(Lending::soon_under_collateralized(&market, &ALICE), Ok(true));
		assert_eq!(Lending::should_liquidate(&market, &ALICE), Ok(false));
	});
}

#[test]
fn current_interest_rate_test() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		// Create a market
		let ((market_id, _), _) = create_simple_vaulted_market(BTC::instance(), manager);

		assert_eq!(
			crate::current_interest_rate::<Runtime>(market_id.0).unwrap(),
			FixedU128::saturating_from_rational(2_u128, 100_u128)
		);

		// Update the market
		let market = crate::Markets::<Runtime>::get(market_id).unwrap();
		let update_input = UpdateInput {
			collateral_factor: market.collateral_factor,
			max_price_age: market.max_price_age,
			under_collateralized_warn_percent: market.under_collateralized_warn_percent,
			liquidators: market.liquidators,
			interest_rate_model: InterestRateModel::Curve(
				CurveModel::new(CurveModel::MAX_BASE_RATE).unwrap(),
			),
		};
		let update_input = update_input.try_into_validated().unwrap();
		assert_ok!(Lending::update_market(Origin::signed(manager), market_id, update_input));

		assert_eq!(
			crate::current_interest_rate::<Runtime>(market_id.0).unwrap(),
			FixedU128::saturating_from_rational(1, 10)
		);
	})
}

#[test]
fn zero_amount_collateral_deposit() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let (market_id, _vault_id) = create_simple_market();
		let expected = 50_000;
		set_price(BTC::ID, expected);
		set_price(USDT::ID, 1);
		let collateral_amount = 0;
		assert_noop!(
			<Lending as LendingTrait>::deposit_collateral(&market_id, &BOB, collateral_amount),
			Error::<Runtime>::CannotDepositZeroCollateral
		);
	})
}

prop_compose! {
	fn valid_amount_without_overflow()
		(x in MINIMUM_BALANCE..u64::MAX as Balance) -> Balance {
		x
	}
}

prop_compose! {
	fn valid_cash_borrow()(cash in 1..u32::MAX)(borrow in 0..cash, cash in Just(cash))
		-> (u32, u32) {
			(cash, borrow)
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_2()
		(x in MINIMUM_BALANCE..u64::MAX as Balance / 2,
		y in MINIMUM_BALANCE..u64::MAX as Balance / 2) -> (Balance, Balance) {
			(x, y)
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_3()
		(x in MINIMUM_BALANCE..u64::MAX as Balance / 3,
		y in MINIMUM_BALANCE..u64::MAX as Balance / 3,
		z in MINIMUM_BALANCE..u64::MAX as Balance / 3) -> (Balance, Balance, Balance) {
			(x, y, z)
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_k
		(max_accounts: usize, limit: Balance)
		(balances in prop::collection::vec(MINIMUM_BALANCE..limit, 3..max_accounts))
		-> Vec<(AccountId, Balance)> {
			let mut result = Vec::with_capacity(balances.len());
			let mut account = U256::from_little_endian(UNRESERVED.as_ref());
			let mut buffer = [0; 32];
			for balance in balances {
				account += U256::one();
				account.to_little_endian(&mut buffer);
				result.push((AccountId::from_raw(buffer), balance))
			};
			result
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_k_with_random_index(max_accounts: usize, limit: Balance)
		(accounts in valid_amounts_without_overflow_k(max_accounts, limit),
		index in 1..max_accounts) -> (usize, Vec<(AccountId, Balance)>) {
			(usize::max(1, index % usize::max(1, accounts.len())), accounts)
		}
}

prop_compose! {
	fn strategy_account()
		(x in u128::from(U256::from_little_endian(UNRESERVED.as_ref()).low_u64())..u128::MAX) -> AccountId {
			let mut account = U256::from_little_endian(UNRESERVED.as_ref());
			account += x.into();
			let mut buffer = [0; 32];
			account.to_little_endian(&mut buffer);
			AccountId::from_raw(buffer)
		}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10_000))]

	#[test]
	fn proptest_math_borrow(
		collateral_balance in 0..u32::MAX as Balance,
		collateral_price in 0..u32::MAX as Balance,
		borrower_balance_with_interest in 0..u32::MAX as Balance,
		borrow_price in 0..u32::MAX as Balance
	) {
		let borrower = BorrowerData::new(
			collateral_balance * collateral_price,
			borrower_balance_with_interest * borrow_price,
			MoreThanOneFixedU128::checked_from_rational(101_u8, 100_u8)
				.unwrap()
				.try_into_validated()
				.unwrap(),
			Percent::from_percent(10),
		);
		let borrow = borrower.get_borrow_limit();
		prop_assert_ok!(borrow);
	}

	#[test]
	fn market_collateral_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _) = create_simple_market();
			let before = Tokens::balance( BTC::ID, &ALICE);
			prop_assert_ok!(Tokens::mint_into( BTC::ID, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, amount));
			let event =
				Event::Lending(crate::Event::CollateralDeposited {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);

			prop_assert_ok!(Lending::withdraw_collateral(Origin::signed(*ALICE), market, amount));
			let event =
				Event::Lending(crate::Event::CollateralWithdrawn {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);
			prop_assert_eq!(Tokens::balance( BTC::ID, &ALICE) - before, amount);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_deposit_withdraw_higher_amount_fails(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _vault) = create_simple_market();
			prop_assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, amount));
			let event =
				Event::Lending(crate::Event::CollateralDeposited {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);

			prop_assert_eq!(
				Lending::withdraw_collateral(Origin::signed(*ALICE), market, amount + 1),
				Err(Error::<Runtime>::NotEnoughCollateralToWithdraw.into())
			);
			let event =
				Event::Lending(crate::Event::CollateralWithdrawn {
					sender: *ALICE,
					amount: amount + 1,
					market_id: market,
				});
			assert_no_event(event);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_vaulted_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let ((market, _), collateral_asset) = create_simple_vaulted_market(BTC::instance(), *ALICE);
			let before = Tokens::balance(collateral_asset, &ALICE);
			prop_assert_ok!(Tokens::mint_into(collateral_asset, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, amount));
			let event =
				Event::Lending(crate::Event::CollateralDeposited {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);
			prop_assert_ok!(Lending::withdraw_collateral(Origin::signed(*ALICE), market, amount));
			let event =
				Event::Lending(crate::Event::CollateralWithdrawn {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE) - before, amount);

			Ok(())
		})?;
	}

	#[test]
	fn calc_utilization_ratio_proptest((cash, borrow) in valid_cash_borrow()) {
		new_test_ext().execute_with(|| {
			prop_assert_eq!(
				Lending::calculate_utilization_ratio(cash.into(), borrow.into()).unwrap(),
				Percent::from_float(borrow as f64 / (cash as f64 + borrow as f64))
			);
			Ok(())
		})?;
	}

	#[test]
	fn market_are_isolated(
		(amount1, amount2) in valid_amounts_without_overflow_2()
	) {
		new_test_ext().execute_with(|| {
			let (market_id1, vault_id1) = create_simple_market();
			let m1 = Tokens::balance(USDT::ID, &Lending::account_id(&market_id1));
			let (market_id2, vault_id2) = create_simple_market();
			let m2 = Tokens::balance(USDT::ID, &Lending::account_id(&market_id2));

			prop_assert_ne!(market_id1, market_id2);
			prop_assert_ne!(Lending::account_id(&market_id1), Lending::account_id(&market_id2));

			prop_assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, amount1));
			prop_assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id1, amount1));

			prop_assert_ok!(Tokens::mint_into(USDT::ID, &BOB, 10*amount2));
			prop_assert_ok!(Vault::deposit(Origin::signed(*BOB), vault_id2, 10*amount2));

		process_and_progress_blocks(1);

			let expected_market1_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(amount1);
			let expected_market2_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(10*amount2);

			prop_assert_acceptable_computation_error!(
				Tokens::balance(USDT::ID, &Lending::account_id(&market_id1)) - m1,
				expected_market1_balance
			);
			prop_assert_acceptable_computation_error!(
				Tokens::balance(USDT::ID, &Lending::account_id(&market_id2)) - m2,
				expected_market2_balance
			);

			Ok(())
		})?;
	}
}

// HELPERS

/// Creates a "default" [`CreateInput`], with the specified [`CurrencyPair`].
fn default_create_input<AssetId, BlockNumber: sp_runtime::traits::Bounded>(
	currency_pair: CurrencyPair<AssetId>,
) -> CreateInput<u32, AssetId, BlockNumber> {
	CreateInput {
		updatable: UpdateInput {
			collateral_factor: default_collateral_factor(),
			under_collateralized_warn_percent: default_under_collateralized_warn_percent(),
			liquidators: vec![],
			interest_rate_model: InterestRateModel::default(),
			max_price_age: BlockNumber::max_value(),
		},
		reserved_factor: DEFAULT_MARKET_VAULT_RESERVE,
		currency_pair,
	}
}

/// Returns a "default" value (`10%`) for the under collateralized warn percentage.
fn default_under_collateralized_warn_percent() -> Percent {
	Percent::from_float(0.10)
}

/// Creates a "default" [`MoreThanOneFixedU128`], equal to [`DEFAULT_COLLATERAL_FACTOR`].
fn default_collateral_factor() -> sp_runtime::FixedU128 {
	MoreThanOneFixedU128::saturating_from_integer(DEFAULT_COLLATERAL_FACTOR)
}

/// Helper to get the price of an asset from the Oracle, in USDT cents.
fn get_price(asset_id: CurrencyId, amount: Balance) -> Balance {
	<Oracle as oracle::Oracle>::get_price(asset_id, amount).unwrap().price
}

/// Creates a very simple vault for the given currency. 100% is reserved.
///
/// Values used:
///
/// - `reserved`: `Perquintill::from_percent(100)`
/// - `strategies`: Empty [`BTreeMap`][std::collection::BTreeMap]
///
/// # Panics
///
/// Panics on any errors. Only for use in testing.
fn create_simple_vault(
	asset: RuntimeCurrency,
	manager: AccountId,
) -> (VaultId, VaultInfo<AccountId, Balance, CurrencyId, BlockNumber>) {
	let config = VaultConfig {
		asset_id: asset.id(),
		manager,
		reserved: Perquintill::from_percent(100),
		strategies: Default::default(),
	};

	Vault::do_create_vault(Deposit::Existential, config.try_into_validated().unwrap()).unwrap()
}

/// Creates a market with the given values and initializes some state.
///
/// State initialized:
///
/// - Price of the `borrow_asset` is set to `NORMALIZED::ONE`
/// - Price of the `collateral_asset` is set to `NORMALIZED::units(NORMALIZED_PRICE)`
/// - `1000` units of `borrow_asset` are minted into the `manager`
/// - `100` units of `collateral_asset` are minted into the `manager`
///
/// Values used:
///
/// - `interest_rate_model`: [`Default`] implementation of [`InterestRateModel`]
/// - `liquidators`: empty [`Vec`]
/// - `under_collateralized_warn_percent`: [`default_under_collateralized_warn_percent()`]
///
/// # Panics
///
/// Panics on any errors. Only for use in testing.
fn create_market<const NORMALIZED_PRICE: u128>(
	borrow_asset: RuntimeCurrency,
	collateral_asset: RuntimeCurrency,
	manager: AccountId,
	reserved_factor: Perquintill,
	collateral_factor: MoreThanOneFixedU128,
) -> (MarketIndex, VaultId) {
	set_price(borrow_asset.id(), NORMALIZED::ONE);
	set_price(collateral_asset.id(), NORMALIZED::units(NORMALIZED_PRICE));

	Tokens::mint_into(borrow_asset.id(), &manager, borrow_asset.units(1000)).unwrap();
	Tokens::mint_into(collateral_asset.id(), &manager, collateral_asset.units(100)).unwrap();

	let config = CreateInput {
		updatable: UpdateInput {
			collateral_factor,
			under_collateralized_warn_percent: default_under_collateralized_warn_percent(),
			liquidators: vec![],
			interest_rate_model: InterestRateModel::default(),
			max_price_age: DEFAULT_ACTUAL_BLOCKS_COUNT,
		},
		reserved_factor,
		currency_pair: CurrencyPair::new(collateral_asset.id(), borrow_asset.id()),
	};

	Lending::create_market(Origin::signed(manager), config.try_into_validated().unwrap()).unwrap();

	let system_events = System::events();
	if let Some(EventRecord {
		event:
			Event::Lending(crate::Event::<Runtime>::MarketCreated {
				market_id,
				vault_id,
				manager: _,
				currency_pair: _,
			}),
		..
	}) = system_events.last()
	{
		(*market_id, *vault_id)
	} else {
		panic!(
			"System::events() did not contain the market creation event. Found {:#?}",
			system_events
		)
	}
}

/// some model with sane parameter
fn new_jump_model() -> (Percent, InterestRateModel) {
	let base_rate = Rate::saturating_from_rational(2, 100);
	let jump_rate = Rate::saturating_from_rational(10, 100);
	let full_rate = Rate::saturating_from_rational(32, 100);
	let optimal = Percent::from_percent(80);
	let interest_rate_model =
		InterestRateModel::Jump(JumpModel::new(base_rate, jump_rate, full_rate, optimal).unwrap());
	(optimal, interest_rate_model)
}

/// Create a market with a USDT vault LP token as collateral.
fn create_simple_vaulted_market(
	borrow_asset: RuntimeCurrency,
	manager: AccountId,
) -> ((MarketIndex, VaultId), CurrencyId) {
	let (_, VaultInfo { lp_token_id, .. }) = create_simple_vault(borrow_asset, manager);

	let market = create_market::<50_000>(
		borrow_asset,
		RuntimeCurrency::new(lp_token_id, 12),
		manager,
		DEFAULT_MARKET_VAULT_RESERVE,
		MoreThanOneFixedU128::saturating_from_integer(2),
	);

	(market, lp_token_id)
}

/// Create a simple  market with USDT as borrow and BTC as collateral.
///
/// `NORMALIZED_PRICE` is set to `50_000`.
///
/// See [`create_market()`] for more information.
fn create_simple_market() -> (MarketIndex, VaultId) {
	create_market::<50_000>(
		USDT::instance(),
		BTC::instance(),
		*ALICE,
		DEFAULT_MARKET_VAULT_RESERVE,
		MoreThanOneFixedU128::saturating_from_integer(DEFAULT_COLLATERAL_FACTOR),
	)
}

/// Mints `amount` of `collateral` into `account`, and then deposits that same `amount` into
/// `market_index`.
///
/// Panics on any errors and checks that the last event was `CollateralDeposited` with the correct/
/// expected values.
fn mint_and_deposit_collateral<T: crate::Config>(
	account: &sp_core::sr25519::Public,
	balance: u128,
	market_index: MarketIndex,
	asset_id: u128,
) {
	assert_ok!(Tokens::mint_into(asset_id, account, balance));
	assert_ok!(Lending::deposit_collateral(Origin::signed(*account), market_index, balance));
	assert_last_event::<Runtime>(Event::Lending(crate::Event::<Runtime>::CollateralDeposited {
		market_id: market_index,
		amount: balance,
		sender: *account,
	}))
}

/// Asserts that the outcome of an extrinsic is `Ok`, and that the last event is the specified
/// event.
fn assert_extrinsic_event<T: crate::Config>(
	result: DispatchResultWithPostInfo,
	event: <T as crate::Config>::Event,
) {
	assert_ok!(result);
	assert_last_event::<T>(event);
}

/// Asserts the event wasn't dispatched.
fn assert_no_event(event: crate::mocks::Event) {
	assert!(System::events().iter().all(|record| record.event != event));
}
