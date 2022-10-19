use crate::{
	ethereum_recover,
	mocks::{
		AccountId, Balance, Balances, CrowdloanRewards, EthKey, ExtBuilder, Moment, Origin, System,
		Test, Timestamp, ALICE, INITIAL_PAYMENT, PROOF_PREFIX, VESTING_STEP,
	},
	models::{Proof, RemoteAccount},
	test_utils::{ethereum_address, generate_accounts, ClaimKey},
	Error, Event, RemoteAccountOf, RewardAmountOf, VestingPeriodOf,
};
use codec::Encode;
use composable_support::types::{EcdsaSignature, EthereumAddress};
use composable_tests_helpers::test::helper::assert_event_with;
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, OriginTrait},
};
use hex_literal::hex;
use sp_core::{ed25519, storage::StateVersion, Pair};

fn with_rewards<R>(
	count: u128,
	reward: Balance,
	vesting_period: Moment,
	execute: impl FnOnce(&dyn Fn(Moment), Vec<(AccountId, ClaimKey)>) -> R,
) -> R {
	let accounts = generate_accounts::<Test>(count as _);
	let rewards = accounts
		.iter()
		.map(|(_, account)| (account.as_remote_public::<Test>(), reward, vesting_period))
		.collect();
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(0xDEADC0DE);
		let random_moment_start = 0xCAFEBABE;
		let set_moment = |x: Moment| Timestamp::set_timestamp(random_moment_start + x);
		set_moment(0);
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), reward * count);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), rewards));
		execute(&set_moment, accounts)
	})
}

const DEFAULT_NB_OF_CONTRIBUTORS: u128 = 100;
const DEFAULT_VESTING_PERIOD: Moment = 3600 * 24 * 7 * 10;
const DEFAULT_REWARD: Balance = 10_000;

fn with_rewards_default<R>(
	execute: impl FnOnce(&dyn Fn(Moment), Vec<(AccountId, ClaimKey)>) -> R,
) -> R {
	with_rewards(DEFAULT_NB_OF_CONTRIBUTORS, DEFAULT_REWARD, DEFAULT_VESTING_PERIOD, execute)
}

pub mod unlock_rewards_for {

	use frame_system::pallet_prelude::OriginFor;
	use sp_runtime::traits::StaticLookup;

	use super::*;

	#[test]
	fn should_set_remove_rewards_lock() {
		with_rewards_default(|_set_moment, accounts| {
			assert_ok!(CrowdloanRewards::initialize(Origin::root()));

			for (picasso_account, remote_account) in accounts.clone().into_iter() {
				assert_ok!(remote_account.associate::<Test>(picasso_account));
			}

			assert!(CrowdloanRewards::remove_reward_locks().is_none());

			let accounts = accounts.into_iter().map(|(account, _claim_key)| account).collect();
			assert_ok!(CrowdloanRewards::unlock_rewards_for(Origin::root(), accounts));
			assert!(CrowdloanRewards::remove_reward_locks().is_some());
		})
	}

	#[test]
	fn test_should_unlock_reward_assets_for_accounts() {
		crate::test_utils::should_unlock_reward_assets_for_accounts::<Test>(
			ExtBuilder::default().build(),
			pallet_balances::Error::<Test>::LiquidityRestrictions.into(),
			DEFAULT_REWARD,
			DEFAULT_NB_OF_CONTRIBUTORS as u64,
			DEFAULT_VESTING_PERIOD,
		);
	}
}

#[test]
fn test_populate_rewards_not_funded() {
	let gen = |c, r| -> Vec<(RemoteAccountOf<Test>, RewardAmountOf<Test>, VestingPeriodOf<Test>)> {
		generate_accounts::<Test>(c)
			.into_iter()
			.map(|(_, account)| (account.as_remote_public::<Test>(), r, DEFAULT_VESTING_PERIOD))
			.collect()
	};
	ExtBuilder::default().build().execute_with(|| {
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), 0);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(100, DEFAULT_REWARD)));
	});
}

#[test]
fn test_incremental_populate() {
	let gen = |c, r| -> Vec<(RemoteAccountOf<Test>, RewardAmountOf<Test>, VestingPeriodOf<Test>)> {
		generate_accounts::<Test>(c)
			.into_iter()
			.map(|(_, account)| (account.as_remote_public::<Test>(), r, DEFAULT_VESTING_PERIOD))
			.collect()
	};
	ExtBuilder::default().build().execute_with(|| {
		let accounts = gen(10_000, DEFAULT_REWARD);
		for i in 0..10 {
			let start = i * 1000;
			let slice = accounts[start..start + 1000].to_vec();
			let expected_total_rewards = (i + 1) as u128 * 1000 * DEFAULT_REWARD;
			Balances::make_free_balance_be(&CrowdloanRewards::account_id(), expected_total_rewards);
			assert_ok!(CrowdloanRewards::populate(Origin::root(), slice));
			assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		}
		// Repopulating using the same accounts must overwrite existing entries.
		let expected_total_rewards = 10_000 * DEFAULT_REWARD;
		for i in 0..10 {
			let start = i * 1000;
			let slice = accounts[start..start + 1000].to_vec();
			assert_ok!(CrowdloanRewards::populate(Origin::root(), slice));
			assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		}
	});
}

