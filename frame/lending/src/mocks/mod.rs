use self::currency::CurrencyId;
use crate::{self as pallet_lending, *};
use composable_traits::{
	defi::DeFiComposableConfig,
	governance::{GovernanceRegistry, SignedRawOrigin},
};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{Everything, GenesisBuild, OnFinalize, OnInitialize},
	weights::{WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial},
	PalletId,
};
use frame_system::EnsureSignedBy;
use hex_literal::hex;
use once_cell::sync::Lazy;
use orml_traits::{parameter_type_with_key, GetByKey};
use smallvec::smallvec;
use sp_arithmetic::traits::Zero;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify,
	},
	Perbill,
};

pub mod currency;
pub mod oracle;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;
pub type Amount = i128;
pub type BlockNumber = u64;
pub type VaultId = u64;

pub type LiquidationStrategyId = u32;
pub type OrderId = u32;

parameter_types! {
	pub const LiquidationsPalletId : PalletId = PalletId(*b"liqd_tns");
}

pub type ParachainId = u32;

pub const MINIMUM_BALANCE: Balance = 1000;

pub static ALICE: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000000"))
});
pub static BOB: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000001"))
});
pub static CHARLIE: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000002"))
});
#[allow(dead_code)]
pub static UNRESERVED: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000003"))
});

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
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Vault: pallet_vault::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage},
		Liquidations: pallet_liquidations::{Pallet, Call, Event<T>},
		Lending: pallet_lending::{Pallet, Call, Config, Storage, Event<T>},
		Oracle: pallet_lending::mocks::oracle::{Pallet},
		DutchAuction: pallet_dutch_auction::{Pallet, Call, Storage, Event<T>},
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

parameter_types! {
	pub const DynamicCurrencyIdInitial: CurrencyId = CurrencyId::LpToken(0);
}

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type DynamicCurrencyId = CurrencyId;
	type DynamicCurrencyIdInitial = DynamicCurrencyIdInitial;
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: CurrencyId = CurrencyId::PICA;
	pub const CreationDeposit: Balance = 10;
	pub const RentPerBlock: Balance = 1;
	pub const MinimumDeposit: Balance = 0;
	pub const MinimumWithdrawal: Balance = 0;
	pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
  pub const TombstoneDuration: u64 = 42;
}

impl pallet_vault::Config for Test {
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

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Zero::zero()
	};
}

parameter_types! {
	pub MaxLocks: u32 = 2;
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = *ALICE;
}

impl GovernanceRegistry<CurrencyId, AccountId> for () {
	fn set(_k: CurrencyId, _value: composable_traits::governance::SignedRawOrigin<AccountId>) {}
}

impl
	GetByKey<
		CurrencyId,
		Result<SignedRawOrigin<sp_core::sr25519::Public>, sp_runtime::DispatchError>,
	> for ()
{
	fn get(
		_k: &CurrencyId,
	) -> Result<SignedRawOrigin<sp_core::sr25519::Public>, sp_runtime::DispatchError> {
		Ok(SignedRawOrigin::Root)
	}
}

impl pallet_assets::Config for Test {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = LpTokenFactory;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureSignedBy<RootAccount, AccountId>;
	type GovernanceRegistry = ();
}

impl crate::mocks::oracle::Config for Test {
	type VaultId = VaultId;
	type Vault = Vault;
}

impl DeFiComposableConfig for Test {
	type MayBeAssetId = CurrencyId;
	type Balance = Balance;
}

parameter_types! {
	pub DutchAuctionPalletId: PalletId = PalletId(*b"dutchauc");
}

// later will reuse mocks from that crate
pub struct DutchAuctionsMocks;

impl pallet_dutch_auction::weights::WeightInfo for DutchAuctionsMocks {
	fn ask() -> frame_support::dispatch::Weight {
		0
	}

	fn take() -> frame_support::dispatch::Weight {
		0
	}

