use crate::{
	self as clearing_house,
	mock::accounts::{AccountId, ADMIN},
	mock::assets::{AssetId, PICA},
	mock::governance_registry::GovernanceRegistry,
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
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage},
		ClearingHouse: clearing_house::{Pallet, Call, Storage, Event<T>},
	}
);

pub type Balance = u128;
pub type Amount = i64;

impl system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
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

impl pallet_currency_factory::Config for Runtime {
	type Event = Event;
	type AssetId = AssetId;
	type AddOrigin = EnsureRoot<AccountId>;
	type ReserveOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

parameter_types! {
	pub const NativeAssetId: AssetId = PICA;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = ADMIN;
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

impl DeFiComposableConfig for Runtime {
	type Balance = Balance;
	type MayBeAssetId = AssetId;
}

parameter_types! {
	pub const ClearingHouseId: PalletId = PalletId(*b"test_pid");
}

impl clearing_house::Config for Runtime {
	type Event = Event;
	type MarketId = u64;
	type Decimal = FixedI128;
	type Timestamp = u64;
	type Duration = u64;
	type VAMMId = u64;
	type Assets = Assets;
	type PalletId = ClearingHouseId;
}

pub struct ExtBuilder {
	pub native_balances: Vec<(AccountId, Balance)>,
	pub balances: Vec<(AccountId, AssetId, Balance)>,
	pub collateral_types: Vec<AssetId>,
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

		storage.into()
	}
}
