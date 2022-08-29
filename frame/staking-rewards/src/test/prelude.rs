pub use crate::{self as pallet_staking_rewards, prelude::*};
use crate::{
	test::{
		runtime::{Origin, StakingRewards, System},
		Test,
	},
	RewardPoolConfigurationOf,
};
use composable_tests_helpers::test::{block::MILLISECS_PER_BLOCK, helper::assert_extrinsic_event};
use frame_support::assert_ok;
use pallet_staking_rewards::test::runtime;
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

// helpers

pub(crate) fn add_to_rewards_pot_and_assert(
	who: <Test as frame_system::Config>::AccountId,
	pool_id: <Test as crate::Config>::RewardPoolId,
	asset_id: <Test as crate::Config>::AssetId,
	amount: <Test as crate::Config>::Balance,
) {
	assert_extrinsic_event::<Test, _, _, _>(
		StakingRewards::add_to_rewards_pot(Origin::signed(who), pool_id, asset_id, amount, false),
		crate::Event::<Test>::RewardsPotIncreased { pool_id, asset_id, amount },
	)
}

pub(crate) fn create_rewards_pool_and_assert(
	reward_config: RewardPoolConfigurationOf<Test>,
) -> <Test as crate::Config>::RewardPoolId {
	assert_ok!(StakingRewards::create_reward_pool(Origin::root(), reward_config.clone()));

	match System::events().last().expect("no events present").event {
		runtime::Event::StakingRewards(crate::Event::<Test>::RewardPoolCreated {
			pool_id,
			owner: event_owner,
			end_block: event_end_block,
		}) => {
			match reward_config {
				RewardPoolConfiguration::RewardRateBasedIncentive { end_block, owner, .. } => {
					assert_eq!(end_block, event_end_block);
					assert_eq!(owner, event_owner);
				},
				_ => unimplemented!(),
			}

			pool_id
		},
		_ => panic!("RewardPoolCreated event not emitted"),
	}
}