#[test]
fn test_populate_ok() {
	let gen = |c, r| -> Vec<(RemoteAccountOf<Test>, RewardAmountOf<Test>, VestingPeriodOf<Test>)> {
		generate_accounts::<Test>(c)
			.into_iter()
			.map(|(_, account)| (account.as_remote_public::<Test>(), r, DEFAULT_VESTING_PERIOD))
			.collect()
	};
	ExtBuilder::default().build().execute_with(|| {
		let expected_total_rewards = 0;
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), expected_total_rewards);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(0, DEFAULT_REWARD)));
		assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);

		let expected_total_rewards = 100 * DEFAULT_REWARD;
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), expected_total_rewards);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(100, DEFAULT_REWARD)));
		assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);

		// Try to repopulate using the same generated accounts
		// In this case, the total shouldn't change as its duplicate
		// No error will be yield but the process should be = identity
		let s = frame_support::storage_root(StateVersion::V1);
		let expected_total_rewards = 100 * DEFAULT_REWARD;
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), expected_total_rewards);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(100, DEFAULT_REWARD)));
		assert_eq!(s, frame_support::storage_root(StateVersion::V1));
		assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);

		// Overwrite rewards + 100 new contributors
		let expected_total_rewards = 200 * (DEFAULT_REWARD + 1);
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), expected_total_rewards);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(200, DEFAULT_REWARD + 1)));
		assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);
	});
}

#[test]
fn populate_should_overwrite_existing_rewards_with_new_values() {
	let gen = |c, r| -> Vec<(RemoteAccountOf<Test>, RewardAmountOf<Test>, VestingPeriodOf<Test>)> {
		generate_accounts::<Test>(c)
			.into_iter()
			.map(|(_, account)| (account.as_remote_public::<Test>(), r, DEFAULT_VESTING_PERIOD))
			.collect()
	};
	ExtBuilder::default().build().execute_with(|| {
		let expected_total_rewards = 100 * 200;
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), expected_total_rewards);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(100, 200)));
		assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);

		let expected_total_rewards = 100 * 100;
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), expected_total_rewards);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(100, 100)));
		assert_eq!(CrowdloanRewards::total_rewards(), expected_total_rewards);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);
	});
}

#[test]
fn test_populate_after_initialize_ko() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		assert_noop!(
			CrowdloanRewards::populate(Origin::root(), vec![]),
			Error::<Test>::AlreadyInitialized
		);
	});
}

#[test]
fn test_initialize_ok() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		assert_eq!(CrowdloanRewards::total_rewards(), 0);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);
	});
}

#[test]
fn test_initialize_at_ok() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CrowdloanRewards::initialize_at(Origin::root(), 10));
		assert_eq!(CrowdloanRewards::total_rewards(), 0);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);
	});
}

#[test]
fn test_initialize_at_ko() {
	ExtBuilder::default().build().execute_with(|| {
		Timestamp::set_timestamp(100);
		assert_noop!(
			CrowdloanRewards::initialize_at(Origin::root(), 99),
			Error::<Test>::BackToTheFuture
		);
	});
}

#[test]
fn test_invalid_early_at_claim() {
	with_rewards_default(|set_moment, accounts| {
		let now = Timestamp::now();
		assert_ok!(CrowdloanRewards::initialize_at(Origin::root(), now + 10));

		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_noop!(
				remote_account.associate::<Test>(picasso_account.clone()),
				Error::<Test>::NotClaimableYet
			);
			assert_noop!(
				remote_account.claim::<Test>(picasso_account),
				Error::<Test>::NotAssociated
			);
		}

		set_moment(11);
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.associate::<Test>(picasso_account.clone()),);
		}
	});
}

#[test]
fn test_initialize_once() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		assert_noop!(
			CrowdloanRewards::initialize(Origin::root()),
			Error::<Test>::AlreadyInitialized
		);
	});
}

#[test]
fn test_not_initialized() {
	with_rewards_default(|_, accounts| {
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_noop!(
				remote_account.associate::<Test>(picasso_account),
				Error::<Test>::NotInitialized
			);
		}
	});
}

