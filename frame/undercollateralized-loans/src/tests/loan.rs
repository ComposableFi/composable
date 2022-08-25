use super::{
	create_test_loan, create_test_loan_input_config, create_test_market, parse_timestamp,
	prelude::*,
};
use crate::{currency::BTC, validation::LoanInputIsValid};
use composable_traits::undercollateralized_loans::LoanInput;

#[test]
fn can_create_loan() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		let loan_input = create_test_loan_input_config();
		// Check if loan was created successfully.
		assert_ok!(pallet_undercollateralized_loans::Pallet::<Runtime>::create_loan(
			origin, loan_input
		));
		// Check if corresponded event was emitted.
		let loan_account_id = UndercollateralizedLoans::loan_account_id(1);
		let event = crate::Event::LoanCreated { loan_account_id };
		System::assert_has_event(Event::UndercollateralizedLoans(event));
		// Check if loan's info and config were added to the storage.
		assert_eq!(
			*crate::LoansStorage::<Runtime>::get(loan_account_id)
				.unwrap()
				.config()
				.account_id(),
			loan_account_id
		);
		// Check if loans counter has adequate value.
		assert_eq!(crate::LoansCounterStorage::<Runtime>::get(), 1)
	})
}

// Check if input loan config validation works properly.
#[test]
fn test_do_create_market_input_validation() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// This method creates market as well.
		// Borrower: *BOB,
		let valid_loan_input_configuration = create_test_loan_input_config();
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
			|set| set.insert(*BOB),
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
        
        // Check activation date value correctness validation. 
        // Set first payment date less than activation date. 
        let activation_date = parse_timestamp("02-03-2222");
        let first_payment_date = parse_timestamp("01-03-2222");
        let mut payment_schedule = BTreeMap::new();
        payment_schedule.insert(first_payment_date, 100);
        let invalid_loan_input_configuration = LoanInput{payment_schedule, activation_date, ..valid_loan_input_configuration};
		assert_err!(
			invalid_loan_input_configuration
				.clone()
				.try_into_validated::<LoanInputIsValid<UndercollateralizedLoans>>(),
			"Contract first date payment is less than activation date."
		);

	});
}

#[test]
fn can_execute_loan_contract() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let borrower = *BOB;
		// Borrower should have some collateral to deposit.
		Tokens::mint_into(BTC::ID, &borrower, BTC::units(10)).unwrap();
		let origin = Origin::signed(borrower);
		let loan_config = create_test_loan();
		let loan_account_id = *loan_config.account_id();
		assert_ok!(pallet_undercollateralized_loans::Pallet::<Runtime>::borrow(
			origin,
			loan_account_id,
			true
		));
	});
}
