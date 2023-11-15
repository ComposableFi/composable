use crate::{
	types::{EraIndex, PersistedValidationData, Rate, Ratio, UnstakeProvider},
	Balance, DecimalProvider,
};
use cumulus_primitives_core::ParaId;
use frame_support::{
	construct_runtime,
	dispatch::Weight,
	pallet_prelude::*,
	parameter_types, sp_io,
	traits::{
		// tokens::BalanceConversion,
		AsEnsureOriginWithArg,
		EitherOfDiverse,
		Everything,
		GenesisBuild,
		Nothing,
		OriginTrait,
		SortedMembers,
	},
	weights::constants::WEIGHT_REF_TIME_PER_SECOND,
	BoundedSlice, PalletId,
};
use frame_system::{EnsureRoot, EnsureSigned, EnsureSignedBy};
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key};
use orml_xcm_support::{IsNativeConcrete, MultiCurrencyAdapter};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::{IsSystem, Sibling};
pub type Price = FixedU128;
use crate::types::ValidationDataProvider;
use composable_support::collections::vec::bounded::BiBoundedVec;
use composable_traits::{assets::AssetInfo, rational};
use sp_runtime::FixedU128;

// use pallet_loans::{InterestRateModel, JumpModel, Market, MarketState};
// // use pallet_traits::{
// //     xcm::MultiCurrencyAdapter, DecimalProvider, PriceFeeder, ValidationDataProvider,
// // };
use polkadot_runtime_parachains::configuration::HostConfiguration;
// use primitives::{
//     tokens::*, Balance, EraIndex, ParaId, PersistedValidationData, Price, PriceDetail, Rate,
// Ratio, };
use sp_core::H256;
use sp_runtime::{
	generic,
	traits::{
		AccountIdConversion, AccountIdLookup, BlakeTwo256, BlockNumberProvider, Convert, One, Zero,
	},
	AccountId32, DispatchError, FixedPointNumber,
	MultiAddress::Id,
};
use std::{cell::RefCell, collections::HashMap};
pub use xcm::latest::prelude::*;
pub use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
	ChildParachainAsNative, ChildParachainConvertsVia, ChildSystemParachainAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds,
	IsConcrete, NativeAsset, ParentAsSuperuser, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
};
use xcm_executor::{traits::ConvertOrigin, Config, XcmExecutor};
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

pub type AccountId = AccountId32;
// pub type CurrencyId = u64;
use crate::{distribution::AverageDistribution, types::StakingLedger, BalanceOf};
pub use kusama_runtime;
use primitives::currency::CurrencyId;

const KSM: CurrencyId = CurrencyId::KSM;
const SKSM: CurrencyId = CurrencyId::xKSM;
const PICA: CurrencyId = CurrencyId::PICA;

parameter_types! {
	pub const ReservedXcmpWeight: Weight = Weight::from_ref_time(WEIGHT_REF_TIME_PER_SECOND.saturating_div(4));
	pub const ReservedDmpWeight: Weight = Weight::from_ref_time(WEIGHT_REF_TIME_PER_SECOND.saturating_div(4));
}

impl cumulus_pallet_parachain_system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = ParachainInfo;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type OutboundXcmpMessageSource = XcmpQueue;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Test {}

parameter_types! {
	pub DotLocation: MultiLocation = MultiLocation::parent();
	pub RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub UniversalLocation: InteriorMultiLocation = X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
}

pub type LocationToAccountId = (
	ParentIsPreset<AccountId>,
	SiblingParachainConvertsVia<Sibling, AccountId>,
	AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type XcmOriginToCallOrigin = (
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	pub const UnitWeightCost: u64 = 1;
	// pub DotPerSecond: (AssetId, u128, u128) = (AssetId::Concrete(MultiLocation::parent()), 1, 1);
}

parameter_types! {
	pub const NativeCurrencyId: CurrencyId = PICA;
	pub GiftAccount: AccountId = PalletId(*b"par/gift").into_account_truncating();
}

// pub type LocalAssetTransactor = MultiCurrencyAdapter<
//     Assets,
//     IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
//     AccountId,
//     Balance,
//     LocationToAccountId,
//     CurrencyIdConvert,
//     NativeCurrencyId,
//     ExistentialDeposit,
// >;

impl orml_unknown_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}

// pub type LocalAssetTransactor = MultiCurrencyAdapter<
// 	Assets,
// 	UnknownTokens,
// 	IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
// 	AccountId,
// 	LocationToAccountId,
// 	CurrencyIdConvert,
// 	DepositToAlternative<TreasuryAccount, Tokens, CurrencyId, AccountId, Balance>,
// 	// PalletMultihopXcmIbc,
// >;

