use crate::{
	self as pallet_dutch_auction,
	mock::currency::{CurrencyId, NativeAssetId},
	weights::SubstrateWeight,
};

use composable_traits::{defi::DeFiComposableConfig, xcm::assets::XcmAssetLocation};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU32, EnsureOneOf, Everything},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use hex_literal::hex;
use orml_traits::parameter_type_with_key;
use sp_core::{
	sr25519::{Public, Signature},
	H256,
};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
};
use xcm::latest::SendXcm;

use super::governance_registry::GovernanceRegistry;
use primitives::currency::ValidateCurrencyId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
pub type Block = frame_system::mocking::MockBlock<Runtime>;
pub type Balance = u128;
pub type OrderId = u32;
pub type Amount = i64;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

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

		CumulusXcm: cumulus_pallet_xcm::{Pallet, Call, Event<T>, Origin},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage},
		DutchAuction: pallet_dutch_auction::{Pallet, Call, Storage, Event<T>},
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
	pub const NativeExistentialDeposit: Balance = 1_000_000_000;
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

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = TokensExistentialDeposit;
	type OnDust = ();
	type MaxLocks = ();
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
	type OnSlash = ();
	type OnDeposit = ();
	type OnTransfer = ();
}

pub static ALICE: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000000"));
pub static BOB: Public =
	Public(hex!("0000000000000000000000000000000000000000000000000000000000000001"));

ord_parameter_types! {
	pub const RootAccount: AccountId = ALICE;
}

impl pallet_assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = LpTokenFactory;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureSignedBy<RootAccount, AccountId>;
	type GovernanceRegistry = GovernanceRegistry;
	type CurrencyValidator = ValidateCurrencyId;
}

impl pallet_currency_factory::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRoot<AccountId>;
	type Balance = Balance;
	type WeightInfo = ();
}

parameter_types! {
	// cspell:disable-next
	pub const DutchAuctionPalletId : PalletId = PalletId(*b"dtch_ctn");
}

// these make some pallets tight coupled onto shared trait
impl DeFiComposableConfig for Runtime {
	type MayBeAssetId = CurrencyId;
	type Balance = Balance;
}

impl pallet_dutch_auction::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UnixTime = Timestamp;
	type OrderId = OrderId;
	type MultiCurrency = Assets;
	type WeightInfo = SubstrateWeight<Self>;
	type PositionExistentialDeposit = NativeExistentialDeposit;
	type PalletId = DutchAuctionPalletId;
	type NativeCurrency = Balances;
	type AdminOrigin = EnsureOneOf<EnsureRoot<AccountId>, EnsureSignedBy<RootAccount, AccountId>>;
	type XcmSender = XcmFake;
	type XcmOrigin = RuntimeOrigin;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = ();
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
	fn send_xcm(
		_destination: impl Into<xcm::latest::MultiLocation>,
		_message: xcm::latest::Xcm<()>,
	) -> xcm::latest::SendResult {
		todo!("please test via local-integration-tests")
	}
}

#[allow(dead_code)] // not really dead
pub fn new_test_externalities() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	let balances =
		vec![(ALICE, 1_000_000_000_000_000_000_000_000), (BOB, 1_000_000_000_000_000_000_000_000)];

	pallet_balances::GenesisConfig::<Runtime> { balances }
		.assimilate_storage(&mut storage)
		.unwrap();

	let mut externalities = sp_io::TestExternalities::new(storage);
	externalities.execute_with(|| {
		System::set_block_number(42);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
	});
	externalities
}
