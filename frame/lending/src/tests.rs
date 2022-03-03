#![allow(unused_imports)]

//! Test for Lending. Runtime is almost real.
//! TODO: cover testing events - so make sure that each even is handled at least once
//! (events can be obtained from System pallet as in banchmarking.rs before this commit)
//! TODO: OCW of liquidations (like in Oracle)
//! TODO: test on small numbers via proptests - detect edge case what is minimal amounts it starts
//! to accure(and miminal block delta), and maximal amounts when it overflows

use std::ops::{Div, Mul};

use crate::{
	self as pallet_lending, accrue_interest_internal, currency::*, mocks::*, models::BorrowerData,
	Error, MarketIndex,
};
use codec::{Decode, Encode};
use composable_support::validation::{TryIntoValidated, Validated};
use composable_tests_helpers::{prop_assert_acceptable_computation_error, prop_assert_ok};
use composable_traits::{
	defi::{CurrencyPair, LiftedFixedBalance, MoreThanOneFixedU128, Rate, ZeroToOneFixedU128},
	lending::{self, math::*, CreateInput, UpdateInput},
	oracle,
	time::SECONDS_PER_YEAR_NAIVE,
	vault::{self, Deposit, VaultConfig},
};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo},
	traits::fungibles::{Inspect, Mutate},
	weights::Pays,
};
use frame_system::{EventRecord, Phase};
use pallet_vault::models::VaultInfo;
use proptest::{prelude::*, test_runner::TestRunner};
use sp_arithmetic::assert_eq_error_rate;
use sp_core::{H256, U256};
use sp_runtime::{ArithmeticError, DispatchError, FixedPointNumber, Percent, Perquintill};

const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);
const DEFAULT_MARKET_VAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);
const DEFAULT_COLLATERAL_FACTOR: u128 = 2;

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
	let (accrued_increase, _) = accrue_interest_internal::<Runtime, InterestRateModel>(
		optimal,
		interest_rate_model,
		borrow_index,
		delta_time,
		total_borrows,
	)
	.unwrap();
	assert_eq!(accrued_increase, 10_000_000_000_000_000_000);

	let delta_time = MILLISECS_PER_BLOCK;
	let (accrued_increase, _) = accrue_interest_internal::<Runtime, InterestRateModel>(
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
		10_000_000_000_000_000_000 * MILLISECS_PER_BLOCK as u128 / SECONDS_PER_YEAR_NAIVE as u128
			+ error
	);
}

#[test]
fn apr_for_zero() {
	let (_, ref mut interest_rate_model) = new_jump_model();
	let utilization = Percent::from_percent(100);
	let borrow_index = Rate::saturating_from_integer(1_u128);

	let (accrued_increase, _) = accrue_interest_internal::<Runtime, InterestRateModel>(
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
				let (accrued_increase_1, borrow_index_1) =
					accrue_interest_internal::<Runtime, InterestRateModel>(
						optimal,
						interest_rate_model,
						borrow_index,
						slot * MILLISECS_PER_BLOCK,
						total_issued - accrued_debt,
					)
					.unwrap();
				let (accrued_increase_2, borrow_index_2) =
					accrue_interest_internal::<Runtime, InterestRateModel>(
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
			let (accrue_increment, _) = accrue_interest_internal::<Runtime, InterestRateModel>(
				optimal,
				interest_rate_model,
				borrow_index,
				MILLISECS_PER_BLOCK,
				total_borrows,
			)
			.unwrap();
			previous += accrue_increment;
			(x, previous)
		})
		.collect();

	let (total_accrued, _) = accrue_interest_internal::<Runtime, InterestRateModel>(
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

		// REVIEW: Does it matter what error.index and error.error are
		// when using a mock runtime?
		assert!(
			matches!(
				should_have_failed,
				Err(DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
					error: DispatchError::Module {
						index: 5,
						error: 0,
						message: Some("BalanceTooLow")
					},
				})
			),
			"Creating a market with insufficient funds should fail, with the error message being \"BalanceTooLow\".
			The other fields are also checked to make sure any changes are tested and accounted, perhaps one of those fields changed?
			Market creation result was {should_have_failed:#?}",
		);

		Tokens::mint_into(BORROW_ASSET_ID, &*ALICE, INITIAL_BORROW_ASSET_AMOUNT).unwrap();

		let should_be_created =
			Lending::create_market(Origin::signed(*ALICE), config.clone().try_into_validated().unwrap());

		assert!(
			matches!(should_be_created, Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },)),
			"Market creation should have succeeded, since ALICE now has BTC.
			Market creation result was {should_be_created:#?}",
		);

		let initial_pool_size = Lending::initial_pool_size(BORROW_ASSET_ID).unwrap();
		let alice_balance_after_market_creation = Tokens::balance(BORROW_ASSET_ID, &*ALICE);

		assert_eq!(
			alice_balance_after_market_creation,
			INITIAL_BORROW_ASSET_AMOUNT - initial_pool_size,
			"ALICE should have 'paid' the inital_pool_size into the market vault.
			alice_balance_after_market_creation: {alice_balance_after_market_creation}
			initial_pool_size: {initial_pool_size}",
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
					Lending::total_cash(&created_market_id).unwrap(),
					initial_pool_size,
					"The market should have {initial_pool_size} in it."
				);

				assert_eq!(
					<Vault as vault::Vault>::asset_id(&created_vault_id).unwrap(),
					BORROW_ASSET_ID,
					"The created market vault should be backed by the borrow asset"
				);

				assert_eq!(
					Lending::borrow_balance_current(&created_market_id, &ALICE),
					Ok(Some(0)),
					"The borrowed balance of ALICE should be 0."
				);
			},
			_ => panic!("Unexpected value for System::events(); found {system_events:#?}"),
		}
	});
}

