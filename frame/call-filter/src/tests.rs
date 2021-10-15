#![cfg(test)]

use super::*;
use mock::{Event, *};
use sp_runtime::traits::BadOrigin;
use support::{assert_noop, assert_ok};

const BALANCE_TRANSFER: &<Runtime as system::Config>::Call =
	&mock::Call::Balances(pallet_balances::Call::transfer { dest: ALICE, value: 10 });
#[test]
fn pause_transaction_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			CallFilter::disable(Origin::signed(5), b"Balances".to_vec(), b"transfer".to_vec()),
			BadOrigin
		);

		assert_eq!(CallFilter::disabled_calls((b"Balances".to_vec(), b"transfer".to_vec())), None);
		assert_ok!(CallFilter::disable(
			Origin::signed(1),
			b"Balances".to_vec(),
			b"transfer".to_vec()
		));
		System::assert_last_event(Event::CallFilter(crate::Event::Disabled(
			b"Balances".to_vec(),
			b"transfer".to_vec(),
		)));
		assert_eq!(
			CallFilter::disabled_calls((b"Balances".to_vec(), b"transfer".to_vec())),
			Some(())
		);

		assert_noop!(
			CallFilter::disable(
				Origin::signed(1),
				b"CallFilter".to_vec(),
				b"pause_transaction".to_vec()
			),
			Error::<Runtime>::CannotPause
		);
		assert_noop!(
			CallFilter::disable(
				Origin::signed(1),
				b"CallFilter".to_vec(),
				b"some_other_call".to_vec()
			),
			Error::<Runtime>::CannotPause
		);
		assert_ok!(CallFilter::disable(
			Origin::signed(1),
			b"OtherPallet".to_vec(),
			b"pause_transaction".to_vec()
		));
	});
}

#[test]
fn enable_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(CallFilter::disable(
			Origin::signed(1),
			b"Balances".to_vec(),
			b"transfer".to_vec()
		));
		assert_eq!(
			CallFilter::disabled_calls((b"Balances".to_vec(), b"transfer".to_vec())),
			Some(())
		);

		assert_noop!(
			CallFilter::enable(Origin::signed(5), b"Balances".to_vec(), b"transfer".to_vec()),
			BadOrigin
		);

		assert_ok!(CallFilter::enable(
			Origin::signed(1),
			b"Balances".to_vec(),
			b"transfer".to_vec()
		));
		System::assert_last_event(Event::CallFilter(crate::Event::TransactionUnpaused(
			b"Balances".to_vec(),
			b"transfer".to_vec(),
		)));
		assert_eq!(CallFilter::disabled_calls((b"Balances".to_vec(), b"transfer".to_vec())), None);
	});
}

#[test]
fn paused_transaction_filter_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert!(!CallFilter::contains(BALANCE_TRANSFER));
		assert_ok!(CallFilter::disable(
			Origin::signed(1),
			b"Balances".to_vec(),
			b"transfer".to_vec()
		));

		assert!(CallFilter::contains(BALANCE_TRANSFER));
		assert_ok!(CallFilter::enable(
			Origin::signed(1),
			b"Balances".to_vec(),
			b"transfer".to_vec()
		));

		assert!(!CallFilter::contains(BALANCE_TRANSFER));
	});
}
