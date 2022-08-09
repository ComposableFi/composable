use super::{create_test_loan, create_test_market, prelude::*};
use crate::currency::BTC;
use composable_traits::undercollateralized_loans::LoanInput;

#[test]
fn can_create_loan() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		let market_info = create_test_market();
		let market_account_id = market_info.config().account_id().clone();
		let loan_input = LoanInput {
			market_account_id,
			borrower_account_id: *BOB,
			principal: 1000,
			collateral: 5,
			payment_schedule: vec![("24-08-1991".to_string(), 100)],
		};
		assert_ok!(pallet_undercollateralized_loans::Pallet::<Runtime>::create_loan(
			origin, loan_input
		));
	})
}

#[test]
fn can_execute_loan_contract() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let borrower = *BOB;
		// Borrwer should have some collateral to deposit.
		orml_tokens::Pallet::<Runtime>::mint_into(BTC::ID, &borrower, BTC::units(10)).unwrap();
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
