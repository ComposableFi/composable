use core::{
	fmt::Debug,
	ops::{Deref, Mul},
};

use codec::Encode;
use composable_support::types::{EcdsaSignature, EthereumAddress};
use frame_support::{
	assert_noop, assert_ok,
	pallet_prelude::DispatchResultWithPostInfo,
	traits::{Currency, ExistenceRequirement, Get, OriginTrait},
};
use frame_system::pallet_prelude::OriginFor;
use sp_core::{ed25519, keccak_256, Pair};
use sp_runtime::{traits::StaticLookup, DispatchError};

use crate::{
	models::{Proof, RemoteAccount},
	AccountIdOf, Config, Pallet,
};

pub type RelayKey = ed25519::Pair;
pub type EthKey = libsecp256k1::SecretKey;

#[derive(Clone)]
pub enum ClaimKey {
	Relay(RelayKey),
	Eth(EthKey),
}

type OriginAccountIdOf<Runtime> =
	<<Runtime as frame_system::Config>::Origin as OriginTrait>::AccountId;

impl ClaimKey {
	pub fn as_remote_public<Runtime>(&self) -> RemoteAccount<Runtime::RelayChainAccountId>
	where
		Runtime: Config,
		Runtime::RelayChainAccountId: From<ed25519::Public>,
	{
		match self {
			ClaimKey::Relay(relay_account) =>
				RemoteAccount::RelayChain(relay_account.public().into()),
			ClaimKey::Eth(ethereum_account) =>
				RemoteAccount::Ethereum(ethereum_address(ethereum_account)),
		}
	}

	pub fn proof<Runtime>(
		&self,
		reward_account: AccountIdOf<Runtime>,
	) -> Proof<Runtime::RelayChainAccountId>
	where
		Runtime: Config,
		AccountIdOf<Runtime>: for<'a> TryFrom<&'a [u8]>,
		for<'a> <AccountIdOf<Runtime> as TryFrom<&'a [u8]>>::Error: Debug,
		OriginAccountIdOf<Runtime>: From<Runtime::AccountId> + Encode,
		Runtime::RelayChainAccountId: From<ed25519::Public>,
	{
		match self {
			ClaimKey::Relay(relay) => relay_proof::<Runtime>(relay, reward_account.into()),
			ClaimKey::Eth(eth) => ethereum_proof::<Runtime>(eth, reward_account),
		}
	}

	pub fn claim<Runtime>(
		&self,
		reward_account: OriginAccountIdOf<Runtime>,
	) -> DispatchResultWithPostInfo
	where
		Runtime: Config,
		OriginAccountIdOf<Runtime>: From<Runtime::AccountId>,
	{
		Pallet::<Runtime>::claim(OriginFor::<Runtime>::signed(reward_account))
	}

	pub fn associate<Runtime>(
		&self,
		reward_account: AccountIdOf<Runtime>,
	) -> DispatchResultWithPostInfo
	where
		Runtime: Config,
		AccountIdOf<Runtime>: for<'a> TryFrom<&'a [u8]>,
		for<'a> <AccountIdOf<Runtime> as TryFrom<&'a [u8]>>::Error: Debug,
		OriginAccountIdOf<Runtime>: From<Runtime::AccountId> + Encode,
		Runtime::RelayChainAccountId: From<ed25519::Public>,
	{
		let proof = self.proof::<Runtime>(reward_account.clone());

		Pallet::<Runtime>::associate(OriginFor::<Runtime>::none(), reward_account, proof)
	}
}

fn relay_proof<Runtime>(
	relay_account: &RelayKey,
	reward_account: OriginAccountIdOf<Runtime>,
) -> Proof<Runtime::RelayChainAccountId>
where
	Runtime: Config,
	OriginAccountIdOf<Runtime>: From<Runtime::AccountId> + Encode,
	Runtime::RelayChainAccountId: From<ed25519::Public>,
{
	let mut msg = b"<Bytes>".to_vec();

	msg.append(&mut Runtime::Prefix::get().to_vec());
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	msg.append(&mut b"</Bytes>".to_vec());

	Proof::RelayChain(relay_account.public().into(), relay_account.sign(&msg).into())
}

