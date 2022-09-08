//! Functions testing if we have correctly forwarding calls to the right pallet implementation.

use crate::*;
use composable_tests_helpers::prop_assert_ok;
use mocks::{
	new_test_ext, new_test_ext_multi_currency, AccountId, AssetId, Balance, Test,
	ACCOUNT_FREE_START, ASSET_FREE_START, BALANCES, MINIMUM_BALANCE,
};
use proptest::prelude::*;

prop_compose! {
	fn valid_amounts_without_overflow_k(max_accounts: usize, limit: Balance)
		(balances in prop::collection::vec(MINIMUM_BALANCE..limit, 3..max_accounts))
		 -> Vec<(AccountId, Balance)> {
			(ACCOUNT_FREE_START..balances.len() as AccountId)
				.zip(balances)
				.collect()
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_1()
		(x in MINIMUM_BALANCE..Balance::MAX) -> Balance {
		x
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_2()
		(x in MINIMUM_BALANCE..Balance::MAX / 2,
		 y in MINIMUM_BALANCE..Balance::MAX / 2) -> (Balance, Balance) {
			(x, y)
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_3()
		(x in (MINIMUM_BALANCE..(Balance::MAX / 3) - 10) ,
		 y in (MINIMUM_BALANCE..(Balance::MAX / 3) - 10) ,
		 z in (MINIMUM_BALANCE..(Balance::MAX / 3) - 10) ) -> (Balance, Balance, Balance) {
			(x, y, z)
		}
}

prop_compose! {
	fn accounts()
		(x in ACCOUNT_FREE_START..AccountId::MAX) -> AccountId {
			x
		}
}

prop_compose! {
	fn accounts_2()
		(x in ACCOUNT_FREE_START..AccountId::MAX / 2, y in (AccountId::MAX/2)+1..AccountId::MAX) -> (AccountId, AccountId) {
			(x, y)
		}
}

prop_compose! {
	fn asset()
		(x in ASSET_FREE_START..AssetId::MAX) -> AssetId {
			x
		}
}

mod currency {
	use super::*;
	use frame_support::traits::{
		tokens::{currency::Currency, Imbalance},
		ExistenceRequirement, WithdrawReasons,
	};
	proptest! {
			#![proptest_config(ProptestConfig::with_cases(10000))]

			/// Covers all the methods from the currency trait.
	#[test]
	fn test_trait_implementation(account in accounts(),
				(first, second, third) in valid_amounts_without_overflow_3()
			) {

			   new_test_ext().execute_with(|| {

				macro_rules! assert_issuance {
				($val:expr) => {
					let issuance = BALANCES.iter().fold(0, | sum, (_, val)| val + sum);
					prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_issuance(),issuance + $val);
				   }
				 }

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::minimum_balance(), MINIMUM_BALANCE);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), 0);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), 0);
				assert_issuance!(0);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account,first).peek(), first);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), first);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), first);
				assert_issuance!(first);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_into_existing(&account,
				second).unwrap().peek(), second);
				prop_assert_eq!(<Pallet::<Test> as
				Currency<AccountId>>::total_balance(&account), first + second);
				prop_assert_eq!(<Pallet::<Test>
				as Currency<AccountId>>::free_balance(&account), first + second);
				assert_issuance!(first + second);

				prop_assert!(<Pallet::<Test> as Currency<AccountId>>::can_slash(&account, first + second));
				let (_, difference) = <Pallet::<Test> as Currency<AccountId>>::slash(&account, third);
				let balance = if first + second > third {
					prop_assert_eq!(difference, 0);
					let balance = (first + second) - third;
					prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), balance);
					assert_issuance!(balance);
					balance
				} else {
					prop_assert_eq!(difference, third - (first + second));
					prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), 0);
					assert_issuance!(0);
					0
				};

				let issue = <Pallet::<Test> as Currency<AccountId>>::issue(second);
				let added = issue.peek();
				prop_assert_eq!(added, second);
				if balance == 0 {
					<Pallet::<Test> as Currency<AccountId>>::resolve_creating(&account, issue);
				} else {
					<Pallet::<Test> as Currency<AccountId>>::resolve_into_existing(&account, issue).unwrap();
				}
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), balance + added);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), balance + added );

				assert_issuance!(balance + added);

				prop_assert!(!<Pallet::<Test> as Currency<AccountId>>::ensure_can_withdraw(&account, balance + added, WithdrawReasons::TRANSFER, 0).is_err());
				prop_assert!( <Pallet::<Test> as Currency<AccountId>>::withdraw(&account, balance + added, WithdrawReasons::TRANSFER, ExistenceRequirement::KeepAlive).is_err());
				prop_assert!(<Pallet::<Test> as Currency<AccountId>>::withdraw(&account, balance + added,WithdrawReasons::TRANSFER, ExistenceRequirement::AllowDeath).is_ok() 				);
				<Pallet::<Test> as Currency<AccountId>>::make_free_balance_be(&account, first);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), first);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), first);

				assert_issuance!(first);

				let burned = <Pallet::<Test> as Currency<AccountId>>::burn(second);
				let diff = <Pallet::<Test> as Currency<AccountId>>::settle(&account, burned,WithdrawReasons::all(), ExistenceRequirement::AllowDeath);

				if second > first {
					prop_assert!(diff.is_err());
				} else {
					prop_assert_ok!(diff);
				}

				<Pallet::<Test> as Currency<AccountId>>::make_free_balance_be(&account, third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_balance(&account), third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account), third);

				let receiver = &(account + 1);
				prop_assert_ok!(<Pallet::<Test> as Currency<AccountId>>::transfer(&account, receiver, third,
				ExistenceRequirement::AllowDeath));
				prop_assert_eq!(<Pallet::<Test> as
				Currency<AccountId>>::total_balance(receiver), third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(receiver), third);
				Ok(())

			 })?;
		}
	}
}

