use crate as pallet_tokenized_options;
use crate::mocks::{accounts::*, assets::*};
use crate::tests::run_to_block;
use composable_traits::{defi::DeFiComposableConfig, governance::SignedRawOrigin, oracle::Price};
use frame_support::traits::{EnsureOneOf, Hooks};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{Everything, GenesisBuild},
	PalletId,
};

use frame_system::{EnsureRoot, EnsureSignedBy};
use orml_traits::{parameter_type_with_key, GetByKey};
use primitives::currency::ValidateCurrencyId;
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{ConvertInto, Extrinsic as ExtrinsicT, IdentityLookup, Verify},
};

pub type BlockNumber = u64;
pub type Balance = u128;
pub type VaultId = u64;
pub type Amount = i128;
pub type Moment = u64;
pub type OptionId = AssetId;

// ----------------------------------------------------------------------------------------------------
//                                             Runtime
// ----------------------------------------------------------------------------------------------------
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

frame_support::construct_runtime!(
	pub enum MockRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		Oracle: pallet_oracle::{Pallet, Storage, Event<T>, Call},
		LpTokenFactory: pallet_currency_factory::{Pallet, Storage, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage},
		Vault: pallet_vault::{Pallet, Call, Storage, Event<T>},
		// GovernanceRegistry: governance::{Pallet, Call, Storage, Event<T>},
		TokenizedOptions: pallet_tokenized_options::{Pallet, Call, Storage, Event<T>},
	}
);

// ----------------------------------------------------------------------------------------------------
//		Frame System Config
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for MockRuntime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// ----------------------------------------------------------------------------------------------------
//		Composable Config
// ----------------------------------------------------------------------------------------------------
impl DeFiComposableConfig for MockRuntime {
	type Balance = Balance;
	type MayBeAssetId = AssetId;
}

// ----------------------------------------------------------------------------------------------------
//		Balances
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const BalanceExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for MockRuntime {
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = BalanceExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

// ----------------------------------------------------------------------------------------------------
//		Timestamp
// ----------------------------------------------------------------------------------------------------

impl pallet_timestamp::Config for MockRuntime {
	type Moment = Moment;
	type OnTimestampSet = ();
	// One second.
	type MinimumPeriod = frame_support::traits::ConstU64<1000>;
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------------------------------
//		Currency Factory
// ----------------------------------------------------------------------------------------------------

impl pallet_currency_factory::Config for MockRuntime {
	type Event = Event;
	type AssetId = AssetId;
	type Balance = Balance;
	type AddOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------------------------------
//		Tokens
// ----------------------------------------------------------------------------------------------------

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		0u128
	};
}

impl orml_tokens::Config for MockRuntime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
	type DustRemovalWhitelist = Everything;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

// ----------------------------------------------------------------------------------------------------
//		Oracle
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const StakeLock: u64 = 1;
	pub const MinStake: u64 = 1;
	pub const StalePrice: u64 = 2;
	pub const MaxAnswerBound: u32 = 5;
	pub const MaxAssetsCount: u32 = 2;
	pub const MaxHistory: u32 = 3;
	pub const MaxPrePrices: u32 = 12;
	pub const TwapWindow: u16 = 3;
}

pub type Extrinsic = TestXt<Call, ()>;

impl frame_system::offchain::SigningTypes for MockRuntime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for MockRuntime
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for MockRuntime
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
	pub const TreasuryAccountId : AccountId = sp_core::sr25519::Public([10u8; 32]);
}

impl pallet_oracle::Config for MockRuntime {
	type Event = Event;
	type AuthorityId = pallet_oracle::crypto::BathurstStId;
	type Currency = Balances;
	type AssetId = AssetId;
	type PriceValue = Balance;
	type StakeLock = StakeLock;
	type StalePrice = StalePrice;
	type MinStake = MinStake;
	type AddOracle = EnsureSignedBy<RootAccount, sp_core::sr25519::Public>;
	type MaxAnswerBound = MaxAnswerBound;
	type MaxAssetsCount = MaxAssetsCount;
	type TreasuryAccount = TreasuryAccountId;
	type MaxHistory = MaxHistory;
	type TwapWindow = TwapWindow;
	type MaxPrePrices = MaxPrePrices;
	type WeightInfo = ();
	type LocalAssets = LpTokenFactory;
}

pub fn set_oracle_price(asset_id: AssetId, balance: Balance) {
	let price = Price { price: balance, block: System::block_number() };
	pallet_oracle::Prices::<MockRuntime>::insert(asset_id, price);
}

pub fn get_oracle_price(asset_id: AssetId, amount: Balance) -> Balance {
	<Oracle as composable_traits::oracle::Oracle>::get_price(asset_id, amount)
		.expect("Error retrieving price")
		.price
}

// ----------------------------------------------------------------------------------------------------
//		Governance Registry
// ----------------------------------------------------------------------------------------------------

pub struct GovernanceRegistry;
impl composable_traits::governance::GovernanceRegistry<AssetId, AccountId> for GovernanceRegistry {
	fn set(_k: AssetId, _value: composable_traits::governance::SignedRawOrigin<AccountId>) {}
}

