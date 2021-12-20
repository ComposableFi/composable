use crate::{self as pallet_lending, *};
use composable_traits::{
	currency::{DynamicCurrencyId, PriceableAsset},
	dex::{Orderbook, Price, SellOrder},
	loans::DeFiComposableConfig,
};
use frame_support::{
	parameter_types,
	sp_runtime::Permill,
	traits::{Everything, OnFinalize, OnInitialize},
	PalletId,
};
use hex_literal::hex;
use once_cell::sync::Lazy;
use orml_traits::parameter_type_with_key;
use scale_info::TypeInfo;
use sp_arithmetic::traits::Zero;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify,
	},
	ArithmeticError, DispatchError,
};

pub mod oracle;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;
pub type Amount = i128;
pub type BlockNumber = u64;

pub type VaultId = u64;

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

#[derive(
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Copy,
	Clone,
	codec::Encode,
	codec::Decode,
	serde::Serialize,
	serde::Deserialize,
	TypeInfo,
)]
pub enum MockCurrencyId {
	Pica,
	Btc,
	Eth,
	Ltc,
	Usdt,
	LpToken(u128),
}

impl From<u128> for MockCurrencyId {
	fn from(id: u128) -> Self {
		match id {
			0 => MockCurrencyId::Pica,
			1 => MockCurrencyId::Btc,
			2 => MockCurrencyId::Eth,
			3 => MockCurrencyId::Ltc,
			4 => MockCurrencyId::Usdt,
			5 => MockCurrencyId::LpToken(0),
			_ => unreachable!(),
		}
	}
}

impl Default for MockCurrencyId {
	fn default() -> Self {
		MockCurrencyId::Pica
	}
}

impl PriceableAsset for MockCurrencyId {
	fn smallest_unit_exponent(self) -> composable_traits::currency::Exponent {
		match self {
			MockCurrencyId::Pica => 0,
			MockCurrencyId::Btc => 8,
			MockCurrencyId::Eth => 18,
			MockCurrencyId::Ltc => 8,
			MockCurrencyId::Usdt => 2,
			MockCurrencyId::LpToken(_) => 0,
		}
	}
}

impl DynamicCurrencyId for MockCurrencyId {
	fn next(self) -> Result<Self, DispatchError> {
		match self {
			MockCurrencyId::LpToken(x) => Ok(MockCurrencyId::LpToken(
				x.checked_add(1).ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)),
			_ => unreachable!(),
		}
	}
}

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
		Liquidations: pallet_liquidations::{Pallet, Call, Event<T>},
		Lending: pallet_lending::{Pallet, Call, Config, Storage, Event<T>},
		Oracle: pallet_lending::mocks::oracle::{Pallet},
		Auction: pallet_dutch_auctions::{Pallet, Event<T>},
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
	pub const DynamicCurrencyIdInitial: MockCurrencyId = MockCurrencyId::LpToken(0);
}

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type DynamicCurrencyId = MockCurrencyId;
	type DynamicCurrencyIdInitial = DynamicCurrencyIdInitial;
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: MockCurrencyId = MockCurrencyId::Pica;
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
	type AssetId = MockCurrencyId;
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
	pub ExistentialDeposits: |_currency_id: MockCurrencyId| -> Balance {
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
	type CurrencyId = MockCurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

impl crate::mocks::oracle::Config for Test {
	type VaultId = VaultId;
	type Vault = Vault;
}

impl DeFiComposableConfig for Test {
	type AssetId = MockCurrencyId;
	type Balance = Balance;
	type Currency = Tokens;
}

pub struct MockOrderbook;
impl Orderbook for MockOrderbook {
	type AssetId = MockCurrencyId;
	type Balance = Balance;
	type AccountId = AccountId;
	type OrderId = u128;
	type GroupId = AccountId;

	fn post(
		_account_from: &Self::AccountId,
		_asset: Self::AssetId,
		_want: Self::AssetId,
		_source_amount: Self::Balance,
		_source_price: Price<Self::GroupId, Self::Balance>,
		_amm_slippage: Permill,
	) -> Result<SellOrder<Self::OrderId, Self::AccountId>, DispatchError> {
		Ok(SellOrder { id: 0, account: *ALICE })
	}

	fn market_sell(
		_account: &Self::AccountId,
		_asset: Self::AssetId,
		_want: Self::AssetId,
		_amount: Self::Balance,
		_amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError> {
		Ok(0)
	}

	fn patch(
		_order_id: Self::OrderId,
		_price: Price<Self::GroupId, Self::Balance>,
	) -> Result<(), DispatchError> {
		Ok(())
	}

	fn ask(
		_account: &Self::AccountId,
		_orders: impl Iterator<Item = Self::OrderId>,
		_up_to: Self::Balance,
	) -> Result<(), DispatchError> {
		Ok(())
	}
}

impl pallet_dutch_auctions::Config for Test {
	type Event = Event;
	type DexOrderId = u128;
	type OrderId = u128;
	type UnixTime = Timestamp;
	type Orderbook = MockOrderbook;
	type GroupId = AccountId;
}

impl pallet_liquidations::Config for Test {
	type Event = Event;
	type UnixTime = Timestamp;
	type Lending = Lending;
	type DutchAuction = Auction;
	type GroupId = AccountId;
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
}

impl pallet_lending::Config for Test {
	type Oracle = Oracle;
	type VaultId = VaultId;
	type Vault = Vault;
	type Event = Event;
	type AssetId = MockCurrencyId;
	type Balance = Balance;
	type Currency = Tokens;
	type CurrencyFactory = LpTokenFactory;
	type MarketDebtCurrency = Tokens;
	type Liquidation = Liquidations;
	type UnixTime = Timestamp;
	type MaxLendingCount = MaxLendingCount;
	type AuthorityId = crypto::TestAuthId;
	type WeightInfo = ();
	type GroupId = AccountId;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let balances = vec![];

	pallet_balances::GenesisConfig::<Test> { balances }
		.assimilate_storage(&mut storage)
		.unwrap();
	pallet_lending::GenesisConfig {}
		.assimilate_storage::<Test>(&mut storage)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| {
		System::set_block_number(0);
		Timestamp::set_timestamp(MILLISECS_PER_BLOCK);
		// Initialize BTC price to 50000
		pallet_lending::mocks::oracle::BTCValue::<Test>::set(50000u128);
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
