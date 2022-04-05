use crate::{
	mock::{
		new_test_ext, process_block, AccountId, AssetId, BlockNumber, Event, Origin,
		StakingRewards, System, Test, Tokens, MILLISECS_PER_BLOCK, REWARD_EPOCH_DURATION_BLOCK,
	},
	Error, StakingConfigOf,
};
use composable_traits::{
	staking_rewards::{ClaimStrategy, Staking, StakingConfig, StakingReward},
	time::DurationSeconds,
};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use sp_runtime::{DispatchError, Perbill, TokenError};
use std::collections::{BTreeMap, BTreeSet};

pub const TREASURY: AccountId = 0;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

pub const BTC: AssetId = 1;
pub const LTC: AssetId = 2;
pub const ETH: AssetId = 3;
pub const XMR: AssetId = 4;

pub const PICA: AssetId = 0xCAFEBABE;
pub const LAYR: AssetId = 0xDEADC0DE;

pub const MINUTE: DurationSeconds = 60;
pub const HOUR: DurationSeconds = 60 * MINUTE;
pub const DAY: DurationSeconds = 24 * HOUR;
pub const WEEK: DurationSeconds = 7 * DAY;
pub const MONTH: DurationSeconds = 30 * DAY;

fn duration_to_block(duration: DurationSeconds) -> BlockNumber {
	duration * 1000 / MILLISECS_PER_BLOCK
}

fn configure_pica(penalty: Perbill) -> StakingConfigOf<Test> {
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
		early_unstake_penalty: penalty,
		penalty_beneficiary: TREASURY,
	};
	assert_ok!(StakingRewards::configure(Origin::root(), PICA, config.clone()));
	config
}

fn configure_default_pica() -> StakingConfigOf<Test> {
	configure_pica(Perbill::from_float(0.2))
}

mod configure {
	use super::*;

	#[test]
	fn generate_event() {
		new_test_ext().execute_with(|| {
			process_block(1);
			let configuration = configure_default_pica();
			System::assert_last_event(Event::StakingRewards(crate::Event::Configured {
				asset: PICA,
				configuration,
			}));
		});
	}

	#[test]
	fn root_can_configure() {
		new_test_ext().execute_with(|| {
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
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
		});
	}

	#[test]
	fn root_can_overwrite() {
		new_test_ext().execute_with(|| {
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
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
			assert_ok!(StakingRewards::configure(Origin::root(), PICA, config.clone()));
			assert_ok!(StakingRewards::configure(Origin::root(), PICA, config));
		});
	}

	#[test]
	fn nonroot_configure_ko() {
		new_test_ext().execute_with(|| {
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
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
			assert_noop!(
				StakingRewards::configure(Origin::signed(ALICE), PICA, config),
				DispatchError::BadOrigin
			);
		});
	}
}

mod stake {
	use super::*;

	#[test]
	fn just_works() {
		new_test_ext().execute_with(|| {
			process_block(1);
			configure_default_pica();
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

	#[test]
	fn stake_invalid_duration_ko() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			assert_noop!(
				<StakingRewards as Staking>::stake(&PICA, &ALICE, stake, DAY, false),
				Error::<Test>::InvalidDurationPreset
			);
		});
	}