#[test]
fn test_initialize_totals() {
	with_rewards_default(|_, _| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		assert_eq!(CrowdloanRewards::total_rewards(), DEFAULT_REWARD * DEFAULT_NB_OF_CONTRIBUTORS);
		assert_eq!(CrowdloanRewards::total_contributors() as u128, DEFAULT_NB_OF_CONTRIBUTORS);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);
	});
}

#[test]
fn initialize_should_fail_when_not_funded() {
	with_rewards_default(|_set_moment, _accounts| {
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), 0);
		assert_noop!(CrowdloanRewards::initialize(Origin::root()), Error::<Test>::RewardsNotFunded);
	});
}

#[test]
fn initialize_should_emit_warning_when_over_funded() {
	with_rewards_default(|_set_moment, _accounts| {
		Balances::make_free_balance_be(
			&CrowdloanRewards::account_id(),
			DEFAULT_REWARD * DEFAULT_NB_OF_CONTRIBUTORS * 2,
		);

		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		assert_eq!(
			assert_event_with::<Test, _, _, _>(|event| match event {
				Event::<Test>::OverFunded { excess_funds } => {
					assert_eq!(excess_funds, DEFAULT_REWARD * DEFAULT_NB_OF_CONTRIBUTORS);
					Some(event)
				},
				_ => None,
			})
			.count(),
			1
		);
	});
}

#[test]
fn test_initial_payment() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.associate::<Test>(picasso_account.clone()));
			assert_eq!(Balances::total_balance(&picasso_account), INITIAL_PAYMENT * DEFAULT_REWARD);
		}
		assert_eq!(
			CrowdloanRewards::claimed_rewards(),
			INITIAL_PAYMENT * DEFAULT_REWARD * DEFAULT_NB_OF_CONTRIBUTORS
		);
	});
}

#[test]
fn test_invalid_early_claim() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.associate::<Test>(picasso_account.clone()));
			assert_noop!(
				remote_account.claim::<Test>(picasso_account),
				Error::<Test>::NothingToClaim
			);
		}
	});
}

#[test]
fn test_not_a_contributor() {
	with_rewards_default(|_, _| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for account in 0..100 {
			assert_noop!(
				ClaimKey::Relay(ed25519::Pair::from_seed(&[account as u8; 32]))
					.associate::<Test>(AccountId::new([account as u8; 32])),
				Error::<Test>::InvalidProof
			);
		}
	});
}

#[test]
fn test_association_ok() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.associate::<Test>(picasso_account));
		}
	});
}

#[test]
fn test_association_ko() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_noop!(
				remote_account.claim::<Test>(picasso_account),
				Error::<Test>::NotAssociated
			);
		}
	});
}

#[test]
fn test_invalid_less_than_a_week() {
	with_rewards_default(|set_moment, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_ok!(remote_account.associate::<Test>(picasso_account));
		}
		set_moment(VESTING_STEP - 1);
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_noop!(
				remote_account.claim::<Test>(picasso_account),
				Error::<Test>::NothingToClaim
			);
		}
		set_moment(VESTING_STEP);
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.claim::<Test>(picasso_account));
		}
	});
}

#[test]
fn test_valid_claim_full() {
	let total_initial_reward = INITIAL_PAYMENT * DEFAULT_NB_OF_CONTRIBUTORS * DEFAULT_REWARD;
	let total_vested_reward = DEFAULT_NB_OF_CONTRIBUTORS * DEFAULT_REWARD - total_initial_reward;
	let nb_of_vesting_step = DEFAULT_VESTING_PERIOD / VESTING_STEP;
	with_rewards_default(|set_moment, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		// Initial payment
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_ok!(remote_account.associate::<Test>(picasso_account));
		}
		assert_eq!(CrowdloanRewards::claimed_rewards(), total_initial_reward);
		for i in 1..(nb_of_vesting_step + 1) {
			set_moment(i * VESTING_STEP);
			for (picasso_account, remote_account) in accounts.clone().into_iter() {
				assert_ok!(remote_account.claim::<Test>(picasso_account));
			}
			assert_eq!(
				CrowdloanRewards::claimed_rewards(),
				total_initial_reward + total_vested_reward * i as u128 / nb_of_vesting_step as u128,
			);
		}
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_noop!(
				remote_account.claim::<Test>(picasso_account),
				Error::<Test>::NothingToClaim
			);
		}
		assert_eq!(CrowdloanRewards::claimed_rewards(), CrowdloanRewards::total_rewards());
	});
}