mod reservable_currency {
	use super::*;
	use frame_support::traits::tokens::{
		currency::{Currency, ReservableCurrency},
		BalanceStatus, Imbalance,
	};

	macro_rules! assert_issuance {
		($val:expr) => {
			let issuance = BALANCES.iter().fold(0, |sum, (_, val)| val + sum);
			prop_assert_eq!(
				<Pallet::<Test> as Currency<AccountId>>::total_issuance(),
				issuance + $val
			);
		};
	}

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn test_can_reserve_implementation(
			account_1 in accounts(),
			(first, _, _) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::can_reserve(&account_1, first), false);
				assert_issuance!(0);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first).peek(), first);
				assert_issuance!(first);
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::can_reserve(&account_1, first), true);

				Ok(())
			})?;
		}

		#[test]
		fn test_reserve_implementation(
			account_1 in accounts(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first).peek(), first);
				assert_issuance!(first);
				//increase user balance
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_into_existing(&account_1, second).unwrap().peek(), second);
				assert_issuance!(first+second);
				//increase user balance
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_into_existing(&account_1, third).unwrap().peek(), third);
				assert_issuance!(first+second+third);
				//reserve
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second));
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), first+second);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), third);
				Ok(())
			})?;
		}

		#[test]
		fn test_slash_reserve_implementation(
			account_1 in accounts(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first+second+third).peek(), first+second+third);
				//reserve
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second));
				let total_issuance = <Pallet::<Test> as Currency<AccountId>>::total_issuance();
				//slash
				let (_, difference) = <Pallet::<Test> as ReservableCurrency<AccountId>>::slash_reserved(&account_1, third);
				let _balance = if  first + second > third {
					prop_assert_eq!(difference, 0);
					let balance = (first + second) - third;
					// check reserve balance after slash
					prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_issuance(),total_issuance-(third-difference));
					prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), balance);

					balance
				} else {
					prop_assert_eq!(difference, third - (first + second ));
					prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::total_issuance(),total_issuance-(third-difference));
					prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), 0);
					0
				};

				Ok(())
			})?;
		}

		#[test]
		fn test_repatriate_reserve_implementation(
			(account_1, account_2) in accounts_2(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first + second + third).peek(), first + second + third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_2, first).peek(), first);
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second+third));
				//repatriate to free balance
				let repatriate_free = <Pallet::<Test> as ReservableCurrency<AccountId>>::repatriate_reserved(&account_1, &account_2, second, BalanceStatus::Free).unwrap();
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_2),first + (second - repatriate_free));
				//repatriate to reserved balance
				let repatriate_reserved = <Pallet::<Test> as ReservableCurrency<AccountId>>::repatriate_reserved(&account_1, &account_2, third, BalanceStatus::Reserved).unwrap();
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_2), third - repatriate_reserved);
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), first + (repatriate_free + repatriate_reserved));

				Ok(())
			})?;
		}

		#[test]
		fn test_unreserve_implementation(
			account_1 in accounts(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::deposit_creating(&account_1, first + second + third).peek(), first + second + third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), first + second + third);
				prop_assert_ok!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserve(&account_1, first+second+third));
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), first + second + third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), 0);
				//repatriate to free balance
				let mut remaining = <Pallet::<Test> as ReservableCurrency<AccountId>>::unreserve(&account_1, third);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), third - remaining);
				let mut free_balance = <Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1);
				remaining = <Pallet::<Test> as ReservableCurrency<AccountId>>::unreserve(&account_1, second);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), free_balance + (second - remaining));

				free_balance = <Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1);
				remaining = <Pallet::<Test> as ReservableCurrency<AccountId>>::unreserve(&account_1, first);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), free_balance + (first - remaining));
				prop_assert_eq!(<Pallet::<Test> as ReservableCurrency<AccountId>>::reserved_balance(&account_1), 0);
				prop_assert_eq!(<Pallet::<Test> as Currency<AccountId>>::free_balance(&account_1), first + second + third);

				Ok(())
			})?;
		}
	}
}

