use crate::*;

use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	xcm::assets::XcmAssetLocation,
};
use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild},
};
use frame_system as system;
use num_traits::Zero;
use orml_traits::parameter_type_with_key;
use primitives::currency::ValidateCurrencyId;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchError,
};
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type AssetId = u64;
pub type Amount = i128;
pub type Balance = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DARWIN: AccountId = 4;

pub const ACCOUNT_FREE_START: AccountId = CHARLIE + 1;

pub const MINIMUM_BALANCE: Balance = 1;

#[allow(dead_code)]
pub const INVALID: AssetId = 0;
pub const ASSET_1: AssetId = 1;
pub const ASSET_2: AssetId = 2;
pub const ASSET_FREE_START: AssetId = ASSET_2 + 1;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 1,
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 2,
		GovernanceRegistry: governance_registry::{Pallet, Call, Storage, Event<T>} = 3,
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>} = 4,
		Assets: crate::{Pallet, Call, Storage} = 5,
	}
);

parameter_type_with_key! {
	pub ExistentialDeposits: |_a: AssetId| -> Balance {
		Zero::zero()
	};
}

parameter_types! {
	pub const NativeAssetId: AssetId = 1;
}

pub struct CurrencyIdGenerator;

impl CurrencyFactory for CurrencyIdGenerator {
	type AssetId = AssetId;
	type Balance = Balance;

	fn create(_: RangeId) -> Result<Self::AssetId, sp_runtime::DispatchError> {
		Ok(1)
	}

	fn protocol_asset_id_to_unique_asset_id(
		_protocol_asset_id: u32,
		_range_id: RangeId,
	) -> Result<Self::AssetId, DispatchError> {
		Ok(1)
	}

	fn unique_asset_id_to_protocol_asset_id(_unique_asset_id: Self::AssetId) -> u32 {
		1
	}
}

impl Config for Test {
	type AssetId = AssetId;
	type Balance = Balance;
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = CurrencyIdGenerator;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type GovernanceRegistry = GovernanceRegistry;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type CurrencyValidator = ValidateCurrencyId;
}

parameter_types! {
	pub const MaxLocks: u32 = 256;
}

pub struct CurrencyHooks;
impl orml_traits::currency::MutationHooks<AccountId, AssetId, Balance> for CurrencyHooks {
	type OnDust = ();
	type OnSlash = ();
	type PreDeposit = ();
	type PostDeposit = ();
	type PreTransfer = ();
	type PostTransfer = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type CurrencyHooks = CurrencyHooks;
}

impl governance_registry::Config for Test {
	type AssetId = AssetId;
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

pub const BALANCES: [(AccountId, Balance); 4] =
	[(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000), (DARWIN, 1000)];

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let genesis = pallet_balances::GenesisConfig::<Test> { balances: Vec::from(BALANCES) };
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}

pub fn new_test_ext_multi_currency() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();

	let balances: Vec<(AccountId, AssetId, Balance)> =
		vec![(ALICE, ASSET_1, 1000), (BOB, ASSET_2, 1000)];

	orml_tokens::GenesisConfig::<Test> { balances }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
