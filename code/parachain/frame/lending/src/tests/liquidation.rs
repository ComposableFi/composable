use super::prelude::*;
use crate::tests::{
	borrow, create_market_for_liquidation_test, mint_and_deposit_collateral,
	process_and_progress_blocks,
};

#[test]
fn test_liquidate_multiple() {
	new_test_ext().execute_with(|| {
		let manager = *ALICE;
		let lender = *CHARLIE;
		let first_borrower = *ALICE;
		let second_borrower = *BOB;
		let third_borrower = *CHARLIE;
		let (market_id, vault_id) = create_market_for_liquidation_test::<Runtime>(manager);
		// Deposit USDT in the vault.
		let vault_value = USDT::units(100_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &lender, vault_value));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(lender), vault_id, vault_value));
		process_and_progress_blocks::<Lending, Runtime>(1);
		// Deposit 1 BTC collateral from borrowers' accounts.
		mint_and_deposit_collateral::<Runtime>(first_borrower, BTC::units(1), market_id, BTC::ID);
		mint_and_deposit_collateral::<Runtime>(second_borrower, BTC::units(1), market_id, BTC::ID);
		mint_and_deposit_collateral::<Runtime>(third_borrower, BTC::units(1), market_id, BTC::ID);
		// Each borrower borrows 20_000 USDT
		borrow::<Runtime>(first_borrower, market_id, USDT::units(20_000));
		borrow::<Runtime>(second_borrower, market_id, USDT::units(20_000));
		borrow::<Runtime>(third_borrower, market_id, USDT::units(20_000));
		// Emulate situation when collateral price has fallen down
		// from 50_000 USDT to 38_000 USDT.
		set_price(BTC::ID, NORMALIZED::units(38_000));
		let borrowers_vec = vec![first_borrower, second_borrower, third_borrower];
		let borrowers = TestBoundedVec::try_from(borrowers_vec.clone()).unwrap();
		assert_extrinsic_event::<Runtime>(
			Lending::liquidate(
				RuntimeOrigin::signed(manager),
				market_id.clone(),
				borrowers.clone(),
			),
			RuntimeEvent::Lending(crate::Event::LiquidationInitiated {
				market_id,
				borrowers: borrowers_vec,
			}),
		);
		// Check if cleanup was done correctly
		borrowers.iter().for_each(|borrower| {
			assert!(!crate::DebtIndex::<Runtime>::contains_key(market_id, borrower))
		});
		borrowers.iter().for_each(|borrower| {
			assert!(!crate::BorrowTimestamp::<Runtime>::contains_key(market_id, borrower))
		});
	})
}

#[test]
fn test_max_liquidation_batch_size_exceeded() {
	new_test_ext().execute_with(|| {
		let manager = *ALICE;
		let (market_id, _vault_id) = create_market_for_liquidation_test::<Runtime>(manager);
		let mut borrowers = vec![];
		let mut bytes = [0; 32];
		for i in 0..=<Runtime as crate::Config>::MaxLiquidationBatchSize::get() {
			let raw_account_id = U256::from(i);
			raw_account_id.to_little_endian(&mut bytes);
			let account_id = AccountId::from_raw(bytes);
			mint_and_deposit_collateral::<Runtime>(account_id, BTC::units(100), market_id, BTC::ID);
			borrowers.push(account_id);
		}

		let borrowers = TestBoundedVec::try_from(borrowers);
		// TryFrom implementation for BoundedVec emits () as error
		assert_err!(borrowers, ());
	})
}

