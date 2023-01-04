#![cfg_attr(not(feature = "std"), no_std)]

use sp_keyring::sr25519::Keyring;
use sp_runtime::AccountId32;

pub mod test;

// pub const ALICE: AccountId32 = AccountId32::new([
// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
// ]);
// pub const BOB: AccountId32 = AccountId32::new([
// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
// ]);
// pub const CHARLIE: AccountId32 = AccountId32::new([
// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
// ]);
// pub const DAVE: AccountId32 = AccountId32::new([
// 	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
// ]);

macro_rules! keyring_accounts {
	($([$who:ident $Who:ident])+) => {
		$(
			pub fn $who() -> AccountId32 {
				Keyring::$Who.to_account_id()
			}
		)+
	};
}

keyring_accounts! {
	[alice Alice]
	[bob Bob]
	[charlie Charlie]
	[dave Dave]
	[eve Eve]
	[ferdie Ferdie]
}
