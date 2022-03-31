use crate::{
	mock::{
		new_test_ext, AccountId, AssetId, Event, Origin, StakingRewards, System, Test, Timestamp,
		Tokens,
	},
	Error, StakingConfigOf,
};
use composable_traits::{
	staking_rewards::{Staking, StakingConfig, StakingReward},
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

pub const HOUR: DurationSeconds = 60 * 60;
pub const DAY: DurationSeconds = 24 * HOUR;
pub const WEEK: DurationSeconds = 7 * DAY;
pub const MONTH: DurationSeconds = 30 * DAY;

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
			System::set_block_number(1);
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
			System::set_block_number(1);
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
			System::set_block_number(1);
			let penalty = Perbill::from_float(0.5);
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
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
	fn early_unstake_apply_penalty() {
		new_test_ext().execute_with(|| {
			let penalty = Perbill::from_float(0.5);
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
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
			Timestamp::set_timestamp(WEEK * 1_000);
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
			System::set_block_number(1);
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

			// Actually check isolation
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
				<StakingRewards as Staking>::claim(&fake_instance_id, &ALICE),
				DispatchError::Token(TokenError::UnknownAsset)
			);
		});
	}
}
