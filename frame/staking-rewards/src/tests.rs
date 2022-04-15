use crate::{
	mock::{
		new_test_ext, process_block, run_to_block, AccountId, AssetId, BlockNumber, Event, Origin,
		StakingRewards, System, Test, Tokens, MILLISECS_PER_BLOCK, REWARD_EPOCH_DURATION_BLOCK,
	},
	Error, PenaltyOf, StakingConfigOf, State,
};
use composable_traits::{
	staking_rewards::{Penalty, Staking, StakingConfig, StakingReward},
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

fn run_to_duration(duration: DurationSeconds) {
	run_to_block(duration_to_block(duration))
}

fn configure_pica(early_unstake_penalty: PenaltyOf<Test>) -> StakingConfigOf<Test> {
	let config = StakingConfig {
		duration_presets: [(WEEK, Perbill::from_float(0.5)), (MONTH, Perbill::from_float(1.0))]
			.into_iter()
			.collect::<BTreeMap<_, _>>()
			.try_into()
			.expect("impossible; qed;"),
		reward_assets: [BTC, LTC, ETH]
			.into_iter()
			.collect::<BTreeSet<_>>()
			.try_into()
			.expect("impossible; qed;"),
		early_unstake_penalty,
	};
	assert_ok!(StakingRewards::configure(Origin::root(), PICA, config.clone()));
	config
}

fn configure_default_pica() -> StakingConfigOf<Test> {
	let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
	configure_pica(penalty)
}

fn advance_state_machine() {
	run_to_block(3);
}

mod hook_state {
	use crate::{mock::ElementToProcessPerBlock, PendingStakers, Stakers};

	use super::*;

	#[test]
	fn state_machine_is_well_formed() {
		new_test_ext().execute_with(|| {
			// State machine constant over block number
			assert_eq!(StakingRewards::current_state(), State::WaitingForEpochEnd);
			process_block(0);
			assert_eq!(StakingRewards::current_state(), State::Rewarding);
			process_block(0);
			assert_eq!(StakingRewards::current_state(), State::Registering);
			process_block(0);
			assert_eq!(StakingRewards::current_state(), State::WaitingForEpochEnd);
			process_block(0);
			assert_eq!(StakingRewards::current_state(), State::WaitingForEpochEnd);
			process_block(0);
			assert_eq!(StakingRewards::current_state(), State::WaitingForEpochEnd);
			process_block(0);
			assert_eq!(StakingRewards::current_state(), State::WaitingForEpochEnd);
		});
	}

	#[test]
	fn generate_event() {
		new_test_ext().execute_with(|| {
			// State machine constant over block number
			assert_eq!(StakingRewards::current_state(), State::WaitingForEpochEnd);
			process_block(1);
			System::assert_last_event(Event::StakingRewards(crate::Event::NewEpoch {
				id: StakingRewards::current_epoch(),
			}));
		});
	}

	#[test]
	fn pending_stakers_transferred_to_stakers() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			let accounts = (BOB + 1..100).collect::<Vec<_>>();
			let nfts = accounts
				.iter()
				.map(|account| {
					assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, account, stake));
					let nft_id =
						<StakingRewards as Staking>::stake(&PICA, account, stake, WEEK, false)
							.expect("impossible; qed;");
					assert!(StakingRewards::pending_stakers(nft_id).is_some());
					assert!(StakingRewards::stakers(nft_id).is_none());
					nft_id
				})
				.collect::<Vec<_>>();
			run_to_duration(MINUTE);
			for nft in nfts {
				assert!(StakingRewards::stakers(nft).is_some());
				assert!(StakingRewards::pending_stakers(nft).is_none());
			}
		});
	}

	#[test]
	fn pending_stakers_processed_by_chunk() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			let account_start = BOB + 1;
			let nb_of_accounts = 1000;
			(account_start..account_start + nb_of_accounts).for_each(|account| {
				assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &account, stake));
				assert_ok!(<StakingRewards as Staking>::stake(&PICA, &account, stake, WEEK, false));
			});
			assert_eq!(PendingStakers::<Test>::iter().count(), nb_of_accounts as usize);
			assert_eq!(Stakers::<Test>::iter().count(), 0);
			run_to_block(1);
			assert_eq!(StakingRewards::current_state(), State::Rewarding);
			run_to_block(2);
			let block_start = System::block_number() + 1;
			let block_end =
				block_start + (nb_of_accounts as u64 / ElementToProcessPerBlock::get() as u64);
			(block_start..block_end).for_each(|block| {
				run_to_block(block);
				assert_eq!(
					Stakers::<Test>::iter().count(),
					(block - block_start + 1) as usize * ElementToProcessPerBlock::get() as usize
				);
			});
			assert_eq!(Stakers::<Test>::iter().count(), nb_of_accounts as usize);
			run_to_block(System::block_number() + 1);
			assert_eq!(PendingStakers::<Test>::iter().count(), 0);
			assert_eq!(StakingRewards::current_state(), State::WaitingForEpochEnd);
		});
	}
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
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
				.into_iter()
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("impossible; qed;"),
				reward_assets: [BTC, LTC, ETH]
					.into_iter()
					.collect::<BTreeSet<_>>()
					.try_into()
					.expect("impossible; qed;"),
				early_unstake_penalty: penalty,
			};
			assert_ok!(StakingRewards::configure(Origin::root(), PICA, config));
		});
	}

	#[test]
	fn root_can_overwrite() {
		new_test_ext().execute_with(|| {
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
				.into_iter()
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("impossible; qed;"),
				reward_assets: [BTC, LTC, ETH]
					.into_iter()
					.collect::<BTreeSet<_>>()
					.try_into()
					.expect("impossible; qed;"),
				early_unstake_penalty: penalty,
			};
			assert_ok!(StakingRewards::configure(Origin::root(), PICA, config.clone()));
			assert_ok!(StakingRewards::configure(Origin::root(), PICA, config));
		});
	}

	#[test]
	fn nonroot_configure_ko() {
		new_test_ext().execute_with(|| {
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
				.into_iter()
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("impossible; qed;"),
				reward_assets: [BTC, LTC, ETH]
					.into_iter()
					.collect::<BTreeSet<_>>()
					.try_into()
					.expect("impossible; qed;"),
				early_unstake_penalty: penalty,
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
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			advance_state_machine();
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
			let initial_total_shares = StakingRewards::total_shares((PICA, BTC));
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			assert_ok!(<StakingRewards as Staking>::stake(&PICA, &ALICE, stake, duration, false));
			let final_total_shares = StakingRewards::total_shares((PICA, BTC));
			assert_eq!(initial_total_shares, final_total_shares);
		});
	}

	#[test]
	fn pending_alter_pending_stakers() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			assert!(StakingRewards::pending_stakers(instance_id).is_some());
			assert!(StakingRewards::stakers(instance_id).is_none());
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
			let initial_total_shares = StakingRewards::total_shares((PICA, BTC));
			assert_ok!(<StakingRewards as Staking>::stake(&PICA, &ALICE, stake, duration, false));
			// Enter new epoch
			advance_state_machine();
			let final_total_shares = StakingRewards::total_shares((PICA, BTC));
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
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
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
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
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
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			advance_state_machine();
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			System::assert_last_event(Event::StakingRewards(crate::Event::Unstaked {
				to: ALICE,
				stake,
				penalty: penalty.value.mul_floor(stake),
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
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			// Enter into first epoch
			advance_state_machine();
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			assert_eq!(Tokens::balance(PICA, &ALICE), penalty.value.mul_floor(stake));
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
			run_to_duration(WEEK + MINUTE);
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			assert_eq!(Tokens::balance(PICA, &ALICE), stake);
		});
	}

	#[test]
	fn penalty_goes_to_beneficiary() {
		new_test_ext().execute_with(|| {
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
			configure_pica(penalty);
			let stake = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			advance_state_machine();
			assert_ok!(<StakingRewards as Staking>::unstake(&instance_id, &ALICE));
			let penalty_amount = penalty.value.mul_floor(stake);
			assert_eq!(Tokens::balance(PICA, &TREASURY), penalty_amount);
		});
	}
}