#[test]
fn test_borrow_repay_in_same_block() {
	new_test_ext().execute_with(|| {
		let (market_id, vault) = create_simple_market();
		let initial_total_cash = Lending::total_cash(&market_id).unwrap();

		let collateral_amount = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral_amount));
		assert_ok!(Lending::deposit_collateral_internal(&market_id, &ALICE, collateral_amount));

		let borrow_asset_deposit = USDT::units(1_000_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_asset_deposit));
		let mut total_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_asset_deposit) + initial_total_cash;

		(1..2).for_each(process_block);

		let limit_normalized = Lending::get_borrow_limit(&market_id, &ALICE).unwrap();
		assert_eq!(Lending::total_cash(&market_id), Ok(total_cash));
		process_block(1);
		assert_ok!(Lending::borrow_internal(&market_id, &ALICE, limit_normalized / 4));
		total_cash -= limit_normalized / 4;
		let total_borrows = limit_normalized / 4;
		assert_eq!(Lending::total_cash(&market_id), Ok(total_cash));
		assert_eq!(Lending::total_borrows(&market_id), Ok(total_borrows));
		let alice_repay_amount = Lending::borrow_balance_current(&market_id, &ALICE).unwrap();
		// MINT required BTC so that ALICE and BOB can repay the borrow.
		assert_ok!(Tokens::mint_into(
			BTC::ID,
			&ALICE,
			alice_repay_amount.unwrap() - (limit_normalized / 4)
		));
		assert_noop!(
			Lending::repay_borrow_internal(&market_id, &ALICE, &ALICE, alice_repay_amount),
			Error::<Runtime>::BorrowAndRepayInSameBlockIsNotSupported
		);
	});
}

#[test]
fn test_calc_utilization_ratio() {
	// 50% borrow
	assert_eq!(Lending::calc_utilization_ratio(1, 1).unwrap(), Percent::from_percent(50));
	assert_eq!(Lending::calc_utilization_ratio(100, 100).unwrap(), Percent::from_percent(50));
	// no borrow
	assert_eq!(Lending::calc_utilization_ratio(1, 0).unwrap(), Percent::zero());
	// full borrow
	assert_eq!(Lending::calc_utilization_ratio(0, 1).unwrap(), Percent::from_percent(100));
}

#[test]
fn test_borrow_math() {
	let borrower = BorrowerData::new(
		100_u128,
		0,
		MoreThanOneFixedU128::from_float(1.0),
		Percent::from_float(0.10),
	);
	let borrow = borrower.borrow_for_collateral().unwrap();
	assert_eq!(borrow, LiftedFixedBalance::from(100));
}

