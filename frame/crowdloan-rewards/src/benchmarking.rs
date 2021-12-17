use super::*;

use crate::{models::*, Pallet as CrowdloanReward};
use codec::Encode;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use sp_core::{ed25519, keccak_256, Pair};
use sp_runtime::AccountId32;
use sp_std::prelude::*;

type RelayKey = ed25519::Pair;
type EthKey = libsecp256k1::SecretKey;
type BlockNumber = u32;
type Balance = u128;
type AccountId = AccountId32;
type RelayChainAccountId = [u8; 32];

const MILLISECS_PER_BLOCK: u64 = 6000;
const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
const HOURS: BlockNumber = MINUTES * 60;
const DAYS: BlockNumber = HOURS * 24;
const WEEKS: BlockNumber = DAYS * 7;

pub const ALICE: AccountId = AccountId32::new([0; 32]);

const VESTING_PERIOD: BlockNumber = 48 * WEEKS;

fn relay_proof<T: Config>(
	relay_account: &RelayKey,
	reward_account: AccountId,
) -> Proof<RelayChainAccountId> {
	let mut msg = b"<Bytes>".to_vec();
	msg.append(&mut T::Prefix::get().to_vec());
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	msg.append(&mut b"</Bytes>".to_vec());
	Proof::RelayChain(relay_account.public().into(), relay_account.sign(&msg).into())
}

fn ethereum_proof<T: Config>(
	ethereum_account: &EthKey,
	reward_account: AccountId,
) -> Proof<RelayChainAccountId> {
	let msg = keccak_256(
		&ethereum_signable_message(
			T::Prefix::get(),
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
			let account_id = [[0u8; 16], (&(i as u128)).to_le_bytes()].concat().try_into().unwrap();
			(
				AccountId::new(account_id),
				ClaimKey::Relay(ed25519::Pair::from_seed(&keccak_256(
					&[(&(seed + i as u128)).to_le_bytes(), (&(seed + i as u128)).to_le_bytes()]
						.concat(),
				))),
			)
		})
		.collect()
}

fn ethereum_generate(count: u64) -> Vec<(AccountId, ClaimKey)> {
	(0..count)
		.map(|i| {
			let account_id = [(&(i as u128)).to_le_bytes(), [0u8; 16]].concat().try_into().unwrap();

			(
				AccountId::new(account_id),
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
	fn sign<T: Config>(&self, reward_account: AccountId) -> Proof<RelayChainAccountId> {
		match self {
			ClaimKey::Relay(relay_account) => relay_proof::<T>(relay_account, reward_account),
			ClaimKey::Eth(ethereum_account) =>
				ethereum_proof::<T>(ethereum_account, reward_account),
		}
	}
}

benchmarks! {
  where_clause {
	  where
		T: frame_system::Config<BlockNumber = BlockNumber>,
	  T: Config<Balance = Balance, RelayChainAccountId = RelayChainAccountId, AccountId = AccountId>,
  }

  populate {
		let x in 1000..5000;
		  let accounts =
				generate_accounts(x as _)
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
  }: _(RawOrigin::Root, accounts)

	initialize {
		  let x in 1000..5000;
		  let accounts =
			  generate_accounts(x as _)
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
		  CrowdloanReward::<T>::do_populate(accounts)?;
  }: _(RawOrigin::Root)

  associate {
		  let x in 1000..5000;
		  let accounts =
				generate_accounts(x as _);
		  let accounts_reward = accounts.clone()
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
		  CrowdloanReward::<T>::do_populate(accounts_reward)?;
			CrowdloanReward::<T>::do_initialize()?;
		  frame_system::Pallet::<T>::set_block_number(VESTING_PERIOD);
	}: _(RawOrigin::Root, ALICE, accounts[0 as usize].1.sign::<T>(ALICE))

  claim {
		  let x in 1000..5000;
		  let accounts =
				  generate_accounts(x as _);
		  let accounts_reward = accounts.clone()
				  .into_iter()
				  .map(|(_, a)| (a.as_remote_public(), 1_000_000_000_000, VESTING_PERIOD)).collect();
		  CrowdloanReward::<T>::do_populate(accounts_reward)?;
			CrowdloanReward::<T>::do_initialize()?;
		  for (reward_account, remote_account) in accounts.clone().into_iter() {
			  CrowdloanReward::<T>::do_associate(reward_account.clone(), remote_account.sign::<T>(reward_account))?;
		  }
		  frame_system::Pallet::<T>::set_block_number(VESTING_PERIOD);
	}: _(RawOrigin::Signed(accounts[0 as usize].0.clone()))
}

impl_benchmark_test_suite!(
	CrowdloanReward,
	crate::mocks::ExtBuilder { balances: vec![(crate::benchmarking::ALICE, 1_000_000_000_000)] }
		.build(),
	crate::mocks::Test,
);
