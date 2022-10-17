use crate::{self as pallet_ibc};
use frame_support::{
	pallet_prelude::ConstU32,
	parameter_types,
	traits::{ConstU64, Everything},
};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::{
	offchain::{testing::TestOffchainExt, OffchainDbExt, OffchainWorkerExt},
	H256,
};
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchError, MultiSignature,
};
use std::time::{Duration, Instant};
use system::EnsureRoot;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Header = generic::Header<u32, BlakeTwo256>;
use composable_traits::currency::{CurrencyFactory as CurrencyFactoryTrait, RangeId};
use ibc::signer::Signer;
use primitives::currency::ValidateCurrencyId;
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type AssetId = u128;
pub type Amount = i128;
pub type Balance = u128;
type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Timestamp: pallet_timestamp,
		Balances: balances,
		ParachainInfo: parachain_info,
		Ping: pallet_ibc_ping,
		IbcTransfer: transfer,
		GovernanceRegistry: governance_registry,
		AssetsRegistry: assets_registry,
		CurrencyFactory: currency_factory,
		Tokens: orml_tokens,
		Assets: assets,
		Ibc: pallet_ibc,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const ExpectedBlockTime: u64 = 1000;
	pub const ExistentialDeposit: u64 = 10000;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u32;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

impl parachain_info::Config for Test {}

impl pallet_ibc_ping::Config for Test {
	type Event = Event;
	type IbcHandler = Ibc;
}

pub struct MockUnixTime;

impl frame_support::traits::UnixTime for MockUnixTime {
	fn now() -> Duration {
		let now_time = Instant::now().elapsed();
		now_time
	}
}

impl composable_traits::defi::DeFiComposableConfig for Test {
	type MayBeAssetId = AssetId;
	type Balance = Balance;
}

parameter_types! {
	pub const NativeAssetId: AssetId = 1;
}

pub struct CurrencyIdGenerator;

impl CurrencyFactoryTrait for CurrencyIdGenerator {
	type AssetId = AssetId;
	type Balance = Balance;

	fn create(_: RangeId, _: Self::Balance) -> Result<Self::AssetId, sp_runtime::DispatchError> {
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

impl assets::Config for Test {
	type AssetId = AssetId;
	type Balance = Balance;
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = CurrencyIdGenerator;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type GovernanceRegistry = GovernanceRegistry;
	type CurrencyValidator = ValidateCurrencyId;
	type WeightInfo = ();
	type AdminOrigin = frame_system::EnsureRoot<AccountId>;
}

parameter_types! {
	pub const MaxLocks: u32 = 256;
	pub const TransferPalletId: frame_support::PalletId = frame_support::PalletId(*b"transfer");
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_a: AssetId| -> Balance {
		0
	};
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnKilledTokenAccount = ();
	type OnNewTokenAccount = ();
}

impl governance_registry::Config for Test {
	type AssetId = AssetId;
	type WeightInfo = ();
	type Event = Event;
}

impl transfer::Config for Test {
	type Event = Event;
	type IbcHandler = Ibc;
	type MultiCurrency = Assets;
	type PalletId = TransferPalletId;
	type CurrencyFactory = CurrencyFactory;
	type AccountIdConversion = IbcAccount<Test>;
	type AssetRegistry = AssetsRegistry;
	type AdminOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

impl currency_factory::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type AddOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type Balance = Balance;
}

impl assets_registry::Config for Test {
	type Event = Event;
	type LocalAssetId = AssetId;
	type CurrencyFactory = CurrencyFactory;
	type ForeignAssetId = composable_traits::xcm::assets::XcmAssetLocation;
	type UpdateAssetRegistryOrigin = EnsureRoot<AccountId>;
	type ParachainOrGovernanceOrigin = EnsureRoot<AccountId>;
	type Balance = Balance;
	type WeightInfo = ();
}

#[derive(Clone)]
pub struct IbcAccount<T: pallet_ibc::Config>(T::AccountId);

impl<T: pallet_ibc::Config> IdentifyAccount for IbcAccount<T> {
	type AccountId = T::AccountId;
	fn into_account(self) -> Self::AccountId {
		self.0
	}
}

impl TryFrom<Signer> for IbcAccount<Test> {
	type Error = &'static str;
	fn try_from(_: Signer) -> Result<Self, Self::Error> {
		Ok(IbcAccount(AccountId::new([0; 32])))
	}
}

impl pallet_ibc::Config for Test {
	type TimeProvider = MockUnixTime;
	type Event = Event;
	type Currency = Balances;
	const INDEXING_PREFIX: &'static [u8] = b"ibc";
	const CONNECTION_PREFIX: &'static [u8] = b"ibc/";
	const CHILD_TRIE_KEY: &'static [u8] = b"IBC";
	type ExpectedBlockTime = ExpectedBlockTime;
	type WeightInfo = ();

	type AdminOrigin = frame_system::EnsureRoot<AccountId>;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

impl balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<2>;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

fn register_offchain_ext(ext: &mut sp_io::TestExternalities) {
	let (offchain, _offchain_state) = TestOffchainExt::with_offchain_db(ext.offchain_db());
	ext.register_extension(OffchainDbExt::new(offchain.clone()));
	ext.register_extension(OffchainWorkerExt::new(offchain));
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	register_offchain_ext(&mut ext);
	ext
}
