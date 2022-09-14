use hex_literal::hex;
use proptest::strategy::Just;
use sp_core::sr25519::{Public, Signature};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub const ADMIN: AccountId =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000000"));
pub const ALICE: AccountId =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000001"));
pub const BOB: AccountId =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000002"));
pub const CHARLIE: AccountId =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000003"));
pub const DAVE: AccountId =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000004"));
pub const EVEN: AccountId =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000005"));

pub const fn accounts() -> [Just<AccountId>; 5] {
	[Just(ALICE), Just(BOB), Just(CHARLIE), Just(DAVE), Just(EVEN)]
}