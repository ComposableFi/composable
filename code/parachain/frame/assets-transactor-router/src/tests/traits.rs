use crate::mocks::{AccountId, ALICE, BOB};
use frame_support::{assert_noop, assert_ok};
use mocks::{new_test_ext, GovernanceRegistry, RuntimeOrigin, Test};
use sp_runtime::DispatchError;

use crate::*;

mod currency {
	use frame_support::traits::{
		tokens::currency::Currency, ExistenceRequirement, Imbalance, WithdrawReasons,
	};

	use super::*;

	mod total_balance {

		use super::*;

		#[test]
		fn should_return_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, imbalance);
				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value);
			});
		}
	}

	mod can_slash {
		use super::*;

		#[test]
		fn should_return_true_when_native_balance_is_available() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, imbalance);
				assert!(Pallet::<Test>::can_slash(&ALICE, 512));
			})
		}
	}

	mod totatl_issuance {
		use super::*;

		#[test]
		fn should_return_total_issuance_of_native_asset() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, imbalance);
				assert_eq!(Pallet::<Test>::total_issuance(), value);
			})
		}
	}

	mod minimum_balance {
		use crate::mocks::MINIMUM_BALANCE;

		use super::*;

		#[test]
		fn should_return_minimum_balance_of_native_asset() {
			new_test_ext().execute_with(|| {
				assert_eq!(Pallet::<Test>::minimum_balance(), MINIMUM_BALANCE);
			})
		}
	}

	mod burn {
		use super::*;

		#[test]
		fn should_burn_from_native_issuance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let burn_value = 512;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				let burn_imbalance = Pallet::<Test>::burn(burn_value);
				assert_ok!(Pallet::<Test>::settle(
					&ALICE,
					burn_imbalance,
					WithdrawReasons::all(),
					ExistenceRequirement::KeepAlive,
				));

				assert_eq!(Pallet::<Test>::total_issuance(), value - burn_value)
			})
		}
	}

	mod issue {
		use super::*;

		#[test]
		fn should_issue_into_native_issuance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				assert_eq!(Pallet::<Test>::total_issuance(), value);
			})
		}
	}

	mod free_balance {
		use super::*;

		#[test]
		fn should_return_native_free_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				assert_eq!(Pallet::<Test>::free_balance(&ALICE), value);
			})
		}
	}

	mod ensure_can_withdraw {
		use super::*;

		#[test]
		fn should_return_true_when_account_has_enough_native_asset() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				assert_ok!(Pallet::<Test>::ensure_can_withdraw(
					&ALICE,
					512,
					WithdrawReasons::all(),
					0
				));
			})
		}
	}

	mod transfer {
		use super::*;

		#[test]
		fn should_transfer_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let transfer_value = 512;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				assert_ok!(<Pallet::<Test> as Currency<AccountId>>::transfer(
					&ALICE,
					&BOB,
					transfer_value,
					ExistenceRequirement::KeepAlive,
				));

				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value - transfer_value);
				assert_eq!(Pallet::<Test>::total_balance(&BOB), transfer_value);
			})
		}
	}

	mod slash {
		use super::*;

		#[test]
		fn should_slash_account_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let slash_value = 512;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				let _slash_imbalance = Pallet::<Test>::slash(&ALICE, slash_value);

				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value - slash_value);
			})
		}
	}

	mod deposit_into_existing {
		use super::*;

		#[test]
		fn should_deposit_into_account_with_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let deposit_value = 512;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				assert_ok!(Pallet::<Test>::deposit_into_existing(&ALICE, deposit_value));

				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value + deposit_value);
			})
		}
	}

	mod deposit_creating {
		use super::*;

		#[test]
		fn should_deposit_into_account_with_no_native_balance() {
			new_test_ext().execute_with(|| {
				let deposit_value = 512;
				assert_eq!(Pallet::<Test>::total_balance(&ALICE), 0);
				let _imbalance = Pallet::<Test>::deposit_creating(&ALICE, deposit_value);

				assert_eq!(Pallet::<Test>::total_balance(&ALICE), deposit_value);
			})
		}
	}

	mod withdraw {
		use super::*;

		#[test]
		fn should_withdraw_from_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let withdraw_value = 512;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				assert_ok!(Pallet::<Test>::withdraw(
					&ALICE,
					withdraw_value,
					WithdrawReasons::all(),
					ExistenceRequirement::KeepAlive
				));

				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value - withdraw_value);
			})
		}
	}

	mod make_free_balance_be {
		use super::*;

		#[test]
		fn should_make_free_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				Pallet::<Test>::make_free_balance_be(&ALICE, value);

				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value);
			})
		}
	}
}
