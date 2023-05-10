use crate::{
	self as pallet_liquidations,
	mock::currency::{CurrencyId, NativeAssetId},
	weights::SubstrateWeight,
};

use composable_traits::defi::DeFiComposableConfig;
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU32, Everything, GenesisBuild},
	weights::{WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial},
	PalletId,
};
use frame_system::EnsureRoot;
use hex_literal::hex;
use orml_traits::parameter_type_with_key;
use primitives::currency::ForeignAssetId;
use smallvec::smallvec;
use sp_core::{
	sr25519::{Public, Signature},
	H256,
};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentifyAccount, IdentityLookup, Verify},
	Perbill,
};
use xcm::latest::SendXcm;

use super::governance_registry::GovernanceRegistry;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
pub type Block = frame_system::mocking::MockBlock<Runtime>;
pub type Balance = u128;
pub type OrderId = u32;
pub type Amount = i64;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type SystemOriginOf<T> = <T as frame_system::Config>::RuntimeOrigin;

frame_support::construct_runtime! {
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System : frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		AssetsRegistry: pallet_assets_registry,
		Assets: pallet_assets_transactor_router,
		DutchAuction: pallet_dutch_auction::{Pallet, Call, Storage, Event<T>},
		Liquidations: pallet_liquidations::{Pallet, Call, Storage, Event<T>},
	}
}

parameter_types! {
	pub const SS58Prefix: u8 = 42;
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
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
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const NativeExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = NativeExistentialDeposit;
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

impl pallet_timestamp::Config for Runtime {
	type Moment = composable_traits::time::Timestamp;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_type_with_key! {
	pub TokensExistentialDeposit: |_currency_id: CurrencyId| -> Balance {
		0
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
impl orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = TokensExistentialDeposit;
	type MaxLocks = ();
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type CurrencyHooks = CurrencyHooks;
}

pub static ALICE: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000000"));
pub static BOB: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000001"));
pub static CHARLIE: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000002"));

ord_parameter_types! {
	pub const RootAccount: AccountId = ALICE;
}

impl pallet_assets_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type LocalAssetId = CurrencyId;
	type ForeignAssetId = ForeignAssetId;
	type UpdateAssetRegistryOrigin = EnsureRoot<AccountId>;
	type ParachainOrGovernanceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type Balance = Balance;
	type Convert = ConvertInto;
}

impl pallet_assets_transactor_router::Config for Runtime {
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeAssetId = NativeAssetId;
	type NativeTransactor = Balances;
	type LocalTransactor = Tokens;
	type ForeignTransactor = Tokens;
	type GovernanceRegistry = GovernanceRegistry;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type AssetLocation = ForeignAssetId;
	type AssetsRegistry = AssetsRegistry;
}

parameter_types! {
	pub const DutchAuctionPalletId : PalletId = PalletId(*b"dtch_ctn");
}

// these make some pallets tight coupled onto shared trait
impl DeFiComposableConfig for Runtime {
	type MayBeAssetId = CurrencyId;
	type Balance = Balance;
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

pub struct XcmFake;
impl Into<Result<cumulus_pallet_xcm::Origin, XcmFake>> for XcmFake {
	fn into(self) -> Result<cumulus_pallet_xcm::Origin, XcmFake> {
		todo!("please test via local-integration-tests")
	}
}
impl From<RuntimeOrigin> for XcmFake {
	fn from(_: RuntimeOrigin) -> Self {
		todo!("please test via local-integration-tests")
	}
}
impl SendXcm for XcmFake {
	type Ticket = ();

	fn validate(
		_destination: &mut Option<xcm::v3::MultiLocation>,
		_message: &mut Option<xcm::v3::Xcm<()>>,
	) -> xcm::v3::SendResult<Self::Ticket> {
		todo!("please test via local-integration-tests")
	}

	fn deliver(
		_ticket: Self::Ticket,
	) -> core::result::Result<xcm::v3::XcmHash, xcm::v3::SendError> {
		todo!("please test via local-integration-tests")
	}
}

impl pallet_dutch_auction::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UnixTime = Timestamp;
	type OrderId = OrderId;
	type MultiCurrency = Assets;
	type WeightInfo = pallet_dutch_auction::weights::SubstrateWeight<Self>;
	type PalletId = DutchAuctionPalletId;
	type NativeCurrency = Balances;
	type PositionExistentialDeposit = NativeExistentialDeposit;
	type AdminOrigin = EnsureRoot<Self::AccountId>;
	type XcmSender = XcmFake;
	type XcmOrigin = XcmFake;
}

parameter_types! {
	pub const LiquidationPalletId : PalletId = PalletId(*b"liquidat");
}

type LiquidationStrategyId = u32;
impl pallet_liquidations::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UnixTime = Timestamp;
	type OrderId = OrderId;
	type WeightInfo = SubstrateWeight<Self>;
	type DutchAuction = DutchAuction;
	type LiquidationStrategyId = LiquidationStrategyId;
	type PalletId = LiquidationPalletId;
	type CanModifyStrategies = EnsureRoot<Self::AccountId>;
	type XcmSender = XcmFake;
	type MaxLiquidationStrategiesAmount = ConstU32<3>;
}

#[allow(dead_code)] // not really dead
pub fn new_test_externalities() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	let balances =
		vec![(ALICE, 1_000_000_000_000_000_000_000_000), (BOB, 1_000_000_000_000_000_000_000_000)];

	pallet_balances::GenesisConfig::<Runtime> { balances }
		.assimilate_storage(&mut storage)
		.unwrap();

	<pallet_liquidations::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
		&pallet_liquidations::GenesisConfig {},
		&mut storage,
	)
	.unwrap();

	let mut externalities = sp_io::TestExternalities::new(storage);
	externalities.execute_with(|| {
		System::set_block_number(42);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
	});
	externalities
}