pub type LocalAssetTransactor = MultiCurrencyAdapter<
	Assets,
	UnknownTokens,
	IsNativeConcrete<CurrencyId, CurrencyIdConvert>,
	AccountId,
	LocationToAccountId,
	CurrencyId,
	CurrencyIdConvert,
	// orml_xcm_support::DepositToAlternative,
	(),
>;

pub type XcmRouter = ParachainXcmRouter<ParachainInfo>;
pub type Barrier = AllowUnpaidExecutionFrom<Everything>;

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToCallOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = ();
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = ();
	type ResponseHandler = ();
	type SubscriptionService = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type AssetLocker = ();
	type AssetExchanger = ();
	type PalletInstancesInfo = ();
	type MaxAssetsIntoHolding = ConstU32<64>;
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
}

pub struct SystemParachainAsSuperuser<Origin>(PhantomData<Origin>);
impl<Origin: OriginTrait> ConvertOrigin<Origin> for SystemParachainAsSuperuser<Origin> {
	fn convert_origin(
		origin: impl Into<MultiLocation>,
		kind: OriginKind,
	) -> Result<Origin, MultiLocation> {
		let origin = origin.into();
		if kind == OriginKind::Superuser &&
			matches!(
				origin,
				MultiLocation {
					parents: 1,
					interior: X1(Parachain(id)),
				} if ParaId::from(id).is_system(),
			) {
			Ok(Origin::root())
		} else {
			Err(origin)
		}
	}
}

impl cumulus_pallet_xcmp_queue::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = SystemParachainAsSuperuser<RuntimeOrigin>;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Test>;
	type PriceForSiblingDelivery = ();
}

impl cumulus_pallet_dmp_queue::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

impl cumulus_pallet_xcm::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

pub type BalanceIdentifier = [u8; 8];

impl pallet_xcm::Config for Test {
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;

	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = ();
	type MaxLockers = ConstU32<8>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<32>;
	type RemoteLockConsumerIdentifier = BalanceIdentifier;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
}

pub struct CurrencyIdConvert;
impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			KSM => Some(MultiLocation::parent()),
			SKSM => Some(MultiLocation::new(
				1,
				X2(
					Parachain(ParachainInfo::parachain_id().into()),
					BoundedSlice::<u8, ConstU32<32>>::truncate_from(b"sKSM".to_vec().as_ref())
						.into(),
				),
			)),
			_ => None,
		}
	}
}

impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(location: MultiLocation) -> Option<CurrencyId> {
		match location {
			MultiLocation { parents: 1, interior: Here } => Some(KSM),
			MultiLocation {
				parents: 1,
				interior: X2(Parachain(id), GeneralKey { data, length }),
			} => {
				let key = &data[..data.len().min(length as usize)];
				if ParaId::from(id) == ParachainInfo::parachain_id() && key == b"sKSM".to_vec() {
					return Some(SKSM)
				}
				None
			},
			_ => None,
		}
	}
}

impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(a: MultiAsset) -> Option<CurrencyId> {
		// if let MultiAsset { id: AssetId::Concrete(id), fun: _ } = a {
		// 	Self::convert(id)
		// } else {
		// 	None
		// }
		todo!();
	}
}

pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account_id: AccountId) -> MultiLocation {
		X1(Junction::AccountId32 { network: None, id: account_id.into() }).into()
	}
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(ParachainInfo::parachain_id().into())));
	pub const BaseXcmWeight: u64 = 100_000_000;
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsForTransfer: usize = 2;
}

parameter_type_with_key! {
	pub ParachainMinFee: |_location: MultiLocation| -> Option<u128> {
		None
	};
}

impl orml_xtokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type CurrencyIdConvert = CurrencyIdConvert;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type SelfLocation = SelfLocation;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type BaseXcmWeight = BaseXcmWeight;
	type UniversalLocation = UniversalLocation;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type ReserveProvider = AbsoluteReserveProvider;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type BlockNumber = u64;

pub const KSM_DECIMAL: u128 = 10u128.pow(12);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
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
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = [u8; 8];

	type HoldIdentifier = [u8; 8];

	type MaxHolds = ConstU32<32>;

	type MaxFreezes = ConstU32<32>;
}

pub struct AliceOrigin;
impl SortedMembers<AccountId> for AliceOrigin {
	fn sorted_members() -> Vec<AccountId> {
		vec![ALICE]
	}
}

pub struct BobOrigin;
impl SortedMembers<AccountId> for BobOrigin {
	fn sorted_members() -> Vec<AccountId> {
		vec![BOB]
	}
}

pub type RelayOrigin =
	EitherOfDiverse<EnsureRoot<AccountId>, EnsureSignedBy<AliceOrigin, AccountId>>;
pub type UpdateOrigin =
	EitherOfDiverse<EnsureRoot<AccountId>, EnsureSignedBy<BobOrigin, AccountId>>;

