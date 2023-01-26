pub use crate::prelude::*;

use composable_tests_helpers::test::block::MILLISECS_PER_BLOCK;
use log::LevelFilter;
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

/// Mock ID for staking fNFT collection
pub(crate) const STAKING_FNFT_COLLECTION_ID: CurrencyId = 1;

pub(crate) const MINIMUM_STAKING_AMOUNT: u128 = 10_000;

pub(crate) fn init_logger() {
	let _ = env_logger::builder().filter_level(LevelFilter::Info).is_test(true).try_init();
}
