pub use crate::prelude::*;

use composable_tests_helpers::test::block::MILLISECS_PER_BLOCK;
pub use sp_core::{
	sr25519::{Public, Signature},
	H256,
};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[cfg(test)]
pub use composable_tests_helpers::test::currency::*;

pub(crate) const fn block_seconds(amount_of_blocks: u64) -> u128 {
	// would use `.into()` instead of `as` but `.into()` is not const
	((MILLISECS_PER_BLOCK / 1_000) * amount_of_blocks) as u128
}

pub(crate) const ONE_YEAR_OF_BLOCKS: u64 = 60 * 60 * 24 * 365 / (block_seconds(1) as u64);

/// Mock ID for staking fNFT collection
pub(crate) const STAKING_FNFT_COLLECTION_ID: CurrencyId = 1;

pub(crate) const MINIMUM_STAKING_AMOUNT: u128 = 10_000;