mod multicurrency {
	use super::*;
	use frame_support::assert_err;
	use orml_traits::currency::{MultiCurrency, MultiLockableCurrency, MultiReservableCurrency};

	#[test]
	fn test_unknown_asset() {
		new_test_ext_multi_currency().execute_with(|| {
			// asset_id 0 is invalid
			assert_err!(
				<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(0, &1, 100),
				Error::<Test>::InvalidCurrency
			);
		});
	}

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn test_minimum_balance_implementation(
			_account in accounts(),
			asset_id in asset(),
			(_first, _second, _third) in valid_amounts_without_overflow_3()) {
			new_test_ext_multi_currency().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::minimum_balance(asset_id), 0);
				Ok(())
			})?;
		}

		#[test]
		fn test_total_balance_implementation(
			account in accounts(),
			asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()) {
			new_test_ext_multi_currency().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_balance(asset_id,&account), 0);
				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account, first));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_balance(asset_id,&account), first);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account, second));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_balance(asset_id,&account), first + second);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account, third));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_balance(asset_id,&account), first + second + third);
				Ok(())
			})?;
		}

		#[test]
		fn test_total_issuance_implementation(
			(account_1, account_2) in accounts_2(),
			asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()
			) {

			new_test_ext_multi_currency().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id), 0);
				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account_1, first));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id), first);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account_2, second));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id), first + second);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account_1, third));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id), first + second + third);
				Ok(())
			})?;
		}

		#[test]
		fn test_free_balance_implementation(
			account in accounts(),
			asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()
			) {

			new_test_ext_multi_currency().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account), 0);
				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account, first));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account), first);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account, second));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account), first + second);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account, third));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account), first + second + third);

				prop_assert_ok!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserve(asset_id, &account, first));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account), second + third);

				prop_assert_ok!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserve(asset_id, &account, second));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),  third);

				prop_assert_ok!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserve(asset_id, &account, third));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),  0);

				let remaining = <Pallet::<Test> as MultiReservableCurrency<AccountId>>::unreserve(asset_id, &account, first);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),  first - remaining);

				let free_balance = <Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account);
				let remaining = <Pallet::<Test> as MultiReservableCurrency<AccountId>>::unreserve(asset_id, &account, second);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),  free_balance + ( second - remaining));

				let free_balance = <Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account);
				let remaining = <Pallet::<Test> as MultiReservableCurrency<AccountId>>::unreserve(asset_id, &account, third);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),  free_balance + ( third - remaining));

				prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::set_lock(*b"prelocks", asset_id, &account, first));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id, &account),  first + second + third);

				Ok(())
			})?;
		}
	}
}

