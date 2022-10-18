use crate::*;

use crate::{
	models::{Proof, RemoteAccount},
	tests::unlock_rewards_for::should_unlock_reward_assets_for_accounts,
};
use composable_support::types::{EcdsaSignature, EthereumAddress};
use ed25519_dalek::{Keypair, Signer};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::{
	pallet_prelude::*,
	traits::{fungible::Mutate, OriginTrait},
};
use frame_system::RawOrigin;
use sp_io::hashing::keccak_256;
use sp_runtime::{traits::StaticLookup, AccountId32};
use sp_std::prelude::*;

type RelayKey = [u8; 32];
type EthKey = libsecp256k1::SecretKey;
type Moment = u64;
type Balance = u128;
type AccountId = AccountId32;
type RelayChainAccountId = [u8; 32];

const PROOF_PREFIX: &[u8] = b"picasso-";
const MILLISECS_PER_BLOCK: u64 = 6000;
const MINUTES: Moment = 60_000 / (MILLISECS_PER_BLOCK as Moment);
const HOURS: Moment = MINUTES * 60;
const DAYS: Moment = HOURS * 24;
const WEEKS: Moment = DAYS * 7;

const VESTING_PERIOD: Moment = 48 * WEEKS;
const ACCOUNT_REWARD: Balance = 1_000_000_000_000;

#[derive(Clone)]
enum ClaimKey {
	Relay(RelayKey),
	Eth(EthKey),
}

impl ClaimKey {
	pub fn as_remote_public(&self) -> RemoteAccount<RelayChainAccountId> {
		match self {
			ClaimKey::Relay(relay_account) =>
				RemoteAccount::RelayChain(*pair_from_seed(*relay_account).public.as_bytes()),
			ClaimKey::Eth(ethereum_account) =>
				RemoteAccount::Ethereum(ethereum_address(ethereum_account)),
		}
	}
	pub fn proof(self, reward_account: AccountId32) -> Proof<[u8; 32]> {
		match self {
			ClaimKey::Relay(relay) => relay_proof(&relay, reward_account),
			ClaimKey::Eth(eth) => ethereum_proof(&eth, reward_account),
		}
	}
}

fn relay_proof(relay_account: &RelayKey, reward_account: AccountId) -> Proof<RelayChainAccountId> {
	let mut msg = b"<Bytes>".to_vec();
	msg.append(&mut PROOF_PREFIX.to_vec());
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	msg.append(&mut b"</Bytes>".to_vec());

	let pair = pair_from_seed(*relay_account);

	Proof::RelayChain(
		*pair.public.as_bytes(),
		sp_core::ed25519::Signature::from_raw(pair.sign(&msg).to_bytes()).into(),
	)
}

fn ethereum_proof(
	ethereum_account: &EthKey,
	reward_account: AccountId,
) -> Proof<RelayChainAccountId> {
	let msg = keccak_256(
		&crate::ethereum_signable_message(
			PROOF_PREFIX,
			&reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()),
		)[..],
	);
	let (sig, recovery_id) =
		libsecp256k1::sign(&libsecp256k1::Message::parse(&msg), ethereum_account);
	let mut r = [0_u8; 65];
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
			let seed_32 = keccak_256(
				&[(seed + i as u128).to_le_bytes(), (seed + i as u128).to_le_bytes()].concat(),
			);
			let account_id = pair_from_seed(seed_32).public.to_bytes();
			(AccountId::new(account_id), ClaimKey::Relay(seed_32))
		})
		.collect()
}

fn pair_from_seed(seed: [u8; 32]) -> Keypair {
	let secret = ed25519_dalek::SecretKey::from_bytes(&seed).expect("Seed is valid; QED");
	let public = ed25519_dalek::PublicKey::from(&secret);

	Keypair { secret, public }
}

