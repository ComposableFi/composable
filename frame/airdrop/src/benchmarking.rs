#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{
	models::Proof, AccountIdOf, Call, Config, IdentityOf, Pallet as Airdrop, Pallet, ProofOf,
};
use composable_support::types::{
	CosmosAddress, CosmosEcdsaSignature, EcdsaSignature, EthereumAddress,
};
use composable_traits::airdrop::Airdropper;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::pallet_prelude::*;
use frame_system::{Pallet as System, RawOrigin};
use multihash::{Hasher, Keccak256, Sha2_256};
use p256::ecdsa::{signature::Signer, SigningKey, VerifyingKey};
use sp_runtime::traits::One;
use sp_std::prelude::*;

pub type EthereumKey = libsecp256k1::SecretKey;
pub type CosmosKey = SigningKey;

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
pub enum Identity {
	Cosmos(CosmosKey),
	Ethereum(EthereumKey),
}

impl Identity {
	pub fn as_remote_public<T>(&self) -> IdentityOf<T>
	where
		T: Config<RelayChainAccountId = [u8; 32]>,
	{
		match self {
			Identity::Cosmos(cosmos_account) => crate::models::Identity::Cosmos(cosmos_address(
				VerifyingKey::from(cosmos_account).to_encoded_point(true).as_bytes(),
			)),
			Identity::Ethereum(eth_account) =>
				crate::models::Identity::Ethereum(ethereum_address(eth_account)),
		}
	}

	pub fn proof<T>(self, reward_account: AccountIdOf<T>) -> ProofOf<T>
	where
		T: Config<RelayChainAccountId = [u8; 32]>,
	{
		match self {
			Identity::Cosmos(cosmos) => cosmos_proof::<T>(&cosmos, reward_account),
			Identity::Ethereum(eth) => ethereum_proof::<T>(&eth, reward_account),
		}
	}
}

fn cosmos_proof<T>(cosmos_account: &CosmosKey, reward_account: AccountIdOf<T>) -> ProofOf<T>
where
	T: Config<RelayChainAccountId = [u8; 32]>,
{
	let mut msg = PROOF_PREFIX.to_vec();
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	let cosmos_address =
		cosmos_address(VerifyingKey::from(cosmos_account).to_encoded_point(true).as_bytes());
	let mut sig: [u8; 64] = [0; 64];
	sig.copy_from_slice(cosmos_account.sign(&hash::<Sha2_256>(&msg)).to_vec().as_slice());
	Proof::Cosmos(cosmos_address, CosmosEcdsaSignature(sig))
}

