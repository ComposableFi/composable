pub use crate::{self as pallet_staking_rewards, prelude::*};
pub use sp_core::{
	sr25519::{Public, Signature},
	H256,
};
use sp_runtime::traits::{IdentifyAccount, Verify};
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[cfg(test)]
pub use composable_tests_helpers::test::currency::*;
