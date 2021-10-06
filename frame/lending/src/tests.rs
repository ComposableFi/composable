use std::ops::Mul;

use crate::{
	accrue_interest_internal,
	mocks::{
		new_test_ext, process_block, AccountId, Balance, BlockNumber, Lending, MockCurrencyId,
		Oracle, Origin, Test, Tokens, Vault, VaultId, ALICE, BOB, CHARLIE, MILLISECS_PER_BLOCK,
		MINIMUM_BALANCE,
	},
	models::BorrowerData,
	Error, MarketIndex,
};
use composable_traits::{
	lending::MarketConfigInput,
	math::LiftedFixedBalance,
	rate_model::*,
	vault::{CapabilityVault, Deposit, VaultConfig},
};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use pallet_vault::models::VaultInfo;
use proptest::{prelude::*, test_runner::TestRunner};
use sp_runtime::{helpers_128bit::multiply_by_rational, FixedPointNumber, Percent, Perquintill};

type BorrowAssetVault = VaultId;

type CollateralAsset = MockCurrencyId;

const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);
const DEFAULT_MARKET_VAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);

/// Create a very simple vault for the given currency, 100% is reserved.
fn create_simple_vault(
	asset_id: MockCurrencyId,
) -> (VaultId, VaultInfo<AccountId, Balance, MockCurrencyId, BlockNumber>) {
	let v = Vault::do_create_vault(
		Deposit::Existential,
		VaultConfig {
			asset_id,
			manager: ALICE,
			reserved: Perquintill::from_percent(100),
			strategies: [].iter().cloned().collect(),
		},
	);
	assert_ok!(&v);
	v.expect("unreachable; qed;")
}

fn create_market(
	borrow_asset: MockCurrencyId,
	collateral_asset: MockCurrencyId,
	manager: AccountId,
	reserved: Perquintill,
	collateral_factor: NormalizedCollateralFactor,
) -> (MarketIndex, BorrowAssetVault) {
	let market_config = MarketConfigInput {
		manager,
		reserved,
		collateral_factor,
		under_collaterized_warn_percent: Percent::from_float(0.10),
	};
	let market = Lending::create(borrow_asset, collateral_asset, market_config);
	assert_ok!(market);
	market.expect("unreachable; qed;")
}

/// Create a market with a USDT vault LP token as collateral
fn create_simple_vaulted_market() -> ((MarketIndex, BorrowAssetVault), CollateralAsset) {
	let (_collateral_vault, VaultInfo { lp_token_id: collateral_asset, .. }) =
		create_simple_vault(MockCurrencyId::USDT);
	(
		create_market(
			MockCurrencyId::BTC,
			collateral_asset,
			ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			NormalizedCollateralFactor::saturating_from_rational(200, 100),
		),
		collateral_asset,
	)
}

/// Create a market with straight USDT as collateral
fn create_simple_market() -> (MarketIndex, BorrowAssetVault) {
	create_market(
		MockCurrencyId::BTC,
		MockCurrencyId::USDT,
		ALICE,
		DEFAULT_MARKET_VAULT_RESERVE,
		NormalizedCollateralFactor::saturating_from_rational(200, 100),
	)
}

#[test]
fn accrue_interest_base_cases() {
	let (optimal, ref interest_rate_model) = new_jump_model();
	let stable_rate = interest_rate_model.get_borrow_rate(optimal).unwrap();
	assert_eq!(stable_rate, Ratio::saturating_from_rational(10, 100));
	let borrow_index = Rate::saturating_from_integer(1);
	let delta_time = SECONDS_PER_YEAR;
	let total_issued = 100000000000000000000;
	let (accrued_increase, _) = accrue_interest_internal::<Test>(
		optimal,
		interest_rate_model,
		borrow_index,
		delta_time,
		total_issued,
		0,
	)
	.unwrap();
	assert_eq!(accrued_increase, 10000000000000000000);

	let delta_time = MILLISECS_PER_BLOCK;
	let (accrued_increase, _) = accrue_interest_internal::<Test>(
		optimal,
		interest_rate_model,
		borrow_index,
		delta_time,
		total_issued,
		0,
	)
	.unwrap();
	// small increments instead one year lead to some loss by design (until we lift calculation to
	// 256 bit)
	let error = 25;
	assert_eq!(
		accrued_increase,
		10000000000000000000 * MILLISECS_PER_BLOCK as u128 / SECONDS_PER_YEAR as u128 + error
	);
}