#[test]
fn test_liquidation_storage_transaction_rollback() {
	new_test_ext().execute_with(|| {
		let manager = *ALICE;
		let lender = *CHARLIE;
		// ALICE is borrower who's borrow going to be liquidated
		let normal_borrower = *ALICE;
		// BOB is borrower who's borrow should, but can not be liquidated,
		// for some reason.
		let borrower_with_a_twist = *BOB;
		let (market_id, vault_id) = create_market_for_liquidation_test::<Runtime>(manager);
		// Deposit USDT in the vault.
		let vault_value = USDT::units(100_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &lender, vault_value));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(lender), vault_id, vault_value));
		process_and_progress_blocks::<Lending, Runtime>(1);
		// Deposit 1 BTC collateral from normal borrower account.
		crate::tests::mint_and_deposit_collateral::<Runtime>(
			normal_borrower,
			BTC::units(1),
			market_id,
			BTC::ID,
		);
		// Normal borrower borrows 20_000 USDT.
		borrow::<Runtime>(normal_borrower, market_id, USDT::units(20_000));
		// Deposit 1 BTC collateral from "borrower with a twist" account.
		crate::tests::mint_and_deposit_collateral::<Runtime>(
			borrower_with_a_twist,
			BTC::units(1),
			market_id,
			BTC::ID,
		);
		// Borrower with a twist borrows 20_000 USDT.
		borrow::<Runtime>(borrower_with_a_twist, market_id, USDT::units(20_000));
		// Twist: borrowers collateral is been vanished.
		// Now it is not possible to liquidate this position.
		crate::AccountCollateral::<Runtime>::remove(market_id, borrower_with_a_twist);
		// Emulate situation when collateral price has fallen down
		// from 50_000 USDT to 38_000 USDT.
		set_price(BTC::ID, NORMALIZED::units(38_000));
		assert_extrinsic_event::<Runtime>(
			Lending::liquidate(
				RuntimeOrigin::signed(manager),
				market_id.clone(),
				TestBoundedVec::try_from(vec![normal_borrower, borrower_with_a_twist]).unwrap(),
			),
			RuntimeEvent::Lending(crate::Event::LiquidationInitiated {
				market_id,
				borrowers: vec![normal_borrower],
			}),
		);
		// Check if cleanup was done correctly
		assert!(crate::DebtIndex::<Runtime>::contains_key(market_id, borrower_with_a_twist));
		assert!(crate::BorrowTimestamp::<Runtime>::contains_key(market_id, borrower_with_a_twist));
		assert!(!crate::DebtIndex::<Runtime>::contains_key(market_id, normal_borrower));
		assert!(!crate::BorrowTimestamp::<Runtime>::contains_key(market_id, normal_borrower));
	})
}

#[test]
fn liquidation() {
	new_test_ext().execute_with(|| {
		let (market_id, vault) = create_market::<Runtime, 50_000>(
			USDT::instance(),
			BTC::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_rational(2_u128, 1_u128),
		);

		let collateral = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(
				RuntimeOrigin::signed(*ALICE),
				market_id,
				collateral,
				false,
			),
			RuntimeEvent::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id,
			}),
		);

		let usdt_amt = 2 * DEFAULT_COLLATERAL_FACTOR * USDT::ONE * get_price(BTC::ID, collateral) /
			get_price(NORMALIZED::ID, NORMALIZED::ONE);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(*CHARLIE), vault, usdt_amt));

		// Allow the market to initialize it's account by withdrawing
		// from the vault
		process_and_progress_blocks::<Lending, Runtime>(1);

		let borrow_limit = Lending::get_borrow_limit(&market_id, &ALICE).expect("impossible");
		assert!(borrow_limit > 0);

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market_id, borrow_limit),
			RuntimeEvent::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow_limit,
				market_id,
			}),
		);

		process_and_progress_blocks::<Lending, Runtime>(10_000);

		assert_extrinsic_event::<Runtime>(
			Lending::liquidate(
				RuntimeOrigin::signed(*ALICE),
				market_id.clone(),
				TestBoundedVec::try_from(vec![*ALICE]).unwrap(),
			),
			RuntimeEvent::Lending(crate::Event::LiquidationInitiated {
				market_id,
				borrowers: vec![*ALICE],
			}),
		);
		// Check if cleanup was done correctly
		assert!(!crate::DebtIndex::<Runtime>::contains_key(market_id, *ALICE));
		assert!(!crate::BorrowTimestamp::<Runtime>::contains_key(market_id, *ALICE));
	});
}

