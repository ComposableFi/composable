/// TODO
///
/// 1. Test each extrinsic
/// 2. Make sure unlocks etc are respected (timing)
/// 3. Add tests for linear decay.
use crate::{mock::*, *};

pub trait OriginExt {
	fn relayer() -> Origin {
		Origin::signed(RELAYER)
	}

	fn alice() -> Origin {
		Origin::signed(ALICE)
	}
}

impl OriginExt for Origin {}

#[test]
fn set_relayer() {
	new_test_ext().execute_with(|| {
		Mosaic::set_relayer(Origin::root(), RELAYER).expect("root may call set_relayer");
		assert_eq!(Mosaic::relayer_account_id(), Some(RELAYER));
		Mosaic::set_relayer(Origin::relayer(), ALICE).expect_err("only root may call set_relayer");
		Mosaic::set_relayer(Origin::none(), ALICE).expect_err("only root may call set_relayer");
	})
}

#[test]
fn rotate_relayer() {
	new_test_ext().execute_with(|| {
		let ttl = 500;
		let current_block = System::block_number();
		Mosaic::set_relayer(Origin::root(), RELAYER).expect("root may call set_relayer");
		Mosaic::rotate_relayer(Origin::relayer(), BOB, ttl).expect("relayer may rotate relayer");
		System::set_block_number(current_block + ttl + 2);
		assert_eq!(Mosaic::relayer_account_id(), Some(BOB))
	})
}

fn initialize() {
    System::set_block_number(1);

	Mosaic::set_relayer(Origin::root(), RELAYER).expect("root may call set_relayer");
	Mosaic::set_network(
		Origin::relayer(),
		1,
		NetworkInfo { enabled: true, max_transfer_size: 100000 },
	)
	.expect("relayer may set network info");
	Mosaic::set_budget(Origin::root(), 1, 10000, BudgetDecay::linear(10))
		.expect("root may set budget");
}

fn do_transfer_to() {

    let ethereum_address = [0; 20];
    let amount = 100;
    let network_id = 1;
    let asset_id = 1;

	Mosaic::transfer_to(Origin::signed(ALICE), network_id, asset_id, ethereum_address, amount, true)
		.expect("transfer_to should work");
	assert_eq!(Mosaic::outgoing_transactions(&ALICE, 1), Some((100, MinimumTimeLockPeriod::get() + System::block_number())));

    // normally we don't unit test events being emitted, but in this case it is very crucial for the
    // relayer to observe the events.


    // When a transfer is made, the nonce is incremented. However, nonce is one of the dependencies
    // for `generate_id`, we want to check if the events match, so we decrement the nonce and
    // increment it back when we're done
    // TODO: this is a hack, cfr: CU-1ubrf2y
    Nonce::<Test>::mutate(|nonce| {
        *nonce = nonce.wrapping_sub(1);
        *nonce
    });

    let id = generate_id::<Test>(&ALICE, &network_id, &asset_id, &ethereum_address, &amount, &System::block_number());
    Nonce::<Test>::mutate(|nonce| {
        *nonce = nonce.wrapping_add(1);
        *nonce
    });


    System::assert_last_event(mock::Event::Mosaic(crate::Event::TransferOut {
        id,
        to: ethereum_address,
        amount,
        network_id,
    }));
}

fn do_timelocked_mint(lock_time: u64) {
	let to = ALICE;
	let asset_id = 1;

    let initial_block = System::block_number();
    let amount = 50;

    Mosaic::timelocked_mint(Origin::relayer(), asset_id, to, amount, lock_time, Default::default())
		.expect("relayer should be able to mint");

	assert_eq!(Mosaic::incoming_transactions(to, asset_id), Some((amount, initial_block + lock_time)));
}

#[test]
fn transfer_to() {
	new_test_ext().execute_with(|| {
		initialize();
		do_transfer_to();
	})
}

#[test]
fn accept_transfer() {
	new_test_ext().execute_with(|| {
		initialize();
		do_transfer_to();
		Mosaic::accept_transfer(Origin::relayer(), ALICE, 1, 100)
			.expect("accepting transfer should work");
	})
}

#[test]
fn claim_stale_to() {
	new_test_ext().execute_with(|| {
		initialize();
		do_transfer_to();
		let current_block = System::block_number();
		System::set_block_number(current_block + Mosaic::timelock_period() + 1);
		Mosaic::claim_stale_to(Origin::signed(ALICE), ALICE, 1)
			.expect("claiming an outgoing transaction should work after the timelock period");
	})
}

#[test]
fn timelocked_mint() {
	new_test_ext().execute_with(|| {
		initialize();
		do_timelocked_mint(10);
	})
}

#[test]
fn rescind_timelocked_mint() {
	new_test_ext().execute_with(|| {
		initialize();
        let lock_time = 10;
		do_timelocked_mint(lock_time);

        let initial_block = System::block_number();

		Mosaic::rescind_timelocked_mint(Origin::relayer(), 1, ALICE, 40)
			.expect("relayer should be able to rescind transactions");
        assert_eq!(Mosaic::incoming_transactions(ALICE, 1), Some((10, initial_block + lock_time)));
        let transfer_amount = 9;
        Mosaic::rescind_timelocked_mint(Origin::relayer(), 1, ALICE, transfer_amount)
			.expect("relayer should be able to rescind transactions");
		assert_eq!(Mosaic::incoming_transactions(ALICE, 1), Some((1, 11)));
	})
}

#[test]
fn set_timelock_duration() {
	new_test_ext().execute_with(|| {
		Mosaic::set_timelock_duration(Origin::root(), MinimumTimeLockPeriod::get() + 1)
			.expect("root may set the timelock period");
	})
}

#[test]
fn claim_to() {
	new_test_ext().execute_with(|| {
		initialize();
        let lock_time = 10;
		do_timelocked_mint(lock_time);
		let current_block = System::block_number();
		Mosaic::claim_to(Origin::alice(), 1, ALICE).expect_err(
			"received funds should only be claimable after waiting for the relayer mandated time",
		);
		System::set_block_number(current_block + lock_time + 1);
		Mosaic::claim_to(Origin::alice(), 1, ALICE)
			.expect("received funds should be claimable after time has passed");
	})
}
