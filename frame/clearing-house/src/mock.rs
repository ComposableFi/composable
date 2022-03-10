use crate as clearing_house;
use composable_traits::defi::DeFiComposableConfig;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64, Everything},
	PalletId,
};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	FixedI128,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		ClearingHouse: clearing_house::{Pallet, Call, Storage, Event<T>},
	}
);

type Balance = u128;
type AssetId = u128;
type Amount = i64;

parameter_types! {
	pub const ClearingHouseId: PalletId = PalletId(*b"test_pid");
}

impl system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_type_with_key! {
	pub TokensExistentialDeposit: |_currency_id: AssetId| -> Balance {
		0
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = TokensExistentialDeposit;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = Everything;
}

impl DeFiComposableConfig for Runtime {
	type Balance = Balance;
	type MayBeAssetId = AssetId;
}

impl clearing_house::Config for Runtime {
	type Event = Event;
	type MarketId = u64;
	type Decimal = FixedI128;
	type Timestamp = u64;
	type Duration = u64;
	type VAMMId = u64;
	type MultiCurrency = Tokens;
	type PalletId = ClearingHouseId;
}
