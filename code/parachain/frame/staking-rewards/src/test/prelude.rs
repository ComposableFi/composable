pub use crate::{self as pallet_staking_rewards, prelude::*};
use crate::{
	test::{
		runtime::{Event, Origin, StakingRewards},
		Test,
	},
	RewardPoolConfigurationOf,
};
use composable_tests_helpers::test::{
	block::MILLISECS_PER_BLOCK,
	helper::{assert_extrinsic_event, assert_extrinsic_event_with},
};
use frame_system::pallet_prelude::OriginFor;
pub use sp_core::{
	sr25519::{Public, Signature},
	H256,
};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub use composable_tests_helpers::test::currency::*;

use super::balance;

pub(crate) const fn block_seconds(amount_of_blocks: u64) -> u128 {
	// would use `.into()` instead of `as` but `.into()` is not const
	((MILLISECS_PER_BLOCK / 1_000) * amount_of_blocks) as u128
}

pub(crate) const ONE_YEAR_OF_BLOCKS: u64 = 60 * 60 * 24 * 365 / (block_seconds(1) as u64);

/// Mock ID for staking fNFT collection
pub(crate) const STAKING_FNFT_COLLECTION_ID: CurrencyId = 1;

// helpers

pub(crate) fn add_to_rewards_pot_and_assert(
	who: <Test as frame_system::Config>::AccountId,
	pool_id: <Test as crate::Config>::AssetId,
	asset_id: <Test as crate::Config>::AssetId,
	amount: <Test as crate::Config>::Balance,
) {
	let pool_account = StakingRewards::pool_account_id(&pool_id);
	let pot_balance_before = balance(asset_id, &pool_account);

	assert_extrinsic_event::<Test, _, _, _>(
		StakingRewards::add_to_rewards_pot(Origin::signed(who), pool_id, asset_id, amount, false),
		crate::Event::<Test>::RewardsPotIncreased { pool_id, asset_id, amount },
	);

	assert_eq!(pot_balance_before + amount, balance(asset_id, &pool_account));

	// TODO(benluelo): Add storage checks
}

pub(crate) fn create_rewards_pool_and_assert<Runtime: crate::Config>(
	reward_config: RewardPoolConfigurationOf<Runtime>,
) {
	assert_extrinsic_event_with::<Runtime, _, _, _>(
		crate::Pallet::<Runtime>::create_reward_pool(
			OriginFor::<Runtime>::root(),
			reward_config.clone(),
		),
		|event| match event {
			Event::StakingRewards(crate::Event::<Runtime>::RewardPoolCreated {
				pool_id,
				owner,
				end_block,
			}) => {
				assert_eq!(end_block, reward_config.end_block);
				assert_eq!(owner, reward_config.owner);
				assert_eq!(pool_id, reward_config.asset_id);

				Some(())
			},
			_ => None,
		},
	);

	// TODO(benluelo): Add storage checks
}
