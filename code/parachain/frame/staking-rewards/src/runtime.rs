use composable_tests_helpers::test::currency::{PICA, XPICA};
use composable_traits::{
	account_proxy::ProxyType,
	governance::{GovernanceRegistry, SignedRawOrigin},
};
use frame_support::pallet_prelude::*;
use sp_core::{
	sr25519::{Public, Signature},
	H256,
};

use composable_traits::{account_proxy::AccountProxyWrapper, fnft::FnftAccountProxyType};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{Everything, InstanceFilter},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use orml_traits::{parameter_type_with_key, GetByKey, LockIdentifier};
use sp_core::sr25519;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
};

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Balance = u128;
pub type Amount = i128;
pub type FinancialNftInstanceId = u64;

type CurrencyId = u128;

pub const ALICE: Public = Public([
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);
pub const BOB: Public = Public([
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
]);
pub const CHARLIE: Public = Public([
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
]);
pub const DAVE: Public = Public([
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
]);

ord_parameter_types! {
	pub const RootAccount: AccountId = ALICE;
}

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
		CurrencyFactory: pallet_currency_factory,
		Tokens: orml_tokens,
		Assets: pallet_assets,
		FinancialNft: pallet_fnft,
		Proxy: pallet_proxy,
		StakingRewards: crate,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
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
	pub const ExistentialDeposit: u64 = 1000;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

pub const MILLISECS_PER_BLOCK: u64 = 6000;

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl pallet_currency_factory::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRoot<AccountId>;
	type Balance = Balance;
	type WeightInfo = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		if *currency_id == XPICA::ID {
			0
		} else {
			5
			}
	};
}

pub struct CurrencyHooks;
impl orml_traits::currency::MutationHooks<AccountId, CurrencyId, Balance> for CurrencyHooks {
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
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = frame_support::traits::ConstU32<2>;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type CurrencyHooks = CurrencyHooks;
}

pub struct NoopRegistry;

impl<CurrencyId, AccountId> GovernanceRegistry<CurrencyId, AccountId> for NoopRegistry {
	fn set(_k: CurrencyId, _value: SignedRawOrigin<AccountId>) {}
}

impl<CurrencyId>
	GetByKey<
		CurrencyId,
		Result<SignedRawOrigin<sp_core::sr25519::Public>, sp_runtime::DispatchError>,
	> for NoopRegistry
{
	fn get(
		_k: &CurrencyId,
	) -> Result<SignedRawOrigin<sp_core::sr25519::Public>, sp_runtime::DispatchError> {
		Ok(SignedRawOrigin::Root)
	}
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: CurrencyId = PICA::ID;
}

impl pallet_assets::Config for Test {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = CurrencyFactory;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureSignedBy<RootAccount, AccountId>;
	type GovernanceRegistry = NoopRegistry;
	type CurrencyValidator = primitives::currency::ValidateCurrencyId;
}

parameter_types! {
	pub const FnftPalletId: PalletId = PalletId(*b"pal_fnft");
}

type AccountProxyWrapperInstance = AccountProxyWrapper<Test>;
impl pallet_fnft::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxProperties = ConstU32<16>;
	type FinancialNftCollectionId = CurrencyId;
	type FinancialNftInstanceId = FinancialNftInstanceId;
	type ProxyType = ProxyType;
	type AccountProxy = AccountProxyWrapperInstance;
	type ProxyTypeSelector = FnftAccountProxyType;
	type PalletId = FnftPalletId;
	type WeightInfo = ();
}

parameter_types! {
	pub MaxProxies : u32 = 4;
	pub MaxPending : u32 = 32;
	// just make dali simple to proxy
	pub ProxyPrice: u32 = 0;
}

impl pallet_proxy::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = ();
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyPrice;
	type ProxyDepositFactor = ProxyPrice;
	type MaxProxies = MaxProxies;
	type WeightInfo = ();
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ProxyPrice;
	type AnnouncementDepositFactor = ProxyPrice;
}

parameter_types! {
	pub const StakingRewardsPalletId : PalletId = PalletId(*b"stk_rwrd");
	pub const MaxStakingDurationPresets : u32 = 10;
	pub const MaxRewardConfigsPerPool : u32 = 10;
	pub const StakingRewardsLockId: LockIdentifier = *b"stk_lock";
	// REVIEW(benluelo): Use a better value for this?
	pub const TreasuryAccountId: AccountId = sr25519::Public([10_u8; 32]);
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type FinancialNft = FinancialNft;
	type FinancialNftInstanceId = FinancialNftInstanceId;
	type CurrencyFactory = CurrencyFactory;
	type Assets = Assets;
	type UnixTime = Timestamp;
	type ReleaseRewardsPoolsBatchSize = frame_support::traits::ConstU8<13>;
	type PalletId = StakingRewardsPalletId;
	type MaxStakingDurationPresets = MaxStakingDurationPresets;
	type MaxRewardConfigsPerPool = MaxRewardConfigsPerPool;
	type RewardPoolCreationOrigin = EnsureRoot<Self::AccountId>;
	type RewardPoolUpdateOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;

	type LockId = StakingRewardsLockId;
	type TreasuryAccount = TreasuryAccountId;
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::Governance => matches!(
				c,
				// TODO democracy
				RuntimeCall::System(..)
			),
			// ProxyType::Staking => {
			// 	matches!(c, Call::Staking(..) | Call::Session(..) | Call::Utility(..))
			// },
			// ProxyType::IdentityJudgement => matches!(
			// 	c,
			// 	Call::Identity(pallet_identity::Call::provide_judgement { .. }) | Call::Utility(..)
			// ),
			// ProxyType::CancelProxy => {
			// 	matches!(c, Call::Proxy(pallet_proxy::Call::reject_announcement { .. }))
			// },
			// ProxyType::Auction => matches!(
			// 	c,
			// 	Call::Auctions(..) | Call::Crowdloan(..) | Call::Registrar(..) | Call::Slots(..)
			// ),
			_ => false,
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
