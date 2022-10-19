use crate::{
	self as pallet_crowdloan_rewards,
	models::{Proof, RemoteAccount},
	Pallet,
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
use system::{pallet_prelude::OriginFor, EnsureRoot};

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
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
	type Event = Event;
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
	type Event = Event;
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
	type Event = Event;
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
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		CrowdloanRewards: pallet_crowdloan_rewards,
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