#[test]
fn borrow_flow() {
	new_test_ext().execute_with(|| {
		let (market, vault) = create_simple_market();
		let initial_total_cash = dbg!(Lending::total_cash(&market).unwrap());

		let borrow_amount = USDT::units(1_000_000);
		let collateral_amount =
			dbg!(dbg!(dbg!(get_price(USDT::ID, dbg!(borrow_amount))).mul(dbg!(BTC::ONE)))
				.div(dbg!(get_price(BTC::ID, BTC::ONE))));

		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, dbg!(collateral_amount)));
		assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, collateral_amount));

		let limit_normalized = dbg!(Lending::get_borrow_limit(&market, &ALICE)).unwrap();

		assert_eq!(
			limit_normalized,
			get_price(USDT::ID, borrow_amount) / DEFAULT_COLLATERAL_FACTOR
		);

		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_amount));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_amount));

		(1..2).for_each(process_block);

		let expected_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_amount) + initial_total_cash;
		assert_eq!(Lending::total_cash(&market), Ok(expected_cash));

		let alice_borrow = borrow_amount / DEFAULT_COLLATERAL_FACTOR / 10;
		assert_ok!(Lending::borrow_internal(&market, &ALICE, alice_borrow));
		assert_eq!(Lending::total_cash(&market), Ok(expected_cash - alice_borrow));
		assert_eq!(Lending::total_borrows(&market), Ok(alice_borrow));
		assert_eq!(Lending::total_interest_accurate(&market), Ok(0));

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let original_limit = limit_normalized * USDT::ONE / get_price(USDT::ID, USDT::ONE);

		assert_eq!(original_limit, borrow_amount / DEFAULT_COLLATERAL_FACTOR - alice_borrow);

		let borrow = Lending::borrow_balance_current(&market, &ALICE).unwrap().unwrap();
		assert_eq!(borrow, alice_borrow);
		let interest_before = Lending::total_interest_accurate(&market).unwrap();
		(2..50).for_each(process_block);
		let interest_after = Lending::total_interest_accurate(&market).unwrap();
		assert!(interest_before < interest_after);

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let new_limit = limit_normalized * USDT::ONE / get_price(USDT::ID, USDT::ONE);

		assert!(new_limit < original_limit);

		let borrow = Lending::borrow_balance_current(&market, &ALICE).unwrap().unwrap();

		assert!(borrow > alice_borrow);
		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, original_limit),
			Error::<Runtime>::NotEnoughCollateralToBorrowAmount
		);

		assert_ok!(Lending::borrow_internal(&market, &ALICE, new_limit,));

		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, USDT::ONE),
			Error::<Runtime>::InvalidTimestampOnBorrowRequest
		);

		process_block(10001);

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral_amount));
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, collateral_amount));

		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		assert!(get_price(BTC::ID, collateral_amount) > alice_limit);

		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, alice_limit * 100),
			Error::<Runtime>::NotEnoughCollateralToBorrowAmount
		);

		assert_ok!(Lending::borrow_internal(&market, &ALICE, 10));
	});
}

#[test]
fn vault_takes_part_of_borrow_so_cannot_withdraw() {
	new_test_ext().execute_with(|| {
		let (market_id, vault_id) = create_simple_market();
		let initial_total_cash = Lending::total_cash(&market_id).unwrap();
		let deposit_usdt = 1_000_000_000;
		let deposit_btc = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, deposit_usdt));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, deposit_btc));

		assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id, deposit_btc));
		assert_ok!(Lending::deposit_collateral_internal(&market_id, &ALICE, deposit_usdt));
		assert_noop!(
			Lending::borrow_internal(&market_id, &ALICE, deposit_btc + initial_total_cash),
			Error::<Runtime>::NotEnoughBorrowAsset
		);
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
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, collateral));

		for i in 1..2 {
			process_block(i);
		}

		// We waited 1 block, the market should have withdraw the funds
		assert_ok!(Lending::borrow_internal(
			&market,
			&ALICE,
			borrow - 1 // DEFAULT_MARKET_VAULT_RESERVE
		),);
	});
}