#[test]
fn accrue_interest_edge_cases() {
	let (_, ref interest_rate_model) = new_jump_model();
	let utilization = Percent::from_percent(100);
	let borrow_index = Rate::saturating_from_integer(1);
	let delta_time = SECONDS_PER_YEAR;
	let total_issued = u128::MAX;
	let (accrued_increase, _) = accrue_interest_internal::<Test>(
		utilization,
		interest_rate_model,
		borrow_index,
		delta_time,
		total_issued,
		0,
	)
	.unwrap();
	assert_eq!(accrued_increase, 108890357414700308308279874378165827666);

	let (accrued_increase, _) = accrue_interest_internal::<Test>(
		utilization,
		interest_rate_model,
		borrow_index,
		delta_time,
		0,
		0,
	)
	.unwrap();
	assert_eq!(accrued_increase, 0);
}

#[test]
fn accrue_interest_induction() {
	let (optimal, ref interest_rate_model) = new_jump_model();
	let borrow_index = Rate::saturating_from_integer(1);

	let minimal = 5; // current precision and minimal time delta do not allow to accrue on less than this power of 10
	let mut runner = TestRunner::default();
	runner
		.run(
			&(
				0..=2 * SECONDS_PER_YEAR / MILLISECS_PER_BLOCK,
				(minimal..=25u32).prop_map(|i| 10u128.pow(i)),
			),
			|(slot, total_issued)| {
				let (accrued_increase_1, borrow_index_1) = accrue_interest_internal::<Test>(
					optimal,
					interest_rate_model,
					borrow_index,
					slot * MILLISECS_PER_BLOCK,
					total_issued,
					0,
				)
				.unwrap();
				let (accrued_increase_2, borrow_index_2) = accrue_interest_internal::<Test>(
					optimal,
					interest_rate_model,
					borrow_index,
					(slot + 1) * MILLISECS_PER_BLOCK,
					total_issued,
					0,
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
	let (optimal, interest_rate_model) = new_jump_model();
	let borrow_index = Rate::checked_from_integer(1).unwrap();
	let total_issued = 10000000;
	// no sure how handle in rust previous + next (so map has access to previous result)
	let mut previous = 0;
	let _data: Vec<_> = (0..=1000)
		.map(|x| {
			let (accrue_increment, _) = accrue_interest_internal::<Test>(
				optimal,
				&interest_rate_model,
				borrow_index,
				MILLISECS_PER_BLOCK,
				total_issued,
				0,
			)
			.unwrap();
			previous += accrue_increment;
			(x, previous)
		})
		.collect();

	let (total_accrued, _) = accrue_interest_internal::<Test>(
		optimal,
		&interest_rate_model,
		Rate::checked_from_integer(1).unwrap(),
		1000 * MILLISECS_PER_BLOCK,
		total_issued,
		0,
	)
	.unwrap();
	let error = 68;
	assert_eq!(previous + error, total_accrued);

	#[cfg(feature = "visualization")]
	{
		use plotters::prelude::*;
		let area = BitMapBackend::new("./accrue_interest.png", (1024, 768)).into_drawing_area();
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
fn test_borrow_repay_in_same_block() {
	new_test_ext().execute_with(|| {
		let collateral_amount = 900000;
		let (market, vault) = create_simple_market();
		// Balance for ALICE
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, collateral_amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), collateral_amount);

		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, collateral_amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);

		// Balance of BTC for CHARLIE
		// CHARLIE is only lender of BTC
		let borrow_asset_deposit = collateral_amount;
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &CHARLIE, borrow_asset_deposit));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), borrow_asset_deposit);
		assert_ok!(Vault::deposit(Origin::signed(CHARLIE), vault, borrow_asset_deposit));
		let mut total_cash = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_asset_deposit);

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		assert_eq!(Lending::borrow_balance_current(&market, &ALICE), Ok(Some(0)));
		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		assert_eq!(Lending::total_cash(&market), Ok(total_cash));
		process_block(1);
		assert_ok!(Lending::borrow_internal(&market, &ALICE, alice_limit / 4));
		total_cash -= alice_limit / 4;
		let total_borrows = alice_limit / 4;
		assert_eq!(Lending::total_cash(&market), Ok(total_cash));
		assert_eq!(Lending::total_borrows(&market), Ok(total_borrows));
		let alice_repay_amount = Lending::borrow_balance_current(&market, &ALICE).unwrap();
		// MINT required BTC so that ALICE and BOB can repay the borrow.
		assert_ok!(Tokens::mint_into(
			MockCurrencyId::BTC,
			&ALICE,
			alice_repay_amount.unwrap() - (alice_limit / 4)
		));
		assert_noop!(
			Lending::repay_borrow_internal(&market, &ALICE, &ALICE, alice_repay_amount),
			Error::<Test>::BorrowAndRepayInSameBlockIsNotSupported
		);
	});
}

