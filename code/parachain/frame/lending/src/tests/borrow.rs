use super::prelude::*;
use crate::{models::borrower_data::BorrowerData, tests::process_and_progress_blocks};
use composable_traits::defi::LiftedFixedBalance;

#[test]
fn test_borrow_repay_in_same_block() {
	new_test_ext().execute_with(|| {
		let (market_id, vault) = create_simple_market();
		let initial_total_cash = Lending::total_available_to_be_borrowed(&market_id).unwrap();

		let collateral_amount = BTC::units(100);
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral_amount));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(
				RuntimeOrigin::signed(*ALICE),
				market_id,
				collateral_amount,
				false,
			),
			RuntimeEvent::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral_amount,
				market_id,
			}),
		);

		let borrow_asset_deposit = USDT::units(1_000_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(*CHARLIE), vault, borrow_asset_deposit));
		let mut total_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_asset_deposit) + initial_total_cash;

		process_and_progress_blocks::<Lending, Runtime>(1);

		let limit_normalized = Lending::get_borrow_limit(&market_id, &ALICE).unwrap();
		assert_eq!(Lending::total_available_to_be_borrowed(&market_id), Ok(total_cash));
		process_and_progress_blocks::<Lending, Runtime>(1); // <- ???

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market_id, limit_normalized / 4),
			RuntimeEvent::Lending(crate::Event::Borrowed {
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
				RuntimeOrigin::signed(*ALICE),
				market_id,
				*ALICE,
				RepayStrategy::PartialAmount(alice_repay_amount),
				false,
			),
			Error::<Runtime>::BorrowAndRepayInSameBlockIsNotSupported,
		);

		assert_no_event::<Runtime>(RuntimeEvent::Lending(crate::Event::BorrowRepaid {
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

		let (market, vault) = create_market::<Runtime, FIRST_PRICE>(
			USDT::instance(),
			BTC::instance(),
			*ALICE,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_integer(DEFAULT_COLLATERAL_FACTOR),
		);

		// Borrow amount
		let borrow_amount = USDT::units(BORROW_AMOUNT);

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, borrow_amount));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(*ALICE), vault, borrow_amount * 2));

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(FIRST_PRICE));

		let collateral_amount = get_price(USDT::ID, borrow_amount) // get price of USDT
			.mul(BTC::ONE) // multiply to BTC ONE
			.div(get_price(BTC::ID, BTC::ONE)) // divide at one BTC price
			.mul(DEFAULT_COLLATERAL_FACTOR);

		// Mint BTC tokens for ALICE
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, collateral_amount));

		// Deposit BTC on the market
		assert_ok!(Lending::deposit_collateral(
			RuntimeOrigin::signed(*ALICE),
			market,
			collateral_amount,
			false
		));

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(SECOND_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// Try to borrow by SECOND_PRICE
		assert_noop!(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, borrow_amount),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(FIRST_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// skip blocks
		process_and_progress_blocks::<Lending, Runtime>(DEFAULT_MAX_PRICE_AGE as usize + 1);

		// Try to borrow by SECOND_PRICE
		assert_noop!(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, borrow_amount),
			Error::<Runtime>::PriceTooOld
		);

		// Refresh price
		set_price(BTC::ID, BTC::ONE.mul(FIRST_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// Try to borrow by FIRST_PRICE
		assert_ok!(Lending::borrow(RuntimeOrigin::signed(*ALICE), market, borrow_amount),);

		// skip blocks
		process_and_progress_blocks::<Lending, Runtime>(DEFAULT_MAX_PRICE_AGE as usize + 1);

		// Set BTC price
		set_price(BTC::ID, BTC::ONE.mul(SECOND_PRICE));
		set_price(USDT::ID, USDT::ONE);

		// Try to borrow by SECOND_PRICE
		assert_noop!(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, borrow_amount),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		// skip blocks
		process_and_progress_blocks::<Lending, Runtime>(DEFAULT_MAX_PRICE_AGE as usize + 1);

		// Try to repay by SECOND_PRICE
		assert_ok!(Lending::repay_borrow(
			RuntimeOrigin::signed(*ALICE),
			market,
			*ALICE,
			RepayStrategy::PartialAmount(borrow_amount),
			false,
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
		assert_ok!(Lending::deposit_collateral(
			RuntimeOrigin::signed(*ALICE),
			market,
			collateral_amount,
			false
		));
		let event = RuntimeEvent::Lending(crate::Event::CollateralDeposited {
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
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(*CHARLIE), vault, borrow_amount));

		process_and_progress_blocks::<Lending, Runtime>(1);

		let expected_cash =
			DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(borrow_amount) + initial_total_cash;
		assert_eq!(Lending::total_available_to_be_borrowed(&market), Ok(expected_cash));

		let alice_borrow = borrow_amount / DEFAULT_COLLATERAL_FACTOR / 10;

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, alice_borrow),
			RuntimeEvent::Lending(crate::Event::Borrowed {
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
		process_and_progress_blocks::<Lending, Runtime>(49);
		let interest_after = Lending::total_interest(&market).unwrap();
		assert!(interest_before < interest_after);

		let limit_normalized = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		let new_limit = limit_normalized * USDT::ONE / get_price(USDT::ID, USDT::ONE);

		assert!(new_limit < original_limit);

		let borrow = Lending::total_debt_with_interest(&market, &ALICE).unwrap().unwrap_or_zero();

		assert!(borrow > alice_borrow);
		assert_noop!(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, original_limit),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		assert_no_event::<Runtime>(RuntimeEvent::Lending(crate::Event::Borrowed {
			sender: *ALICE,
			amount: original_limit,
			market_id: market,
		}));

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, new_limit),
			RuntimeEvent::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: new_limit,
				market_id: market,
			}),
		);

		assert_noop!(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, USDT::ONE),
			Error::<Runtime>::InvalidTimestampOnBorrowRequest
		);

		assert_no_event::<Runtime>(RuntimeEvent::Lending(crate::Event::Borrowed {
			sender: *ALICE,
			amount: USDT::ONE,
			market_id: market,
		}));

		process_and_progress_blocks::<Lending, Runtime>(20);

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral_amount));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(RuntimeOrigin::signed(*ALICE), market, collateral_amount, false),
			RuntimeEvent::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral_amount,
				market_id: market,
			}),
		);

		let alice_limit = Lending::get_borrow_limit(&market, &ALICE).unwrap();
		assert!(get_price(BTC::ID, collateral_amount) > alice_limit);

		assert_noop!(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, alice_limit * 100),
			Error::<Runtime>::NotEnoughCollateralToBorrow
		);

		assert_no_event::<Runtime>(RuntimeEvent::Lending(crate::Event::Borrowed {
			sender: *ALICE,
			amount: alice_limit * 100,
			market_id: market,
		}));

		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market, 10),
			RuntimeEvent::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: 10,
				market_id: market,
			}),
		);
	});
}

#[test]
fn zero_amount_collateral_deposit_or_withdraw() {
	new_test_ext().execute_with(|| {
		let (market, _vault) = create_simple_market();
		let collateral_amount = 0;
		let error_message = "Can not deposit or withdraw zero collateral";
		assert_noop!(
			Lending::deposit_collateral(RuntimeOrigin::signed(*ALICE), market, collateral_amount, false),
			error_message
		);

		assert_noop!(
			Lending::withdraw_collateral(RuntimeOrigin::signed(*ALICE), market, collateral_amount,),
			error_message
		);
	})
}

prop_compose! {
	fn valid_cash_borrow()(cash in 1..u32::MAX)(borrow in 0..cash, cash in Just(cash))
		-> (u32, u32) {
			(cash, borrow)
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
	fn calc_utilization_ratio_proptest((cash, borrow) in valid_cash_borrow()) {
		new_test_ext().execute_with(|| {
			prop_assert_eq!(
				Lending::calculate_utilization_ratio(cash.into(), borrow.into()).unwrap(),
				Percent::from_float(borrow as f64 / (cash as f64 + borrow as f64))
			);
			Ok(())
		})?;
	}


}
