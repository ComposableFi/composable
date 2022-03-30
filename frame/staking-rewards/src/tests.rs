use crate::mock::{
	new_test_ext, AccountId, AssetId, Event, Origin, StakingRewards, System, Tokens,
};
use composable_traits::{
	staking_rewards::{Staking, StakingConfig},
	time::DurationSeconds,
};
use frame_support::{assert_ok, traits::fungibles::Mutate};
use sp_runtime::Perbill;
use std::collections::{BTreeMap, BTreeSet};

pub const TREASURY: AccountId = 0;
pub const ALICE: AccountId = 1;

pub const PICA: AssetId = 0;
pub const BTC: AssetId = 1;
pub const LTC: AssetId = 2;
pub const ETH: AssetId = 3;

pub const HOUR: DurationSeconds = 60 * 60;
pub const DAY: DurationSeconds = 24 * HOUR;
pub const WEEK: DurationSeconds = 7 * DAY;
pub const MONTH: DurationSeconds = 30 * DAY;

#[test]
fn test_stake() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let config = StakingConfig {
			duration_presets: [(WEEK, Perbill::from_float(0.5)), (MONTH, Perbill::from_float(1.0))]
				.into_iter()
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("impossible; qed;"),
			rewards: [BTC, LTC, ETH]
				.into_iter()
				.collect::<BTreeSet<_>>()
				.try_into()
				.expect("impossible; qed;"),
			early_unstake_penalty: Perbill::from_float(0.2),
			penalty_beneficiary: TREASURY,
		};
		assert_ok!(StakingRewards::configure(Origin::root(), PICA, config));

		let stake = 1_000_000_000_000;
		assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));

		let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
			.expect("impossible; qed;");

		System::assert_last_event(Event::StakingRewards(crate::Event::Staked {
			who: ALICE,
			stake,
			nft: instance_id,
		}));
	});
}