mod lockable_multicurrency {
	use super::*;
	use orml_traits::currency::{MultiCurrency, MultiLockableCurrency};

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

	   #[test]
	   fn test_set_lock_implementation(
			account in accounts(),
			asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()) {

			new_test_ext_multi_currency().execute_with(|| {

			 prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),0);
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account,first + second + third));

			 prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::set_lock(*b"prelocks", asset_id, &account, first));
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, second + third ));
			 prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second + third).is_err());

			 // ensure set_lock updates lock with same id
			 prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::set_lock(*b"prelocks", asset_id, &account,  second));
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account,  first + third ));
			 prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second + third).is_err());

			  // ensure set_lock updates lock with same id
			 prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::set_lock(*b"prelocks", asset_id, &account,  third));
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account,  first + second ));
			 prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second + third).is_err());

			Ok(())
		  })?;
	   }

		 #[test]
	   fn test_extend_lock_implementation(
			account in accounts(),
			asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()) {

			new_test_ext_multi_currency().execute_with(|| {

			 prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),0);
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account,first + second + third));

			 prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::set_lock(*b"prelocks", asset_id, &account, first));
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, second + third ));
			 prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second + third).is_err());

			 prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::extend_lock(*b"prelocks", asset_id, &account, second));
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, third ));
			 prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second + third).is_err());

			 prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::set_lock(*b"prelocks", asset_id, &account,  third));
			 prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second));
			 prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second + third).is_err());
			Ok(())
		  })?;
	   }

		#[test]
	   fn test_remove_lock_implementation(
			account in accounts(),
			asset_id in asset(),
			(first, second, _) in valid_amounts_without_overflow_3()) {

			new_test_ext_multi_currency().execute_with(|| {
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id,&account),0);
				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::deposit(asset_id, &account, first + second));
				prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::set_lock(*b"prelocks", asset_id, &account, first + second));
				prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second).is_err());
				prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first ).is_err());
				prop_assert!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, second ).is_err());
				prop_assert_ok!(<Pallet::<Test> as MultiLockableCurrency<AccountId>>::remove_lock(*b"prelocks", asset_id, &account));
				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(asset_id, &account, first + second));
				Ok(())
		  })?;
	   }

	}
}

