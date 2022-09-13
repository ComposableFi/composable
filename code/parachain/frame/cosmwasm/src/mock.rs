use crate::*;

use frame_support::{pallet_prelude::ConstU32, parameter_types, traits::ConstU64, PalletId};
use primitives::currency::CurrencyId;
use sp_core::H256;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, Convert, IdentifyAccount, IdentityLookup, Verify},
	AccountId32, MultiSignature,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Header = generic::Header<u32, BlakeTwo256>;
type Balance = u64;
type AccountId = AccountId32;

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
	}
);

parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const ExistentialDeposit: u64 = 10000;
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

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = u64;
	type AssetId = CurrencyId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = ConstU64<1>;
	type AssetAccountDeposit = ConstU64<10>;
	type MetadataDepositBase = ConstU64<1>;
	type MetadataDepositPerByte = ConstU64<1>;
	type ApprovalDeposit = ConstU64<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type WeightInfo = ();
	type Extra = ();
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

/// Native <-> Cosmwasm account mapping
/// TODO(hussein-aitlahcen): Probably nicer to have SS58 representation here.
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
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
