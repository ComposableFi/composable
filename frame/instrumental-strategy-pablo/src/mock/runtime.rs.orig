use frame_support::{
	parameter_types,
	traits::{Everything, GenesisBuild},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSigned};
use orml_traits::parameter_type_with_key;
use pallet_collective::EnsureProportionAtLeast;
use primitives::currency::{CurrencyId, ValidateCurrencyId};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{ConvertInto, IdentityLookup},
	Permill,
};

use crate as pallet_pablo_strategy;
use crate::mock::account_id::AccountId;

pub type Amount = i128;
pub type BlockNumber = u64;
pub type Balance = u128;
pub type PoolId = u128;
pub type Moment = composable_traits::time::Timestamp;
pub type VaultId = u64;

pub const MAX_ASSOCIATED_VAULTS: u32 = 10;
const NATIVE_ASSET: CurrencyId = CurrencyId::PICA;
// These time units are defined in number of blocks.
pub const MILLISECS_PER_BLOCK: Moment = 3000;
pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// -------------------------------------------------------------------------------------------------
//                                              Config
// -------------------------------------------------------------------------------------------------

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

// -------------------------------------------------------------------------------------------------
//                                             Balances
// -------------------------------------------------------------------------------------------------

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

// -------------------------------------------------------------------------------------------------
//                                             Collective
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

type InstrumentalPabloCollective = pallet_collective::Instance1;
impl pallet_collective::Config<InstrumentalPabloCollective> for MockRuntime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<MockRuntime>;
}

// -------------------------------------------------------------------------------------------------
//                                              Tokens
// -------------------------------------------------------------------------------------------------

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		0_u128
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

// -------------------------------------------------------------------------------------------------
//                                         Currency Factory
// -------------------------------------------------------------------------------------------------

impl pallet_currency_factory::Config for MockRuntime {
	type Event = Event;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type AddOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

// -------------------------------------------------------------------------------------------------
//                                               Vault
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const MaxStrategies: usize = 255;
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
	type CreationDeposit = CreationDeposit;
	type ExistentialDeposit = ExistentialDeposit;
	type RentPerBlock = RentPerBlock;
	type NativeCurrency = Balances;
	type VaultId = VaultId;
	type TombstoneDuration = TombstoneDuration;
	type WeightInfo = ();
	type PalletId = VaultPalletId;
}

// -------------------------------------------------------------------------------------------------
//                                    Instrumental Pablo Strategy
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const MaxAssociatedVaults: u32 = MAX_ASSOCIATED_VAULTS;
	pub const InstrumentalPabloStrategyPalletId: PalletId = PalletId(*b"strmxpab");
}

impl pallet_pablo_strategy::Config for MockRuntime {
	type Event = Event;
	type WeightInfo = ();
	type Convert = ConvertInto;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type VaultId = VaultId;
	type Vault = Vault;
	type MaxAssociatedVaults = MaxAssociatedVaults;
	type PoolId = PoolId;
	type Currency = Tokens;
	type Pablo = Pablo;
	type PalletId = InstrumentalPabloStrategyPalletId;
	type ExternalOrigin = EnsureProportionAtLeast<AccountId, InstrumentalPabloCollective, 2, 3>;
}

// -------------------------------------------------------------------------------------------------
//                                        Governance Registry
// -------------------------------------------------------------------------------------------------

impl pallet_governance_registry::Config for MockRuntime {
	type Event = Event;
	type AssetId = CurrencyId;
	type WeightInfo = ();
}

// -------------------------------------------------------------------------------------------------
//                                              Assets
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const NativeAssetId: CurrencyId = NATIVE_ASSET;
}

impl pallet_assets::Config for MockRuntime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = LpTokenFactory;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type CurrencyValidator = ValidateCurrencyId;
	type GovernanceRegistry = GovernanceRegistry;
}

// -------------------------------------------------------------------------------------------------
//                                             Timestamp
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for MockRuntime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

