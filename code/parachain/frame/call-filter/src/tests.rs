#![cfg(test)]

use super::*;
use mock::{Event, *};
use sp_runtime::traits::BadOrigin;
use support::{assert_noop, assert_ok};

const BALANCE_TRANSFER: &<Runtime as system::Config>::RuntimeCall =
	&mock::Call::Balances(pallet_balances::Call::transfer { dest: ALICE, value: 10 });
#[test]
fn pause_transaction_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let balances_transfer = CallFilterEntry {
			pallet_name: b"Balances".to_vec().try_into().unwrap(),
			function_name: b"transfer".to_vec().try_into().unwrap(),
		};
		assert_noop!(Filter::disable(Origin::signed(5), balances_transfer.clone()), BadOrigin);

		assert_eq!(Filter::disabled_calls(&balances_transfer), None);
		assert_ok!(Filter::disable(Origin::signed(1), balances_transfer.clone()));
		System::assert_last_event(Event::Filter(crate::Event::Disabled {
			entry: balances_transfer.clone(),
		}));
		assert_eq!(Filter::disabled_calls(&balances_transfer), Some(()));

		let filter_pause = CallFilterEntry {
			pallet_name: b"Filter".to_vec().try_into().unwrap(),
			function_name: b"disable".to_vec().try_into().unwrap(),
		};
		let filter_pause_2 = CallFilterEntry {
			pallet_name: b"Filter".to_vec().try_into().unwrap(),
			function_name: b"another_call".to_vec().try_into().unwrap(),
		};

		assert_noop!(
			Filter::disable(Origin::signed(1), filter_pause),
			Error::<Runtime>::CannotDisable
		);
		assert_noop!(
			Filter::disable(Origin::signed(1), filter_pause_2),
			Error::<Runtime>::CannotDisable
		);

		let other = CallFilterEntry {
			pallet_name: b"OtherPallet".to_vec().try_into().unwrap(),
			function_name: b"disable".to_vec().try_into().unwrap(),
		};
		assert_ok!(Filter::disable(Origin::signed(1), other));
	});
}

#[test]
fn enable_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let balances_transfer = CallFilterEntry {
			pallet_name: b"Balances".to_vec().try_into().unwrap(),
			function_name: b"transfer".to_vec().try_into().unwrap(),
		};

		assert_ok!(Filter::disable(Origin::signed(1), balances_transfer.clone()));
		assert_eq!(Filter::disabled_calls(&balances_transfer), Some(()));

		assert_noop!(Filter::enable(Origin::signed(5), balances_transfer.clone()), BadOrigin);

		assert_ok!(Filter::enable(Origin::signed(1), balances_transfer.clone()));
		System::assert_last_event(Event::Filter(crate::Event::Enabled {
			entry: balances_transfer.clone(),
		}));
		assert_eq!(Filter::disabled_calls(&balances_transfer), None);
	});
}

#[test]
fn paused_transaction_filter_work() {
	ExtBuilder::default().build().execute_with(|| {
		let balances_transfer = CallFilterEntry {
			pallet_name: b"Balances".to_vec().try_into().unwrap(),
			function_name: b"transfer".to_vec().try_into().unwrap(),
		};

		assert!(!Filter::contains(BALANCE_TRANSFER));
		assert_ok!(Filter::disable(Origin::signed(1), balances_transfer.clone()));

		assert!(Filter::contains(BALANCE_TRANSFER));
		assert_ok!(Filter::enable(Origin::signed(1), balances_transfer));

		assert!(!Filter::contains(BALANCE_TRANSFER));
	});
}