#[test]
fn test_warn_soon_under_collateralized() {
	new_test_ext().execute_with(|| {
		const NORMALIZED_UNITS: u128 = 50_000;
		let (market, vault) = create_market::<Runtime, NORMALIZED_UNITS>(
			USDT::instance(),
			BTC::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_rational(2_u128, 1_u128),
		);

		// dbg!(&Vault::vault_info(vault));
		let two_btc_amount = BTC::units(2);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, two_btc_amount));
		assert_ok!(Lending::deposit_collateral(
			RuntimeOrigin::signed(*ALICE),
			market,
			two_btc_amount,
			false
		));
		let event = RuntimeEvent::Lending(crate::Event::CollateralDeposited {
			sender: *ALICE,
			amount: two_btc_amount,
			market_id: market,
		});
		System::assert_last_event(event);

		let usdt_amt = USDT::units(100_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, usdt_amt));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(*CHARLIE), vault, usdt_amt));

		process_and_progress_blocks::<Lending, Runtime>(1);

		assert_eq!(Lending::get_borrow_limit(&market, &ALICE), Ok(50_000_000_000_000_000));

		let borrow_amount = USDT::units(80);

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, borrow_amount),
			RuntimeEvent::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow_amount,
				market_id: market,
			}),
		);

		process_and_progress_blocks::<Lending, Runtime>(10000);

		assert_eq!(Lending::soon_under_collateralized(&market, &ALICE), Ok(false));
		set_price(BTC::ID, NORMALIZED::units(85));
		assert_eq!(Lending::soon_under_collateralized(&market, &ALICE), Ok(true));
		assert_eq!(Lending::should_liquidate(&market, &ALICE), Ok(false));
	});
}

#[test]
// As part of HAL03
fn market_owner_cannot_retroactively_liquidate() {
	new_test_ext().execute_with(|| {
		let (market_id, vault) = create_simple_market();

		let collateral_amount = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &BOB, collateral_amount));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(
				RuntimeOrigin::signed(*BOB),
				market_id,
				collateral_amount,
				false,
			),
			RuntimeEvent::Lending(crate::Event::CollateralDeposited {
				sender: *BOB,
				amount: collateral_amount,
				market_id,
			}),
		);

		let borrow_asset_deposit = USDT::units(1_000_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(*CHARLIE), vault, borrow_asset_deposit));

		process_and_progress_blocks::<Lending, Runtime>(1);

		let limit_normalized = Lending::get_borrow_limit(&market_id, &BOB).unwrap();
		process_and_progress_blocks::<Lending, Runtime>(2);

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*BOB), market_id, limit_normalized),
			RuntimeEvent::Lending(crate::Event::Borrowed {
				sender: *BOB,
				amount: limit_normalized,
				market_id,
			}),
		);

		assert_eq!(Lending::should_liquidate(&market_id, &BOB), Ok(false));

		// Update collateral factor to big value
		let collateral_factor = MoreThanOneFixedU128::saturating_from_rational(2000_u128, 99_u128);
		let updatable = UpdateInput {
			collateral_factor,
			under_collateralized_warn_percent: Percent::from_float(1.1),
			liquidators: vec![],
			max_price_age: DEFAULT_MAX_PRICE_AGE,
		};
		// ALICE is the creator of the market.
		assert_noop!(
			Lending::update_market(RuntimeOrigin::signed(*ALICE), market_id, updatable),
			Error::<Runtime>::CannotIncreaseCollateralFactorOfOpenMarket
		);
		// if above update was succeeded BOB's loan would have to be liquidated.
		assert_eq!(Lending::should_liquidate(&market_id, &BOB), Ok(false));
	})
}
