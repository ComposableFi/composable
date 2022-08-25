// #generalization?
use crate as pallet_undercollateralized_loans;
use crate::{
	currency::{NORMALIZED, PICA},
	types::LoanId,
};
use composable_traits::{
	currency::{Exponent, LocalAssets},
	defi::DeFiComposableConfig,
	governance::{GovernanceRegistry, SignedRawOrigin},
	oracle::Price,
};
use frame_support::{
	construct_runtime, ord_parameter_types, parameter_types,
	traits::{ConstI64, ConstU128, ConstU16, ConstU32, ConstU64, Everything, GenesisBuild},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use hex_literal::hex;
use num_traits::Zero;
use once_cell::sync::Lazy;
use orml_traits::{parameter_type_with_key, GetByKey};
use primitives::currency::ValidateCurrencyId;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, ConvertInto, Extrinsic as ExtrinsicTrait, IdentifyAccount, IdentityLookup,
		Verify,
	},
	DispatchError,
};
use xcm::latest::SendXcm;

type Block = frame_system::mocking::MockBlock<Runtime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type ReserveIdentifier = [u8; 8];
type ExistentialDeposit = ConstU128<1000>;
type CounterValue = ConstU128<10>;
type ZeroU32 = ConstU32<0>;
type ZeroU64 = ConstU64<0>;

pub type Amount = i128;
pub type Balance = u128;
pub type BlockNumber = u64;
pub type Counter = u128;
pub type LiquidationStrategyId = u32;
pub type OrderId = u32;
pub type Moment = u64;
pub type VaultId = u64;
pub type CurrencyId = crate::currency::CurrencyId;
pub type WhiteListBound = ConstU32<10>;
pub type ScheduleBound = ConstU32<100>;

pub static ALICE: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000000"))
});
pub static BOB: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000001"))
});
pub static CHARLIE: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000002"))
});
pub static JEREMY: Lazy<AccountId> = Lazy::new(|| {
	AccountId::from_raw(hex!("0000000000000000000000000000000000000000000000000000000000000003"))
});
pub const ACCOUNT_INITIAL_AMOUNT: Balance = 1_000_000;
pub const MILLISECS_PER_BLOCK: u64 = 6000;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Assets: pallet_assets::{Pallet, Call, Storage},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		DutchAuction: pallet_dutch_auction::{Pallet, Call, Storage, Event<T>},
		Oracle: pallet_oracle::{Pallet, Call, Storage, Event<T>},
		Liquidations: pallet_liquidations::{Pallet, Call, Event<T>},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Tokens: orml_tokens::{Pallet, Call, Storage, Config<T>, Event<T>},
		Vault: pallet_vault::{Pallet, Call, Storage, Event<T>},
		UndercollateralizedLoans: pallet_undercollateralized_loans::{Pallet, Call, Config, Storage, Event<T>, ValidateUnsigned},
	}
);

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const NativeAssetId: CurrencyId = PICA::ID;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = *ALICE;
}

pub struct NoopRegistry;

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

impl<CurrencyId, AccountId> GovernanceRegistry<CurrencyId, AccountId> for NoopRegistry {
	fn set(_k: CurrencyId, _value: SignedRawOrigin<AccountId>) {}
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
	type GovernanceRegistry = NoopRegistry;
	type CurrencyValidator = ValidateCurrencyId;
}