/// some model with sane parameter
fn new_jump_model() -> (Percent, InterestRateModel) {
	let base_rate = Rate::saturating_from_rational(2, 100);
	let jump_rate = Rate::saturating_from_rational(10, 100);
	let full_rate = Rate::saturating_from_rational(32, 100);
	let optimal = Percent::from_percent(80);
	let interest_rate_model = InterestRateModel::Jump(
		JumpModel::new_model(base_rate, jump_rate, full_rate, optimal).unwrap(),
	);
	(optimal, interest_rate_model)
}

#[test]
fn test_calc_utilization_ratio() {
	// 50% borrow
	assert_eq!(Lending::calc_utilization_ratio(&1, &1).unwrap(), Percent::from_percent(50));
	assert_eq!(Lending::calc_utilization_ratio(&100, &100).unwrap(), Percent::from_percent(50));
	// no borrow
	assert_eq!(Lending::calc_utilization_ratio(&1, &0).unwrap(), Percent::zero());
	// full borrow
	assert_eq!(Lending::calc_utilization_ratio(&0, &1).unwrap(), Percent::from_percent(100));
}

#[test]
fn test_borrow_math() {
	let borrower = BorrowerData::new(
		100,
		1,
		0,
		1,
		NormalizedCollateralFactor::from_float(1.0),
		Percent::from_float(0.10),
	);
	let borrow = borrower.borrow_for_collateral().unwrap();
	assert_eq!(borrow, LiftedFixedBalance::from(100));
}

#[test]
fn test_borrow() {
	new_test_ext().execute_with(|| {
		let collateral_amount = 900000;
		let (market, vault) = create_simple_market();
		// alice deposit collaterals
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, collateral_amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), collateral_amount);
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, collateral_amount));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);

		// Balance of BTC for CHARLIE
		// CHARLIE is only lender of BTC
		let borrow_asset_deposit = collateral_amount;
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &CHARLIE, borrow_asset_deposit));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), borrow_asset_deposit);
		assert_ok!(Vault::deposit(Origin::signed(CHARLIE), vault, borrow_asset_deposit));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), 0);

		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();

		let alice_expected_borrowable = 9;
		// Can only borrow 9 BTC
		assert_eq!(alice_limit, alice_expected_borrowable);

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		let expected_cash = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_asset_deposit);
		assert_eq!(Lending::total_cash(&market), Ok(expected_cash));

		let alice_borrow = 1;
		assert_ok!(Lending::borrow_internal(&market, &ALICE, alice_borrow));
		assert_eq!(Lending::total_cash(&market), Ok(expected_cash - alice_borrow));
		assert_eq!(Lending::total_borrows(&market), Ok(alice_borrow));
		for i in 2..10000 {
			process_block(i);
		}
		assert_eq!(Lending::total_interest_accurate(&market), Ok(38040334854886));
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, collateral_amount));
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, collateral_amount));
		assert_eq!(Lending::get_borrow_limit(&market, &ALICE).expect("impossible"), 17);
		// Add more borrow
		assert_ok!(Lending::borrow_internal(&market, &ALICE, 10));
		assert_eq!(Lending::get_borrow_limit(&market, &ALICE).expect("impossible"), 7);
		// more than one borrow request in same block is invalid.
		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, 1),
			Error::<Test>::InvalidTimestampOnBorrowRequest
		);
		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, alice_expected_borrowable - alice_borrow + 1),
			Error::<Test>::InvalidTimestampOnBorrowRequest
		);
		process_block(10001);
		// Try to borrow more than limit
		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, alice_limit + 1),
			Error::<Test>::NotEnoughCollateralToBorrowAmount
		);
	});
}