impl pallet_utility::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = pallet_utility::weights::SubstrateWeight<Test>;
}

parameter_types! {
	pub const XcmHelperPalletId: PalletId = PalletId(*b"par/fees");
	pub const NotifyTimeout: BlockNumber = 100;
	pub RefundLocation: AccountId = para_a_id().into_account_truncating();
}

impl pallet_xcm_helper::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type UpdateOrigin = UpdateOrigin;
	type Assets = Assets;
	type XcmSender = XcmRouter;
	type PalletId = XcmHelperPalletId;
	type RelayNetwork = RelayNetwork;
	type NotifyTimeout = NotifyTimeout;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type RefundLocation = RefundLocation;
	type BlockNumberProvider = frame_system::Pallet<Test>;
	type WeightInfo = ();
	type RelayCurrency = StakingCurrency;
}

impl BlockNumberProvider for RelayChainValidationDataProvider {
	type BlockNumber = BlockNumber;

	fn current_block_number() -> Self::BlockNumber {
		Self::get()
	}
}

impl ValidationDataProvider for RelayChainValidationDataProvider {
	fn validation_data() -> Option<PersistedValidationData> {
		Some(PersistedValidationData {
			parent_head: Default::default(),
			relay_parent_number: 100,
			relay_parent_storage_root: sp_core::hash::H256::from_slice(
				&hex::decode(ROOT_HASH).unwrap(),
			),
			max_pov_size: Default::default(),
		})
	}
}

// block_hash on Kusama
// 0x5a5bc2c15e160df11a7468cb91aca2f6b9db8faa87354099674e955e180b8ee2
// Get proof_bytes
// await api.rpc.state.getReadProof(["0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc405aae5fc2c15c1fd7a2b6d9562c689875d199b535508990c59f411757617904ce65c905fced6878bacfbf26d3b4a1e97"],"0x5a5bc2c15e160df11a7468cb91aca2f6b9db8faa87354099674e955e180b8ee2");
pub const MOCK_KEY: &str = "5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc405aae5fc2c15c1fd7a2b6d9562c689875d199b535508990c59f411757617904ce65c905fced6878bacfbf26d3b4a1e97";
pub const MOCK_DATA: &str = "5d199b535508990c59f411757617904ce65c905fced6878bacfbf26d3b4a1e970f1157e968fea1010f1157e968fea101005101310d0000320d0000330d0000340d0000350d0000360d0000370d0000380d0000390d00003a0d00003b0d00003c0d00003d0d00003e0d00003f0d0000400d0000410d0000420d0000430d0000440d0000450d0000460d0000470d0000480d0000490d00004a0d00004b0d00004c0d00004d0d00004e0d00004f0d0000500d0000510d0000520d0000530d0000540d0000550d0000560d0000570d0000580d0000590d00005a0d00005b0d00005c0d00005d0d00005e0d00005f0d0000600d0000610d0000620d0000630d0000640d0000650d0000660d0000670d0000680d0000690d00006a0d00006b0d00006c0d00006d0d00006e0d00006f0d0000700d0000710d0000720d0000730d0000740d0000750d0000760d0000770d0000780d0000790d00007a0d00007b0d00007c0d00007d0d00007e0d00007f0d0000800d0000810d0000820d0000830d0000840d0000";
pub const ROOT_HASH: &str = "6f5c11cf6bfe2721697af3cecd0a6c5e5a0a6e1bf0671dfd5b68abd433f09764";
pub const MOCK_LEDGER_AMOUNT: Balance = 459589030598417;