pub fn ethereum_proof<Runtime>(
	ethereum_account: &EthKey,
	reward_account: Runtime::AccountId,
) -> Proof<Runtime::RelayChainAccountId>
where
	Runtime: Config,
	AccountIdOf<Runtime>: for<'a> TryFrom<&'a [u8]>,
	for<'a> <AccountIdOf<Runtime> as TryFrom<&'a [u8]>>::Error: Debug,
	OriginAccountIdOf<Runtime>: From<Runtime::AccountId>,
{
	let msg = keccak_256(
		&crate::ethereum_signable_message(
			Runtime::Prefix::get(),
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

pub fn ethereum_public(secret: &EthKey) -> libsecp256k1::PublicKey {
	libsecp256k1::PublicKey::from_secret_key(secret)
}

pub fn ethereum_address(secret: &EthKey) -> EthereumAddress {
	let mut res = EthereumAddress::default();
	res.0
		.copy_from_slice(&keccak_256(&ethereum_public(secret).serialize()[1..65])[12..]);
	res
}

pub fn relay_generate<Runtime>(count: u64) -> Vec<(Runtime::AccountId, ClaimKey)>
where
	Runtime: Config,
	AccountIdOf<Runtime>: for<'a> TryFrom<&'a [u8]>,
	for<'a> <AccountIdOf<Runtime> as TryFrom<&'a [u8]>>::Error: Debug,
	OriginAccountIdOf<Runtime>: From<Runtime::AccountId>,
{
	let seed: u128 = 12345678901234567890123456789012;
	(0..count)
		.map(|i| {
			let account_id: Runtime::AccountId = [[0_u8; 16], (i as u128 + 1).to_le_bytes()]
				.concat()
				.deref()
				.try_into()
				.expect("Account ID is valid; QED");
			(
				account_id,
				ClaimKey::Relay(ed25519::Pair::from_seed(&keccak_256(
					&[(seed + i as u128).to_le_bytes(), (seed + i as u128).to_le_bytes()].concat(),
				))),
			)
		})
		.collect()
}

pub fn ethereum_generate<Runtime>(count: u64) -> Vec<(Runtime::AccountId, ClaimKey)>
where
	Runtime: Config,
	AccountIdOf<Runtime>: for<'a> TryFrom<&'a [u8]>,
	for<'a> <AccountIdOf<Runtime> as TryFrom<&'a [u8]>>::Error: Debug,
	OriginAccountIdOf<Runtime>: From<Runtime::AccountId>,
{
	(0..count)
		.map(|i| {
			let account_id: Runtime::AccountId = [(i as u128 + 1).to_le_bytes(), [0_u8; 16]]
				.concat()
				.deref()
				.try_into()
				.expect("Account ID is valid; QED");
			(
				account_id,
				ClaimKey::Eth(
					EthKey::parse(&keccak_256(&i.to_le_bytes())).expect("Account ID is valid; QED"),
				),
			)
		})
		.collect()
}

pub fn generate_accounts<Runtime>(count: u64) -> Vec<(Runtime::AccountId, ClaimKey)>
where
	Runtime: Config,
	AccountIdOf<Runtime>: for<'a> TryFrom<&'a [u8]>,
	for<'a> <AccountIdOf<Runtime> as TryFrom<&'a [u8]>>::Error: Debug,
	OriginAccountIdOf<Runtime>: From<Runtime::AccountId>,
{
	let mut x = relay_generate::<Runtime>(count / 2);
	let mut y = ethereum_generate::<Runtime>(count / 2);
	x.append(&mut y);
	x
}

pub fn should_unlock_reward_assets_for_accounts<Runtime>(
	mut externalities: sp_io::TestExternalities,
	expected_error: DispatchError,
	reward: <Runtime as Config>::Balance,
	contributors: u64,
	vesting_period: <Runtime as Config>::Moment,
) where
	Runtime: Config + pallet_timestamp::Config,
	Runtime::RelayChainAccountId: From<[u8; 32]> + From<ed25519::Public>,
	<Runtime as Config>::Balance: From<u128>,
	<Runtime as pallet_timestamp::Config>::Moment: From<u64>,
	<Runtime as Config>::Moment: From<u64>,
	<<Runtime as frame_system::Config>::Origin as OriginTrait>::AccountId:
		From<Runtime::AccountId> + Encode,
	<<Runtime as frame_system::Config>::Lookup as StaticLookup>::Source: From<Runtime::AccountId>,
	// <Runtime as frame_system::Config>::AccountId: From<Runtime::AccountId>,
	<Runtime as frame_system::Config>::BlockNumber: From<u32>,
	<Runtime as Config>::Balance: Mul<u128, Output = Runtime::Balance>,

	AccountIdOf<Runtime>: for<'a> TryFrom<&'a [u8]>,
	for<'a> <AccountIdOf<Runtime> as TryFrom<&'a [u8]>>::Error: Debug,
{
	let accounts = generate_accounts::<Runtime>(contributors);

	let rewards = accounts
		.iter()
		.map(|(_, account)| (account.as_remote_public::<Runtime>(), reward, vesting_period))
		.collect();

	externalities.execute_with(|| {
		frame_system::Pallet::<Runtime>::set_block_number(0xDEADC0DE.into());
		let random_moment_start: u64 = 0xCAFEBABE;

		pallet_timestamp::Pallet::<Runtime>::set_timestamp(random_moment_start.into());
		Runtime::RewardAsset::make_free_balance_be(
			&Pallet::<Runtime>::account_id(),
			reward * contributors as u128,
		);

		assert_ok!(Pallet::<Runtime>::populate(OriginFor::<Runtime>::root(), rewards));
		assert_ok!(Pallet::<Runtime>::initialize(OriginFor::<Runtime>::root()));

		for (picasso_account, remote_account) in accounts.clone().into_iter() {
			assert_ok!(remote_account.associate::<Runtime>(picasso_account));
		}

		assert_noop!(
			Runtime::RewardAsset::transfer(
				&accounts[0].0,
				&accounts[1].0,
				reward / 10_u128.into(),
				ExistenceRequirement::AllowDeath
			),
			expected_error
		);

		let accounts: Vec<Runtime::AccountId> =
			accounts.into_iter().map(|(account, _claim_key)| account).collect();

		assert_ok!(crate::Pallet::<Runtime>::unlock_rewards_for(
			OriginFor::<Runtime>::root(),
			accounts
				.iter()
				.cloned()
				.map(<Runtime as frame_system::Config>::AccountId::from)
				.collect()
		));

		assert_ok!(Runtime::RewardAsset::transfer(
			&accounts[0],
			&accounts[1],
			reward / 10_u128.into(),
			ExistenceRequirement::AllowDeath
		));
	})
}
