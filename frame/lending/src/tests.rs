use std::ops::Mul;

use crate::{
	accrue_interest_internal,
	mocks::{
		currency::CurrencyId, new_test_ext, process_block, AccountId, Balance, BlockNumber,
		Lending, Oracle, Origin, Test, Tokens, Vault, VaultId, ALICE, BOB, CHARLIE,
		MILLISECS_PER_BLOCK, MINIMUM_BALANCE, UNRESERVED,
	},
	models::BorrowerData,
	Error, MarketIndex,
};
use composable_tests_helpers::{prop_assert_acceptable_computation_error, prop_assert_ok};
use composable_traits::{
	defi::{CurrencyPair, LiftedFixedBalance, MoreThanOneFixedU128, Rate, ZeroToOneFixedU128},
	lending::{math::*, CreateInput, UpdateInput},
	time::SECONDS_PER_YEAR_NAIVE,
	vault::{Deposit, VaultConfig},
};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use pallet_vault::models::VaultInfo;
use proptest::{prelude::*, test_runner::TestRunner};
use sp_arithmetic::assert_eq_error_rate;
use sp_core::U256;
use sp_runtime::{ArithmeticError, FixedPointNumber, Percent, Perquintill};

type BorrowAssetVault = VaultId;

type CollateralAsset = CurrencyId;

const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);
const DEFAULT_MARKET_VAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);
const DEFAULT_COLLATERAL_FACTOR: u128 = 2;
const INITIAL_BORROW_ASSET_AMOUNT: u128 = 10 ^ 30;

/// Create a very simple vault for the given currency, 100% is reserved.
fn create_simple_vault(
	asset_id: CurrencyId,
) -> (VaultId, VaultInfo<AccountId, Balance, CurrencyId, BlockNumber>) {
	let v = Vault::do_create_vault(
		Deposit::Existential,
		VaultConfig {
			asset_id,
			manager: *ALICE,
			reserved: Perquintill::from_percent(100),
			strategies: [].iter().cloned().collect(),
		},
	);
	assert_ok!(&v);
	v.expect("unreachable; qed;")
}

fn create_market(
	borrow_asset: CurrencyId,
	collateral_asset: CurrencyId,
	manager: AccountId,
	reserved: Perquintill,
	collateral_factor: MoreThanOneFixedU128,
) -> (MarketIndex, BorrowAssetVault) {
	let config = CreateInput {
		updatable: UpdateInput {
			collateral_factor,
			under_collaterized_warn_percent: Percent::from_float(0.10),
			liquidators: vec![],
			interest_rate_model: InterestRateModel::default(),
		},
		reserved_factor: reserved,
		currency_pair: CurrencyPair::new(collateral_asset, borrow_asset),
	};
	Tokens::mint_into(borrow_asset, &manager, 1_000_000_000).unwrap();
	<Lending as composable_traits::lending::Lending>::create(manager, config).unwrap()
}

/// Create a market with a USDT vault LP token as collateral
fn create_simple_vaulted_market() -> ((MarketIndex, BorrowAssetVault), CollateralAsset) {
	let (_collateral_vault, VaultInfo { lp_token_id: collateral_asset, .. }) =
		create_simple_vault(CurrencyId::USDT);
	(
		create_market(
			CurrencyId::BTC,
			collateral_asset,
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_rational(200, 100),
		),
		collateral_asset,
	)
}