pub fn get_mock_proof_bytes() -> Vec<Vec<u8>> {
	[
        hex::decode("800c02809497c8d51db26995948a33a004fca43442ba654fdedb15f5e123059896c634b080ef30f3b11273df6aa7dd4c3cf02c1249822edf25b66854fb98fb269ad0dd066280cd249b8479cf1cc37f8a0fe68fcb610f75503ee68180134814020c92c43ffcb8").unwrap(),
        hex::decode("7f1de5fc2c15c1fd7a2b6d9562c689875d199b535508990c59f411757617904ce65c905fced6878bacfbf26d3b4a1e970d065d199b535508990c59f411757617904ce65c905fced6878bacfbf26d3b4a1e970f1157e968fea1010f1157e968fea101005101310d0000320d0000330d0000340d0000350d0000360d0000370d0000380d0000390d00003a0d00003b0d00003c0d00003d0d00003e0d00003f0d0000400d0000410d0000420d0000430d0000440d0000450d0000460d0000470d0000480d0000490d00004a0d00004b0d00004c0d00004d0d00004e0d00004f0d0000500d0000510d0000520d0000530d0000540d0000550d0000560d0000570d0000580d0000590d00005a0d00005b0d00005c0d00005d0d00005e0d00005f0d0000600d0000610d0000620d0000630d0000640d0000650d0000660d0000670d0000680d0000690d00006a0d00006b0d00006c0d00006d0d00006e0d00006f0d0000700d0000710d0000720d0000730d0000740d0000750d0000760d0000770d0000780d0000790d00007a0d00007b0d00007c0d00007d0d00007e0d00007f0d0000800d0000810d0000820d0000830d0000840d0000").unwrap(),
        hex::decode("8004418026367743456425703e1ce51c51ba1742a59c02411b89b06196477363461e23de785e7df464e44a534ba6b0cbb32407b58734a40d0000019933f2aa7f0100004c5e7b9012096b41c4eb3aaf947f6ea429080000").unwrap(),
        hex::decode("80ffff80232944b9808759b768ba9bbee33bcecc6bb678a845cee7cade527ab250dffee380590491ed9db2709fccd5896bd45755ff6fdaaab133b04af77c1579279e4b68e180005623d575e3acef57307bf4482475f56f6ee5a86c0b1a7163b6b547ea2b893080594ddb56a8bce7f68a3629232fc9e6216126541b96c355d087f4c692bac8de98806a805dd205ac73d747f2432e85c49a151081bbdad5704ea2c92316b09507a0fd8000dffd90f405477faa114057decbb3393717f023f7a854d77b13e1a4b7bbd0c78038a7486fe59f2289b44702b92c2c10e0a098ed325fa9259bc123f6a2e0a52e5180aa131c6196a12fa526137db11fcf1ba5d06c4cf909e720bd9946e830bd41e3cb806bdef693878cb5be73af25d494153e00a8a8507b6152f52ef908b8c29d4a543d80706d1ca2892e685610e4ed6ccd7565a6efe9f76e25c99077b4b831fd11144caa80adc203db51d5ddb3a87e24a416f842946d574a4fcb8893e5f4b898548e8e3d6080b25987b0de9667f44fa319ef2886eabddc49e057c58bf6a66f2efe8db6fbccac804b48b1cee9ba183e6b8414092a41f92f095474465e13bd59e887121e7bf4de8580ee3e1870838e81990e8dd2348a8d1f4cc01507a9e33bc0099b3c5ffedb7ffdd780a3d43738dca1ff59f4ed0ab18099c610544dbb81139c950aace1a39793f626b3801b88e87aec4d02ac33d60c2e2b10dd29c9b9c735b51a25d70a91961b8f300dd6").unwrap(),
        hex::decode("80f7ff80038713baa7e6bb357c33e159ebb93fcf652aa7ce34aa7ad2adbe593aff2e918a80daead2ede3b3393205a686158d62003f19351ea300356e545ae80a31f7ae8f9780238fd143e6c5dc2129e07f85a027269e030242202437d1c869c81fa42fd7bd79806bac257abc7bfe0a91f65b048e201e8ea1de778e5df0e4d9d7de6babdf8e441a8008acfe2a42dd0ae9b9d7726c9cc2c954d4a53d7d5dc543630026b2bd809aafd7805f852336bb8f25b5e9729223f27b4c380f538082ded97c54ea735d46245ddd11804b86acd2e1972801654ddfe795e20ca59d48d69d64ef32ae6bad6af8b9770fee80184b10002d45521f502b344e06793bf3cf77934d58588a024d9dbd8041b015f7802717ad2e00b2bcfea59a0179e69072f05fc65884d1c0b42cd29ce34d17b2062d80888bbb1ca4a539482087f2ed338227526bf9f518bf4dd67130469a264b95e69e807a698f5008c02bc7dcbf2ea0e974914b56924d5440fef85179b2a08f1cb73add80ff5db327160b6d5edbbf01c548f5841f8325ca55954c99a54e22aa9ea39cba528042194d5944a25c37885e422c647092b97a3ae70e1ad247f0292164da7862303f80bd7fa002fc6cc9960365833889016a48d7ef9e32df363b59d76cfb39b9f53d1a800f6e6ea64b62457b0e9ddacbe978bd79cd6fa1dbb422eec5a223f49ae2bb85bc").unwrap(),
        hex::decode("80ffff80a9ed75e03215b907b35e5fb98346cac4b9ca9a19a182d6ccdba36bada8fef05b80a972ea9a27023f0935df491962b994e6f6c4ee3a7805cf27aab8611d2f06ed6b8060b390cbb48376bb848cad68fb6fcff10f687b7045088dfc5d0699a0db15b4ea804313dfe81d8f811456223a5fc72cd1be994e51218e72dfca4f7cd9331ec247eb802515e2ae8b12222edb90600fb147c7459a7c265c753619c9014cc5aecd04ed3a807ce6a9911f470062b2772fee859f7b9e4739c0867a3746b5d0b5dcc324f6a1ad806538ed1ff3452f5a690624e1541b0466ca3f452ad60b70e21c27b1dec3d1be1c80136ac180b31bc0e4f3b798262f5eb618d82aca5c85a0d0e2620324e23d288bd8809594c690f7fbc4db8db681f5da18df3226897b17c20fd8002897d2bcc840e135800bac1a3ccd071e8c6b04c6b96e16cc002286da0b0cadbc79d5cacb2ce7b2b44a803981f53505969f281ce224234791c8db7e7d0377ceb1d24ce29fdba27a2aa55c809824f0eed4dd3fddc2aa20c9c9b0199278beaee525fa2ddc6c8cf7e23f9d59c480a8c945b098ebc6fdb5b073147642d4deef487b016c8b27054a00280c638384a6802f3b98d55b2ad4fcea5c405dc5d343afecb007c057b8a60ae4100545042aa729805e56350f28813b295c870c0003b0130cbe66f5d11881387114003564ba8c78578065e8ef797423da01cee91b0b790e6a9c81b5504ea2786be972d5c961447dd4d2").unwrap(),
        hex::decode("800180807c2c31b3747d7767c40b41f0e75d60d8045f32dfa2ce397b25b641205c77e57580c1df2a29725bf95921b5f4cee86c913b58118f6dfc59c9822b16712dcbf84122").unwrap(),
        hex::decode("80040280c56aedca768a4e919df533325b6fc6a368455fb599d7ad9271a2249e49e65c82803560432335d6e87f5c6c0da59797aee0423cecc207c3a37304511c3a8d031455").unwrap(),
        hex::decode("9d0e4907f716ac89b6347d15ececedcaffff585f0b6a45321efae92aea15e0740ec7afe710a40d0000585f038e71612491192d68deab7e6f563fe110e8030000806ec80d7afc89a3ccebe10fcdf04ca16518ff88d6b29462ec861b00d31239db3580c04da45c5e5cddac29c9e21d75662d40adfabd88bf3010dda8cf5a13f22e32d980f981a50341cdae9bd105be756196698e91d8be2fb0dc6397a0aaf82cddf0fc5d80f42d10621daacf0ea0e3268a8bc2fe3121d9e250a88b3024ed7c56cba35b7bb980bfa6614065f0fadaa9d7f71840a3d29f63a189f2f3aef8241fe5b06b3b9f0cd180c7da650afb99c538063c05d6c02504e4d6555ad682412479d189e85e994fd6cd80a3aaade6ba17d9833da300f436bc3985fb2fe6c61670fea253d5d58d5edd5bfd80f11d7ff327f21936e99db2f172c95d464b81c557d6662f46a0991d01f5683be7803eb67870b1c418c6c7a26103aa386a17ecda2e481f97c8ff37f390b98e4bd6bc585f049a2738eeb30896aacb8b3fb46471bd100400000080009dacd73537f8a2f5933ded58c0102b9f9af9d0379079b8f6b1f63027e77ef9585f0642c00af119adf30dc11d32e9f0886d10204e0000809bee97f441e7fe63916b3923c1279e7f401558ab1df7ffb4b51e864e820b48e1585f099b25852d3d69419882da651375cdb310861c0000").unwrap(),
        hex::decode("9d0adb579f1dbf4f3886c5cfa3bb8cc4ffff80a7b28f89318d1fd3eb7a23c78ba24134b89cdd332efec5615cf2139e87777b0c8020766326bb8cfa97296157a7cc3d502dcfc606fc21c823ff98f5dc6062559d3380f46475b1c59ce606753e77b74d1448ffab5a35b853a6d6f49795b027019325f380c2e52883c83b9b71a2ad00a0500eab314fc3c22a3af221075e5f57ebbe53b02180bab606d8bc597e34133306bf3cf8b6da31d745053bdc02a9352f3910562f368f801f8d5f3e91bc4195b0f2ce3c8f762d92288f099a3cff19544383cf5ee2b138f680f88564d53bfc2e0a524bf7d288f6736b5e6d5eb8ccb9dd253fed49954775070e80f67ae62ea2d122fd11ad9af0db42d7a087062679856b83f7f1fec3ff07d9a95d8082260fc9c9d39d941f8647142b6fa9c59c7cbbd1a62713dede381d011a7f1535807eaca35e2da21f58d204ee089e4e0a758a03fa8352f421c80dac4b932e78e53080a922540b71ca78d22691f4d5b35de64e2cc86c1016fbf964a883e5be2aa2718c802dc3aebb89be3c7f47f75b0e1af2ca818953f8001715b9ad50ef62d64831d176807d725bdd58f12b77f22953139d49cd785292a6576aaff12d7204fc42dfbd363780c0fbc5bfc21820c07c09ab52a7f7b7d113062c861dc772bb8fd480a48eac90fd80a753133ae3c9ea1ec743a363e1908f10511a7457cecddd836c3ebcc9d272b09a800174ba5e2235e5f210c26e1ca3ddd7b98b5a375626be5c54307597bbcfe70514").unwrap(),
        hex::decode("804c148041384dfbc07ad3401c8d4464793fa06360e54d7c21358998cc3c4cf848899145802bbf85831deea0e456e8061b4112ad00ffea89e53b6f8d0a4ea1b7ff4b83ff4b80fb786dead294471fc3c0f868831403b0ca753840d73cf92d4e52c6b9090b1ee080399b822502d71b5bdc3fe2de3f1773269f3c259ce65d89d008392bf394395302807927cf5a1a4859edd77a2be0cd98010622e7ccff6b5e8281667f60016d8f68ca").unwrap(),
    ].to_vec()
}

