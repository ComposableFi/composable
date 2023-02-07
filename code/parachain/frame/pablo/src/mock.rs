#![cfg(test)]

use crate as pablo;
use composable_tests_helpers::test::currency;
use composable_traits::{
	governance::{GovernanceRegistry, SignedRawOrigin},
	xcm::assets::XcmAssetLocation,
};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{EitherOfDiverse, Everything},
	PalletId,
};
use frame_system::{self as system, EnsureRoot, EnsureSignedBy};
use orml_traits::{parameter_type_with_key, GetByKey, LockIdentifier};
use sp_arithmetic::traits::Zero;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
	Permill,
};

pub type CurrencyId = u128;
pub type BlockNumber = u64;
pub type Moment = composable_traits::time::Timestamp;

pub const BTC: CurrencyId = currency::BTC::ID;
pub const USDT: CurrencyId = currency::USDT::ID;
pub const USDC: CurrencyId = 4;
pub const LP_TOKEN_ID: CurrencyId = 100;
pub const TWAP_INTERVAL_BLOCKS: Moment = 10;
// TODO(benluelo): Inline this
pub const MILLISECS_PER_BLOCK: u64 = composable_tests_helpers::test::block::MILLISECS_PER_BLOCK;

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
		Pablo: pablo::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances,
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		AssetsRegistry: pallet_assets_registry,
		AssetsTransactor: pallet_assets_transactor_router,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		StakingRewards: pallet_staking_rewards::{Pallet, Storage, Call, Event<T>},
	}
);

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

pub type AccountId = u128;

#[allow(dead_code)]
pub static ALICE: AccountId = 1;
#[allow(dead_code)]
pub static BOB: AccountId = 2;
#[allow(dead_code)]
pub static CHARLIE: AccountId = 3;
#[allow(dead_code)]
pub static DAVE: AccountId = 4;
#[allow(dead_code)]
pub static CURVE_ADMIN_FEE_ACC_ID: AccountId = 5;

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
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub type Balance = u128;
pub type AssetId = u128;
pub type Amount = i128;
pub type PoolId = u128;

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
	pub Precision: u128 = 100_u128;
	pub TestPalletID : PalletId = PalletId(*b"pablo_pa");
	pub MinSaleDuration: BlockNumber = 3600 / 12;
	pub MaxSaleDuration: BlockNumber = 30 * 24 * 3600 / 12;
	pub MaxInitialWeight: Permill = Permill::from_percent(95);
	pub MinFinalWeight: Permill = Permill::from_percent(5);
	pub const TWAPInterval: Moment = MILLISECS_PER_BLOCK * TWAP_INTERVAL_BLOCKS;
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

pub struct NoopRegistry;

impl<AssetId, AccountId> GovernanceRegistry<AssetId, AccountId> for NoopRegistry {
	fn set(_k: AssetId, _value: SignedRawOrigin<AccountId>) {}
}

impl<AssetId> GetByKey<AssetId, Result<SignedRawOrigin<u128>, sp_runtime::DispatchError>>
	for NoopRegistry
{
	fn get(_k: &AssetId) -> Result<SignedRawOrigin<u128>, sp_runtime::DispatchError> {
		Ok(SignedRawOrigin::Root)
	}
}

parameter_types! {
	pub const StakingRewardsPalletId: PalletId = PalletId(*b"stk_rwrd");
	pub const StakingRewardsLockId: LockIdentifier = *b"stk_lock";
	pub const MaxStakingDurationPresets: u32 = 10;
	pub const MaxRewardConfigsPerPool: u32 = 10;
	// REVIEW(benluelo): Use a better value for this?
	pub const TreasuryAccountId: AccountId = 123_456_789_u128;
	pub const ShareAssetExistentialDeposit: Balance = 10_000;
	pub const NativeAssetId: AssetId = 1;
}

impl pallet_assets_registry::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type LocalAssetId = AssetId;
	type ForeignAssetId = XcmAssetLocation;
	type UpdateAssetRegistryOrigin = EnsureRoot<AccountId>;
	type ParachainOrGovernanceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type Balance = Balance;
	type Convert = ConvertInto;
}

impl pallet_assets_transactor_router::Config for Test {
	type AssetId = AssetId;
	type Balance = Balance;
	type NativeAssetId = NativeAssetId;
	type NativeTransactor = Balances;
	type LocalTransactor = Tokens;
	type ForeignTransactor = Tokens;
	type GovernanceRegistry = NoopRegistry;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type AssetLocation = XcmAssetLocation;
	type AssetsRegistry = AssetsRegistry;
}

impl pallet_staking_rewards::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type FinancialNft = pablo::mock_fnft::MockFnft;
	type FinancialNftInstanceId = u64;
	type UnixTime = Timestamp;
	type ReleaseRewardsPoolsBatchSize = frame_support::traits::ConstU8<13>;
	type PalletId = StakingRewardsPalletId;
	type MaxStakingDurationPresets = MaxStakingDurationPresets;
	type MaxRewardConfigsPerPool = MaxRewardConfigsPerPool;
	type RewardPoolCreationOrigin = EnsureRoot<Self::AccountId>;
	type RewardPoolUpdateOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
	type LockId = StakingRewardsLockId;
	type TreasuryAccount = TreasuryAccountId;
	type ExistentialDeposits = ExistentialDeposits;
	type AssetsTransactor = AssetsTransactor;
	type ShareAssetExistentialDeposit = ShareAssetExistentialDeposit;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = ALICE;
	pub const LPTokenED: Balance = 10_000;
}

impl pablo::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = AssetId;
	type Balance = Balance;
	type LPTokenFactory = AssetsTransactor;
	type Assets = Tokens;
	type Convert = ConvertInto;
	type PoolId = PoolId;
	type PalletId = TestPalletID;
	type PoolCreationOrigin = EitherOfDiverse<
		EnsureSignedBy<RootAccount, AccountId>, // for tests
		EnsureRoot<AccountId>,                  // for benchmarks
	>;
	type EnableTwapOrigin = EnsureRoot<AccountId>;
	type Time = Timestamp;
	type TWAPInterval = TWAPInterval;
	type WeightInfo = ();
	type LPTokenExistentialDeposit = LPTokenED;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.expect("success")
		.into()
}
