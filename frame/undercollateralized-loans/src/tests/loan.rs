use super::{create_test_loan, create_test_loan_input_config, parse_timestamp, prelude::*};
use crate::{
	currency::{BTC, USDT},
	validation::LoanInputIsValid,
};
use composable_traits::undercollateralized_loans::{DelayedPaymentTreatment, LoanInput};

#[test]
fn can_create_loan() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		let loan_input = create_test_loan_input_config(*BOB);
		// Check if loan was created successfully.
		assert_ok!(UndercollateralizedLoans::create_loan(origin, loan_input));
		// Check if corresponded event was emitted.
		let loan_account_id = UndercollateralizedLoans::loan_account_id(1);
		let event = crate::Event::LoanCreated { loan_account_id };
		System::assert_has_event(Event::UndercollateralizedLoans(event));
		// Check if loan's info was added to the storage.
		assert_eq!(
			*crate::LoansStorage::<Runtime>::get(loan_account_id)
				.unwrap()
				.config()
				.account_id(),
			loan_account_id
		);
		// Check if loan was marked as non-active.
		assert!(crate::NonActiveLoansStorage::<Runtime>::contains_key(loan_account_id));
		// Check if loans counter has adequate value.
		assert_eq!(crate::LoansCounterStorage::<Runtime>::get(), 1)
	})
}

// Check if input loan config validation works properly.
#[test]
fn test_do_create_market_input_validation() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let borrower = *BOB;
		// This method creates market as well.
		let valid_loan_input_configuration = create_test_loan_input_config(borrower);
		// Original market input configuration is valid.
		assert_ok!(valid_loan_input_configuration
			.clone()
			.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>());

		// Check non-whitelisted borrowers validation.
		let invalid_loan_input_configuration =
			LoanInput { borrower_account_id: *ALICE, ..valid_loan_input_configuration.clone() };
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Mentioned borrower is not included in the market's white-list of borrowers."
		);

		// Check blacklisted borrower validation.
		// Add loan's borrower *BOB to the blacklist.
		crate::BlackListPerMakretStorage::<Runtime>::mutate(
			valid_loan_input_configuration.market_account_id,
			|set| set.insert(borrower),
		);
		assert_err!(
			valid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Mentioned borrower is presented in the market's blacklist of borrowers."
		);
		// Clean blacklist.
		crate::BlackListPerMakretStorage::<Runtime>::mutate(
			valid_loan_input_configuration.market_account_id,
			|set| set.clear(),
		);

		// Check payment schedule length validation.
		// Create empty payment schedule.
		let payment_schedule = BTreeMap::new();
		let invalid_loan_input_configuration =
			LoanInput { payment_schedule, ..valid_loan_input_configuration.clone() };
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Payment schedule is empty."
		);
		// Create large payment schedule.
		let payment_schedule = (0..<ScheduleBound as Get<u32>>::get() + 1)
			.map(|num| {
				(valid_loan_input_configuration.activation_date + (num as i64) * 3600 * 24, 100)
			})
			.collect();
		let invalid_loan_input_configuration =
			LoanInput { payment_schedule, ..valid_loan_input_configuration.clone() };
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Payment schedule exceeded maximum size."
		);

		// Check activation date value validation.
		// Set first payment date less than activation date.
		let activation_date = parse_timestamp("02-03-2222");
		let first_payment_date = parse_timestamp("01-03-2222");
		let mut payment_schedule = BTreeMap::new();
		payment_schedule.insert(first_payment_date, 100);
		let invalid_loan_input_configuration = LoanInput {
			payment_schedule,
			activation_date,
			..valid_loan_input_configuration.clone()
		};
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Contract first date payment is less than activation date."
		);

		// Check validation of delayed payment treatment input.
		let invalid_delayed_payment_treatement = Some(DelayedPaymentTreatment {
			delayed_payments_shift_in_days: 1,
			delayed_payments_threshold: 0,
		});
		let invalid_loan_input_configuration = LoanInput {
			delayed_payment_treatment: invalid_delayed_payment_treatement,
			..valid_loan_input_configuration.clone()
		};
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Delayed payments threshold equals zero."
		);
		let invalid_delayed_payment_treatement = Some(DelayedPaymentTreatment {
			delayed_payments_shift_in_days: 0,
			delayed_payments_threshold: 3,
		});
		let invalid_loan_input_configuration = LoanInput {
			delayed_payment_treatment: invalid_delayed_payment_treatement,
			..valid_loan_input_configuration.clone()
		};
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Delayed payments shift equals zero."
		);
		let invalid_delayed_payment_treatement = Some(DelayedPaymentTreatment {
			delayed_payments_shift_in_days: <MaxDateShiftingInDays as Get<i64>>::get() + 1,
			delayed_payments_threshold: 3,
		});
		let invalid_loan_input_configuration = LoanInput {
			delayed_payment_treatment: invalid_delayed_payment_treatement,
			..valid_loan_input_configuration.clone()
		};
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Maximum date shifting exceeded."
		);
	});
}