impl GetByKey<AssetId, Result<SignedRawOrigin<AccountId>, sp_runtime::DispatchError>>
	for GovernanceRegistry
{
	fn get(_k: &AssetId) -> Result<SignedRawOrigin<AccountId>, sp_runtime::DispatchError> {
		Ok(SignedRawOrigin::Root)
	}
}

// ----------------------------------------------------------------------------------------------------
//		Assets
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const NativeAssetId: AssetId = PICA;
}

ord_parameter_types! {
	pub const RootAccount: AccountId = ADMIN;
}

impl pallet_assets::Config for MockRuntime {
	type NativeAssetId = NativeAssetId;
	type GenerateCurrencyId = LpTokenFactory;
	type AssetId = AssetId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureSignedBy<RootAccount, AccountId>;
	type GovernanceRegistry = GovernanceRegistry;
	type CurrencyValidator = ValidateCurrencyId;
}

// ----------------------------------------------------------------------------------------------------
//		Vault
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const CreationDeposit: Balance = 10;
	pub const ExistentialDeposit: Balance = 1000;
	pub const RentPerBlock: Balance = 1;
	pub const MinimumDeposit: Balance = 0;
	pub const MinimumWithdrawal: Balance = 0;
	pub const VaultPalletId: PalletId = PalletId(*b"cubic___");
	pub const TombstoneDuration: u64 = 42;
}

impl pallet_vault::Config for MockRuntime {
	type Event = Event;
	type Balance = Balance;
	type MaxStrategies = MaxStrategies;
	type AssetId = AssetId;
	type CurrencyFactory = LpTokenFactory;
	type Convert = ConvertInto;
	type MinimumDeposit = MinimumDeposit;
	type MinimumWithdrawal = MinimumWithdrawal;
	type PalletId = VaultPalletId;
	type CreationDeposit = CreationDeposit;
	type ExistentialDeposit = ExistentialDeposit;
	type RentPerBlock = RentPerBlock;
	type NativeCurrency = Balances;
	type Currency = Assets;
	type VaultId = VaultId;
	type TombstoneDuration = TombstoneDuration;
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------------------------------
//		Options
// ----------------------------------------------------------------------------------------------------

parameter_types! {
	pub const TokenizedOptionsPalletId: PalletId = PalletId(*b"options_");
	pub const StablecoinAssetId: AssetId = USDC;
}

impl pallet_tokenized_options::Config for MockRuntime {
	type Event = Event;
	type PalletId = TokenizedOptionsPalletId;
	type WeightInfo = ();
	type Oracle = Oracle;
	type Moment = Moment;
	type Convert = ConvertInto;
	type Time = Timestamp;
	type StablecoinAssetId = StablecoinAssetId;
	type LocalAssets = LpTokenFactory;
	type ProtocolOrigin =
		EnsureOneOf<EnsureSignedBy<RootAccount, AccountId>, EnsureRoot<AccountId>>;
	type CurrencyFactory = LpTokenFactory;
	type Assets = Assets;
	type VaultId = VaultId;
	type Vault = Vault;
}

// ----------------------------------------------------------------------------------------------------
//		ExtBuilder
// ----------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct ExtBuilder {
	native_balances: Vec<(AccountId, Balance)>,
	balances: Vec<(AccountId, AssetId, Balance)>,
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage =
			frame_system::GenesisConfig::default().build_storage::<MockRuntime>().unwrap();

		pallet_balances::GenesisConfig::<MockRuntime> { balances: self.native_balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		orml_tokens::GenesisConfig::<MockRuntime> { balances: self.balances }
			.assimilate_storage(&mut storage)
			.unwrap();

		let mut ext: sp_io::TestExternalities = storage.into();

		ext.execute_with(|| {
			System::set_block_number(0);
			System::on_initialize(System::block_number());
			Timestamp::on_initialize(System::block_number());
			TokenizedOptions::on_initialize(System::block_number());
			Timestamp::set(Origin::none(), 0).unwrap();
			Timestamp::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
			run_to_block(1);
		});

		ext
	}

	pub fn initialize_balances(
		mut self,
		balances: impl IntoIterator<Item = (AccountId, AssetId, Balance)>,
	) -> ExtBuilder {
		balances.into_iter().for_each(|(account, asset, balance)| {
			if asset == PICA {
				self.native_balances.push((account, balance));
			} else {
				self.balances.push((account, asset, balance));
			}
		});

		self
	}

	/// Initialize balances from slice of length = account_count * ASSETS_WITH_USDC.len();
	/// account ids will be from 1 to account_count inclusively.
	pub fn initialize_balances_simpl(self, balances: &[Balance]) -> ExtBuilder {
		let asset_count = ASSETS_WITH_USDC.len();
		let iter = balances.iter().enumerate().map(|(j, balance)| {
			let account_id = account_id_from_u64(1 + (j / asset_count) as u64);
			let asset_id = ASSETS_WITH_USDC[j % asset_count];
			(account_id, asset_id, *balance)
		});
		self.initialize_balances(iter)
	}
}