	fn liquidate() -> frame_support::dispatch::Weight {
		0
	}

	fn known_overhead_for_on_finalize() -> frame_support::dispatch::Weight {
		0
	}
}

impl WeightToFeePolynomial for DutchAuctionsMocks {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let one = WeightToFeeCoefficient {
			degree: 1,
			coeff_frac: Perbill::zero(),
			coeff_integer: WEIGHT_TO_FEE.with(|v| *v.borrow()),
			negative: false,
		};
		smallvec![one]
	}
}

impl pallet_dutch_auction::Config for Test {
	type Event = Event;
	type OrderId = OrderId;
	type UnixTime = Timestamp;
	type MultiCurrency = Assets;
	type WeightInfo = DutchAuctionsMocks;
	type NativeCurrency = Assets;
	type PalletId = DutchAuctionPalletId;
	type WeightToFee = DutchAuctionsMocks;
}

impl pallet_liquidations::Config for Test {
	type Event = Event;
	type UnixTime = Timestamp;
	type DutchAuction = DutchAuction;
	type LiquidationStrategyId = LiquidationStrategyId;
	type OrderId = OrderId;
	type PalletId = LiquidationsPalletId;
	type WeightInfo = ();
	type ParachainId = ParachainId;
}

pub type Extrinsic = TestXt<Call, ()>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

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

parameter_types! {
	pub const MaxLendingCount: u32 = 10;
	pub LendingPalletId: PalletId = PalletId(*b"liqiudat");
	pub MarketCreationStake : Balance = 10^15;
}

parameter_types! {
	pub static WeightToFee: Balance = 1;
}
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let one = WeightToFeeCoefficient {
			degree: 1,
			coeff_frac: Perbill::zero(),
			coeff_integer: WEIGHT_TO_FEE.with(|v| *v.borrow()),
			negative: false,
		};
		smallvec![one]
	}
}

impl pallet_lending::Config for Test {
	type Oracle = Oracle;
	type VaultId = VaultId;
	type Vault = Vault;
	type Event = Event;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type CurrencyFactory = LpTokenFactory;
	type Liquidation = Liquidations;
	type UnixTime = Timestamp;
	type MaxLendingCount = MaxLendingCount;
	type AuthorityId = crypto::TestAuthId;
	type WeightInfo = ();
	type LiquidationStrategyId = LiquidationStrategyId;
	type PalletId = LendingPalletId;
	type MarketCreationStake = MarketCreationStake;

	type WeightToFee = WeightToFee;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let balances = vec![(*ALICE, 1_000_000_000), (*BOB, 1_000_000_000), (*CHARLIE, 1_000_000_000)];

	pallet_balances::GenesisConfig::<Test> { balances }
		.assimilate_storage(&mut storage)
		.unwrap();
	pallet_lending::GenesisConfig {}
		.assimilate_storage::<Test>(&mut storage)
		.unwrap();
	GenesisBuild::<Test>::assimilate_storage(&pallet_liquidations::GenesisConfig {}, &mut storage)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| {
		System::set_block_number(0);
		Timestamp::set_timestamp(MILLISECS_PER_BLOCK);
		// Initialize BTC price to 50000
		pallet_lending::mocks::oracle::BTCValue::<Test>::set(50000_u128);
	});
	ext
}

/// Progress to the given block, and then finalize the block.
#[allow(dead_code)]
pub fn run_to_block(n: BlockNumber) {
	Lending::on_finalize(System::block_number());
	for b in (System::block_number() + 1)..=n {
		next_block(b);
		if b != n {
			Lending::on_finalize(System::block_number());
		}
	}
}

pub fn process_block(n: BlockNumber) {
	next_block(n);
	Lending::on_finalize(n);
}

fn next_block(n: u64) {
	System::set_block_number(n);
	Lending::on_initialize(n);
	Timestamp::set_timestamp(MILLISECS_PER_BLOCK * n);
}