/// Create a market with straight USDT as collateral
fn create_simple_market() -> (MarketIndex, BorrowAssetVault) {
	create_market(
		CurrencyId::BTC,
		CurrencyId::USDT,
		*ALICE,
		DEFAULT_MARKET_VAULT_RESERVE,
		MoreThanOneFixedU128::saturating_from_rational(DEFAULT_COLLATERAL_FACTOR * 100, 100),
	)
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
	let (accrued_increase, _) = accrue_interest_internal::<Test, InterestRateModel>(
		optimal,
		interest_rate_model,
		borrow_index,
		delta_time,
		total_borrows,
	)
	.unwrap();
	assert_eq!(accrued_increase, 10_000_000_000_000_000_000);

	let delta_time = MILLISECS_PER_BLOCK;
	let (accrued_increase, _) = accrue_interest_internal::<Test, InterestRateModel>(
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

	let (accrued_increase, _) = accrue_interest_internal::<Test, InterestRateModel>(
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
	let result = accrue_interest_internal::<Test, InterestRateModel>(
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
					accrue_interest_internal::<Test, InterestRateModel>(
						optimal,
						interest_rate_model,
						borrow_index,
						slot * MILLISECS_PER_BLOCK,
						total_issued - accrued_debt,
					)
					.unwrap();
				let (accrued_increase_2, borrow_index_2) =
					accrue_interest_internal::<Test, InterestRateModel>(
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
			let (accrue_increment, _) = accrue_interest_internal::<Test, InterestRateModel>(
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

	let (total_accrued, _) = accrue_interest_internal::<Test, InterestRateModel>(
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
fn can_create_valid_market() {
	new_test_ext().execute_with(|| {
		let borrow_asset = CurrencyId::BTC;
		let collateral_asset = CurrencyId::USDT;
		let manager = *ALICE;
		let collateral_factor =
			MoreThanOneFixedU128::saturating_from_rational(DEFAULT_COLLATERAL_FACTOR * 100, 100);
		let config = CreateInput {
			updatable: UpdateInput {
				collateral_factor,
				under_collaterized_warn_percent: Percent::from_float(0.10),
				liquidators: vec![],
				interest_rate_model: InterestRateModel::default(),
			},
			reserved_factor: DEFAULT_MARKET_VAULT_RESERVE,
			currency_pair: CurrencyPair::new(collateral_asset, borrow_asset),
		};
		let created =
			<Lending as composable_traits::lending::Lending>::create(manager, config.clone());
		assert!(!created.is_ok());
		Tokens::mint_into(borrow_asset, &manager, INITIAL_BORROW_ASSET_AMOUNT).unwrap();
		let created = <Lending as composable_traits::lending::Lending>::create(manager, config);
		assert_ok!(created);
		let new_balance = Tokens::balance(borrow_asset, &manager);
		assert!(new_balance < INITIAL_BORROW_ASSET_AMOUNT);
		let (market_id, borrow_vault_id) = created.unwrap();

		let vault_borrow_id =
			<Vault as composable_traits::vault::Vault>::asset_id(&borrow_vault_id).unwrap();
		assert_eq!(vault_borrow_id, borrow_asset);

		let initial_total_cash = Lending::total_cash(&market_id).unwrap();
		assert!(initial_total_cash > 0);
	});
}

#[test]
fn test_borrow_repay_in_same_block() {
	new_test_ext().execute_with(|| {
		let collateral_amount = 900000000;
		let (market_id, vault) = create_simple_market();
		let initial_total_cash = Lending::total_cash(&market_id).unwrap();
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, collateral_amount));

		assert_ok!(Lending::deposit_collateral_internal(&market_id, &ALICE, collateral_amount));
		assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);

		let borrow_asset_deposit = 900000;
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_asset_deposit));
		let mut total_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_asset_deposit) + initial_total_cash;

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		let price =
			|currency_id, amount| Oracle::get_price(currency_id, amount).expect("impossible").price;

		assert_eq!(Lending::borrow_balance_current(&market_id, &ALICE), Ok(Some(0)));
		let limit_normalized = Lending::get_borrow_limit(&market_id, &ALICE).unwrap();
		let alice_limit = limit_normalized / price(CurrencyId::BTC, 1);
		assert_eq!(Lending::total_cash(&market_id), Ok(total_cash));
		process_block(1);
		assert_ok!(Lending::borrow_internal(&market_id, &ALICE, alice_limit / 4));
		total_cash -= alice_limit / 4;
		let total_borrows = alice_limit / 4;
		assert_eq!(Lending::total_cash(&market_id), Ok(total_cash));
		assert_eq!(Lending::total_borrows(&market_id), Ok(total_borrows));
		let alice_repay_amount = Lending::borrow_balance_current(&market_id, &ALICE).unwrap();
		// MINT required BTC so that ALICE and BOB can repay the borrow.
		assert_ok!(Tokens::mint_into(
			CurrencyId::BTC,
			&ALICE,
			alice_repay_amount.unwrap() - (alice_limit / 4)
		));
		assert_noop!(
			Lending::repay_borrow_internal(&market_id, &ALICE, &ALICE, alice_repay_amount),
			Error::<Test>::BorrowAndRepayInSameBlockIsNotSupported
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
		let initial_total_cash = Lending::total_cash(&market).unwrap();
		assert!(initial_total_cash > 0);
		let unit = 1_000_000_000;
		Oracle::set_btc_price(50000);

		let price =
			|currency_id, amount| Oracle::get_price(currency_id, amount).expect("impossible").price;

		let alice_capable_btc = 100 * unit;
		let collateral_amount =
			alice_capable_btc * price(CurrencyId::BTC, 1000) / price(CurrencyId::USDT, 1000);
		assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, collateral_amount));
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, collateral_amount));
		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let limit = limit_normalized / price(CurrencyId::BTC, 1);
		assert_eq!(limit, alice_capable_btc / DEFAULT_COLLATERAL_FACTOR);

		let borrow_asset_deposit = 100000 * unit;
		assert_eq!(Tokens::balance(CurrencyId::BTC, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &CHARLIE, borrow_asset_deposit));
		assert_eq!(Tokens::balance(CurrencyId::BTC, &CHARLIE), borrow_asset_deposit);
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_asset_deposit));
		assert_eq!(Tokens::balance(CurrencyId::BTC, &CHARLIE), 0);

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		let expected_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_asset_deposit) + initial_total_cash;
		assert_eq!(Lending::total_cash(&market), Ok(expected_cash));

		let alice_borrow = alice_capable_btc / DEFAULT_COLLATERAL_FACTOR / 10;
		assert_ok!(Lending::borrow_internal(&market, &ALICE, alice_borrow));
		assert_eq!(Lending::total_cash(&market), Ok(expected_cash - alice_borrow));
		assert_eq!(Lending::total_borrows(&market), Ok(alice_borrow));
		assert_eq!(Lending::total_interest_accurate(&market), Ok(0));

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let original_limit = limit_normalized / price(CurrencyId::BTC, 1);

		assert_eq!(original_limit, alice_capable_btc / DEFAULT_COLLATERAL_FACTOR - alice_borrow);

		let borrow = Lending::borrow_balance_current(&market, &ALICE).unwrap().unwrap();
		assert_eq!(borrow, alice_borrow);
		let interest_before = Lending::total_interest_accurate(&market).unwrap();
		for i in 2..50 {
			process_block(i);
		}
		let interest_after = Lending::total_interest_accurate(&market).unwrap();
		assert!(interest_before < interest_after);

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let new_limit = limit_normalized / price(CurrencyId::BTC, 1);

		assert!(new_limit < original_limit);

		let borrow = Lending::borrow_balance_current(&market, &ALICE).unwrap().unwrap();

		assert!(borrow > alice_borrow);
		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, original_limit),
			Error::<Test>::NotEnoughCollateralToBorrowAmount
		);

		// Borrow less because of interests.
		assert_ok!(Lending::borrow_internal(&market, &ALICE, new_limit,));

		// More than one borrow request in same block is invalid.
		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, 1),
			Error::<Test>::InvalidTimestampOnBorrowRequest
		);

		process_block(10001);

		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, collateral_amount));
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, collateral_amount));

		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		assert!(price(CurrencyId::BTC, alice_capable_btc) > alice_limit);
		assert!(alice_limit > price(CurrencyId::BTC, alice_borrow));

		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, alice_limit),
			Error::<Test>::NotEnoughCollateralToBorrowAmount
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
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, deposit_usdt));
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &ALICE, deposit_btc));

		assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id, deposit_btc));
		assert_ok!(Lending::deposit_collateral_internal(&market_id, &ALICE, deposit_usdt));
		assert_noop!(
			Lending::borrow_internal(&market_id, &ALICE, deposit_btc + initial_total_cash),
			Error::<Test>::NotEnoughBorrowAsset
		);
	});
}