pub fn get_mock_staking_ledger(derivative_index: u16) -> StakingLedger<AccountId, BalanceOf<Test>> {
	let mut staking_ledger = <StakingLedger<AccountId, BalanceOf<Test>>>::new(
		LiquidStaking::derivative_sovereign_account_id(derivative_index),
		MOCK_LEDGER_AMOUNT,
	);
	staking_ledger.claimed_rewards = vec![
		3377, 3378, 3379, 3380, 3381, 3382, 3383, 3384, 3385, 3386, 3387, 3388, 3389, 3390, 3391,
		3392, 3393, 3394, 3395, 3396, 3397, 3398, 3399, 3400, 3401, 3402, 3403, 3404, 3405, 3406,
		3407, 3408, 3409, 3410, 3411, 3412, 3413, 3414, 3415, 3416, 3417, 3418, 3419, 3420, 3421,
		3422, 3423, 3424, 3425, 3426, 3427, 3428, 3429, 3430, 3431, 3432, 3433, 3434, 3435, 3436,
		3437, 3438, 3439, 3440, 3441, 3442, 3443, 3444, 3445, 3446, 3447, 3448, 3449, 3450, 3451,
		3452, 3453, 3454, 3455, 3456, 3457, 3458, 3459, 3460,
	];
	staking_ledger
}

