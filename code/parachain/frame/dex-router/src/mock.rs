use crate as dex_router;
use crate::mock_fnft::MockFnft;
use frame_support::{parameter_types, traits::Everything, PalletId};
use frame_system as system;
use orml_traits::{parameter_type_with_key, LockIdentifier};
use scale_info::TypeInfo;
use sp_arithmetic::traits::Zero;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
	Permill,
};
use system::{EnsureRoot, EnsureSigned};

pub type Balance = u128;
pub type AssetId = u128;
pub type Amount = i128;
pub type PoolId = u128;
pub type BlockNumber = u64;
pub type AccountId = u128;
pub type CurrencyId = u128;

#[allow(dead_code)]
pub static ALICE: AccountId = 1;
#[allow(dead_code)]
pub static BOB: AccountId = 2;
#[allow(dead_code)]
pub static CHARLIE: AccountId = 3;
#[allow(dead_code)]
pub static EVE: AccountId = 4;

pub type Moment = composable_traits::time::Timestamp;
pub const USDT: AssetId = 2;
pub const ETH: AssetId = 3;
pub const USDC: AssetId = 4;
pub const DAI: AssetId = 5;
pub const TWAP_INTERVAL: Moment = 10;
pub const MILLISECS_PER_BLOCK: u64 = 12000;

parameter_types! {
	pub const NativeAssetId: AssetId = 0;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Pablo : pallet_pablo::{Pallet, Call, Storage, Event<T>},
		StakingRewards: pallet_staking_rewards::{Pallet, Storage, Call, Event<T>},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		AssetsRegistry: pallet_assets_registry,
		DexRouter: dex_router::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
	}
);

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type AddOrigin = EnsureRoot<AccountId>;
	type Balance = Balance;
	type WeightInfo = ();
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
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		Zero::zero()
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
	type MaxLocks = ();
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnKilledTokenAccount = ();
	type OnNewTokenAccount = ();
}

parameter_types! {
	pub Precision: u128 = 100_u128;
	pub TestPalletID : PalletId = PalletId(*b"pablo_pa");
	pub MinSaleDuration: BlockNumber = 3600 / 12;
	pub MaxSaleDuration: BlockNumber = 30 * 24 * 3600 / 12;
	pub MaxInitialWeight: Permill = Permill::from_percent(95);
	pub MinFinalWeight: Permill = Permill::from_percent(5);
	pub const TWAPInterval: Moment = MILLISECS_PER_BLOCK * TWAP_INTERVAL;
}

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	// cspell:disable-next
	pub const StakingRewardsPalletId: PalletId = PalletId(*b"stk_rwrd");
	pub const StakingRewardsLockId: LockIdentifier = *b"stk_lock";
	pub const MaxStakingDurationPresets: u32 = 10;
	pub const MaxRewardConfigsPerPool: u32 = 10;
	pub const PicaAssetId : CurrencyId = 1;
	pub const PbloAssetId : CurrencyId = 2;
	pub const XPicaAssetId: CurrencyId = 101;
	pub const XPbloAssetId: CurrencyId = 102;
	pub const PicaStakeFinancialNftCollectionId: CurrencyId = 1001;
	pub const PbloStakeFinancialNftCollectionId: CurrencyId = 1002;
	// TODO(benluelo): Use a better value here?
	pub const TreasuryAccountId: AccountId = 123_456_789_u128;
	pub const RootAccount: AccountId = 0;
}

impl pallet_assets_registry::Config for Test {
	type Event = Event;
	type LocalAssetId = CurrencyId;
	type Balance = Balance;
	type ForeignAssetId = composable_traits::xcm::assets::XcmAssetLocation;
	type UpdateAssetRegistryOrigin = EnsureRoot<AccountId>;
	type ParachainOrGovernanceOrigin = EnsureRoot<AccountId>;
	type CurrencyFactory = LpTokenFactory;
	type WeightInfo = ();
}

impl pallet_staking_rewards::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type AssetId = AssetId;
	type Assets = Tokens;
	type CurrencyFactory = LpTokenFactory;
	type UnixTime = Timestamp;
	type ReleaseRewardsPoolsBatchSize = frame_support::traits::ConstU8<13>;
	type PalletId = StakingRewardsPalletId;
	type MaxStakingDurationPresets = MaxStakingDurationPresets;
	type MaxRewardConfigsPerPool = MaxRewardConfigsPerPool;
	type RewardPoolCreationOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
	type RewardPoolUpdateOrigin = EnsureRoot<Self::AccountId>;
	type FinancialNft = MockFnft;
	type FinancialNftInstanceId = u64;
	type PicaAssetId = PicaAssetId;
	type PbloAssetId = PbloAssetId;
	type XPicaAssetId = XPicaAssetId;
	type XPbloAssetId = XPbloAssetId;
	type PicaStakeFinancialNftCollectionId = PicaStakeFinancialNftCollectionId;
	type PbloStakeFinancialNftCollectionId = PbloStakeFinancialNftCollectionId;
	type LockId = StakingRewardsLockId;
	type TreasuryAccount = TreasuryAccountId;
	type ExistentialDeposits = ExistentialDeposits;
}

parameter_types! {
	pub const MaxStakingRewardPools: u32 = 10;
	pub const MillisecsPerBlock: u32 = 12000;
}

impl pallet_pablo::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type Assets = Tokens;
	type AssetsRegistry = AssetsRegistry;
	type Convert = ConvertInto;
	type PoolId = PoolId;
	type PalletId = TestPalletID;
	type LocalAssets = LpTokenFactory;
	type PoolCreationOrigin = EnsureSigned<Self::AccountId>;
	type EnableTwapOrigin = EnsureRoot<AccountId>;
	type Time = Timestamp;
	type TWAPInterval = TWAPInterval;
	type WeightInfo = ();
	type MaxStakingRewardPools = MaxStakingRewardPools;
	type MaxRewardConfigsPerPool = MaxRewardConfigsPerPool;
	type MaxStakingDurationPresets = MaxStakingDurationPresets;
	type MsPerBlock = MillisecsPerBlock;
	type PicaAssetId = PicaAssetId;
	type PbloAssetId = PbloAssetId;
}

parameter_types! {
	#[derive(TypeInfo, codec::MaxEncodedLen, codec::Encode)]
	pub const MaxHopsCount: u32 = 4;
	// cspell:disable-next
	pub TestDexRouterPalletID: PalletId = PalletId(*b"dex_rout");
}

impl dex_router::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type MaxHopsInRoute = MaxHopsCount;
	type PoolId = PoolId;
	type Pablo = Pablo;
	type PalletId = TestDexRouterPalletID;
	type WeightInfo = ();
	type UpdateRouteOrigin = EnsureRoot<AccountId>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.expect("Storage is vlaid; QED")
		.into()
}