#[test]
fn test_vault_interactions() {
	new_test_ext().execute_with(|| {
		let (market, vault_id) = create_simple_market();
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, 1_000_000_000_000));
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 1_000_000_000_000));

		Vault::deposit(Origin::signed(ALICE), vault_id, 1_000_000_000).unwrap();
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, 1_000_000_000));

		<Vault as CapabilityVault>::stop(&vault_id).unwrap();

		assert_noop!(
			Lending::borrow_internal(&market, &ALICE, 1_000),
			Error::<Test>::NotEnoughBorrowAsset
		);
	});
}

#[test]
fn borrow_repay() {
	new_test_ext().execute_with(|| {
		let alice_balance = 1_000_000;
		let bob_balance = 1_000_000;
		let (market, vault) = create_simple_market();
		// Balance for ALICE
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, alice_balance));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), alice_balance);
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, alice_balance));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);

		// Balance for BOB
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &BOB, bob_balance));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), bob_balance);
		assert_ok!(Lending::deposit_collateral_internal(&market, &BOB, bob_balance));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &BOB), 0);

		// Balance of BTC for CHARLIE
		// CHARLIE is only lender of BTC
		let borrow_asset_deposit = 1_000_000;
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &CHARLIE, borrow_asset_deposit));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &CHARLIE), borrow_asset_deposit);
		assert_ok!(Vault::deposit(Origin::signed(CHARLIE), vault, borrow_asset_deposit));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		// ALICE borrows
		assert_eq!(Lending::borrow_balance_current(&market, &ALICE), Ok(Some(0)));
		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		assert_ok!(Lending::borrow_internal(&market, &ALICE, alice_limit));

		for i in 2..10000 {
			process_block(i);
		}

		// BOB borrows
		assert_eq!(Lending::borrow_balance_current(&market, &BOB), Ok(Some(0)));
		assert_ok!(Lending::borrow_internal(
			&market,
			&BOB,
			Lending::get_borrow_limit(&market, &BOB).expect("impossible")
		));

		let bob_limit = Lending::get_borrow_limit(&market, &BOB).unwrap();

		for i in 10000..20000 {
			process_block(i);
		}

		let alice_repay_amount = Lending::borrow_balance_current(&market, &ALICE).unwrap();
		let bob_repay_amount = Lending::borrow_balance_current(&market, &BOB).unwrap();

		// MINT required BTC so that ALICE and BOB can repay the borrow.
		assert_ok!(Tokens::mint_into(
			MockCurrencyId::BTC,
			&ALICE,
			alice_repay_amount.unwrap() - alice_limit
		));
		assert_ok!(Tokens::mint_into(
			MockCurrencyId::BTC,
			&BOB,
			bob_repay_amount.unwrap() - bob_limit
		));
		// ALICE , BOB both repay's loan. their USDT balance should have decreased because of
		// interest paid on borrows
		assert_ok!(Lending::repay_borrow_internal(&market, &BOB, &BOB, bob_repay_amount));
		assert_ok!(Lending::repay_borrow_internal(&market, &ALICE, &ALICE, alice_repay_amount));
		assert!(alice_balance > Tokens::balance(MockCurrencyId::USDT, &ALICE));
		assert!(bob_balance > Tokens::balance(MockCurrencyId::USDT, &BOB));
	});
}

