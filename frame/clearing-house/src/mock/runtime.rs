use crate::{
	self as clearing_house,
	mock::{
		accounts::{AccountId, ALICE},
		assets::{AssetId, PICA},
		oracle as mock_oracle, vamm as mock_vamm,
	},
};
use composable_traits::defi::DeFiComposableConfig;
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU16, ConstU64, Everything, GenesisBuild},
	PalletId,
};
use frame_system as system;
use frame_system::{EnsureRoot, EnsureSignedBy};
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	FixedI128,
};

// ----------------------------------------------------------------------------------------------------
//                                             Construct Runtime
// ----------------------------------------------------------------------------------------------------

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		GovernanceRegistry: governance_registry::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage},
		Vamm: mock_vamm::{Pallet, Storage},
		Oracle: mock_oracle::{Pallet, Storage},
		ClearingHouse: clearing_house::{Pallet, Call, Storage, Event<T>},
	}
);

pub type Balance = u128;
pub type Amount = i64;
pub type VammId = u64;
pub type Decimal = FixedI128;

// ----------------------------------------------------------------------------------------------------
//                                                FRAME System
// ----------------------------------------------------------------------------------------------------

impl system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// ----------------------------------------------------------------------------------------------------
//                                                 Balances
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const NativeExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = NativeExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

// ----------------------------------------------------------------------------------------------------
//                                             Governance Registry
// ----------------------------------------------------------------------------------------------------

impl governance_registry::Config for Runtime {
	type AssetId = AssetId;
	type WeightInfo = ();
	type Event = Event;
}

// ----------------------------------------------------------------------------------------------------
//                                                 ORML Tokens
// ----------------------------------------------------------------------------------------------------

parameter_type_with_key! {
	pub TokensExistentialDeposit: |_currency_id: AssetId| -> Balance {
		0
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = TokensExistentialDeposit;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

// ----------------------------------------------------------------------------------------------------
//                                               Currency Factory
// ----------------------------------------------------------------------------------------------------

impl pallet_currency_factory::Config for Runtime {
	type Event = Event;
	type AssetId = AssetId;
	type AddOrigin = EnsureRoot<AccountId>;
	type ReserveOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------------------------------
//                                                   Assets
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const NativeAssetId: AssetId = PICA;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = ALICE;
}

impl pallet_assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = LpTokenFactory;
	type AssetId = AssetId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureSignedBy<RootAccount, AccountId>;
	type GovernanceRegistry = GovernanceRegistry;
}

// ----------------------------------------------------------------------------------------------------
//                                                   VAMM
// ----------------------------------------------------------------------------------------------------

impl mock_vamm::Config for Runtime {
	type VammId = VammId;
	type Decimal = Decimal;
}

// ----------------------------------------------------------------------------------------------------
//                                                   Oracle
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const MaxAnswerBound: u32 = 5;
}

impl mock_oracle::Config for Runtime {
	type AssetId = AssetId;
	type Balance = Balance;
	type Timestamp = u64;
	type LocalAssets = ();
	type MaxAnswerBound = MaxAnswerBound;
}

// ----------------------------------------------------------------------------------------------------
//                                               Clearing House
// ----------------------------------------------------------------------------------------------------

impl DeFiComposableConfig for Runtime {
	type Balance = Balance;
	type MayBeAssetId = AssetId;
}

parameter_types! {
	pub const ClearingHouseId: PalletId = PalletId(*b"test_pid");
}

impl clearing_house::Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
	type MarketId = u64;
	type Decimal = Decimal;
	type Vamm = Vamm;
	type Oracle = Oracle;
	type Assets = Assets;
	type PalletId = ClearingHouseId;
}

// ----------------------------------------------------------------------------------------------------
//                                             Externalities Builder
// ----------------------------------------------------------------------------------------------------

pub struct ExtBuilder {
	pub native_balances: Vec<(AccountId, Balance)>,
	pub balances: Vec<(AccountId, AssetId, Balance)>,
	pub collateral_types: Vec<AssetId>,
	pub vamm_id: Option<VammId>,
	pub oracle_asset_support: Option<bool>,
}

impl ExtBuilder {
	#[allow(clippy::disallowed_methods)]
	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage =
			frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		pallet_balances::GenesisConfig::<Runtime> { balances: self.native_balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		orml_tokens::GenesisConfig::<Runtime> { balances: self.balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		clearing_house::GenesisConfig::<Runtime> { collateral_types: self.collateral_types }
			.assimilate_storage(&mut storage)
			.unwrap();

		mock_vamm::GenesisConfig::<Runtime> { vamm_id: self.vamm_id }
			.assimilate_storage(&mut storage)
			.unwrap();

		let oracle_genesis =
			mock_oracle::GenesisConfig { supports_assets: self.oracle_asset_support };
		GenesisBuild::<Runtime>::assimilate_storage(&oracle_genesis, &mut storage).unwrap();

		let mut ext: sp_io::TestExternalities = storage.into();
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