mod transfer_reward {
	use super::*;

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
	fn increment_collected_rewards() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let reward_asset = BTC;
			let reward = 1_000_000_000_000;
			assert_ok!(Tokens::mint_into(reward_asset, &TREASURY, reward));
			assert_eq!(
				StakingRewards::epoch_rewards((StakingRewards::current_epoch(), PICA), BTC),
				0
			);
			assert_eq!(
				StakingRewards::epoch_rewards((StakingRewards::current_epoch(), PICA), LTC),
				0
			);
			assert_ok!(StakingRewards::transfer_reward(
				&PICA,
				&reward_asset,
				&TREASURY,
				reward,
				false
			));
			assert_eq!(
				StakingRewards::epoch_rewards((StakingRewards::current_epoch(), PICA), BTC),
				reward.into()
			);
			assert_eq!(
				StakingRewards::epoch_rewards((StakingRewards::current_epoch(), PICA), LTC),
				0
			);
		});
	}

	#[test]
	fn isolated_accounts() {
		new_test_ext().execute_with(|| {
			let penalty = Penalty { value: Perbill::from_float(0.5), beneficiary: TREASURY };
			let config = StakingConfig {
				duration_presets: [
					(WEEK, Perbill::from_float(0.5)),
					(MONTH, Perbill::from_float(1.0)),
				]
				.into_iter()
				.collect::<BTreeMap<_, _>>()
				.try_into()
				.expect("impossible; qed;"),
				reward_assets: [BTC, LTC, ETH]
					.into_iter()
					.collect::<BTreeSet<_>>()
					.try_into()
					.expect("impossible; qed;"),
				early_unstake_penalty: penalty,
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
				<StakingRewards as Staking>::claim(&fake_instance_id, &ALICE,),
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
			assert_ok!(StakingRewards::claim(Origin::signed(ALICE), instance_id, ALICE,));
		});
	}

	#[test]
	fn anyone_can_claim() {
		new_test_ext().execute_with(|| {
			configure_default_pica();
			let stake = 1_000_000_000_000;
			assert_ok!(<Tokens as Mutate<AccountId>>::mint_into(PICA, &ALICE, stake));
			let instance_id = <StakingRewards as Staking>::stake(&PICA, &ALICE, stake, WEEK, false)
				.expect("impossible; qed;");
			process_block(REWARD_EPOCH_DURATION_BLOCK);
			assert_ok!(StakingRewards::claim(Origin::signed(TREASURY), instance_id, TREASURY,));
			assert_ok!(StakingRewards::claim(Origin::signed(ALICE), instance_id, ALICE,));
		});
	}
}
