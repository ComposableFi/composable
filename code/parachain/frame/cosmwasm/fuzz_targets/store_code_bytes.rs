//! clear && cargo-fuzz run store-code-bytes --fuzz-dir .  -- -max_len=255535
#![no_main]

use frame_benchmarking::account;
use frame_support::traits::fungible;
use libfuzzer_sys::fuzz_target;
use sp_runtime::AccountId32;

extern crate pallet_cosmwasm;

pub fn create_funded_account(key: &'static str) -> AccountId32 {
	let origin = account(key, 0, 0xCAFEBABE);

	<pallet_balances::Pallet<Test> as fungible::Mutate<AccountId32>>::mint_into(
		&origin,
		u64::MAX as u128,
	)
	.unwrap();
	origin
}

use pallet_cosmwasm::{mock::*, *};

fuzz_target!(|code: &[u8]| {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		crate::mock::Timestamp::set_timestamp(1);
		let origin = create_funded_account("origin");
		if let Ok(_uploaded) = pallet_cosmwasm::Pallet::<Test>::upload(
			RuntimeOrigin::signed(origin),
			code.to_vec()
				.try_into()
				.expect("please reduce fuzzer input size to config of runtime"),
		) {
			// just pushing random bytes
		}
	})
});