parameter_types! {
	pub const StakingPalletId: PalletId = PalletId(*b"par/lqsk");
	pub const EraLength: BlockNumber = 10;
	pub SelfParaId: ParaId = para_a_id();
	pub const MinStake: Balance = 0;
	pub const MinUnstake: Balance = 0;
	pub const StakingCurrency: CurrencyId = KSM;
	pub const LiquidCurrency: CurrencyId = SKSM;
	pub const XcmFees: Balance = 0;
	pub LoansInstantUnstakeFee: Rate = Rate::saturating_from_rational(8u32, 1000u32);
	pub MatchingPoolFastUnstakeFee: Rate = Rate::saturating_from_rational(1u32, 1000u32);
	pub const BondingDuration: EraIndex = 3;
	pub const MinNominatorBond: Balance = 0;
	pub const NumSlashingSpans: u32 = 0;
	pub static DerivativeIndexList: Vec<u16> = vec![0];
	pub static RelayChainValidationDataProvider: BlockNumber = 0;
	pub const ElectionSolutionStoredOffset: BlockNumber = 10;
	pub const DefaultProtocolFeeReceiver: AccountId32 = AccountId32::new([100u8; 32]);
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type UpdateOrigin = UpdateOrigin;
	type PalletId = StakingPalletId;
	type SelfParaId = SelfParaId;
	type WeightInfo = ();
	type StakingCurrency = StakingCurrency;
	type LiquidCurrency = LiquidCurrency;
	type DerivativeIndexList = DerivativeIndexList;
	type XcmFees = XcmFees;
	type MatchingPoolFastUnstakeFee = MatchingPoolFastUnstakeFee;
	type Assets = Assets;
	type RelayOrigin = RelayOrigin;
	type EraLength = EraLength;
	type MinStake = MinStake;
	type MinUnstake = MinUnstake;
	type XCM = XcmHelper;
	type BondingDuration = BondingDuration;
	type MinNominatorBond = MinNominatorBond;
	type RelayChainValidationDataProvider = RelayChainValidationDataProvider;
	type Members = BobOrigin;
	type NumSlashingSpans = NumSlashingSpans;
	type DistributionStrategy = AverageDistribution;
	type ElectionSolutionStoredOffset = ElectionSolutionStoredOffset;
	type ProtocolFeeReceiver = DefaultProtocolFeeReceiver;
	type Decimal = Decimal;
	type NativeCurrency = NativeCurrencyId;
}