// -------------------------------------------------------------------------------------------------
//                                            Pablo (AMM)
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const PabloPalletId: PalletId = PalletId(*b"pablo_pa");
	pub const MinSaleDuration: BlockNumber = 3600 / 12;
	pub const MaxSaleDuration: BlockNumber = 30 * 24 * 3600 / 12;
	pub const MaxInitialWeight: Permill = Permill::from_percent(95);
	pub const MinFinalWeight: Permill = Permill::from_percent(5);
	pub const TWAPInterval: Moment = MILLISECS_PER_BLOCK * 10;
}

impl pallet_pablo::Config for MockRuntime {
	type Event = Event;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type Convert = ConvertInto;
	type CurrencyFactory = LpTokenFactory;
	type Assets = Assets;
	type PoolId = PoolId;
	type PalletId = PabloPalletId;
	type LocalAssets = LpTokenFactory;
	type LbpMinSaleDuration = MinSaleDuration;
	type LbpMaxSaleDuration = MaxSaleDuration;
	type LbpMaxInitialWeight = MaxInitialWeight;
	type LbpMinFinalWeight = MinFinalWeight;
	type PoolCreationOrigin = EnsureSigned<Self::AccountId>;
	type EnableTwapOrigin = EnsureRoot<AccountId>;
	type Time = Timestamp;
	type TWAPInterval = TWAPInterval;
	type WeightInfo = ();
}

// -------------------------------------------------------------------------------------------------
//                                       Instrumental Strategy
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const InstrumentalStrategyPalletId: PalletId = PalletId(*b"dynamic_");
}

impl instrumental_strategy::Config for MockRuntime {
	type Event = Event;
	type WeightInfo = ();
	type AssetId = CurrencyId;
	type Balance = Balance;
	type VaultId = VaultId;
	type Vault = Vault;
	type PoolId = PoolId;
	type PabloStrategy = PabloStrategy;
	type MaxAssociatedVaults = MaxAssociatedVaults;
	type PalletId = InstrumentalStrategyPalletId;
}

// -------------------------------------------------------------------------------------------------
//                                           Instrumental
// -------------------------------------------------------------------------------------------------

parameter_types! {
	pub const InstrumentalPalletId: PalletId = PalletId(*b"strm____");
}

impl pallet_instrumental::Config for MockRuntime {
	type Event = Event;
	type WeightInfo = ();
	type Balance = Balance;
	type AssetId = CurrencyId;
	type VaultId = VaultId;
	type Vault = Vault;
	type InstrumentalStrategy = InstrumentalStrategy;
	type PalletId = InstrumentalPalletId;
}

// -------------------------------------------------------------------------------------------------
//                                         Construct Runtime
// -------------------------------------------------------------------------------------------------

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
		GovernanceRegistry: pallet_governance_registry::{Pallet, Call, Storage, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Pablo: pallet_pablo::{Pallet, Call, Storage, Event<T>},
		CollectiveInstrumental: pallet_collective::<Instance1>::{Pallet, Call, Event<T>, Origin<T>, Config<T>},

		InstrumentalStrategy: instrumental_strategy::{Pallet, Call, Storage, Event<T>},
		Instrumental: pallet_instrumental::{Pallet, Call, Storage, Event<T>},
		PabloStrategy: pallet_pablo_strategy::{Pallet, Call, Storage, Event<T>},
	}
);

// -------------------------------------------------------------------------------------------------
//                                       Externalities Builder
// -------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct ExtBuilder {
	native_balances: Vec<(AccountId, Balance)>,
	balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage =
			frame_system::GenesisConfig::default().build_storage::<MockRuntime>().unwrap();

		pallet_balances::GenesisConfig::<MockRuntime> { balances: self.native_balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		orml_tokens::GenesisConfig::<MockRuntime> { balances: self.balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		storage.into()
	}

	pub fn initialize_balance(
		mut self,
		user: AccountId,
		asset: CurrencyId,
		balance: Balance,
	) -> ExtBuilder {
		if asset == NATIVE_ASSET {
			self.native_balances.push((user, balance));
		} else {
			self.balances.push((user, asset, balance));
		}
		self
	}
}
