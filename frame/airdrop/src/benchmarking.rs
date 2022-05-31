#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Airdrop;
use crate::models::{Proof, RemoteAccount};
use crate::{AccountIdOf, Call, Config, Pallet, ProofOf, RemoteAccountOf};
use composable_support::types::{EcdsaSignature, EthereumAddress};
use composable_traits::airdrop::AirdropManagement;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::pallet_prelude::*;
use frame_system::{Pallet as System, RawOrigin};
use sp_core::{ed25519, keccak_256, Pair};
use sp_runtime::{traits::One, AccountId32};
use sp_std::prelude::*;

pub type EthKey = libsecp256k1::SecretKey;
pub type RelayKey = ed25519::Pair;

pub type BlockNumber = u32;
pub type Moment = u32;

pub const PROOF_PREFIX: &[u8] = b"picasso-";
pub const VESTING_STEP: Moment = 3600 * 24 * 7;
pub const VESTING_PERIOD: BlockNumber = 48 * WEEKS;

const MILLISECS_PER_BLOCK: u64 = 6000;
const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
const HOURS: BlockNumber = MINUTES * 60;
const DAYS: BlockNumber = HOURS * 24;
const WEEKS: BlockNumber = DAYS * 7;

#[derive(Clone)]
pub enum ClaimKey {
	Relay(RelayKey),
	Eth(EthKey),
}

impl ClaimKey {
	pub fn as_remote_public<T>(&self) -> RemoteAccountOf<T> 
    where
        T: Config<RelayChainAccountId = [u8; 32]>,
    {
		match self {
			ClaimKey::Relay(relay_account) =>
				RemoteAccount::RelayChain(*(relay_account.public().as_array_ref())),
			ClaimKey::Eth(eth_account) => RemoteAccount::Ethereum(ethereum_address(eth_account)),
		}
	}

	pub fn proof<T>(self, reward_account: AccountIdOf<T>) -> ProofOf<T> 
    where
        T: Config<RelayChainAccountId = [u8; 32]>,
    {
		match self {
			ClaimKey::Relay(relay) => relay_proof::<T>(&relay, reward_account),
			ClaimKey::Eth(eth) => ethereum_proof::<T>(&eth, reward_account),
		}
	}
}

fn relay_proof<T>(relay_account: &RelayKey, reward_account: AccountIdOf<T>) -> ProofOf<T> 
where 
    T: Config<RelayChainAccountId = [u8; 32]>,
{
	let mut msg = b"<Bytes>".to_vec();
	msg.append(&mut PROOF_PREFIX.to_vec());
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	msg.append(&mut b"</Bytes>".to_vec());
	Proof::RelayChain(*(relay_account.public().as_array_ref()), relay_account.sign(&msg).into())
}

pub fn ethereum_proof<T>(
	ethereum_account: &EthKey,
	reward_account: AccountIdOf<T>,
) -> ProofOf<T> 
where 
    T: Config<RelayChainAccountId = [u8; 32]>,
{
	let msg = keccak_256(
		&Airdrop::<T>::ethereum_signable_message(
			PROOF_PREFIX,
			&reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()),
		)[..],
	);
	let (sig, recovery_id) =
		libsecp256k1::sign(&libsecp256k1::Message::parse(&msg), ethereum_account);
	let mut recovered_signature = [0_u8; 65];

	recovered_signature[0..64].copy_from_slice(&sig.serialize()[..]);
	recovered_signature[64] = recovery_id.serialize();
	Proof::Ethereum(EcdsaSignature(recovered_signature))
}

pub fn ethereum_public(secret: &EthKey) -> libsecp256k1::PublicKey {
	libsecp256k1::PublicKey::from_secret_key(secret)
}

pub fn ethereum_address(secret: &EthKey) -> EthereumAddress {
	let mut res = EthereumAddress::default();
	res.0
		.copy_from_slice(&keccak_256(&ethereum_public(secret).serialize()[1..65])[12..]);
	res
}