#[test]
fn test_vault_market_can_withdraw() {
	new_test_ext().execute_with(|| {
		let (market, vault_id) = create_simple_market();

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &ALICE, borrow));

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
fn borrow_repay() {
	new_test_ext().execute_with(|| {
		let alice_balance = 1_000_000;
		let bob_balance = 1_000_000;

		let (market, vault) = create_simple_market();

		// Balance for ALICE
		assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, alice_balance));
		assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), alice_balance);
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, alice_balance));
		assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);

		// Balance for BOB
		assert_eq!(Tokens::balance(CurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &BOB, bob_balance));
		assert_eq!(Tokens::balance(CurrencyId::USDT, &BOB), bob_balance);
		assert_ok!(Lending::deposit_collateral_internal(&market, &BOB, bob_balance));
		assert_eq!(Tokens::balance(CurrencyId::USDT, &BOB), 0);

		let borrow_asset_deposit = 10_000_000_000;
		assert_eq!(Tokens::balance(CurrencyId::BTC, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &CHARLIE, borrow_asset_deposit));
		assert_eq!(Tokens::balance(CurrencyId::BTC, &CHARLIE), borrow_asset_deposit);
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, borrow_asset_deposit));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		// ALICE borrows
		assert_eq!(Lending::borrow_balance_current(&market, &ALICE), Ok(Some(0)));
		let alice_limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let alice_limit =
			alice_limit_normalized / Oracle::get_price(CurrencyId::BTC, 1).unwrap().price;
		assert_ok!(Lending::borrow_internal(&market, &ALICE, alice_limit));

		for i in 2..10000 {
			process_block(i);
		}

		// BOB borrows
		assert_eq!(Lending::borrow_balance_current(&market, &BOB), Ok(Some(0)));
		let limit_normalized = Lending::get_borrow_limit(&market, &BOB).unwrap();
		let limit = limit_normalized / Oracle::get_price(CurrencyId::BTC, 1).unwrap().price;
		assert_ok!(Lending::borrow_internal(&market, &BOB, limit,));

		let bob_limit = Lending::get_borrow_limit(&market, &BOB).unwrap();

		for i in 10000..20000 {
			process_block(i);
		}

		let alice_repay_amount = Lending::borrow_balance_current(&market, &ALICE).unwrap();
		let bob_repay_amount = Lending::borrow_balance_current(&market, &BOB).unwrap();

		// MINT required BTC so that ALICE and BOB can repay the borrow.
		assert_ok!(Tokens::mint_into(
			CurrencyId::BTC,
			&ALICE,
			alice_repay_amount.unwrap() - alice_limit
		));
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &BOB, bob_repay_amount.unwrap() - bob_limit));
		// ALICE , BOB both repay's loan. their USDT balance should have decreased because of
		// interest paid on borrows
		assert_ok!(Lending::repay_borrow_internal(&market, &BOB, &BOB, bob_repay_amount));
		assert_ok!(Lending::repay_borrow_internal(&market, &ALICE, &ALICE, alice_repay_amount));
		assert!(alice_balance > Tokens::balance(CurrencyId::USDT, &ALICE));
		assert!(bob_balance > Tokens::balance(CurrencyId::USDT, &BOB));
	});
}

