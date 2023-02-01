use crate as pallet_mosaic;
use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild},
	PalletId,
};
use frame_system as system;

use num_traits::Zero;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureRoot;

pub type AccountId = u128;
pub type BlockNumber = u64;
pub type NetworkId = u32;
pub type Balance = u128;
pub type Amount = i128;
pub type AssetId = u128;
pub type RemoteAssetId = [u8; 20];
pub type RemoteAmmId = u128;
pub type AmmMinimumAmountOut = u128;

type Block = frame_system::mocking::MockBlock<Test>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

pub const ALICE: AccountId = 1_u128;
pub const BOB: AccountId = 2_u128;
pub const CHARLIE: AccountId = 3_u128;
pub const RELAYER: AccountId = 4_u128;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		Mosaic: pallet_mosaic::{Pallet, Storage, Event<T>}
	}
);

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
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		Zero::zero()
	};
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
	type MaxLocks = ();
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type CurrencyHooks = CurrencyHooks;
}

parameter_types! {
	pub const MosaicPalletId: PalletId = PalletId(*b"plt_msac");
	pub const MinimumTTL: BlockNumber = 10;
	pub const MinimumTimeLockPeriod: BlockNumber = 20;
}

impl pallet_mosaic::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = MosaicPalletId;
	type Assets = Tokens;
	type MinimumTTL = MinimumTTL;
	type MinimumTimeLockPeriod = MinimumTimeLockPeriod;
	type BudgetPenaltyDecayer = pallet_mosaic::BudgetPenaltyDecayer<Balance, BlockNumber>;
	type NetworkId = NetworkId;
	type RemoteAssetId = RemoteAssetId;
	type ControlOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
	type RemoteAmmId = RemoteAmmId;
	type AmmMinimumAmountOut = AmmMinimumAmountOut;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	ExtBuilder::default().build()
}

pub struct ExtBuilder {
	pub balances: Vec<(AccountId, AssetId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: vec![(ALICE, 1, 1000000)] }
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