pub struct Decimal;
#[allow(non_upper_case_globals)]
impl DecimalProvider<CurrencyId> for Decimal {
	fn get_decimal(asset_id: &CurrencyId) -> Option<u8> {
		match *asset_id {
			DOT => Some(10),
			SDOT => Some(10),
			KSM => Some(12),
			SKSM => Some(12),
			PARA => Some(12),
			PICA => Some(12),
			USDT => Some(6),
			CLV => Some(18),
			_ => None,
		}
	}
}

parameter_types! {
	pub const AssetDeposit: Balance = KSM_DECIMAL;
	pub const ApprovalDeposit: Balance = 0;
	pub const AssetAccountDeposit: Balance = 0;
	pub const AssetsStringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 0;
	pub const MetadataDepositPerByte: Balance = 0;
}

parameter_types! {
	pub const NativeAssetId: CurrencyId = PICA;
}

pub struct Valid;
impl composable_support::validation::Validate<CurrencyId, primitives::currency::ValidateCurrencyId>
	for Valid
{
	fn validate(input: CurrencyId) -> Result<CurrencyId, &'static str> {
		Ok(input)
	}
}

impl pallet_assets::Config for Test {
	type RuntimeHoldReason = ();
	type NativeAssetId = NativeAssetId;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type MultiCurrency = Tokens;
	type NativeCurrency = Balances;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type CurrencyValidator = Valid;
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

parameter_type_with_key! {
	pub ExistentialDeposits: |_a: AssetIdX| -> Balance {
		Zero::zero()
	};
}

pub type AssetIdX = CurrencyId;
pub type AmountX = i128;
type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = AmountX;
	type CurrencyId = AssetIdX;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = Everything;
	type CurrencyHooks = CurrencyHooks;
}

// impl pallet_currency_adapter::Config for Test {
//     type Assets = Assets;
//     type Balances = Balances;
//     type GetNativeCurrencyId = NativeCurrencyId;
//     type LockOrigin = EnsureRoot<AccountId>;
// }

use primitives::currency::ForeignAssetId;

parameter_types! {
	pub const ComposableNetworkId: u32 = 1;
}

use sp_runtime::traits::ConvertInto;
pub type AssetId = u128;

impl pallet_assets_registry::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type LocalAssetId = u128;
	type ForeignAssetId = ForeignAssetId;
	type UpdateAssetRegistryOrigin = EnsureRoot<AccountId>;
	type ParachainOrGovernanceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type Balance = Balance;
	type Convert = ConvertInto;
	type NetworkId = ComposableNetworkId;
}

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Utility: pallet_utility::{Pallet, Call, Event},
		Assets: pallet_assets::{Pallet, Call, Storage},
		Tokens: orml_tokens::{Pallet, Storage, /*Config<T>,*/ Event<T>},
		LiquidStaking: crate::{Pallet, Storage, Call, Event<T>},
		ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Config, Storage, Inherent, Event<T>},
		ParachainInfo: parachain_info::{Pallet, Storage, Config},
		XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>},
		DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>},
		CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin},
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin},
		XcmHelper: pallet_xcm_helper::{Pallet, Storage, Call, Event<T>},
		XTokens: orml_xtokens::{Pallet, Storage, Call, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		UnknownTokens: orml_unknown_tokens::{Pallet, Event},
		AssetsRegistry: pallet_assets_registry,
	}
);

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([2u8; 32]);
pub const RESERVE_FACTOR: Ratio = Ratio::from_perthousand(5);

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	GenesisBuild::<Test>::assimilate_storage(
		&crate::GenesisConfig { exchange_rate: Rate::one(), reserve_factor: RESERVE_FACTOR },
		&mut t,
	)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		let name = Some(BiBoundedVec::from_vec(b"Kusama".to_vec()).unwrap());
		let symbol = Some(BiBoundedVec::from_vec(b"KSM".to_vec()).unwrap());
		let asset_info = AssetInfo {
			name,
			symbol,
			decimals: Some(12),
			existential_deposit: 375000000,
			ratio: Some(rational!(42 / 123)),
		};
		AssetsRegistry::register_asset(RuntimeOrigin::root(), [0; 4], 99, None, asset_info);

		let name = Some(BiBoundedVec::from_vec(b"Liquid Staked Kusama".to_vec()).unwrap());
		let symbol = Some(BiBoundedVec::from_vec(b"LSKSM".to_vec()).unwrap());
		let asset_info = AssetInfo {
			name,
			symbol,
			decimals: Some(12),
			existential_deposit: 375000000,
			ratio: Some(rational!(42 / 123)),
		};
		AssetsRegistry::register_asset(RuntimeOrigin::root(), [0; 4], 20, None, asset_info);

		Assets::mint_into(RuntimeOrigin::signed(ALICE), KSM.into(), Id(ALICE), ksm(100f64))
			.unwrap();
		Assets::mint_into(RuntimeOrigin::signed(ALICE), SKSM.into(), Id(ALICE), ksm(100f64))
			.unwrap();
		Assets::mint_into(RuntimeOrigin::signed(ALICE), KSM.into(), Id(BOB), ksm(20000f64))
			.unwrap();
		LiquidStaking::update_staking_ledger_cap(RuntimeOrigin::signed(BOB), ksm(10000f64))
			.unwrap();

		Assets::mint_into(
			RuntimeOrigin::signed(ALICE),
			KSM.into(),
			Id(XcmHelper::account_id()),
			ksm(100f64),
		)
		.unwrap();

		System::set_block_number(1);
		Timestamp::set_timestamp(6000);
	});

	ext
}

