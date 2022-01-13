use crate::{
	ethereum_recover,
	mocks::{
		ethereum_address, generate_accounts, AccountId, Balance, Balances, BlockNumber, ClaimKey,
		CrowdloanRewards, EthKey, ExtBuilder, Origin, System, Test, ALICE, INITIAL_PAYMENT,
		PROOF_PREFIX, VESTING_STEP, WEEKS,
	},
	models::{EcdsaSignature, EthereumAddress, Proof, RemoteAccount},
	Error, RemoteAccountOf, RewardAmountOf, VestingPeriodOf,
};
use codec::Encode;
use frame_support::{assert_noop, assert_ok, traits::Currency};
use hex_literal::hex;
use sp_core::{ed25519, Pair};

fn with_rewards<R>(
	count: u128,
	reward: Balance,
	vesting_period: BlockNumber,
	execute: impl FnOnce(&dyn Fn(BlockNumber), Vec<(AccountId, ClaimKey)>) -> R,
) -> R {
	let accounts = generate_accounts(count as _);
	let rewards = accounts
		.iter()
		.map(|(_, account)| (account.as_remote_public(), reward, vesting_period))
		.collect();
	ExtBuilder::default().build().execute_with(|| {
		let random_block_start = 0xCAF * WEEKS;
		let set_block = |x: BlockNumber| System::set_block_number(random_block_start + x);
		set_block(0);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), rewards));
		execute(&set_block, accounts)
	})
}

const DEFAULT_NB_OF_CONTRIBUTORS: u128 = 100;
const DEFAULT_VESTING_PERIOD: BlockNumber = 10 * WEEKS;
const DEFAULT_REWARD: Balance = 10_000;

fn with_rewards_default<R>(
	execute: impl FnOnce(&dyn Fn(BlockNumber), Vec<(AccountId, ClaimKey)>) -> R,
) -> R {
	with_rewards(DEFAULT_NB_OF_CONTRIBUTORS, DEFAULT_REWARD, DEFAULT_VESTING_PERIOD, execute)
}

#[test]
fn test_populate_ok() {
	let gen = |c, r| -> Vec<(RemoteAccountOf<Test>, RewardAmountOf<Test>, VestingPeriodOf<Test>)> {
		generate_accounts(c)
			.into_iter()
			.map(|(_, account)| (account.as_remote_public(), r, DEFAULT_VESTING_PERIOD))
			.collect()
	};
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(0, DEFAULT_REWARD)));
		assert_eq!(CrowdloanRewards::total_rewards(), 0);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(100, DEFAULT_REWARD)));
		assert_eq!(CrowdloanRewards::total_rewards(), 100 * DEFAULT_REWARD);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);

		// Try to repopulate using the same generated accounts
		// In this case, the total shouldn't change as its duplicate
		// No error will be yield but the process should be = identity
		let s = frame_support::storage_root();
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(100, DEFAULT_REWARD)));
		assert_eq!(s, frame_support::storage_root());
		assert_eq!(CrowdloanRewards::total_rewards(), 100 * DEFAULT_REWARD);
		assert_eq!(CrowdloanRewards::claimed_rewards(), 0);

		// Overwrite rewards + 100 new contributors
		assert_ok!(CrowdloanRewards::populate(Origin::root(), gen(200, DEFAULT_REWARD + 1)));
		assert_eq!(CrowdloanRewards::total_rewards(), 200 * (DEFAULT_REWARD + 1));
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
			assert_noop!(remote_account.associate(picasso_account), Error::<Test>::NotInitialized);
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
fn test_initial_payment() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.associate(picasso_account.clone()));
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
			assert_ok!(remote_account.associate(picasso_account.clone()));
			assert_noop!(remote_account.claim(picasso_account), Error::<Test>::NothingToClaim);
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
					.associate(AccountId::new([account as u8; 32])),
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
			assert_ok!(remote_account.associate(picasso_account));
		}
	});
}

#[test]
fn test_association_ko() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_noop!(remote_account.claim(picasso_account), Error::<Test>::NotAssociated);
		}
	});
}

#[test]
fn test_invalid_less_than_a_week() {
	with_rewards_default(|set_block, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_ok!(remote_account.associate(picasso_account));
		}
		set_block(VESTING_STEP - 1);
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_noop!(remote_account.claim(picasso_account), Error::<Test>::NothingToClaim);
		}
		set_block(VESTING_STEP);
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_ok!(remote_account.claim(picasso_account));
		}
	});
}

