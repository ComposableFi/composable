use crate::{
    self as pallet_airdrop,
    models::{Proof, RemoteAccount}
}
use codec::Encode;
use composable_support::types::{EcdsaSignature, EthereumAddress};
use frame_support::{
	construct_runtime, dispatch::DispatchResultWithPostInfo, parameter_types, traits::Everything,
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

pub type EthKey = libsecp256k1::SecretKey;
pub type RelayKey = ed25519::Pair;

pub type AccountId = AccountId32;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Moment = u64;
pub type RelayChainAccountId = [u8; 32];

pub const ALICE: AccountId = AccountId32::new([0_u8; 32]);
pub const PROOF_PREFIX: &[u8] = b"picasso-";
pub const VESTING_STEP: Moment = 3600 * 24 * 7;


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
    pub const AirdropPalletId: PalletId = PalletId(*b"pal_aird");
    pub const Prefix: &'static [u8] = PROOF_PREFIX;
    pub const Stake: Balance = 10_000;
    pub const VestingStep: Moment = VESTING_STEP;
}

impl pallet_airdrop::Config for Test {
    type AirdropId = u64;
    type Balance = Balance;
    type Convert = ConvertInto;
    type Moment = Moment;
    type RelayChainAccountId = RelayChainAccountId;
    type RecipientFundAsset = Balances;
    type Time = Timestamp;
    type PalletId = AirdropPalletId;
    type Prefix = Prefix;
    type Stake = Stake;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 6000;
}

impl pallet_timestamp::Config for Test {
    type MinimumPeriod = MinimumPeriod;
    type Moment = Moment;
    type OnTimestampSet = ();
    type WeightInfo = ();
}
