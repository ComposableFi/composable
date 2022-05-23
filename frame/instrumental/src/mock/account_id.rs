use hex_literal::hex;
use sp_core::sr25519::{Public, Signature};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub static ADMIN: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000000"));
pub static ALICE: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000001"));
pub static BOB: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000002"));
pub static CHARLIE: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000003"));
pub static DAVE: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000004"));
pub static EVEN: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000005"));

use proptest::{
	prop_oneof,
	strategy::{Just, Strategy},
};

#[allow(dead_code)]
pub fn pick_account() -> impl Strategy<Value = AccountId> {
	prop_oneof![Just(ALICE), Just(BOB), Just(CHARLIE), Just(DAVE), Just(EVEN),]
}
