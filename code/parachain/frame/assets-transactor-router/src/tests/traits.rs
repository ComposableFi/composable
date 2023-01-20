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

mod lockable_currency {
	use frame_support::traits::{
		tokens::currency::{Currency, LockIdentifier, LockableCurrency},
		ExistenceRequirement, WithdrawReasons,
	};

	use super::*;

	const TEST_LOCK_ID: LockIdentifier = *b"unittest";

	mod set_lock {
		use super::*;

		#[test]
		fn should_prevent_locked_native_balance_transfer() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let locked_value = 1024;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				Pallet::<Test>::set_lock(
					TEST_LOCK_ID,
					&ALICE,
					locked_value,
					WithdrawReasons::all(),
				);

				assert_noop!(
					<Pallet<Test> as Currency<AccountId>>::transfer(
						&ALICE,
						&BOB,
						locked_value + 1,
						ExistenceRequirement::KeepAlive,
					),
					pallet_balances::pallet::Error::<Test>::LiquidityRestrictions
				);
				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value);
			})
		}
	}

	mod extend_lock {
		use super::*;

		#[test]
		fn should_extend_lock_on_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let locked_value = 1024;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				Pallet::<Test>::set_lock(
					TEST_LOCK_ID,
					&ALICE,
					locked_value,
					WithdrawReasons::all(),
				);

				assert_ok!(<Pallet<Test> as Currency<AccountId>>::transfer(
					&ALICE,
					&BOB,
					locked_value,
					ExistenceRequirement::KeepAlive,
				));

				let issue_imbalance = Pallet::<Test>::issue(locked_value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);
				Pallet::<Test>::extend_lock(TEST_LOCK_ID, &ALICE, value, WithdrawReasons::all());

				assert_noop!(
					<Pallet<Test> as Currency<AccountId>>::transfer(
						&ALICE,
						&BOB,
						locked_value,
						ExistenceRequirement::KeepAlive,
					),
					pallet_balances::pallet::Error::<Test>::LiquidityRestrictions
				);
			})
		}
	}

	mod remove_lock {
		use super::*;

		#[test]
		fn should_remove_lock_on_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let locked_value = 1024;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				Pallet::<Test>::set_lock(
					TEST_LOCK_ID,
					&ALICE,
					locked_value,
					WithdrawReasons::all(),
				);

				assert_noop!(
					<Pallet<Test> as Currency<AccountId>>::transfer(
						&ALICE,
						&BOB,
						locked_value + 1,
						ExistenceRequirement::KeepAlive,
					),
					pallet_balances::pallet::Error::<Test>::LiquidityRestrictions
				);

				Pallet::<Test>::remove_lock(TEST_LOCK_ID, &ALICE);

				assert_ok!(<Pallet<Test> as Currency<AccountId>>::transfer(
					&ALICE,
					&BOB,
					locked_value + 1,
					ExistenceRequirement::KeepAlive,
				),);
			})
		}
	}
}

mod reservable_currency {
	use frame_support::traits::{
		tokens::{
			currency::{Currency, ReservableCurrency},
			imbalance::Imbalance,
		},
		ExistenceRequirement,
	};

	use super::*;

	mod can_reserve {
		use super::*;

		#[test]
		fn should_return_true_when_native_balance_is_reservable() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let reserved_value = 1024;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);

				assert!(Pallet::<Test>::can_reserve(&ALICE, reserved_value))
			})
		}
	}

	mod slash_reserved {
		use super::*;

		#[test]
		fn should_slash_reserved_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let slash_value = 512;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);
				assert_ok!(Pallet::<Test>::reserve(&ALICE, value));

				assert_eq!(
					Pallet::<Test>::slash_reserved(&ALICE, slash_value).0.peek(),
					slash_value
				);
				assert_eq!(Pallet::<Test>::reserved_balance(&ALICE), value - slash_value);
			})
		}
	}

	mod reserved_balance {
		use super::*;

		#[test]
		fn should_return_reserved_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let reserved_value = 512;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);
				assert_ok!(Pallet::<Test>::reserve(&ALICE, reserved_value));

				assert_eq!(Pallet::<Test>::reserved_balance(&ALICE), reserved_value);
			})
		}
	}

	mod reserve {
		use super::*;

		#[test]
		fn should_reserve_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let reserved_value = 1024;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);
				assert_ok!(Pallet::<Test>::reserve(&ALICE, reserved_value));

				assert_eq!(Pallet::<Test>::reserved_balance(&ALICE), reserved_value);

				assert_noop!(
					<Pallet<Test> as Currency<AccountId>>::transfer(
						&ALICE,
						&BOB,
						reserved_value + 1,
						ExistenceRequirement::KeepAlive,
					),
					pallet_balances::pallet::Error::<Test>::InsufficientBalance
				);
			})
		}
	}

	mod unreserve {
		use super::*;

		#[test]
		fn should_unreserve_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let reserved_value = 1024;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);
				assert_ok!(Pallet::<Test>::reserve(&ALICE, reserved_value));

				assert_eq!(Pallet::<Test>::reserved_balance(&ALICE), reserved_value);

				assert_noop!(
					<Pallet<Test> as Currency<AccountId>>::transfer(
						&ALICE,
						&BOB,
						reserved_value + 1,
						ExistenceRequirement::KeepAlive,
					),
					pallet_balances::pallet::Error::<Test>::InsufficientBalance
				);

				assert_eq!(Pallet::<Test>::unreserve(&ALICE, reserved_value), 0);
				assert_eq!(Pallet::<Test>::reserved_balance(&ALICE), 0);

				assert_ok!(<Pallet<Test> as Currency<AccountId>>::transfer(
					&ALICE,
					&BOB,
					reserved_value + 1,
					ExistenceRequirement::KeepAlive,
				));
			})
		}
	}

	mod repatriate_reserved {
		use super::*;
		use frame_support::traits::tokens::BalanceStatus;

		#[test]
		fn should_move_native_balance() {
			new_test_ext().execute_with(|| {
				let value = 2048;
				let reserved_value = 2048;
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&ALICE, issue_imbalance);
				let issue_imbalance = Pallet::<Test>::issue(value);
				Pallet::<Test>::resolve_creating(&BOB, issue_imbalance);

				assert_ok!(Pallet::<Test>::reserve(&ALICE, reserved_value));

				assert_ok!(Pallet::<Test>::repatriate_reserved(
					&ALICE,
					&BOB,
					reserved_value,
					BalanceStatus::Free,
				));

				assert_eq!(Pallet::<Test>::total_balance(&ALICE), value - reserved_value);
				assert_eq!(Pallet::<Test>::total_balance(&BOB), value + reserved_value);
			})
		}
	}
}
