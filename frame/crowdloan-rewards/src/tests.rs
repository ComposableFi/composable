use crate::{
	ethereum_recover, ethereum_signable_message,
	mocks::{
		AccountId, Balance, Balances, BlockNumber, CrowdloanRewards, ExtBuilder, Origin,
		RelayChainAccountId, System, Test, ACCOUNT_FREE_START, ALICE, BOB, CHARLIE, DAYS,
		INITIAL_PAYMENT, MINIMUM_BALANCE, PROOF_PREFIX, VESTING_STEP, WEEKS,
	},
	models::{EcdsaSignature, EthereumAddress, Proof, RemoteAccount},
	verify_relay, Error, RemoteAccountOf, RewardAmountOf, VestingPeriodOf,
};
use codec::Encode;
use frame_support::{
	assert_noop, assert_ok,
	dispatch::{DispatchResult, DispatchResultWithPostInfo},
	traits::Currency,
};
use hex_literal::hex;
use sp_core::{ed25519, keccak_256, Pair};
use sp_runtime::MultiSignature;

type RelayKey = ed25519::Pair;
type EthKey = libsecp256k1::SecretKey;

#[derive(Clone)]
enum ClaimKey {
	Relay(RelayKey),
	Eth(EthKey),
}

impl ClaimKey {
	fn as_remote_public(&self) -> RemoteAccount<RelayChainAccountId> {
		match self {
			ClaimKey::Relay(relay_account) =>
				RemoteAccount::RelayChain(relay_account.public().into()),
			ClaimKey::Eth(ethereum_account) =>
				RemoteAccount::Ethereum(ethereum_address(ethereum_account)),
		}
	}
	fn claim(&self, reward_account: AccountId) -> DispatchResultWithPostInfo {
		CrowdloanRewards::claim(Origin::signed(reward_account))
	}
	fn associate(&self, reward_account: AccountId) -> DispatchResultWithPostInfo {
		let proof = match self {
			ClaimKey::Relay(relay_account) => relay_proof(relay_account, reward_account),
			ClaimKey::Eth(ethereum_account) => ethereum_proof(ethereum_account, reward_account),
		};
		CrowdloanRewards::associate(Origin::root(), reward_account, proof)
	}
}

fn relay_proof(relay_account: &RelayKey, reward_account: AccountId) -> Proof<RelayChainAccountId> {
	let mut msg = b"<Bytes>".to_vec();
	msg.append(&mut PROOF_PREFIX.to_vec());
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	msg.append(&mut b"</Bytes>".to_vec());
	Proof::RelayChain(relay_account.public().into(), relay_account.sign(&msg).into())
}

fn ethereum_proof(
	ethereum_account: &EthKey,
	reward_account: AccountId,
) -> Proof<RelayChainAccountId> {
	let msg = keccak_256(
		&ethereum_signable_message(
			PROOF_PREFIX,
			&reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()),
		)[..],
	);
	let (sig, recovery_id) =
		libsecp256k1::sign(&libsecp256k1::Message::parse(&msg), ethereum_account);
	let mut r = [0u8; 65];
	r[0..64].copy_from_slice(&sig.serialize()[..]);
	r[64] = recovery_id.serialize();
	Proof::Ethereum(EcdsaSignature(r))
}

fn ethereum_public(secret: &EthKey) -> libsecp256k1::PublicKey {
	libsecp256k1::PublicKey::from_secret_key(secret)
}

fn ethereum_address(secret: &EthKey) -> EthereumAddress {
	let mut res = EthereumAddress::default();
	res.0
		.copy_from_slice(&keccak_256(&ethereum_public(secret).serialize()[1..65])[12..]);
	res
}

fn relay_generate(count: u64) -> Vec<(AccountId, ClaimKey)> {
	let seed: u128 = 12345678901234567890123456789012;
	(0..count)
		.map(|i| {
			(
				ACCOUNT_FREE_START + i,
				ClaimKey::Relay(ed25519::Pair::from_seed(
					(seed + i as u128).to_string().as_bytes().try_into().unwrap(),
				)),
			)
		})
		.collect()
}

fn ethereum_generate(count: u64) -> Vec<(AccountId, ClaimKey)> {
	(0..count)
		.map(|i| {
			(
				AccountId::MAX - i,
				ClaimKey::Eth(EthKey::parse(&keccak_256(&i.to_le_bytes())).unwrap()),
			)
		})
		.collect()
}

fn generate_accounts(count: u64) -> Vec<(AccountId, ClaimKey)> {
	let mut x = relay_generate(count / 2);
	let mut y = ethereum_generate(count / 2);
	x.append(&mut y);
	x
}

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
		let random_block_start = 0xCAFEBEEF * WEEKS;
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
			assert_ok!(remote_account.associate(picasso_account));
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
			assert_ok!(remote_account.associate(picasso_account));
			assert_noop!(remote_account.claim(picasso_account), Error::<Test>::NothingToClaim);
		}
	});
}

#[test]
fn test_not_a_contributor() {
	with_rewards_default(|_, _| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for account in 0..ACCOUNT_FREE_START {
			assert_noop!(
				ClaimKey::Relay(ed25519::Pair::from_seed(&[account as u8; 32])).associate(account),
				Error::<Test>::InvalidProof
			);
		}
	});
}

#[test]
fn test_association_ok() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_ok!(remote_account.associate(picasso_account));
		}
	});
}

#[test]
fn test_association_ko() {
	with_rewards_default(|_, accounts| {
		assert_ok!(CrowdloanRewards::initialize(Origin::root()));
		for (picasso_account, remote_account) in accounts.clone().into_iter() {
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
	let eth_proof = EcdsaSignature(hex!("1a26960da5f53c369582268dc59465cdb29911daefe780451e255a2b8f5a253b67cc5208fc3fc489ec8e479670d7330b63e758cbf600c6a87e5d14c0559484e11b"));

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
		assert_ok!(CrowdloanRewards::associate(Origin::root(), ALICE, proof));
		System::set_block_number(VESTING_STEP);
		assert_ok!(CrowdloanRewards::claim(Origin::signed(ALICE)));
		System::set_block_number(DEFAULT_VESTING_PERIOD);
		assert_ok!(CrowdloanRewards::claim(Origin::signed(ALICE)));
		assert_eq!(CrowdloanRewards::claimed_rewards(), CrowdloanRewards::total_rewards());
	});
}
