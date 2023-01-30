use std::sync::Once;

pub use crate::prelude::*;

use composable_tests_helpers::test::block::MILLISECS_PER_BLOCK;
pub use sp_core::{
	sr25519::{Public, Signature},
	H256,
};

#[cfg(test)]
pub use composable_tests_helpers::test::currency::*;
use tracing::metadata::LevelFilter;
use tracing_tree::HierarchicalLayer;

pub(crate) const fn block_seconds(amount_of_blocks: u64) -> u128 {
	// would use `.into()` instead of `as` but `.into()` is not const
	((MILLISECS_PER_BLOCK / 1_000) * amount_of_blocks) as u128
}

/// Mock ID for staking fNFT collection
pub(crate) const STAKING_FNFT_COLLECTION_ID: CurrencyId = 1;

pub(crate) const MINIMUM_STAKING_AMOUNT: u128 = 10_000;

pub(crate) fn init_logger() {
	use tracing_subscriber::prelude::*;

	static LOGGER: Once = Once::new();

	LOGGER.call_once(|| {
		// tracing_subscriber::filter::LevelFilter
		let subscriber = tracing_subscriber::Registry::default().with(HierarchicalLayer::new(2));

		let filter = LevelFilter::WARN;

		let subscriber = filter.with_subscriber(subscriber);

		tracing::subscriber::set_global_default(subscriber).unwrap();
	});

	// let _ = tracing_subscriber::fmt()
	// 	.compact()
	// 	// .pretty()
	// 	.with_max_level(tracing::Level::INFO)
	// 	.try_init();
	// tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");
	// let _ = env_logger::builder().filter_level(LevelFilter::Info).is_test(true).try_init();
}