fn ethereum_generate(count: u64) -> Vec<(AccountId, ClaimKey)> {
	(0..count)
		.map(|i| {
			let account_id = [(i as u128 + 1).to_le_bytes(), [0_u8; 16]]
				.concat()
				.try_into()
				.expect("Account ID is valid; QED");
			(
				AccountId::new(account_id),
				ClaimKey::Eth(
					EthKey::parse(&keccak_256(&i.to_le_bytes())).expect("EthKey is valid; QED"),
				),
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

benchmarks! {
	where_clause {
		where
			T: pallet_timestamp::Config<Moment = Moment>,
			T: Config<
				Balance = Balance,
				Moment = Moment,
				RelayChainAccountId = RelayChainAccountId,
				AccountId = AccountId,
				Time = pallet_timestamp::Pallet<T>,
			>,
			T: pallet_balances::Config<Balance = u128>,
			<<T as frame_system::Config>::Origin as OriginTrait>::AccountId: From<AccountId>,
			<<T as frame_system::Config>::Lookup as StaticLookup>::Source: From<AccountId>,
			<T as frame_system::Config>::AccountId: From<AccountId>,
	}

	populate {
		let x in 100..1000;
		let accounts =
			generate_accounts(x as _)
			.into_iter()
			.map(|(_, a)| (a.as_remote_public(), ACCOUNT_REWARD, VESTING_PERIOD)).collect();
	}: _(RawOrigin::Root, accounts)

	initialize {
		let x in 100..1000;
		let accounts =
			generate_accounts (x as _)
			.into_iter()
			.map(|(_, a)| (a.as_remote_public(), ACCOUNT_REWARD, VESTING_PERIOD)).collect();

		<T::RewardAsset as Mutate<AccountId>>::mint_into(
			&Pallet::<T>::account_id(),
			ACCOUNT_REWARD * x as Balance
		)?;

		Pallet::<T>::do_populate(accounts)?;
	}: _(RawOrigin::Root)

	associate {
		let x in 100..1000;
		let accounts =
			generate_accounts(x as _);
		let accounts_reward = accounts.clone()
			.into_iter()
			.map(|(_, a)| (a.as_remote_public(), ACCOUNT_REWARD, VESTING_PERIOD)).collect();

		<T::RewardAsset as Mutate<AccountId>>::mint_into(
			&Pallet::<T>::account_id(),
			ACCOUNT_REWARD * x as Balance
		)?;
		Pallet::<T>::populate(RawOrigin::Root.into(), accounts_reward)?;
		Pallet::<T>::initialize(RawOrigin::Root.into())?;
	}: _(RawOrigin::None, accounts[0].0.clone(), accounts[0].1.clone().proof(accounts[0].0.clone()))

	claim {
		let x in 100..1000;
		let accounts =
			generate_accounts(x as _);
		let accounts_reward = accounts.clone()
			.into_iter()
			.map(|(_, a)| (a.as_remote_public(), ACCOUNT_REWARD, VESTING_PERIOD)).collect();

		Pallet::<T>::populate(RawOrigin::Root.into(), accounts_reward)?;

		<T::RewardAsset as Mutate<AccountId>>::mint_into(
			&Pallet::<T>::account_id(),
			ACCOUNT_REWARD * x as Balance
		)?;
		Pallet::<T>::initialize(RawOrigin::Root.into())?;

		for (reward_account, remote_account) in accounts.clone() {
			Pallet::<T>::associate(
				RawOrigin::None.into(),
				reward_account.clone(),
				remote_account.proof(reward_account)

			)?;
		}

		T::Time::set_timestamp(VESTING_PERIOD);
	}: _(RawOrigin::Signed(accounts[0].0.clone()))

	unlock_rewards_for {
		let x in 100..1000;

		let accounts =
			generate_accounts(x as _);
		let accounts_reward = accounts.clone()
			.into_iter()
			.map(|(_, a)| (a.as_remote_public(), ACCOUNT_REWARD, VESTING_PERIOD)).collect();

		Pallet::<T>::populate(RawOrigin::Root.into(), accounts_reward)?;

		<T::RewardAsset as Mutate<AccountId>>::mint_into(
			&Pallet::<T>::account_id(),
			ACCOUNT_REWARD * x as Balance
		)?;
		Pallet::<T>::initialize(RawOrigin::Root.into())?;

		for (reward_account, remote_account) in accounts.clone() {
			Pallet::<T>::associate(
				RawOrigin::None.into(),
				reward_account.clone(),
				remote_account.proof(reward_account)

			)?;
		}

		let reward_accounts = accounts.into_iter().map(|(account_id, _)| account_id).collect();
	}: _(RawOrigin::Root, reward_accounts)

	test {
		should_unlock_reward_assets_for_accounts::<T>();
	}: {}

	impl_benchmark_test_suite!(
		Pallet,
		crate::mocks::ExtBuilder::default().build(),
		crate::mocks::Test
	);
}