#[test]
fn test_valid_claim_full() {
	let total_initial_reward = INITIAL_PAYMENT * DEFAULT_NB_OF_CONTRIBUTORS * DEFAULT_REWARD;
	let total_vested_reward = DEFAULT_NB_OF_CONTRIBUTORS * DEFAULT_REWARD - total_initial_reward;
	let nb_of_vesting_step = DEFAULT_VESTING_PERIOD / VESTING_STEP;
	with_rewards_default(|set_block, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		// Initial payment
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_ok!(remote_account.associate(picasso_account));
		}
		assert_eq!(CrowdloanRewards::claimed_rewards(), total_initial_reward);
		for i in 1..(nb_of_vesting_step + 1) {
			set_block(i * VESTING_STEP);
			for (picasso_account, remote_account) in accounts.clone().into_iter() {
				assert_ok!(remote_account.claim(picasso_account));
			}
			assert_eq!(
				CrowdloanRewards::claimed_rewards(),
				total_initial_reward + total_vested_reward * i as u128 / nb_of_vesting_step as u128,
			);
		}
		for (picasso_account, remote_account) in accounts.into_iter() {
			assert_noop!(remote_account.claim(picasso_account), Error::<Test>::NothingToClaim);
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
			assert_ok!(remote_account.associate(picasso_account));
		}
		assert_eq!(CrowdloanRewards::claimed_rewards(), CrowdloanRewards::total_rewards());
	});
}

#[test]
fn test_valid_eth_hardcoded() {
	let eth_address = EthereumAddress(hex!("176FD6F90730E02D2AF55681c65a115C174bA2C7"));
	let eth_account =
		EthKey::parse(&hex!("29134835563739bae90483ee3d80945edf2c87a9b55c9193a694291cfdf23a05"))
			.unwrap();

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

	let rewards =
		vec![(RemoteAccount::Ethereum(eth_address), DEFAULT_REWARD, DEFAULT_VESTING_PERIOD)];

	let proof = Proof::Ethereum(eth_proof);
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CrowdloanRewards::populate(Origin::root(), rewards));
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		assert_ok!(CrowdloanRewards::associate(Origin::none(), ALICE, proof));
		System::set_block_number(VESTING_STEP);
		assert_ok!(CrowdloanRewards::claim(Origin::signed(ALICE)));
		System::set_block_number(DEFAULT_VESTING_PERIOD);
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
		mocks::{Call, CrowdloanRewards, Origin, Test},
		PrevalidateAssociation, ValidityError,
	};

	use frame_support::{
		assert_ok,
		dispatch::{Dispatchable, GetDispatchInfo},
		pallet_prelude::InvalidTransaction,
		unsigned::TransactionValidity,
		weights::Pays,
	};
	use sp_runtime::{traits::SignedExtension, AccountId32};

	fn setup_call(
		remote_account: ClaimKey,
		reward_account: &AccountId32,
	) -> (TransactionValidity, Call) {
		let proof = remote_account.proof(reward_account.clone());
		let call = Call::CrowdloanRewards(crate::Call::associate {
			reward_account: reward_account.clone(),
			proof,
		});
		let dispatch_info = call.get_dispatch_info();
		let validate_result = PrevalidateAssociation::<Test>::new().validate(
			reward_account,
			&call,
			&dispatch_info,
			0,
		);
		(validate_result, call)
	}

	#[test]
	fn already_associated_associate_transactions_are_recognized() {
		with_rewards_default(|_, accounts| {
			assert_ok!(CrowdloanRewards::initialize(Origin::root()));

			for (reward_account, remote_account) in accounts.clone() {
				assert_ok!(CrowdloanRewards::associate(
					Origin::none(),
					reward_account.clone(),
					remote_account.proof(reward_account.clone()),
				));
			}

			for (reward_account, remote_account) in accounts {
				let (validate_result, call) = setup_call(remote_account, &reward_account);

				assert_eq!(
					validate_result,
					Err(InvalidTransaction::Custom(ValidityError::AlreadyAssociated as u8).into())
				);

				// make sure that invalid transactions are not free
				assert!(matches!(
					call.dispatch(Origin::root()),
					Err(sp_runtime::DispatchErrorWithPostInfo {
						post_info: frame_support::dispatch::PostDispatchInfo {
							actual_weight: _,
							pays_fee: Pays::Yes
						},
						error: _
					})
				));
			}
		});
	}

	#[test]
	fn no_reward_associate_transactions_are_recognized() {
		with_rewards(DEFAULT_NB_OF_CONTRIBUTORS, 0, DEFAULT_VESTING_PERIOD, |_, accounts| {
			assert_ok!(CrowdloanRewards::initialize(Origin::root()));

			for (reward_account, remote_account) in accounts {
				let (validate_result, call) = setup_call(remote_account, &reward_account);

				assert_eq!(
					validate_result,
					Err(InvalidTransaction::Custom(ValidityError::NoReward as u8).into())
				);

				// make sure that invalid transactions are not free
				assert!(matches!(
					call.dispatch(Origin::root()),
					Err(sp_runtime::DispatchErrorWithPostInfo {
						post_info: frame_support::dispatch::PostDispatchInfo {
							actual_weight: _,
							pays_fee: Pays::Yes
						},
						error: _
					})
				));
			}
		});
	}
}
