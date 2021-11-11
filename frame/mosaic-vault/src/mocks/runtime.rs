
use crate::{self as mosaic_vault, *};
//use crate as mosaic_vault;
use super::currency_factory::MockCurrencyId;
use super::*;
// use crate::*;
use frame_support::{
    ord_parameter_types,
    construct_runtime,parameter_types,
    traits::{Everything, GenesisBuild},
    PalletId,
};
use sp_keystore::{testing::KeyStore, SyncCryptoStore};
use frame_system::EnsureSignedBy;
use frame_system as system;
use orml_traits::parameter_type_with_key;
use num_traits::Zero;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
	RuntimeAppPublic,
};
// use sp_runtime::generic::UncheckedExtrinsic;

pub type BlockNumber = u64;
pub type Balance = u128;
pub type Amount = i128;
pub type Nonce = u128;
pub type TransferDelay = u128;
pub type VaultId = u64;
pub type DepositId = [u8; 32];
pub type RemoteNetworkId = u64;
pub type AccountId = u128;

pub const MINIMUM_BALANCE: Balance = 1000;
// accounts 
pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 0;
pub const CHARLIE: AccountId = 0;
pub const JEREMY: AccountId = 0;
pub const ACCOUNT_FREE_START: AccountId = JEREMY + 1;
pub const ACCOUNT_INITIAL_AMOUNT: u128 = 1_000_000;

pub const MILLISECS_PER_BLOCK: u64 = 6000;

parameter_types!{
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
    type AccountId = u64;
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
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
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: MockCurrencyId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = MockCurrencyId;
    type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1000;
}

// impl pallet_balances::Config for Test {
// 	type Balance = Balance;
// 	type Event = Event;
// 	type DustRemoval = ();
// 	type ExistentialDeposit = ExistentialDeposit;
// 	type AccountStore = System;
// 	type WeightInfo = ();
// 	type MaxLocks = ();
// 	type MaxReserves = ();
// 	type ReserveIdentifier = [u8; 8];
// }

parameter_types! {
	pub const DynamicCurrencyIdInitial: MockCurrencyId = MockCurrencyId::LpToken(0);
}

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type DynamicCurrencyId = MockCurrencyId;
	type DynamicCurrencyIdInitial = DynamicCurrencyIdInitial;
}

parameter_types!{
   pub const MaxStrategies: usize = 255;
   pub const MinimumDeposit: Balance = 0;
   pub const MinimumWithdrawal: Balance = 0;
   pub const CreationDeposit: Balance = 10;
   pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
   pub const RentPerBlock: Balance = 1;
   pub const NativeAssetId: MockCurrencyId = MockCurrencyId::A;
}

impl pallet_vault::Config for Test {
    type Event = Event;
	type Currency = Tokens;
	type AssetId = MockCurrencyId;
	type Balance = Balance;
	type MaxStrategies = MaxStrategies;
	type CurrencyFactory = LpTokenFactory;
	type Convert = ConvertInto;
	type MinimumDeposit = MinimumDeposit;
	type MinimumWithdrawal = MinimumWithdrawal;
	type PalletId = VaultPalletId;
	type CreationDeposit = CreationDeposit;
	type ExistentialDeposit = ExistentialDeposit;
	type RentPerBlock = RentPerBlock;
	type NativeAssetId = NativeAssetId;
}

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	
    type WeightInfo = ();
}

parameter_types! {
    pub const FeeFactor: Balance = 100;
    pub const ThresholdFactor:Balance = 100;
    pub const FeeAddress: u64 = 1;
    pub const MosaicVaultId: PalletId = PalletId(*b"test_pid");
    pub const MaxFeeDefault: Balance = 500;
    pub const MinFeeDefault: Balance = 0;
    pub const One: u128 = AccountId = 1;
}

impl mosaic_vault::Config for Test {
    type Event = Event;
    type Currency = Tokens;
    type Convert = ConvertInto;
    type Balance = Balance;
    type Nonce = Nonce;
    type TransferDelay = TransferDelay;
    type VaultId = VaultId;
    type Vault = Vault;
    type AssetId = MockCurrencyId;
    type RemoteAssetId = MockCurrencyId;
    type RemoteNetworkId = RemoteNetworkId;
    type DepositId = DepositId;
    type FeeFactor = FeeFactor;
    type ThresholdFactor = ThresholdFactor;
    type PalletId = MosaicVaultId;
    type FeeAddress = FeeAddress;
    type BlockTimestamp = Timestamp;
    type MaxFeeDefault = MaxFeeDefault;
    type MinFeeDefault = MinFeeDefault;
    type RelayerOrigin = EnsureSignedBy<One, AccountId>;//<RootAccount, sp_core::sr25519::Public>;
    type AdminOrigin = EnsureSignedBy<One, AccountId>;//<RootAccount, sp_core::sr25519::Public>;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
        System: system::{ Pallet, Call, Storage, Config, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage},
        LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
        Tokens: orml_tokens::{Pallet,Storage, Event<T>, Config<T>},
        Vault: pallet_vault::{Pallet, Call, Storage, Event<T>},
        MosaicVault: mosaic_vault::{ Pallet, Call, Storage, Event<T>},
	}
);

pub struct ExtBuilder {
	balances: Vec<(u64, MockCurrencyId, u128)>, // accoun_id // cuurency_id // balance
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: Vec::new() }
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();


		orml_tokens::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.unwrap();

		t.into()
	}
}