	#[test]
	fn pending_does_not_alter_total_shares() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			let duration = WEEK;
			let initial_total_shares = StakingRewards::total_shares(PICA);
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			assert_ok!(<StakingRewards as Staking>::stake(&PICA, &ALICE, stake, duration, false));
			let final_total_shares = StakingRewards::total_shares(PICA);
			assert_eq!(initial_total_shares, final_total_shares);
		});
	}

	#[test]
	fn pending_alter_total_shares_pending() {
		new_test_ext().execute_with(|| {
			let config = configure_default_pica();
			let stake = 1_000_000_000_000;
			let duration = WEEK;
			let shares = config
				.duration_presets
				.get(&duration)
				.expect("impossible; qed;")
				.mul_floor(stake);
			let initial_total_shares_pending = StakingRewards::total_shares_pending(PICA);
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			assert_ok!(<StakingRewards as Staking>::stake(&PICA, &ALICE, stake, duration, false));
			let final_total_shares_pending = StakingRewards::total_shares_pending(PICA);
			let delta_total_shares_pending = final_total_shares_pending
				.checked_sub(initial_total_shares_pending)
				.expect("impossible; qed;");
			assert_eq!(delta_total_shares_pending, shares);
		});
	}

	#[test]
	fn rewarding_reflected_in_total_shares() {
		new_test_ext().execute_with(|| {
			let config = configure_default_pica();
			let stake = 1_000_000_000_000;
			let duration = WEEK;
			let shares = config
				.duration_presets
				.get(&duration)
				.expect("impossible; qed;")
				.mul_floor(stake);
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			let initial_total_shares = StakingRewards::total_shares(PICA);
			assert_ok!(<StakingRewards as Staking>::stake(&PICA, &ALICE, stake, duration, false));
			// Enter new epoch
			process_block(1);
			let final_total_shares = StakingRewards::total_shares(PICA);
			let delta_total_shares =
				final_total_shares.checked_sub(initial_total_shares).expect("impossible; qed;");
			assert_eq!(delta_total_shares, shares);
		});
	}
}

mod unstake {
	use super::*;

	#[test]
	fn owner_can_unstake() {
		new_test_ext().execute_with(|| {
			let penalty = Perbill::from_float(0.5);
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
		});
	}

	#[test]
	fn non_owner_cant_unstake() {
		new_test_ext().execute_with(|| {
			let penalty = Perbill::from_float(0.5);
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");

			assert_noop!(
				StakingRewards::unstake(Origin::signed(BOB), instance_id, ALICE),
				DispatchError::BadOrigin
			);
		});
	}

	#[test]
	fn generate_event() {
		new_test_ext().execute_with(|| {
			// process_block(1);
			let penalty = Perbill::from_float(0.5);
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(1);
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			System::assert_last_event(Event::StakingRewards(crate::Event::Unstaked {
				to: ALICE,
				stake,
				penalty: penalty.mul_floor(stake),
				nft: instance_id,
			}));
		});
	}

	#[test]
	fn early_unstake_before_epoch_doesnt_apply_penalty() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			assert_eq!(Tokens::balance(PICA, &ALICE), stake);
		});
	}

	#[test]
	fn early_unstake_apply_penalty() {
		new_test_ext().execute_with(|| {
			let penalty = Perbill::from_float(0.5);
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			// Enter into first epoch
			process_block(1);
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			assert_eq!(Tokens::balance(PICA, &ALICE), penalty.mul_floor(stake));
		});
	}

	#[test]
	fn complete_duration_unstake_does_not_apply_penalty() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(duration_to_block(WEEK + MINUTE));
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			assert_eq!(Tokens::balance(PICA, &ALICE), stake);
		});
	}

	#[test]
	fn penalty_goes_to_beneficiary() {
		new_test_ext().execute_with(|| {
			let penalty = Perbill::from_float(0.5);
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(1);
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			let penalty_amount = penalty.mul_floor(stake);
			assert_eq!(Tokens::balance(PICA, &TREASURY), penalty_amount);
		});
	}
}

mod transfer_reward {
	use super::*;

