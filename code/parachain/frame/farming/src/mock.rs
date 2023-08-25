use crate::{self as farming, Config, Error};
use composable_traits::currency::{CurrencyFactory, RangeId};
use frame_support::{
	parameter_types,
	traits::{ConstU32, Everything},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use orml_traits::parameter_type_with_key;
use sp_arithmetic::FixedI128;
use sp_core::H256;
use sp_runtime::{
	generic::Header as GenericHeader,
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
	DispatchError,
};
type Header = GenericHeader<BlockNumber, BlakeTwo256>;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, /*Config<T>,*/ Event<T>},
		Rewards: reward::{Pallet, Call, Storage, Event<T>},
		Farming: farming::{Pallet, Call, Storage, Event<T>},

		Assets: pallet_assets::{Pallet, Call, Storage},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},

	}
);

pub type CurrencyId = u128;

pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u128;
pub type Index = u64;
pub type SignedFixedPoint = FixedI128;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = [u8; 8];

	type HoldIdentifier = [u8; 8];

	type MaxHolds = ConstU32<32>;

	type MaxFreezes = ConstU32<32>;
}
impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxLocks: u32 = 50;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		0
	};
}

impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = i128;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type CurrencyHooks = ();
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = Everything;
	type MaxReserves = ConstU32<0>; // we don't use named reserves
	type ReserveIdentifier = (); // we don't use named reserves
}

pub type AssetId = u128;
parameter_types! {
	pub const NativeAssetId: AssetId = 1;
}
pub struct Valid;
impl composable_support::validation::Validate<CurrencyId, primitives::currency::ValidateCurrencyId>
	for Valid
{
	fn validate(input: CurrencyId) -> Result<CurrencyId, &'static str> {
		Ok(input)
	}
}

impl pallet_assets::Config for Test {
	type RuntimeHoldReason = ();
	type NativeAssetId = NativeAssetId;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type MultiCurrency = Tokens;
	type NativeCurrency = Balances;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type CurrencyValidator = Valid;
}

impl reward::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type SignedFixedPoint = SignedFixedPoint;
	type PoolId = CurrencyId;
	type StakeId = AccountId;
	type CurrencyId = CurrencyId;
	type MaxRewardCurrencies = ConstU32<10>;
}

parameter_types! {
	pub const FarmingPalletId: PalletId = PalletId(*b"farmings");
	pub TreasuryAccountId: AccountId = PalletId(*b"treasury").into_account_truncating();
	pub const RewardPeriod: BlockNumber = 10;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type FarmingPalletId = FarmingPalletId;
	type TreasuryAccountId = TreasuryAccountId;
	type RewardPeriod = RewardPeriod;
	type RewardPools = Rewards;
	type AssetId = CurrencyId;
	type MultiCurrency = Assets;
	type WeightInfo = ();
}

pub type TestEvent = RuntimeEvent;
pub type TestError = Error<Test>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		storage.into()
	}
}

pub fn run_test<T>(test: T)
where
	T: FnOnce(),
{
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);
		test();
	});
}
