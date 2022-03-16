use crate as pallet_instrumental;
use crate::currency::{CurrencyId, PICA};

use frame_system::EnsureRoot;
use frame_support::{
	parameter_types,
	traits::Everything, PalletId,
};

use sp_runtime::{
	testing::Header,
	traits::{ConvertInto, IdentityLookup}
};
use sp_core::H256;

use orml_traits::parameter_type_with_key;

pub type BlockNumber = u64;
pub type AccountId = u128;
pub type Balance = u128;
pub type VaultId = u64;
pub type Amount = i128;

pub const ALICE: AccountId = 0;

// ----------------------------------------------------------------------------------------------------
//                                                Config                                               
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for MockRuntime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// ----------------------------------------------------------------------------------------------------
//                                                Balances                                               
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const BalanceExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for MockRuntime {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = BalanceExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

// ----------------------------------------------------------------------------------------------------
//                                                Tokens                                               
// ----------------------------------------------------------------------------------------------------

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		0u128 as Balance
	};
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for MockRuntime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

// ----------------------------------------------------------------------------------------------------
//                                           Currency Factory                                          
// ----------------------------------------------------------------------------------------------------


impl pallet_currency_factory::Config for MockRuntime {
	type Event = Event;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type AddOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------------------------------
//                                                Vault                                                
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: CurrencyId = PICA::ID;
	pub const CreationDeposit: Balance = 10;
	pub const ExistentialDeposit: Balance = 1000;
	pub const RentPerBlock: Balance = 1;
	pub const MinimumDeposit: Balance = 0;
	pub const MinimumWithdrawal: Balance = 0;
	pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
  	pub const TombstoneDuration: u64 = 42;
}

impl pallet_vault::Config for MockRuntime {
	type Event = Event;
	type Currency = Tokens;
	type AssetId = CurrencyId;
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
	type NativeCurrency = Balances;
	type VaultId = VaultId;
	type TombstoneDuration = TombstoneDuration;
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------------------------------
//                                             Instrumental                                            
// ----------------------------------------------------------------------------------------------------

impl pallet_instrumental::Config for MockRuntime {
	type Event = Event;
	type WeightInfo = ();
	type Balance = Balance;
	type AssetId = CurrencyId;
	type VaultId = VaultId;
	type Vault = Vault;
}

// ----------------------------------------------------------------------------------------------------
//                                           Construct Runtime                                         
// ----------------------------------------------------------------------------------------------------

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

frame_support::construct_runtime!(
	pub enum MockRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},

		Vault: pallet_vault::{Pallet, Call, Storage, Event<T>},
		Instrumental: pallet_instrumental::{Pallet, Call, Storage, Event<T>},
	}
);

#[derive(Default)]
pub struct ExtBuilder {
	// balances: Vec<(AccountId, MockCurrencyId, Balance)>,
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<MockRuntime>().unwrap();

		// pallet_balances::GenesisConfig::<Test> { balances: vec![(ALICE, 1_000_000)] }
		// 	.assimilate_storage(&mut t)
		// 	.unwrap();

		t.into()
	}
}