	#[test]
	fn not_configured_ko() {
		new_test_ext().execute_with(|| {
			let reward_asset = BTC;
			let reward = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(reward_asset, &TREASURY, reward));
			assert_noop!(
				StakingRewards::transfer_reward(&PICA, &reward_asset, &TREASURY, reward, false),
				Error::<Test>::NotConfigured
			);
		});
	}

	#[test]
	fn just_works() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let reward_asset = BTC;
			let reward = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(reward_asset, &TREASURY, reward));
			assert_eq!(Tokens::balance(BTC, &StakingRewards::account_id(&PICA)), 0);
			assert_ok!(StakingRewards::transfer_reward(
				&PICA,
				&reward_asset,
				&TREASURY,
				reward,
				false
			));
			assert_eq!(Tokens::balance(BTC, &StakingRewards::account_id(&PICA)), reward);
		});
	}

	#[test]
	fn generate_event() {
		new_test_ext().execute_with(|| {
			process_block(1);
			configure_default_pica();
			let reward_asset = BTC;
			let reward = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(reward_asset, &TREASURY, reward));
			assert_ok!(StakingRewards::transfer_reward(
				&PICA,
				&reward_asset,
				&TREASURY,
				reward,
				false
			));
			System::assert_last_event(Event::StakingRewards(crate::Event::NewReward {
				rewarded_asset: PICA,
				reward_asset,
				amount: reward,
			}));
		});
	}

	#[test]
	fn disabled_reward_asset_ko() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let reward_asset = XMR;
			let reward = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(reward_asset, &TREASURY, reward));
			assert_noop!(
				StakingRewards::transfer_reward(&PICA, &reward_asset, &TREASURY, reward, false),
				Error::<Test>::RewardAssetDisabled
			);
		});
	}

	#[test]
	fn increment_collected_rewards() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let reward_asset = BTC;
			let reward = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(reward_asset, &TREASURY, reward));
			assert!(StakingRewards::collected_rewards(PICA, BTC).is_none());
			assert!(StakingRewards::collected_rewards(PICA, LTC).is_none());
			assert_ok!(StakingRewards::transfer_reward(
				&PICA,
				&reward_asset,
				&TREASURY,
				reward,
				false
			));
			assert_eq!(
				StakingRewards::collected_rewards(PICA, BTC).expect("impossible; qed;"),
				reward.into()
			);
			assert!(StakingRewards::collected_rewards(PICA, LTC).is_none());
		});
	}

	#[test]
	fn isolated_accounts() {
		new_test_ext().execute_with(|| {
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
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
			assert_ok!(StakingRewards::configure(Origin::root(), PICA, config.clone()));
			assert_ok!(StakingRewards::configure(Origin::root(), LAYR, config));

			let reward_asset = BTC;
			let reward = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(reward_asset, &TREASURY, reward));
			assert_ok!(StakingRewards::transfer_reward(
				&PICA,
				&reward_asset,
				&TREASURY,
				reward,
				false
			));

			// ACTUALLY check isolation
			assert_eq!(Tokens::balance(BTC, &StakingRewards::account_id(&PICA)), reward);
			assert_eq!(Tokens::balance(BTC, &StakingRewards::account_id(&LAYR)), 0);
		});
	}
}

mod claim {
	use super::*;

	#[test]
	fn non_existing_nft_ko() {
		new_test_ext().execute_with(|| {
			let fake_instance_id = 0;
			assert_noop!(
				<StakingRewards as Staking>::claim(
					&fake_instance_id,
					&ALICE,
					ClaimStrategy::Canonical
				),
				DispatchError::Token(TokenError::UnknownAsset)
			);
		});
	}

	#[test]
	fn just_works() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(REWARD_EPOCH_DURATION_BLOCK);
			assert_ok!(StakingRewards::claim(
				Origin::signed(ALICE),
				instance_id,
				ALICE,
				ClaimStrategy::Canonical
			));
		});
	}

	#[test]
	fn tagger_and_owner_can_claim() {
		new_test_ext().execute_with(|| {
			process_block(1);
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(duration_to_block(WEEK + MINUTE));
			assert_ok!(StakingRewards::tag(Origin::signed(TREASURY), instance_id, TREASURY));
			assert_ok!(StakingRewards::claim(
				Origin::signed(TREASURY),
				instance_id,
				TREASURY,
				ClaimStrategy::Canonical
			));
			assert_ok!(StakingRewards::claim(
				Origin::signed(ALICE),
				instance_id,
				ALICE,
				ClaimStrategy::Canonical
			));
		});
	}
}

mod tag {
	use super::*;

	#[test]
	fn can_tag_if_expired() {
		new_test_ext().execute_with(|| {
			process_block(1);
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(duration_to_block(WEEK + MINUTE));
			assert_ok!(StakingRewards::tag(Origin::signed(TREASURY), instance_id, TREASURY));
		});
	}

	#[test]
	fn cannot_tag_already_tagged() {
		new_test_ext().execute_with(|| {
			process_block(1);
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(duration_to_block(WEEK + MINUTE));
			assert_ok!(StakingRewards::tag(Origin::signed(TREASURY), instance_id, TREASURY));
			assert_noop!(
				StakingRewards::tag(Origin::signed(TREASURY), instance_id, TREASURY),
				Error::<Test>::AlreadyTagged
			);
		});
	}
}
