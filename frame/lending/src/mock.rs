use crate as pallet_lending;
use composable_traits::currency::CurrencyFactory;
use frame_support::{ord_parameter_types, parameter_types, traits::Contains, PalletId};
use frame_system::{self as system, EnsureSignedBy};
use orml_tokens::TransferDust;
use orml_traits::parameter_type_with_key;
use sp_arithmetic::traits::Zero;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{
		AccountIdConversion, BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount,
		IdentityLookup, Verify,
	},
	DispatchError,
};

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Extrinsic = TestXt<Call, ()>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
type MockCurrencyId = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Oracle: pallet_oracle::{Pallet, Call, Storage, Event<T>},
		Vault: pallet_vault::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		Lending: pallet_lending::{Pallet, Storage},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sp_core::sr25519::Public;
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
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1000;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

const MILLISECS_PER_BLOCK: u64 = 6000;

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const StakeLock: u64 = 1;
	pub const MinStake: u64 = 1;
	pub const StalePrice: u64 = 2;
	pub const RequestCost: u64 = 1;
	pub const RewardAmount: u64 = 5;
	pub const SlashAmount: u64 = 5;
	pub const MaxAnswerBound: u32 = 5;
	pub const MaxAssetsCount: u32 = 2;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = root_account();
}

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

impl pallet_oracle::Config for Test {
	type Event = Event;
	type AuthorityId = pallet_oracle::crypto::TestAuthId;
	type Currency = Balances;
	type StakeLock = StakeLock;
	type StalePrice = StalePrice;
	type MinStake = MinStake;
	type AddOracle = EnsureSignedBy<RootAccount, sp_core::sr25519::Public>;
	type RequestCost = RequestCost;
	type RewardAmount = RewardAmount;
	type SlashAmount = SlashAmount;
	type MaxAnswerBound = MaxAnswerBound;
	type MaxAssetsCount = MaxAssetsCount;
	type WeightInfo = ();
}

pub struct SimpleFactory;

impl CurrencyFactory<MockCurrencyId> for SimpleFactory {
	fn create() -> Result<MockCurrencyId, DispatchError> {
		Ok(1)
	}
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: MockCurrencyId = 1;
	pub const CreationDeposit: Balance = 10;
	pub const RentPerBlock: Balance = 1;
}

impl pallet_vault::Config for Test {
	type Event = Event;
	type Currency = Tokens;
	type CurrencyId = MockCurrencyId;
	type Balance = Balance;
	type MaxStrategies = MaxStrategies;
	type CurrencyFactory = SimpleFactory;
	type Convert = ConvertInto;
	type StrategyReport = ();

	type CreationDeposit = CreationDeposit;
	type ExistentialDeposit = ExistentialDeposit;
	type RentPerBlock = RentPerBlock;
	type NativeAssetId = NativeAssetId;
}

pub struct MockDustRemovalWhitelist;

impl Contains<AccountId> for MockDustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		*a == DustReceiver::get()
	}
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: MockCurrencyId| -> Balance {
		Zero::zero()
	};
}

parameter_types! {
	pub DustReceiver: AccountId = PalletId(*b"orml/dst").into_account();
	pub MaxLocks: u32 = 2;
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = i64;
	type CurrencyId = MockCurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = TransferDust<Test, DustReceiver>;
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = MockDustRemovalWhitelist;
}

impl pallet_lending::Config for Test {
	type Oracle = Oracle;
	type VaultId = u64;
	type Vault = Vault;
	type AssetId = MockCurrencyId;
	type Balance = Balance;
	type Currency = Tokens;
	type UnixTime = Timestamp;
}

fn root_account() -> AccountId {
	AccountId::from_raw([0; 32])
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let balances = vec![(root_account(), 100_000_000)];

	pallet_balances::GenesisConfig::<Test> { balances }.assimilate_storage(&mut t).unwrap();

	t.into()
}