// TODO: @mikolaichuk: move to the borrow.rs
#[test]
fn authoriezed_borrower_can_execute_loan_contract() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let borrower = *BOB;
		// Borrower should have some collateral to deposit.
		Tokens::mint_into(BTC::ID, &borrower, BTC::units(10)).unwrap();
		let origin = Origin::signed(borrower);
		let loan_config = create_test_loan(borrower);
		let loan_account_id = *loan_config.account_id();
		assert_ok!(UndercollateralizedLoans::borrow(origin, loan_account_id.clone(), true));
		// Check if corresponded event was emitted.
		let event = crate::Event::LoanContractExecuted { loan_account_id };
		System::assert_has_event(Event::UndercollateralizedLoans(event));
		// Check if loan's account id was removed from non-active loans list.
		assert!(!crate::NonActiveLoansStorage::<Runtime>::contains_key(loan_account_id));
	});
}

#[test]
fn can_not_activate_improper_loan() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let borrower = *BOB;
		// Borrower should have some collateral to deposit.
		Tokens::mint_into(BTC::ID, &borrower, BTC::units(10)).unwrap();
		let origin = Origin::signed(borrower);
		// Create loan.
		let loan_info = create_test_loan(borrower);
		// Create fake loan account id.
		let fake_loan_account_id = UndercollateralizedLoans::loan_account_id(42);
		// Try to activate fake loan.
		assert_err!(
			UndercollateralizedLoans::borrow(origin.clone(), fake_loan_account_id, true),
			crate::Error::<Runtime>::LoanDoesNotExistOrWasActivated
		);
		// Activate real loan.
		let real_loan_account_id = loan_info.account_id();
		assert_ok!(UndercollateralizedLoans::borrow(
			origin.clone(),
			real_loan_account_id.clone(),
			true
		));
		// Try to activate real loan once again.
		assert_err!(
			UndercollateralizedLoans::borrow(origin.clone(), real_loan_account_id.clone(), true),
			crate::Error::<Runtime>::LoanDoesNotExistOrWasActivated
		);
		// Create loan once again.
		let loan_config = create_test_loan(borrower);
		let loan_account_id = loan_config.account_id();
		// Add borrower to the blacklist after the loan was created.
		crate::BlackListPerMakretStorage::<Runtime>::mutate(
			loan_config.market_account_id(),
			|set| set.insert(borrower),
		);
		// Try to activate loan.
		assert_err!(
			UndercollateralizedLoans::borrow(origin.clone(), loan_account_id.clone(), true),
			crate::Error::<Runtime>::BlacklistedBorrowerAccount
		);
		let wrong_borrower = *ALICE;
		let wrong_origin = Origin::signed(wrong_borrower);
		// Try to activate loan with non-authorized borrower.
		assert_err!(
			UndercollateralizedLoans::borrow(wrong_origin.clone(), loan_account_id.clone(), true),
			crate::Error::<Runtime>::NonAuthorizedToExecuteContract,
		);
	});
}

#[test]
fn anybody_can_repay() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let payer = *ALICE;
		let payer_origin = Origin::signed(payer);
		let borrower = *BOB;
        let borrower_origin = Origin::signed(borrower);
		let repay_amount = 100;
		// Borrower should have some collateral to deposit.
		Tokens::mint_into(BTC::ID, &borrower, BTC::units(10)).unwrap();
		// Payer should have something to repay.
		Tokens::mint_into(USDT::ID, &payer, USDT::units(repay_amount)).unwrap();
		let loan_config = create_test_loan(*BOB);
		let loan_account_id = loan_config.account_id().clone();
	    // Activate the loan. Otherwise we have nothing to be repaid.
        assert_ok!(UndercollateralizedLoans::borrow(borrower_origin, loan_account_id.clone(), true));
		assert_ok!(UndercollateralizedLoans::repay(payer_origin, loan_account_id, repay_amount, true));
		let event = crate::Event::<Runtime>::SomeAmountRepaid { loan_account_id, repay_amount };
		System::assert_has_event(Event::UndercollateralizedLoans(event));
	});
}

#[test]
fn does_not_process_wrong_repayments() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let payer = *ALICE;
		let origin = Origin::signed(payer);
		let borrower = *BOB;
		let repay_amount = 100;
		// Borrower should have some collateral to deposit.
		Tokens::mint_into(BTC::ID, &borrower, BTC::units(10)).unwrap();
		// Payer should have something to repay.
		Tokens::mint_into(USDT::ID, &payer, USDT::units(repay_amount)).unwrap();
		let loan_config = create_test_loan(*BOB);
		let loan_account_id = loan_config.account_id().clone();
       // Try to repay non-activated loan.	
        assert_err!(
			UndercollateralizedLoans::repay(origin.clone(), loan_account_id, repay_amount, true),
			crate::Error::<Runtime>::LoanIsNotActive
		);
		let fake_loan_account_id = UndercollateralizedLoans::loan_account_id(42);
	    // Try to repay fake loan.	
        assert_err!(
			UndercollateralizedLoans::repay(origin, fake_loan_account_id, repay_amount, true),
			crate::Error::<Runtime>::LoanNotFound
		);
	});
}
