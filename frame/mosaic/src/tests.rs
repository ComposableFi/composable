/// TODO
///
/// 1. Test each extrinsic
/// 2. Make sure unlocks etc are respected (timing)
/// 3. Add tests for linear decay.
use crate::{mock::*, *};

#[test]
fn set_relayer() {
	new_test_ext().execute_with(|| {
		Mosaic::set_relayer(Origin::root(), ALICE).expect("root may call set_relayer");
		assert_eq!(Mosaic::relayer_account_id(), Some(ALICE));
		Mosaic::set_relayer(Origin::signed(ALICE), ALICE)
			.expect_err("only root may call set_relayer");
		Mosaic::set_relayer(Origin::none(), ALICE).expect_err("only root may call set_relayer");
	})
}

#[test]
fn rotate_relayer() {
	new_test_ext().execute_with(|| {
		let ttl = 500;
		let current_block = System::block_number();
		Mosaic::set_relayer(Origin::root(), ALICE).expect("root may call set_relayer");
		Mosaic::rotate_relayer(Origin::signed(ALICE), BOB, ttl)
			.expect("relayer may rotate relayer");
		System::set_block_number(current_block + ttl + 2);
		assert_eq!(Mosaic::relayer_account_id(), Some(BOB))
	})
}

#[test]
fn transfer_to() {
	new_test_ext().execute_with(|| {
		let _ttl = 500;
		let _current_block = System::block_number();
		Mosaic::set_relayer(Origin::root(), ALICE).expect("root may call set_relayer");
		Mosaic::set_network(
			Origin::signed(ALICE),
			1,
			NetworkInfo { enabled: true, max_transfer_size: 100000 },
		)
		.expect("relayer may set network info");
		Mosaic::set_budget(Origin::root(), 1, 10000, BudgetDecay::linear(10))
			.expect("root may set budget");
		Mosaic::transfer_to(Origin::signed(ALICE), 1, 1, [0; 20], 100, true)
			.expect("transfer_to should work");
		assert_eq!(
			Mosaic::outgoing_transactions(&ALICE, 1),
			Some((100, MinimumTimeLockPeriod::get()))
		)
	})
}