//initial parchain and relaychain for testing
decl_test_parachain! {
	pub struct ParaA {
		Runtime = Test,
		XcmpMessageHandler = XcmpQueue,
		DmpMessageHandler = DmpQueue,
		new_ext = para_ext(2085),
	}
}

use pallet_message_queue;
use xcm_simulator::TestExt;

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = kusama_runtime::Runtime,
		RuntimeCall = kusama_runtime::RuntimeCall,
		RuntimeEvent = kusama_runtime::RuntimeEvent,
		XcmConfig = kusama_runtime::xcm_config::XcmConfig,
		MessageQueue = kusama_runtime::MessageQueue,
		System = kusama_runtime::System,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct TestNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaA),
		],
	}
}

pub type KusamaRuntime = kusama_runtime::Runtime;
pub type RelayBalances = pallet_balances::Pallet<KusamaRuntime>;
pub type RelayStaking = pallet_staking::Pallet<KusamaRuntime>;
pub type RelayStakingEvent = pallet_staking::Event<KusamaRuntime>;
pub type RelaySystem = frame_system::Pallet<KusamaRuntime>;
pub type RelayEvent = kusama_runtime::RuntimeEvent;
pub type ParaSystem = frame_system::Pallet<Test>;

pub fn para_a_id() -> ParaId {
	ParaId::from(2085)
}

pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let parachain_info_config = parachain_info::GenesisConfig { parachain_id: para_id.into() };
	<parachain_info::GenesisConfig as GenesisBuild<Test, _>>::assimilate_storage(
		&parachain_info_config,
		&mut t,
	)
	.unwrap();

	GenesisBuild::<Test>::assimilate_storage(
		&crate::GenesisConfig {
			exchange_rate: Rate::one(),
			reserve_factor: Ratio::from_perthousand(5),
		},
		&mut t,
	)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		let name = Some(BiBoundedVec::from_vec(b"Kusama".to_vec()).unwrap());
		let symbol = Some(BiBoundedVec::from_vec(b"KSM".to_vec()).unwrap());
		let asset_info = AssetInfo {
			name,
			symbol,
			decimals: Some(12),
			existential_deposit: 375000000,
			ratio: Some(rational!(42 / 123)),
		};
		AssetsRegistry::register_asset(RuntimeOrigin::root(), [0; 4], 99, None, asset_info);

		let name = Some(BiBoundedVec::from_vec(b"Liquid Staked Kusama".to_vec()).unwrap());
		let symbol = Some(BiBoundedVec::from_vec(b"LSKSM".to_vec()).unwrap());
		let asset_info = AssetInfo {
			name,
			symbol,
			decimals: Some(12),
			existential_deposit: 375000000,
			ratio: Some(rational!(42 / 123)),
		};
		AssetsRegistry::register_asset(RuntimeOrigin::root(), [0; 4], 20, None, asset_info);

		Assets::mint_into(RuntimeOrigin::signed(ALICE), KSM.into(), Id(ALICE), ksm(10000f64))
			.unwrap();
		Assets::mint_into(RuntimeOrigin::signed(ALICE), KSM.into(), Id(BOB), ksm(20000f64))
			.unwrap();
		Assets::mint_into(
			RuntimeOrigin::signed(ALICE),
			KSM.into(),
			Id(XcmHelper::account_id()),
			ksm(30f64),
		)
		.unwrap();

		LiquidStaking::update_staking_ledger_cap(RuntimeOrigin::signed(BOB), ksm(10000f64))
			.unwrap();

		System::set_block_number(1);
		Timestamp::set_timestamp(6000);
	});

	ext
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use kusama_runtime::{Runtime, System};
	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![
			(ALICE, ksm(100f64)),
			(para_a_id().into_account_truncating(), ksm(1_000_000f64)),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
		config: HostConfiguration { max_code_size: 1024u32, ..Default::default() },
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn ksm(n: f64) -> Balance {
	((n * 1000000f64) as u128) * KSM_DECIMAL / 1000000u128
}