#[test]
fn test_liquidation() {
	new_test_ext().execute_with(|| {
		let (market, vault) = create_market(
			MockCurrencyId::USDT,
			MockCurrencyId::BTC,
			ALICE,
			Perquintill::from_percent(10),
			NormalizedCollateralFactor::saturating_from_rational(200, 100),
		);
		// Balance for ALICE
		Oracle::set_btc_price(100u128);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 2));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 2);
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, 2));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 0);

		// Balance of USDT for CHARLIE
		// CHARLIE is only lender of USDT
		let usdt_amt = u32::MAX as Balance;
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, usdt_amt));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), usdt_amt);
		assert_ok!(Vault::deposit(Origin::signed(CHARLIE), vault, usdt_amt));

		// ALICE borrows
		assert_eq!(Lending::borrow_balance_current(&market, &ALICE), Ok(Some(0)));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		assert_ok!(Lending::borrow_internal(&market, &ALICE, 100));

		for i in 2..10000 {
			process_block(i);
		}

		// change price of colleteral to initiate liquidation
		Oracle::set_btc_price(98u128);
		assert_ok!(Lending::liquidate_internal(&market, &ALICE));
	});
}

#[test]
fn test_warn_soon_under_collaterized() {
	new_test_ext().execute_with(|| {
		let (market, vault) = create_market(
			MockCurrencyId::USDT,
			MockCurrencyId::BTC,
			ALICE,
			Perquintill::from_percent(10),
			NormalizedCollateralFactor::saturating_from_rational(200, 100),
		);
		// Balance for ALICE
		Oracle::set_btc_price(100u128);
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 2));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 2);
		assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, 2));
		assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 0);

		// Balance of USDT for CHARLIE
		// CHARLIE is only lender of USDT
		let usdt_amt = u32::MAX as Balance;
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), 0);
		assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &CHARLIE, usdt_amt));
		assert_eq!(Tokens::balance(MockCurrencyId::USDT, &CHARLIE), usdt_amt);
		assert_ok!(Vault::deposit(Origin::signed(CHARLIE), vault, usdt_amt));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		for i in 1..2 {
			process_block(i);
		}

		// ALICE borrows
		assert_eq!(Lending::borrow_balance_current(&market, &ALICE), Ok(Some(0)));
		assert_ok!(Lending::borrow_internal(&market, &ALICE, 80));

		for i in 2..10000 {
			process_block(i);
		}

		Oracle::set_btc_price(90u128);
		assert_eq!(Lending::soon_under_collaterized(&market, &ALICE), Ok(false));
		// change price of colleteral to warn
		Oracle::set_btc_price(84u128);
		assert_eq!(Lending::soon_under_collaterized(&market, &ALICE), Ok(true));
	});
}

/*
  TODO(hussein-aitlahcen):
  Extract all proptests helpers into a composable-test-helper crate?
*/
macro_rules! prop_assert_ok {
    ($cond:expr) => {
        prop_assert_ok!($cond, concat!("assertion failed: ", stringify!($cond)))
    };

    ($cond:expr, $($fmt:tt)*) => {
        if let Err(e) = $cond {
            let message = format!($($fmt)*);
            let message = format!("{} unexpected {:?} at {}:{}", message, e, file!(), line!());
            return ::std::result::Result::Err(
                proptest::test_runner::TestCaseError::fail(message));
        }
    };
}