#[test]
fn borrow_repay_repay() {
	new_test_ext().execute_with(|| {
		let alice_balance = BTC::ONE;
		let bob_balance = BTC::ONE;

		let (market, vault) = create_simple_market();

		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, alice_balance));
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, alice_balance));

		assert_ok!(Tokens::mint_into(BTC::ID, &BOB, bob_balance));
		assert_ok!(Lending::deposit_collateral_internal(&market, &BOB, bob_balance));

		let borrow_asset_deposit = USDT::units(1_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_asset_deposit));

		(1..2).for_each(process_block);

		let alice_limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let alice_limit = alice_limit_normalized * BTC::ONE / get_price(BTC::ID, BTC::ONE);
		assert_ok!(Lending::borrow_internal(&market, &ALICE, alice_limit));

		(2..1000).for_each(process_block);

		let bob_limit_normalized = Lending::get_borrow_limit(&market, &BOB).unwrap();
		let bob_limit = bob_limit_normalized * BTC::ONE / get_price(BTC::ID, BTC::ONE);
		assert_ok!(Lending::borrow_internal(&market, &BOB, bob_limit,));

		let _bob_limit = Lending::get_borrow_limit(&market, &BOB).unwrap();

		(1000..1000 + 100).for_each(process_block);

		let alice_repay_amount = Lending::borrow_balance_current(&market, &ALICE).unwrap();
		let bob_repay_amount = Lending::borrow_balance_current(&market, &BOB).unwrap();

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, alice_repay_amount.unwrap()));
		assert_ok!(Tokens::mint_into(USDT::ID, &BOB, bob_repay_amount.unwrap()));

		assert_ok!(Lending::repay_borrow_internal(&market, &BOB, &BOB, bob_repay_amount));
		assert!(bob_balance > Tokens::balance(BTC::ID, &BOB));

		// TODO: fix bug with partial repay:
		//assert_ok!(Lending::repay_borrow_internal(&market, &ALICE, &ALICE, alice_repay_amount));
		// assert!(alice_balance > Tokens::balance(BTC::ID, &ALICE));
	});
}

#[test]
fn liquidation() {
	new_test_ext().execute_with(|| {
		let (market_id, vault) = create_market(
			USDT::instance(),
			BTC::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_rational(2, 1),
		);

		let collateral = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral));
		assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market_id, collateral));

		// TODO: check deposit_collateral event

		let usdt_amt = 2 * DEFAULT_COLLATERAL_FACTOR * USDT::ONE * get_price(BTC::ID, collateral)
			/ get_price(NORMALIZED::ID, NORMALIZED::ONE);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, usdt_amt));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		let borrow_limit = Lending::get_borrow_limit(&market_id, &ALICE).expect("impossible");
		assert!(borrow_limit > 0);

		assert_ok!(Lending::borrow_internal(&market_id, &ALICE, borrow_limit));

		for i in 2..10000 {
			process_block(i);
		}

		assert_ok!(Lending::liquidate_internal(&ALICE, &market_id, vec![*ALICE]));
	});
}