mod reservable_multicurrency {
	use super::*;
	use frame_support::traits::tokens::BalanceStatus;
	use orml_traits::currency::{MultiCurrency, MultiReservableCurrency};

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn test_can_reserve_implementation(
			account_1 in accounts(),
			asset_id in asset(),
			(first, _, _) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::can_reserve(asset_id, &account_1, first), false);
				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id,&account_1, first));
				prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::can_reserve(asset_id, &account_1, first), true);

				Ok(())
			})?;
		}

		#[test]
		fn test_reserve_implementation(
			account_1 in accounts(),
			asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext_multi_currency().execute_with(|| {

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id,&account_1, first));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::total_issuance(asset_id), first);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id,&account_1, second));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::total_issuance(asset_id), first + second);

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id,&account_1, third));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::total_issuance(asset_id), first + second + third);

				//reserve
				prop_assert_ok!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserve(asset_id, &account_1, first + second));
				prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(asset_id, &account_1), first + second);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id, &account_1), third);

				Ok(())
			}).unwrap();
		}

		#[test]
		fn test_slash_reserve_implementation(
			account_1 in accounts(),
			asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext_multi_currency().execute_with(|| {

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id, &account_1, first+second+third));
				//reserve
				prop_assert_ok!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::reserve(asset_id, &account_1, first+second));
				 let total_issuance = <Pallet::<Test> as MultiCurrency<AssetId>>::total_issuance(asset_id);
				//slash
				let  difference = <Pallet::<Test> as MultiReservableCurrency<AssetId>>::slash_reserved(asset_id, &account_1, third);
				let _balance = if  first + second > third {
					prop_assert_eq!(difference, 0);
					let balance = (first + second) - third;
					// check reserve balance after slash
					prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::total_issuance(asset_id),total_issuance-(third-difference));
					prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::reserved_balance(asset_id, &account_1), balance);

					balance
				} else {
					prop_assert_eq!(difference, third - (first + second ));
					prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::total_issuance(asset_id),total_issuance-(third-difference));
					prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::reserved_balance(asset_id, &account_1), 0);
					0
				};

				Ok(())
			})?;
		}

		#[test]
		fn test_repatriate_reserve_implementation(
			(account_1, account_2) in accounts_2(),
			   asset_id in asset(),
			(first, second, _) in valid_amounts_without_overflow_3()
		) {
			new_test_ext_multi_currency().execute_with(|| {

			   prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id, &account_1, first+second));
			   prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id, &account_2, first));
			   prop_assert_ok!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::reserve(asset_id, &account_1, first + second ));
			   //repatriate to free balance
			   let repatriate_free = <Pallet::<Test> as MultiReservableCurrency<AssetId>>::repatriate_reserved(asset_id, &account_1, &account_2, second, BalanceStatus::Free).unwrap();
			   prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::free_balance(asset_id, &account_2), first + (second - repatriate_free));
			   prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::reserved_balance(asset_id, &account_1), first + repatriate_free );

				Ok(())
			})?;
		}


		#[test]
		fn test_unreserve_implementation(
			 account_1 in accounts(),
			 asset_id in asset(),
			(first, second, third) in valid_amounts_without_overflow_3()
		) {
			new_test_ext().execute_with(|| {

				prop_assert_ok!(<Pallet::<Test> as MultiCurrency<AssetId>>::deposit(asset_id, &account_1, first + second + third));
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::free_balance(asset_id, &account_1), first + second + third);
				prop_assert_ok!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::reserve(asset_id,&account_1, first+second+third));
				prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AssetId>>::reserved_balance(asset_id, &account_1), first + second + third);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::free_balance(asset_id, &account_1), 0);
				//repatriate to free balance
				let mut remaining = <Pallet::<Test> as MultiReservableCurrency<AssetId>>::unreserve(asset_id, &account_1, third);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::free_balance(asset_id, &account_1), third - remaining);
				let mut free_balance = <Pallet::<Test> as MultiCurrency<AssetId>>::free_balance(asset_id, &account_1);
				remaining = <Pallet::<Test> as MultiReservableCurrency<AssetId>>::unreserve(asset_id, &account_1, second);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AssetId>>::free_balance(asset_id, &account_1), free_balance + (second - remaining));

				free_balance = <Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id, &account_1);
				remaining = <Pallet::<Test> as MultiReservableCurrency<AccountId>>::unreserve(asset_id, &account_1, first);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id, &account_1), free_balance + (first - remaining));
				prop_assert_eq!(<Pallet::<Test> as MultiReservableCurrency<AccountId>>::reserved_balance(asset_id, &account_1), 0);
				prop_assert_eq!(<Pallet::<Test> as MultiCurrency<AccountId>>::free_balance(asset_id, &account_1), first + second + third);

				Ok(())
			})?;
		}
	}
}
