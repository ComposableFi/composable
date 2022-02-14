use crate as dex_router;
use composable_traits::dex::{ConversionError, SafeConvert};
use frame_support::{parameter_types, traits::Everything, PalletId};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use scale_info::TypeInfo;
use sp_arithmetic::{traits::Zero, FixedU128};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, Convert, ConvertInto, IdentityLookup},
	FixedPointNumber, Permill,
};
use system::EnsureRoot;

pub type CurrencyId = u128;

pub const USDT: CurrencyId = 2;
pub const ETH: CurrencyId = 3;
pub const USDC: CurrencyId = 4;

parameter_types! {
	pub const NativeAssetId: CurrencyId = 0;
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
		CurveAmm: pallet_curve_amm::{Pallet, Call, Storage, Event<T>},
		ConstantProductAmm: pallet_uniswap_v2::{Pallet, Call, Storage, Event<T>},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		DexRouter: dex_router::{Pallet, Call, Storage, Event<T>},
	}
);

impl pallet_currency_factory::Config for Test {
	type Event = Event;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRoot<AccountId>;
	type ReserveOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

pub type AccountId = u64;

#[allow(dead_code)]
pub static ALICE: AccountId = 1;
#[allow(dead_code)]
pub static BOB: AccountId = 2;
#[allow(dead_code)]
pub static CHARLIE: AccountId = 3;
#[allow(dead_code)]
pub static CURVE_ADMIN_FEE_ACC_ID: AccountId = 4;

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

pub type Balance = u128;
pub type AssetId = u128;
pub type Amount = i128;
pub type PoolId = u32;

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

parameter_types! {
	pub CurveAmmPrecision: u128 = 100;
	pub CurveAmmTestPalletID : PalletId = PalletId(*b"curve_am");
}

pub type Number = FixedU128;
pub struct ConvertType;

impl SafeConvert<Balance, Number> for ConvertType {
	fn convert(a: Balance) -> Result<Number, composable_traits::dex::ConversionError> {
		let accuracy = 1_000_000_000_000;
		let value = a.checked_mul(accuracy).ok_or(ConversionError)?;
		Ok(FixedU128::from_inner(value))
	}
}

impl Convert<Permill, Number> for ConvertType {
	fn convert(a: Permill) -> Number {
		a.into()
	}
}

impl Convert<u16, Number> for ConvertType {
	fn convert(a: u16) -> Number {
		FixedU128::saturating_from_integer(a)
	}
}

impl SafeConvert<Number, Balance> for ConvertType {
	fn convert(a: Number) -> Result<Balance, composable_traits::dex::ConversionError> {
		let accuracy = 1_000_000_000_000;
		(a.into_inner() / accuracy).try_into().map_err(|_| ConversionError)
	}
}

impl pallet_curve_amm::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type Number = Number;
	type CurrencyFactory = LpTokenFactory;
	type Convert = ConvertType;
	type Precision = CurveAmmPrecision;
	type Assets = Tokens;
	type PoolId = PoolId;
	type PalletId = CurveAmmTestPalletID;
	type WeightInfo = ();
}

parameter_types! {
	pub ConstantProductAmmPrecision: FixedU128 = FixedU128::saturating_from_rational(1, 1_000_000_000);
	pub ConstantProductAmmTestPalletID : PalletId = PalletId(*b"const_am");
}

impl pallet_uniswap_v2::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type CurrencyFactory = LpTokenFactory;
	type Convert = ConvertInto;
	type Assets = Tokens;
	type PoolId = PoolId;
	type PalletId = ConstantProductAmmTestPalletID;
	type WeightInfo = ();
}
parameter_types! {
  #[derive(codec::Encode, codec::Decode, codec::MaxEncodedLen, TypeInfo)]
	pub const MaxHopsCount: u32 = 4;
}

impl dex_router::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type MaxHopsInRoute = MaxHopsCount;
	type PoolId = u32;
	type StableSwapDex = CurveAmm;
	type ConstantProductDex = ConstantProductAmm;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