#[test]
fn test_warn_soon_under_collateralized() {
	new_test_ext().execute_with(|| {
		let (market, vault) = create_market(
			BTC::instance(),
			USDT::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_rational(2, 1),
		);

		let two_btc_amount = BTC::units(2);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, two_btc_amount));
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, two_btc_amount));

		let usdt_amt = USDT::units(100_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, usdt_amt));

		(1..2).for_each(process_block);

		assert_eq!(Lending::get_borrow_limit(&market, &ALICE), Ok(50000000000000000));

		let borrow_amount = USDT::units(80);
		assert_ok!(Lending::borrow_internal(&market, &ALICE, borrow_amount));

		(2..10000).for_each(process_block);

		assert_eq!(Lending::soon_under_collateralized(&market, &ALICE), Ok(false));
		set_price(BTC::ID, NORMALIZED::units(85));
		assert_eq!(Lending::soon_under_collateralized(&market, &ALICE), Ok(true));
		assert_eq!(Lending::should_liquidate(&market, &ALICE), Ok(false));
	});
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
	fn proptest_math_borrow(collateral_balance in 0..u32::MAX as Balance,
							collateral_price in 0..u32::MAX as Balance,
							borrower_balance_with_interest in 0..u32::MAX as Balance,
							borrow_price in 0..u32::MAX as Balance
	) {
		let borrower = BorrowerData::new(
			collateral_balance * collateral_price,
			borrower_balance_with_interest * borrow_price,
			MoreThanOneFixedU128::from_float(1.0),
			Percent::from_float(0.10), // 10%
		);
		let borrow = borrower.borrow_for_collateral();
		prop_assert_ok!(borrow);
	}

	#[test]
	fn market_collateral_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _) = create_simple_market();
			let before = Tokens::balance( BTC::ID, &ALICE);
			prop_assert_ok!(Tokens::mint_into( BTC::ID, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount));
			prop_assert_ok!(Lending::withdraw_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance( BTC::ID, &ALICE) - before, amount);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_deposit_withdraw_higher_amount_fails(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _vault) = create_simple_market();
			prop_assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount ));

			prop_assert_eq!(
				Lending::withdraw_collateral_internal(&market, &ALICE, amount  + 1),
				Err(Error::<Runtime>::NotEnoughCollateral.into())
			);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_vaulted_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let ((market, _), collateral_asset) = create_simple_vaulted_market(BTC::instance(), *ALICE);
			let before = Tokens::balance(collateral_asset, &ALICE);
			prop_assert_ok!(Tokens::mint_into(collateral_asset, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount));
			prop_assert_ok!(Lending::withdraw_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE) - before, amount);

			Ok(())
		})?;
	}

	#[test]
	fn calc_utilization_ratio_proptest((cash, borrow) in valid_cash_borrow()) {
		new_test_ext().execute_with(|| {
			prop_assert_eq!(Lending::calc_utilization_ratio(cash.into(), borrow.into()).unwrap(), Percent::from_float(borrow as f64 / (cash as f64 + borrow as f64)));
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

			(1..2).for_each(process_block);

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

// Event tests

#[test]
fn test_market_created_event() {
	new_test_ext().execute_with(|| {
		// progress to an arbitrary block to reset events
		System::set_block_number(1000);

		#[allow(non_camel_case_types)]
		type ASSET_1 = Currency<123_456_789, 12>;
		#[allow(non_camel_case_types)]
		type ASSET_2 = Currency<987_654_321, 12>;

		set_price(ASSET_1::ID, 50_000 * NORMALIZED::ONE);
		set_price(ASSET_2::ID, NORMALIZED::ONE);

		Tokens::mint_into(ASSET_1::ID, &*ALICE, ASSET_1::units(1000)).unwrap();
		Tokens::mint_into(ASSET_2::ID, &*ALICE, ASSET_2::units(100)).unwrap();

		let input = default_create_input(CurrencyPair::new(ASSET_1::ID, ASSET_2::ID));

		Lending::create_market(Origin::signed(*ALICE), input.clone().try_into_validated().unwrap())
			.unwrap();

		assert!(matches!(
			System::events().last(),
			Some(EventRecord {
				topics: event_topics,
				phase: Phase::Initialization,
				event: Event::Lending(crate::Event::MarketCreated {
					currency_pair: CurrencyPair {
						base: ASSET_1::ID, quote: ASSET_2::ID,
					},
					market_id: MarketIndex(1),
					vault_id: 1,
					manager: event_manager,
				}),
			})
			if event_manager == &*ALICE
			&& event_topics.is_empty()
		))
	})
}

// HELPERS

/// Creates a "default" [`CreateInput`], with the specified [`CurrencyPair`].
fn default_create_input<AssetId>(
	currency_pair: CurrencyPair<AssetId>,
) -> CreateInput<u32, AssetId> {
	CreateInput {
		updatable: UpdateInput {
			collateral_factor: default_collateral_factor(),
			under_collateralized_warn_percent: default_under_collateralized_warn_percent(),
			liquidators: vec![],
			interest_rate_model: InterestRateModel::default(),
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

/// Helper to get the price of an asset from the Oracle.
fn get_price(asset_id: CurrencyId, amount: Balance) -> Balance {
	<Oracle as oracle::Oracle>::get_price(asset_id, amount)
		.expect("impossible")
		.price
}
/// Create a very simple vault for the given currency. 100% is reserved.
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
	let v = Vault::do_create_vault(Deposit::Existential, config.try_into_validated().unwrap());
	assert_ok!(&v);
	v.expect("unreachable; qed;")
}

/// Creates a market with the given values.
///
/// Sets the price of the `borrow_asset` to [`NORMALIZED::ONE`], and the `collateral_asset`
/// `50_000 * `[`NORMALIZED::ONE`].
///
/// Mints `1000` units of `borrow_asset` and `100` units of `collateral_asset` into the `manager`.
///
/// The default [`InterestRateModel`] is used.
///
/// # Panics
///
/// Panics on any errors. Only for use in testing.
fn create_market(
	borrow_asset: RuntimeCurrency,
	collateral_asset: RuntimeCurrency,
	manager: AccountId,
	reserved_factor: Perquintill,
	collateral_factor: MoreThanOneFixedU128,
) -> (MarketIndex, VaultId) {
	// fn create_market(
	// 	borrow_asset: CurrencyId,
	// 	collateral_asset: CurrencyId,
	// 	manager: AccountId,
	// 	reserved: Perquintill,
	// 	collateral_factor: MoreThanOneFixedU128,
	// ) -> (MarketIndex, BorrowAssetVault) {
	// 	set_price(USDT::ID, NORMALIZED::one());
	// 	set_price(BTC::ID, 50_000 * NORMALIZED::one());

	// 	dbg!(get_price(USDT::ID, NORMALIZED::one()));
	// 	dbg!(get_price(BTC::ID, NORMALIZED::one()));

	// 	let config = CreateInput {
	// 		updatable: UpdateInput {
	// 			collateral_factor,
	// 			under_collaterized_warn_percent: Percent::from_float(0.10),
	// 			liquidators: vec![],
	// 			interest_rate_model: InterestRateModel::default(),
	// 		},
	// 		reserved_factor: reserved,
	// 		currency_pair: CurrencyPair::new(collateral_asset, borrow_asset),
	// 	};
	// 	Tokens::mint_into(borrow_asset, &manager, USDT::units(1000)).unwrap();
	// 	Tokens::mint_into(collateral_asset, &manager, BTC::units(100)).unwrap();

	// 	// dbg!(tokens)
	// 	<Lending as composable_traits::lending::Lending>::create(manager, config).unwrap()
	// }

	set_price(borrow_asset.id(), NORMALIZED::ONE);
	set_price(collateral_asset.id(), NORMALIZED::units(50_000));

	dbg!(get_price(borrow_asset.id(), NORMALIZED::ONE));
	dbg!(get_price(collateral_asset.id(), NORMALIZED::ONE));

	mint_currency_into(borrow_asset, manager, 1000).unwrap();
	mint_currency_into(collateral_asset, manager, 100).unwrap();

	let config = CreateInput {
		updatable: UpdateInput {
			collateral_factor,
			under_collateralized_warn_percent: default_under_collateralized_warn_percent(),
			liquidators: vec![],
			interest_rate_model: InterestRateModel::default(),
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
			"System::events() did not contain the market creation event. Found {system_events:#?}"
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

/// Mints the specified amount of the currency into the specified manager.
fn mint_currency_into(
	asset: RuntimeCurrency,
	manager: sp_core::sr25519::Public,
	amount: u128,
) -> Result<(), DispatchError> {
	Tokens::mint_into(asset.id(), &manager, asset.units(amount))
}

/// Create a market with a USDT vault LP token as collateral.
fn create_simple_vaulted_market(
	borrow_asset: RuntimeCurrency,
	manager: AccountId,
) -> ((MarketIndex, VaultId), CurrencyId) {
	let (_, VaultInfo { lp_token_id, .. }) = create_simple_vault(borrow_asset, manager);

	let market = create_market(
		borrow_asset,
		RuntimeCurrency::new(lp_token_id, 12),
		manager,
		DEFAULT_MARKET_VAULT_RESERVE,
		MoreThanOneFixedU128::saturating_from_integer(2),
	);

	(market, lp_token_id)
}

/// Create a market with BTC as borrow and USDT as collateral
fn create_simple_market() -> (MarketIndex, VaultId) {
	create_market(
		BTC::instance(),
		USDT::instance(),
		*ALICE,
		DEFAULT_MARKET_VAULT_RESERVE,
		MoreThanOneFixedU128::saturating_from_integer(DEFAULT_COLLATERAL_FACTOR),
	)
}