pub fn ethereum_proof<T>(
	ethereum_account: &EthereumKey,
	reward_account: AccountIdOf<T>,
) -> ProofOf<T>
where
	T: Config<RelayChainAccountId = [u8; 32]>,
{
	let msg = hash::<Keccak256>(
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

pub fn ethereum_public(secret: &EthereumKey) -> libsecp256k1::PublicKey {
	libsecp256k1::PublicKey::from_secret_key(secret)
}

pub fn cosmos_address(pub_key_slice: &[u8]) -> CosmosAddress {
	let mut pub_key: [u8; 33] = [0; 33];
	pub_key.copy_from_slice(pub_key_slice);
	CosmosAddress::Secp256r1(pub_key)
}

pub fn ethereum_address(secret: &EthereumKey) -> EthereumAddress {
	let mut res = EthereumAddress::default();
	res.0
		.copy_from_slice(&hash::<Keccak256>(&ethereum_public(secret).serialize()[1..65])[12..]);
	res
}

pub fn cosmos_generate<T>(count: u64) -> Vec<(AccountIdOf<T>, Identity)>
where
	T: Config<RelayChainAccountId = [u8; 32]>,
{
	let seed: u128 = 12345678901234567890123456789012;
	(0..count)
		.map(|i| {
			let account_id = account("recipient", i as u32, 0xCAFEBABE);
			(
				account_id,
				Identity::Cosmos(
					SigningKey::from_bytes(
						&[(&(seed + i as u128)).to_le_bytes(), (&(seed + i as u128)).to_le_bytes()]
							.concat(),
					)
					.unwrap(),
				),
			)
		})
		.collect()
}

#[allow(clippy::disallowed_methods)] // Allow unwrap
pub fn ethereum_generate<T>(count: u64) -> Vec<(AccountIdOf<T>, Identity)>
where
	T: Config<RelayChainAccountId = [u8; 32]>,
{
	(0..count)
		.map(|i| {
			let account_id = account("recipient", i as u32, 0xCAFEBABE);
			(
				account_id,
				Identity::Ethereum(
					EthereumKey::parse(&hash::<Keccak256>(&i.to_le_bytes())).unwrap(),
				),
			)
		})
		.collect()
}

/// `count % 2 == 0` should hold for all x
pub fn generate_accounts<T>(count: u64) -> Vec<(AccountIdOf<T>, Identity)>
where
	T: Config<RelayChainAccountId = [u8; 32]>,
{
	assert!(count % 2 == 0, "`x % 2 == 0` should hold for all x");
	let mut x = cosmos_generate::<T>(count / 2);
	let mut y = ethereum_generate::<T>(count / 2);
	x.append(&mut y);
	x
}

pub fn hash<T>(input: &[u8]) -> [u8; 32]
where
	T: Hasher + Default,
{
	let mut hasher: T = Default::default();
	let mut hash: [u8; 32] = [0; 32];

	hasher.update(input);
	hash.copy_from_slice(hasher.finalize());
	hasher.reset();

	hash
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
		let accounts: Vec<(IdentityOf<T>, BalanceOf<T>, MomentOf<T>,bool)> = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), VESTING_PERIOD.into(), false)).collect();
		let airdrop_id = T::AirdropId::one();
		let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as Airdropper>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
	}: add_recipient(RawOrigin::Signed(creator), airdrop_id, accounts)

	remove_recipient_benchmark {
		let x in 100..1000;
		let accounts: Vec<(IdentityOf<T>, BalanceOf<T>, MomentOf<T>,bool)> = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), VESTING_PERIOD.into(), false)).collect();
		let airdrop_id = T::AirdropId::one();
		let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as Airdropper>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
		<Airdrop<T> as Airdropper>::add_recipient(creator.clone(), airdrop_id, accounts.clone())?;
	}: remove_recipient(RawOrigin::Signed(creator), airdrop_id, accounts[0].0.clone())

	enable_airdrop_benchmark {
		let x in 100..1000;
		let accounts: Vec<(IdentityOf<T>, BalanceOf<T>, MomentOf<T>,bool)> = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), VESTING_PERIOD.into(), false)).collect();
		let airdrop_id = T::AirdropId::one();
		let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as Airdropper>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
		<Airdrop<T> as Airdropper>::add_recipient(creator.clone(), airdrop_id, accounts)?;
	}: enable_airdrop(RawOrigin::Signed(creator), airdrop_id)

	disable_airdrop_benchmark {
		let x in 100..1000;
		let accounts: Vec<(IdentityOf<T>, BalanceOf<T>, MomentOf<T>,bool)> = generate_accounts::<T>(x as _).into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), VESTING_PERIOD.into(), false)).collect();
		let airdrop_id = T::AirdropId::one();
		let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as Airdropper>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
		<Airdrop<T> as Airdropper>::add_recipient(creator.clone(), airdrop_id, accounts)?;
	}: disable_airdrop(RawOrigin::Signed(creator), airdrop_id)

	claim_benchmark {
		let x in 100..1000;
		let accounts = generate_accounts::<T>(x as _);
		let remote_accounts = accounts.clone().into_iter().map(|(_, a)| (a.as_remote_public::<T>(), T::Balance::from(1_000_000_000_000), VESTING_PERIOD.into(), false)).collect();
		let airdrop_id = T::AirdropId::one();
		let creator: AccountIdOf<T> = account("creator", 0, 0xCAFEBABE);
		<Airdrop<T> as Airdropper>::create_airdrop(creator.clone(), None, VESTING_STEP.into())?;
		<Airdrop<T> as Airdropper>::add_recipient(creator, airdrop_id, remote_accounts)?;
		let reward_account = accounts[0].0.clone();
		System::<T>::set_block_number(VESTING_PERIOD.into());
	}: claim(RawOrigin::None, airdrop_id, reward_account, accounts[0].1.clone().proof::<T>(accounts[0].0.clone()))
}

impl_benchmark_test_suite!(
	Airdrop,
	crate::mocks::ExtBuilder::default().build(),
	crate::mocks::MockRuntime
);