impl pallet_balances::Config for Runtime {
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

impl DeFiComposableConfig for Runtime {
	type MayBeAssetId = CurrencyId;
	type Balance = Balance;
}

parameter_types! {
	pub const MinimumDeposit: Balance = 0;
	pub DutchAuctionPalletId: PalletId = PalletId(*b"dutchauc");
}

pub struct XcmFake;
impl Into<Result<cumulus_pallet_xcm::Origin, XcmFake>> for XcmFake {
	fn into(self) -> Result<cumulus_pallet_xcm::Origin, XcmFake> {
		unimplemented!("please test via local-integration-tests")
	}
}
impl From<Origin> for XcmFake {
	fn from(_: Origin) -> Self {
		unimplemented!("please test via local-integration-tests")
	}
}
impl SendXcm for XcmFake {
	fn send_xcm(
		_destination: impl Into<xcm::latest::MultiLocation>,
		_message: xcm::latest::Xcm<()>,
	) -> xcm::latest::SendResult {
		unimplemented!("please test via local-integration-tests")
	}
}

impl pallet_dutch_auction::Config for Runtime {
	type Event = Event;
	type UnixTime = Timestamp;
	type OrderId = OrderId;
	type MultiCurrency = Assets;
	type WeightInfo = pallet_dutch_auction::weights::SubstrateWeight<Self>;
	type PositionExistentialDeposit = MinimumDeposit;
	type PalletId = DutchAuctionPalletId;
	type NativeCurrency = Balances;
	type XcmOrigin = XcmFake;
	type AdminOrigin = EnsureRoot<Self::AccountId>;
	type XcmSender = XcmFake;
}

parameter_types! {
	pub const MinBalance: Balance = 0;
	pub const OraclePalletId: PalletId = PalletId(*b"plt_orac");
}

pub struct Decimals;
impl LocalAssets<CurrencyId> for Decimals {
	fn decimals(_currency_id: CurrencyId) -> Result<Exponent, DispatchError> {
		Ok(12)
	}
}

impl pallet_oracle::Config for Runtime {
	type Event = Event;
	type Currency = Assets;
	type AssetId = CurrencyId;
	type PriceValue = Balance;
	type AuthorityId = pallet_oracle::crypto::BathurstStId;
	type MinStake = MinBalance;
	type StakeLock = ZeroU64;
	type StalePrice = ZeroU64;
	type AddOracle = EnsureSignedBy<RootAccount, AccountId>;
	type MaxAnswerBound = ZeroU32;
	type MaxAssetsCount = ZeroU32;
	type MaxHistory = ZeroU32;
	type MaxPrePrices = ZeroU32;
	type WeightInfo = ();
	type LocalAssets = Decimals;
	type TreasuryAccount = RootAccount;
	type TwapWindow = ConstU16<3>;
	type Balance = Balance;
	type RewardOrigin = EnsureRoot<AccountId>;
	type MsPerBlock = ConstU64<MILLISECS_PER_BLOCK>;
	type Moment = Moment;
	type Time = Timestamp;
	type PalletId = OraclePalletId;
}

parameter_types! {
	pub const LiquidationsPalletId : PalletId = PalletId(*b"liqd_tns");
}

impl pallet_liquidations::Config for Runtime {
	type Event = Event;
	type UnixTime = Timestamp;
	type DutchAuction = DutchAuction;
	type LiquidationStrategyId = LiquidationStrategyId;
	type OrderId = OrderId;
	type PalletId = LiquidationsPalletId;
	type WeightInfo = pallet_liquidations::weights::SubstrateWeight<Self>;
	type CanModifyStrategies = EnsureRoot<Self::AccountId>;
	type XcmSender = XcmFake;
	type MaxLiquidationStrategiesAmount = ConstU32<3>;
}

impl pallet_currency_factory::Config for Runtime {
	type Event = Event;
	type AssetId = CurrencyId;
	type AddOrigin = EnsureRoot<AccountId>;
	type Balance = Balance;
	type WeightInfo = ();
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<{ MILLISECS_PER_BLOCK / 2 }>;
	type WeightInfo = ();
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const CreationDeposit: Balance = 10;
	pub const RentPerBlock: Balance = 1;
	pub const MinimumWithdrawal: Balance = 0;
	pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
	pub const TombstoneDuration: u64 = 42;
}

impl pallet_vault::Config for Runtime {
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

parameter_types! {
	pub UncollateralizedLoansPalletId: PalletId = PalletId(*b"ucLoans!");
	pub UncollateralizedLoanId: LoanId = LoanId(*b"UCloanID");
	pub OracleMarketCreationStake: Balance = NORMALIZED::ONE;
}

pub type Extrinsic = TestXt<Call, ()>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicTrait>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

impl pallet_undercollateralized_loans::Config for Runtime {
	type Event = Event;
	type Oracle = Oracle;
	type VaultId = VaultId;
	type Vault = Vault;
	type MultiCurrency = Tokens;
	type NativeCurrency = Balances;
	type CurrencyFactory = LpTokenFactory;
	type Liquidation = Liquidations;
	type LiquidationStrategyId = LiquidationStrategyId;
	type PalletId = UncollateralizedLoansPalletId;
	type LoanId = UncollateralizedLoanId;
	type MaxMarketsCounterValue = CounterValue;
	type MaxLoansPerMarketCounterValue = CounterValue;
	type OracleMarketCreationStake = OracleMarketCreationStake;
	type UnixTime = Timestamp;
	type CheckPaymentsBatchSize = ConstU32<5>;
	type CheckNonActivatedLoansBatchSize = ConstU32<5>;
	type WhiteListBound = WhiteListBound;
	type ScheduleBound = ScheduleBound;
	type MaxRepyamentFails = ConstU128<4>;
	type MaxDateShiftingInDays = ConstI64<365>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Zero::zero()
	};
}

/// Convenience function to set the price of an asset in [`pallet_oracle::Prices`].
///
/// Sets the price at the current `System::block_number()`.
pub fn set_price(asset_id: CurrencyId, new_price: Balance) {
	let price = Price { price: new_price, block: System::block_number() };
	pallet_oracle::Prices::<Runtime>::insert(asset_id, price);
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	let balances = vec![(*ALICE, 1_000_000_000), (*BOB, 1_000_000_000), (*CHARLIE, 1_000_000_000)];

	pallet_balances::GenesisConfig::<Runtime> { balances }
		.assimilate_storage(&mut storage)
		.unwrap();
	pallet_undercollateralized_loans::GenesisConfig {}
		.assimilate_storage::<Runtime>(&mut storage)
		.unwrap();
	GenesisBuild::<Runtime>::assimilate_storage(
		&pallet_liquidations::GenesisConfig {},
		&mut storage,
	)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| {
		System::set_block_number(1);
		Timestamp::set_timestamp(MILLISECS_PER_BLOCK);
	});
	ext
}