#[test]
fn test_valid_claim_no_vesting() {
	with_rewards(DEFAULT_NB_OF_CONTRIBUTORS, DEFAULT_REWARD, 0, |_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		// Initial payment = full reward
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.associate::<Test>(picasso_account));
		}
		assert_eq!(CrowdloanRewards::claimed_rewards(), CrowdloanRewards::total_rewards());
	});
}

#[test]
fn test_valid_eth_hardcoded() {
	let eth_address = EthereumAddress(hex!("176FD6F90730E02D2AF55681c65a115C174bA2C7"));
	let eth_account =
		EthKey::parse(&hex!("29134835563739bae90483ee3d80945edf2c87a9b55c9193a694291cfdf23a05"))
			.expect("Hex is valid EthKey; QED");

	assert_eq!(ethereum_address(&eth_account), eth_address);

	// Signed for alice
	// sign(concat("picasso-"), ALICE) = sign(concat("picasso-", [0u8; 32]))
	let eth_proof = EcdsaSignature(hex!("42f2fa6a3db41e6654891e4408ce56ba31fc2b4dea18e82db1c78e33a3f65a55119a23fa7b3fe7a5088197a74a0102266836bb721461b9eaef128bec120db0401c"));

	// Make sure we are able to recover the address
	let recovered_address = ethereum_recover(
		PROOF_PREFIX,
		&ALICE.using_encoded(|x| hex::encode(x).as_bytes().to_vec()),
		&eth_proof,
	);

	assert_eq!(Some(eth_address), recovered_address);

	let reward_amount = DEFAULT_REWARD;
	let rewards =
		vec![(RemoteAccount::Ethereum(eth_address), reward_amount, DEFAULT_VESTING_PERIOD)];

	let proof = Proof::Ethereum(eth_proof);
	ExtBuilder::default().build().execute_with(|| {
		Balances::make_free_balance_be(&CrowdloanRewards::account_id(), reward_amount);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), rewards));
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		assert_ok!(CrowdloanRewards::associate(Origin::none(), ALICE, proof));
		Timestamp::set_timestamp(VESTING_STEP);
		assert_ok!(CrowdloanRewards::claim(Origin::signed(ALICE)));
		Timestamp::set_timestamp(DEFAULT_VESTING_PERIOD);
		assert_ok!(CrowdloanRewards::claim(Origin::signed(ALICE)));
		assert_eq!(CrowdloanRewards::claimed_rewards(), CrowdloanRewards::total_rewards());
	});
}

mod test_prevalidate_association {
	use super::{
		with_rewards, with_rewards_default, ClaimKey, DEFAULT_NB_OF_CONTRIBUTORS,
		DEFAULT_VESTING_PERIOD,
	};
	use crate::{
		mocks::{CrowdloanRewards, Origin, Test},
		ValidityError,
	};
	use frame_support::{
		assert_ok,
		pallet_prelude::{InvalidTransaction, ValidateUnsigned},
		unsigned::TransactionValidity,
	};
	use sp_runtime::{transaction_validity::TransactionSource, AccountId32};

	fn setup_call(remote_account: ClaimKey, reward_account: &AccountId32) -> TransactionValidity {
		let proof = remote_account.proof::<Test>(reward_account.clone());
		let call = crate::Call::associate { reward_account: reward_account.clone(), proof };
		CrowdloanRewards::validate_unsigned(TransactionSource::External, &call)
	}

	#[test]
	fn already_associated_associate_transactions_are_recognized() {
		with_rewards_default(|_, accounts| {
			assert_ok!(CrowdloanRewards::initialize(Origin::root()));

			for (reward_account, remote_account) in accounts.clone() {
				assert_ok!(CrowdloanRewards::associate(
					Origin::none(),
					reward_account.clone(),
					remote_account.proof::<Test>(reward_account.clone()),
				));
			}

			for (reward_account, remote_account) in accounts {
				let validate_result = setup_call(remote_account, &reward_account);
				assert_eq!(
					validate_result,
					Err(InvalidTransaction::Custom(ValidityError::AlreadyAssociated as u8).into())
				);
			}
		});
	}

	#[test]
	fn no_reward_associate_transactions_are_recognized() {
		with_rewards(DEFAULT_NB_OF_CONTRIBUTORS, 0, DEFAULT_VESTING_PERIOD, |_, accounts| {
			assert_ok!(CrowdloanRewards::initialize(Origin::root()));

			for (reward_account, remote_account) in accounts {
				let validate_result = setup_call(remote_account, &reward_account);
				assert_eq!(
					validate_result,
					Err(InvalidTransaction::Custom(ValidityError::NoReward as u8).into())
				);
			}
		});
	}
}