#[test]
fn liquidation() {
	new_test_ext().execute_with(|| {
		let (market, vault) = create_market(
			CurrencyId::USDT,
			CurrencyId::BTC,
			*ALICE,
			Perquintill::from_percent(10),
			MoreThanOneFixedU128::saturating_from_rational(2, 1),
		);

		Oracle::set_btc_price(100);

		let two_btc_amount = 2;
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &ALICE, two_btc_amount));
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, two_btc_amount));
		assert_eq!(Tokens::balance(CurrencyId::BTC, &ALICE), 0);

		let usdt_amt = u32::MAX as Balance;
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, usdt_amt));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		let borrow_limit = Lending::get_borrow_limit(&market, &ALICE).expect("impossible");
		assert!(borrow_limit > 0);

		assert_ok!(Lending::borrow_internal(&market, &ALICE, borrow_limit));

		for i in 2..10000 {
			process_block(i);
		}

		// Collateral going down imply liquidation
		Oracle::set_btc_price(99);

		assert_ok!(Lending::liquidate_internal(None, &market, vec![*ALICE]));
	});
}

#[test]
fn test_warn_soon_under_collaterized() {
	new_test_ext().execute_with(|| {
		let (market, vault) = create_market(
			CurrencyId::USDT,
			CurrencyId::BTC,
			*ALICE,
			Perquintill::from_percent(10),
			MoreThanOneFixedU128::saturating_from_rational(2, 1),
		);

		// 1 BTC = 100 USDT
		Oracle::set_btc_price(100);

		// Deposit 2 BTC
		let two_btc_amount = 2;
		assert_eq!(Tokens::balance(CurrencyId::BTC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(CurrencyId::BTC, &ALICE, two_btc_amount));
		assert_eq!(Tokens::balance(CurrencyId::BTC, &ALICE), two_btc_amount);
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, two_btc_amount));
		assert_eq!(Tokens::balance(CurrencyId::BTC, &ALICE), 0);

		// Balance of USDT for CHARLIE
		// CHARLIE is only lender of USDT
		let usdt_amt = u32::MAX as Balance;
		assert_eq!(Tokens::balance(CurrencyId::USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(CurrencyId::USDT, &CHARLIE, usdt_amt));
		assert_eq!(Tokens::balance(CurrencyId::USDT, &CHARLIE), usdt_amt);
		assert_ok!(Vault::deposit(Origin::signed(*CHARLIE), vault, usdt_amt));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		// ALICE borrows
		assert_eq!(Lending::borrow_balance_current(&market, &ALICE), Ok(Some(0)));

		/* Can
			 = 1/collateral_factor * deposit
			 = 1/2 * two_btc_price
			 = 10000 cents

		*/
		assert_eq!(Lending::get_borrow_limit(&market, &ALICE), Ok(100));

		// Borrow 80 USDT
		let borrow_amount = 80;
		assert_ok!(Lending::borrow_internal(&market, &ALICE, borrow_amount));

		for i in 2..10000 {
			process_block(i);
		}

		Oracle::set_btc_price(90);

		/*
			Collateral = 2*90 = 180
			Borrow = 80
			Ratio = 2.25
		*/
		assert_eq!(Lending::soon_under_collaterized(&market, &ALICE), Ok(false));

		Oracle::set_btc_price(87);

		/*
		   Collateral = 2*87 = 174
		   Borrow = 80
		   Ratio = ~2.18
		*/
		assert_eq!(Lending::soon_under_collaterized(&market, &ALICE), Ok(true));
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
			prop_assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), amount);

			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Lending::withdraw_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), amount);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_deposit_withdraw_higher_amount_fails(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _vault) = create_simple_market();
			prop_assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(CurrencyId::USDT, &ALICE, amount ));
			prop_assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), amount );

			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount ));
			prop_assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 0);
			prop_assert_eq!(
				Lending::withdraw_collateral_internal(&market, &ALICE, amount  + 1),
				Err(Error::<Test>::NotEnoughCollateral.into())
			);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_vaulted_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let ((market, _), collateral_asset) = create_simple_vaulted_market();

			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(collateral_asset, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), amount);

			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), 0);
			prop_assert_ok!(Lending::withdraw_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE), amount);

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
	fn market_creation_with_multi_level_priceable_lp(depth in 0..20) {
		new_test_ext().execute_with(|| {
			// Assume we have a pricable base asset
			let base_asset = CurrencyId::ETH;
			let base_vault = create_simple_vault(base_asset);

			let (_, VaultInfo { lp_token_id, ..}) =
				(0..depth).fold(base_vault, |(_, VaultInfo { lp_token_id, .. }), _| {
					// No stock dilution, 1:1 price against the quote asset.
					create_simple_vault(lp_token_id)
				});

			// A market with two priceable assets can be created
			create_market(
				CurrencyId::BTC,
				lp_token_id,
				*ALICE,
				Perquintill::from_percent(10),
				MoreThanOneFixedU128::saturating_from_rational(200, 100),
			);

			// Top level lp price should be transitively resolvable to the base asset price.
			prop_assert_ok!(Oracle::get_price(lp_token_id, 1));

			// Without stock dilution, prices should be equals
			prop_assert_eq!(Oracle::get_price(lp_token_id, 1), Oracle::get_price(base_asset, 1));

			Ok(())
		})?;
	}

	#[test]
	fn market_are_isolated(
		(amount1, amount2) in valid_amounts_without_overflow_2()
	) {
		new_test_ext().execute_with(|| {
			let (market_id1, vault_id1) = create_simple_market();
			let (market_id2, vault_id2) = create_simple_market();

			// Ensure markets are unique
			prop_assert_ne!(market_id1, market_id2);
			prop_assert_ne!(Lending::account_id(&market_id1), Lending::account_id(&market_id2));

			// Alice lend an amount in market1 vault
			prop_assert_ok!(Tokens::mint_into(CurrencyId::BTC, &ALICE, amount1));
			prop_assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id1, amount1));

			// Bob lend an amount in market2 vault
			prop_assert_eq!(Tokens::balance(CurrencyId::BTC, &BOB), 0);
			prop_assert_ok!(Tokens::mint_into(CurrencyId::BTC, &BOB, amount2));
			prop_assert_eq!(Tokens::balance(CurrencyId::BTC, &BOB), amount2);
			prop_assert_ok!(Vault::deposit(Origin::signed(*BOB), vault_id2, amount2));

			// Allow the market to initialize it's account by withdrawing
			// from the vault
			for i in 1..2 {
				process_block(i);
			}

			let expected_market1_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(amount1);
			let expected_market2_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(amount2);

			// // The funds should not be shared.
			prop_assert_acceptable_computation_error!(
				Tokens::balance(CurrencyId::BTC, &Lending::account_id(&market_id1)),
				expected_market1_balance
			);
			prop_assert_acceptable_computation_error!(
				Tokens::balance(CurrencyId::BTC, &Lending::account_id(&market_id2)),
				expected_market2_balance
			);

			Ok(())
		})?;
	}
}
