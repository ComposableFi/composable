use crate::{
	self as pallet_crowdloan_rewards,
	models::{Proof, RemoteAccount},
};
use codec::Encode;
use composable_support::types::{EcdsaSignature, EthereumAddress};
use frame_support::{
	construct_runtime,
	dispatch::DispatchResultWithPostInfo,
	parameter_types,
	traits::{Everything, LockIdentifier},
	PalletId,
};
use frame_system as system;
use sp_core::{ed25519, keccak_256, Pair, H256};
use sp_runtime::{
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
	AccountId32, Perbill,
};
use sp_std::vec::Vec;
use system::EnsureRoot;

pub type RelayKey = ed25519::Pair;
pub type EthKey = libsecp256k1::SecretKey;

pub type Moment = u64;
pub type BlockNumber = u32;
pub type AccountId = AccountId32;
pub type RelayChainAccountId = [u8; 32];
pub type Balance = u128;

pub const VESTING_STEP: Moment = 3600 * 24 * 7;
pub const INITIAL_PAYMENT: Perbill = Perbill::from_percent(50);
pub const OVER_FUNDED_THRESHOLD: Perbill = Perbill::from_percent(1);

pub const ALICE: AccountId = AccountId32::new([0_u8; 32]);

// picasso-{account_id}
pub const PROOF_PREFIX: &[u8] = b"picasso-";

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub const MaxConsumers: u32 = 10;
	pub const MaxOverFlow: u32 = 10;
}

impl system::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = (MaxConsumers, MaxOverFlow);
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ();
	type AccountStore = System;
	type MaxLocks = ();
	type ReserveIdentifier = [u8; 8];
	type MaxReserves = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const CrowdloanRewardsPalletId: PalletId = PalletId(*b"pal_crow");
	pub const CrowdloanRewardsLockId: LockIdentifier = *b"crs_lock";
	pub const InitialPayment: Perbill = INITIAL_PAYMENT;
	pub const OverFundedThreshold: Perbill = OVER_FUNDED_THRESHOLD;
	pub const VestingStep: Moment = VESTING_STEP;
	pub const Prefix: &'static [u8] = PROOF_PREFIX;
	pub const LockCrowdloanRewards: bool = true;
}

impl pallet_crowdloan_rewards::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RewardAsset = Balances;
	type Balance = Balance;
	type Convert = ConvertInto;
	type RelayChainAccountId = RelayChainAccountId;
	type InitialPayment = InitialPayment;
	type OverFundedThreshold = OverFundedThreshold;
	type VestingStep = VestingStep;
	type Prefix = Prefix;
	type AdminOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type PalletId = CrowdloanRewardsPalletId;
	type Moment = Moment;
	type Time = Timestamp;
	type LockId = CrowdloanRewardsLockId;
	type LockByDefault = LockCrowdloanRewards;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 6000;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the Unix epoch.
	type Moment = Moment;
	/// What to do when SLOT_DURATION has passed?
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Balances: pallet_balances::{Pallet, Storage, Event<T>, Config<T>},
	  CrowdloanRewards: pallet_crowdloan_rewards::{Pallet, Storage, Call, Event<T>},
	}
);

#[derive(Default)]
pub struct ExtBuilder {
	pub(crate) balances: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().expect("QED");
		pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.expect("Storage is correct; QED");
		t.into()
	}
}

#[derive(Clone)]
pub enum ClaimKey {
	Relay(RelayKey),
	Eth(EthKey),
}

impl ClaimKey {
	pub fn as_remote_public(&self) -> RemoteAccount<RelayChainAccountId> {
		match self {
			ClaimKey::Relay(relay_account) =>
				RemoteAccount::RelayChain(relay_account.public().into()),
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
	pub fn claim(&self, reward_account: AccountId) -> DispatchResultWithPostInfo {
		CrowdloanRewards::claim(RuntimeOrigin::signed(reward_account))
	}
	pub fn associate(&self, reward_account: AccountId) -> DispatchResultWithPostInfo {
		let proof = match self {
			ClaimKey::Relay(relay_account) => relay_proof(relay_account, reward_account.clone()),
			ClaimKey::Eth(ethereum_account) =>
				ethereum_proof(ethereum_account, reward_account.clone()),
		};
		CrowdloanRewards::associate(RuntimeOrigin::none(), reward_account, proof)
	}
}

fn relay_proof(relay_account: &RelayKey, reward_account: AccountId) -> Proof<RelayChainAccountId> {
	let mut msg = b"<Bytes>".to_vec();
	msg.append(&mut PROOF_PREFIX.to_vec());
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	msg.append(&mut b"</Bytes>".to_vec());
	Proof::RelayChain(relay_account.public().into(), relay_account.sign(&msg).into())
}

pub fn ethereum_proof(
	ethereum_account: &EthKey,
	reward_account: AccountId,
) -> Proof<RelayChainAccountId> {
	let msg = keccak_256(
		&pallet_crowdloan_rewards::ethereum_signable_message(
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

pub fn ethereum_public(secret: &EthKey) -> libsecp256k1::PublicKey {
	libsecp256k1::PublicKey::from_secret_key(secret)
}

pub fn ethereum_address(secret: &EthKey) -> EthereumAddress {
	let mut res = EthereumAddress::default();
	res.0
		.copy_from_slice(&keccak_256(&ethereum_public(secret).serialize()[1..65])[12..]);
	res
}

pub fn relay_generate(count: u64) -> Vec<(AccountId, ClaimKey)> {
	let seed: u128 = 12345678901234567890123456789012;
	(0..count)
		.map(|i| {
			let account_id = [[0_u8; 16], (i as u128 + 1).to_le_bytes()]
				.concat()
				.try_into()
				.expect("Account ID is valid; QED");
			(
				AccountId::new(account_id),
				ClaimKey::Relay(ed25519::Pair::from_seed(&keccak_256(
					&[(seed + i as u128).to_le_bytes(), (seed + i as u128).to_le_bytes()].concat(),
				))),
			)
		})
		.collect()
}

pub fn ethereum_generate(count: u64) -> Vec<(AccountId, ClaimKey)> {
	(0..count)
		.map(|i| {
			let account_id = [(i as u128 + 1).to_le_bytes(), [0_u8; 16]]
				.concat()
				.try_into()
				.expect("Account ID is valid; QED");
			(
				AccountId::new(account_id),
				ClaimKey::Eth(
					EthKey::parse(&keccak_256(&i.to_le_bytes())).expect("Account ID is valid; QED"),
				),
			)
		})
		.collect()
}

pub fn generate_accounts(count: u64) -> Vec<(AccountId, ClaimKey)> {
	let mut x = relay_generate(count / 2);
	let mut y = ethereum_generate(count / 2);
	x.append(&mut y);
	x
}