pub fn relay_generate<T>(count: u64) -> Vec<(AccountIdOf<T>, ClaimKey)> 
where
    T: Config<RelayChainAccountId = [u8; 32]>,
{
	let seed: u128 = 12345678901234567890123456789012;
	(0..count)
		.map(|i| {
			let account_id = account("recipient", i as u32, 0xCAFEBABE);
			(
                account_id,
				ClaimKey::Relay(ed25519::Pair::from_seed(&keccak_256(
					&[(&(seed + i as u128)).to_le_bytes(), (&(seed + i as u128)).to_le_bytes()]
						.concat(),
				))),
			)
		})
		.collect()
}

pub fn ethereum_generate<T>(count: u64) -> Vec<(AccountIdOf<T>, ClaimKey)> 
where
    T: Config<RelayChainAccountId = [u8; 32]>,
{
	(0..count)
		.map(|i| {
			let account_id = account("recipient", i as u32, 0xCAFEBABE);
			(
				account_id,
				ClaimKey::Eth(EthKey::parse(&keccak_256(&i.to_le_bytes())).unwrap()),
			)
		})
		.collect()
}

pub fn generate_accounts<T>(count: u64) -> Vec<(AccountIdOf<T>, ClaimKey)> 
where
    T: Config<RelayChainAccountId = [u8; 32]>,
{
	let mut x = relay_generate::<T>(count / 2);
	let mut y = ethereum_generate::<T>(count / 2);
	x.append(&mut y);
	x
}

benchmarks! {
    where_clause {
        where 
            T: Config<RelayChainAccountId = [u8; 32]>,
            BalanceOf<T>: From<u128>,
    }

	create_airdrop_benchmark {
		let x in 100..1000;
        let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
	}: create_airdrop(RawOrigin::Signed(creator), None, VESTING_STEP.into())

	add_recipient_benchmark {
		let x in 100..1000;
		let accounts: Vec<(RemoteAccountOf<T>, BalanceOf<T>, bool)> = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), false)).collect();
		let airdrop_id = T::AirdropId::one();
        let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as AirdropManagement>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
	}: add_recipient(RawOrigin::Signed(creator), airdrop_id, accounts)

	remove_recipient_benchmark {
		let x in 100..1000;
		let accounts: Vec<(RemoteAccountOf<T>, BalanceOf<T>, bool)> = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), false)).collect();
		let airdrop_id = T::AirdropId::one();
        let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as AirdropManagement>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
		<Airdrop<T> as AirdropManagement>::add_recipient(creator.clone(), airdrop_id, accounts.clone())?;
	}: remove_recipient(RawOrigin::Signed(creator), airdrop_id, accounts[0 as usize].0.clone())

	enable_airdrop_benchmark {
		let x in 100..1000;
		let accounts = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), false)).collect();
		let airdrop_id = T::AirdropId::one();
        let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as AirdropManagement>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?; 
		<Airdrop<T> as AirdropManagement>::add_recipient(creator.clone(), airdrop_id, accounts)?;
	}: enable_airdrop(RawOrigin::Signed(creator), airdrop_id)

	disable_airdrop_benchmark {
		let x in 100..1000;
		let accounts = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), false)).collect();
		let airdrop_id = T::AirdropId::one();
        let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as AirdropManagement>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
		<Airdrop<T> as AirdropManagement>::add_recipient(creator.clone(), airdrop_id, accounts)?;
	}: disable_airdrop(RawOrigin::Signed(creator), airdrop_id)

	claim_benchmark {
		let x in 100..1000;
		let accounts = generate_accounts::<T>(x as _);
		let remote_accounts = accounts.clone().into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), false)).collect();
		let airdrop_id = T::AirdropId::one();
        let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as AirdropManagement>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
		<Airdrop<T> as AirdropManagement>::add_recipient(creator, airdrop_id, remote_accounts)?;
        let reward_account = AccountIdOf::<T>::from(accounts[0 as usize].0.clone().into());
		System::<T>::set_block_number(VESTING_PERIOD.into());
	}: claim(RawOrigin::None, airdrop_id, reward_account, accounts[0 as usize].1.clone().proof::<T>(accounts[0 as usize].0.clone()))
}

impl_benchmark_test_suite!(Airdrop, crate::mocks::ExtBuilder::default().build(), crate::mocks::MockRuntime);
