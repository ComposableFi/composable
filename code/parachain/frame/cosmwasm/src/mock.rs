use crate::*;

use composable_traits::currency::{CurrencyFactory, RangeId};
use frame_support::{
	pallet_prelude::ConstU32,
	parameter_types,
	traits::{ConstU64, Everything},
	PalletId,
};
use frame_system::EnsureRoot;
use num_traits::Zero;
use orml_traits::parameter_type_with_key;
use primitives::currency::{CurrencyId, ValidateCurrencyId};
use sp_core::H256;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, Convert, IdentityLookup},
	AccountId32, DispatchError,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Header = generic::Header<u32, BlakeTwo256>;
type Balance = u128;
type AccountId = AccountId32;
type Amount = i128;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Cosmwasm: crate,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Timestamp: pallet_timestamp,
		GovernanceRegistry: governance_registry,
		Tokens: orml_tokens,
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const ExistentialDeposit: u64 = 10000;
	pub const NativeAssetId: CurrencyId = CurrencyId(1);
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
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
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

impl governance_registry::Config for Test {
	type AssetId = CurrencyId;
	type WeightInfo = ();
	type Event = Event;
}

parameter_types! {
	pub const MaxLocks: u32 = 256;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_a: CurrencyId| -> Balance {
		Zero::zero()
	};
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

impl pallet_balances::Config for Test {
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

pub struct CurrencyIdGenerator;

impl CurrencyFactory for CurrencyIdGenerator {
	type AssetId = CurrencyId;
	type Balance = Balance;

	fn create(_: RangeId, _: Self::Balance) -> Result<Self::AssetId, sp_runtime::DispatchError> {
		Ok(CurrencyId(1))
	}

	fn protocol_asset_id_to_unique_asset_id(
		_protocol_asset_id: u32,
		_range_id: RangeId,
	) -> Result<Self::AssetId, DispatchError> {
		Ok(CurrencyId(1))
	}

	fn unique_asset_id_to_protocol_asset_id(_unique_asset_id: Self::AssetId) -> u32 {
		1
	}
}

impl pallet_assets::Config for Test {
	type AssetId = CurrencyId;
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

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

/// Native <-> Cosmwasm account mapping
pub struct AccountToAddr;
impl Convert<alloc::string::String, Result<AccountId, ()>> for AccountToAddr {
	fn convert(a: alloc::string::String) -> Result<AccountId, ()> {
		match a.strip_prefix("0x") {
			Some(account_id) => Ok(<[u8; 32]>::try_from(hex::decode(account_id).map_err(|_| ())?)
				.map_err(|_| ())?
				.into()),
			_ => Err(()),
		}
	}
}
impl Convert<AccountId, alloc::string::String> for AccountToAddr {
	fn convert(a: AccountId) -> alloc::string::String {
		alloc::format!("0x{}", hex::encode(a))
	}
}

/// Native <-> Cosmwasm asset mapping
pub struct AssetToDenom;
impl Convert<alloc::string::String, Result<CurrencyId, ()>> for AssetToDenom {
	fn convert(currency_id: alloc::string::String) -> Result<CurrencyId, ()> {
		core::str::FromStr::from_str(&currency_id).map_err(|_| ())
	}
}
impl Convert<CurrencyId, alloc::string::String> for AssetToDenom {
	fn convert(CurrencyId(currency_id): CurrencyId) -> alloc::string::String {
		alloc::format!("{}", currency_id)
	}
}

parameter_types! {
	pub const CosmwasmPalletId: PalletId = PalletId(*b"cosmwasm");
	pub const ChainId: &'static str = "composable-network-dali";
	pub const MaxFrames: u32 = 64;
	pub const MaxCodeSize: u32 = 512 * 1024;
	pub const MaxInstrumentedCodeSize: u32 = 1024 * 1024;
	pub const MaxMessageSize: u32 = 256 * 1024;
	pub const MaxContractLabelSize: u32 = 64;
	pub const MaxContractTrieIdSize: u32 = H256::len_bytes() as u32;
	pub const MaxInstantiateSaltSize: u32 = 128;
	pub const MaxFundsAssets: u32 = 32;
	pub const CodeTableSizeLimit: u32 = 4096;
	pub const CodeGlobalVariableLimit: u32 = 256;
	pub const CodeParameterLimit: u32 = 128;
	pub const CodeBranchTableSizeLimit: u32 = 256;
	// Not really required as it's embedded.
	pub const CodeStackLimit: u32 = u32::MAX;

	// TODO: benchmark for proper values
	pub const CodeStorageByteDeposit: u32 = 1;
	pub const ContractStorageByteReadPrice: u32 = 1;
	pub const ContractStorageByteWritePrice: u32 = 1;
}

impl Config for Test {
	type Event = Event;
	type AccountIdExtended = AccountId;
	type PalletId = CosmwasmPalletId;
	type MaxFrames = MaxFrames;
	type MaxCodeSize = MaxCodeSize;
	type MaxInstrumentedCodeSize = MaxInstrumentedCodeSize;
	type MaxMessageSize = MaxMessageSize;
	type AccountToAddr = AccountToAddr;
	type AssetToDenom = AssetToDenom;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type Assets = Assets;
	type NativeAsset = Balances;
	type ChainId = ChainId;
	type MaxContractLabelSize = MaxContractLabelSize;
	type MaxContractTrieIdSize = MaxContractTrieIdSize;
	type MaxInstantiateSaltSize = MaxInstantiateSaltSize;
	type MaxFundsAssets = MaxFundsAssets;
	type CodeTableSizeLimit = CodeTableSizeLimit;
	type CodeGlobalVariableLimit = CodeGlobalVariableLimit;
	type CodeParameterLimit = CodeParameterLimit;
	type CodeBranchTableSizeLimit = CodeBranchTableSizeLimit;
	type CodeStackLimit = CodeStackLimit;
	type CodeStorageByteDeposit = CodeStorageByteDeposit;
	type ContractStorageByteReadPrice = ContractStorageByteReadPrice;
	type ContractStorageByteWritePrice = ContractStorageByteWritePrice;
	type UnixTime = Timestamp;
	// TODO: proper weights
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let origin = frame_benchmarking::account("signer", 0, 0xCAFEBABE);
	let balances: Vec<(AccountId, Balance)> = vec![(origin, 1_000_000_000_000_000_000)];
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	let genesis = pallet_balances::GenesisConfig::<Test> { balances };
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}