prop_compose! {
	fn valid_amount_without_overflow()
		(x in MINIMUM_BALANCE..u64::MAX as Balance) -> Balance {
		x
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
			((CHARLIE + 1)..balances.len() as AccountId)
				.zip(balances)
				.collect()
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
		(x in (CHARLIE + 1)..AccountId::MAX) -> AccountId {
			x
		}
}

/// Accept a 'dust' deviation
macro_rules! prop_assert_epsilon {
	($x:expr, $y:expr) => {{
		let precision = 100000;
		let epsilon = 10;
		let upper = precision + epsilon;
		let lower = precision - epsilon;
		let q = multiply_by_rational($x, precision, $y).expect("qed;");
		prop_assert!(
			upper >= q && q >= lower,
			"({}) => {} >= {} * {} / {} >= {}",
			q,
			upper,
			$x,
			precision,
			$y,
			lower
		);
	}};
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]

	#[test]
	fn proptest_math_borrow(collateral_balance in 0..u32::MAX as Balance,
							collateral_price in 0..u32::MAX as Balance,
							borrower_balance_with_interest in 0..u32::MAX as Balance,
							borrow_price in 0..u32::MAX as Balance
	) {
		let borrower = BorrowerData::new(
			collateral_balance,
			collateral_price,
			borrower_balance_with_interest,
			borrow_price,
			NormalizedCollateralFactor::from_float(1.0),
			Percent::from_float(0.10), // 10%
		);
		let borrow = borrower.borrow_for_collateral();
		prop_assert_ok!(borrow);
	}

	#[test]
	fn market_collateral_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _) = create_simple_market();
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), amount);

			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Lending::withdraw_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), amount);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_deposit_withdraw_higher_amount_fails(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _vault) = create_simple_market();
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(MockCurrencyId::USDT, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), amount);

			prop_assert_ok!(Lending::deposit_collateral_internal(&market, &ALICE, amount));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::USDT, &ALICE), 0);
			prop_assert_eq!(
				Lending::withdraw_collateral_internal(&market, &ALICE, amount + 1),
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
	fn market_creation_with_multi_level_priceable_lp(depth in 0..20) {
		new_test_ext().execute_with(|| {
			// Assume we have a pricable base asset
			let base_asset = MockCurrencyId::ETH;
			let base_vault = create_simple_vault(base_asset);

			let (_, VaultInfo { lp_token_id, ..}) =
				(0..depth).fold(base_vault, |(_, VaultInfo { lp_token_id, .. }), _| {
					// No stock dilution, 1:1 price against the quote asset.
					create_simple_vault(lp_token_id)
				});

			// A market with two priceable assets can be created
			create_market(
				MockCurrencyId::BTC,
				lp_token_id,
				ALICE,
				Perquintill::from_percent(10),
				NormalizedCollateralFactor::saturating_from_rational(200, 100),
			);

			// Top level lp price should be transitively resolvable to the base asset price.
			prop_assert_ok!(Oracle::get_price(lp_token_id));

			// Without stock dilution, prices should be equals
			prop_assert_eq!(Oracle::get_price(lp_token_id), Oracle::get_price(base_asset));

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
			prop_assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), 0);
			prop_assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &ALICE, amount1));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::BTC, &ALICE), amount1);
			prop_assert_ok!(Vault::deposit(Origin::signed(ALICE), vault_id1, amount1));

			// Bob lend an amount in market2 vault
			prop_assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), 0);
			prop_assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &BOB, amount2));
			prop_assert_eq!(Tokens::balance(MockCurrencyId::BTC, &BOB), amount2);
			prop_assert_ok!(Vault::deposit(Origin::signed(BOB), vault_id2, amount2));

			// Allow the market to initialize it's account by withdrawing
			// from the vault
			for i in 1..2 {
				process_block(i);
			}

			let expected_market1_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(amount1);
			let expected_market2_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(amount2);

			// // The funds should not be shared.
			prop_assert_epsilon!(
				Tokens::balance(MockCurrencyId::BTC, &Lending::account_id(&market_id1)),
				expected_market1_balance
			);
			prop_assert_epsilon!(
				Tokens::balance(MockCurrencyId::BTC, &Lending::account_id(&market_id2)),
				expected_market2_balance
			);

			Ok(())
		})?;
	}
}
