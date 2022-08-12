pub use crate::{self as pallet_staking_rewards, prelude::*};
use composable_tests_helpers::test::block::MILLISECS_PER_BLOCK;
pub use sp_core::{
	sr25519::{Public, Signature},
	H256,
};
use sp_runtime::traits::{IdentifyAccount, Verify};
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[cfg(test)]
pub use composable_tests_helpers::test::currency::*;

pub(crate) const fn block_seconds(block_number: u64) -> u128 {
	((MILLISECS_PER_BLOCK / 1_000) * block_number) as u128
}

pub(crate) const ONE_YEAR_OF_BLOCKS: u64 = 60 * 60 * 24 * 365 / (block_seconds(1) as u64);
