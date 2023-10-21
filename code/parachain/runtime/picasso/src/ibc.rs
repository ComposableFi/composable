pub(crate) use ::ibc::{
	applications::transfer::{MODULE_ID_STR, PORT_ID_STR},
	core::{
		ics24_host::identifier::PortId,
		ics26_routing::context::{Module, ModuleId},
	},
};
use common::{
	fees::{IbcIcs20FeePalletId, IbcIcs20ServiceCharge},
	ibc::{ForeignIbcIcs20Assets, MinimumConnectionDelaySeconds},
};
use frame_system::EnsureSigned;
use hex_literal::hex;
pub(crate) use pallet_ibc::{
	light_client_common::RelayChain, routing::ModuleRouter, DenomToAssetId, IbcAssetIds, IbcAssets,
};
use sp_core::ConstU64;
use sp_runtime::{AccountId32, DispatchError, Either};

use super::*;

#[allow(clippy::derivable_impls)]
impl Default for Runtime {
	fn default() -> Self {
		Self {}
	}
}

pub struct IbcDenomToAssetIdConversion;

impl DenomToAssetId<Runtime> for IbcDenomToAssetIdConversion {
	type Error = DispatchError;

	fn from_denom_to_asset_id(denom: &core::primitive::str) -> Result<CurrencyId, Self::Error> {
		ForeignIbcIcs20Assets::<AssetsRegistry>::from_denom_to_asset_id(denom)
	}

	fn from_asset_id_to_denom(id: CurrencyId) -> Option<String> {
		ForeignIbcIcs20Assets::<AssetsRegistry>::from_asset_id_to_denom(id)
	}

	fn ibc_assets(start_key: Option<Either<CurrencyId, u32>>, limit: u64) -> IbcAssets<CurrencyId> {
		let mut iterator = match start_key {
			None => IbcAssetIds::<Runtime>::iter().skip(0),
			Some(Either::Left(asset_id)) => {
				let raw_key = asset_id.encode();
				IbcAssetIds::<Runtime>::iter_from(raw_key).skip(0)
			},
			Some(Either::Right(offset)) => IbcAssetIds::<Runtime>::iter().skip(offset as usize),
		};

		let denoms = iterator.by_ref().take(limit as usize).map(|(_, denom)| denom).collect();
		let maybe_currency_id = iterator.next().map(|(id, ..)| id);
		IbcAssets {
			denoms,
			total_count: IbcAssetIds::<Runtime>::count() as u64,
			next_id: maybe_currency_id,
		}
	}
}

parameter_types! {
	pub const RelayChainId: RelayChain = RelayChain::Rococo;
	pub const SpamProtectionDeposit: Balance = 1_000_000_000_000_000;
}

parameter_types! {
	pub const GRANDPA: pallet_ibc::LightClientProtocol = pallet_ibc::LightClientProtocol::Grandpa;
	pub const IbcTriePrefix : &'static [u8] = b"ibc/";
	// converted from 5xMXcPsD9B9xDMvLyNBLmn9uhK7sTXTfubGVTZmXwVJmTVWa using https://www.shawntabrizi.com/substrate-js-utilities/
	pub FeeAccount: <Runtime as pallet_ibc::Config>::AccountIdConversion = ibc_primitives::IbcAccount(AccountId32::from(hex!("a72ef3ce1ecd46163bc5e23fd3e6a4623d9717c957fb59001a5d4cb949150f28")));
	pub const IbcPalletId: PalletId = PalletId(*b"cntr_ibc");
}

use pallet_ibc::{ics20::IbcMemoHandler, ics20_fee::NonFlatFeeConverter};

type CosmwasmRouter = cosmwasm::ibc::Router<Runtime>;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Router {
	ics20: pallet_ibc::ics20::memo::Memo<
		Runtime,
		pallet_ibc::ics20_fee::Ics20ServiceCharge<Runtime, pallet_ibc::ics20::IbcModule<Runtime>>,
	>,
	pallet_cosmwasm: CosmwasmRouter,
}

impl ModuleRouter for Router {
	fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module> {
		match module_id.as_ref() {
			MODULE_ID_STR => Some(&mut self.ics20),
			_ => self.pallet_cosmwasm.get_route_mut(module_id),
		}
	}

	fn has_route(module_id: &ModuleId) -> bool {
		matches!(module_id.as_ref(), MODULE_ID_STR) || CosmwasmRouter::has_route(module_id)
	}

	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId> {
		match port_id.as_str() {
			PORT_ID_STR => ModuleId::from_str(MODULE_ID_STR).ok(),
			_ => CosmwasmRouter::lookup_module_by_port(port_id),
		}
	}
}

impl pallet_ibc::ics20_fee::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ServiceChargeIn = IbcIcs20ServiceCharge;
	type PalletId = IbcIcs20FeePalletId;
}

impl pallet_ibc::Config for Runtime {
	type TimeProvider = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type NativeCurrency = Balances;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type NativeAssetId = NativeAssetId;
	type IbcDenomToAssetIdConversion = IbcDenomToAssetIdConversion;
	type PalletPrefix = IbcTriePrefix;
	type LightClientProtocol = GRANDPA;
	type AccountIdConversion = ibc_primitives::IbcAccount<AccountId>;
	type Fungibles = Assets;
	type ExpectedBlockTime = ConstU64<SLOT_DURATION>;
	type Router = Router;
	type MinimumConnectionDelay = MinimumConnectionDelaySeconds;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type WeightInfo = weights::pallet_ibc::WeightInfo<Self>;
	type SpamProtectionDeposit = SpamProtectionDeposit;
	type IbcAccountId = Self::AccountId;
	type HandleMemo = IbcMemoHandler<xcvm_memo_processing::XcvmMemoHandler<(), Runtime>, Runtime>;
	type MemoMessage = alloc::string::String;
	type SubstrateMultihopXcmHandler = pallet_multihop_xcm_ibc::Pallet<Runtime>;
	type IsReceiveEnabled = ConstBool<true>;
	type IsSendEnabled = ConstBool<true>;

	type AdminOrigin = EnsureRootOrOneThirdNativeTechnical;
	type FreezeOrigin = EnsureRootOrOneThirdNativeTechnical;

	type TransferOrigin = EnsureSigned<Self::AccountId>;

	#[cfg(feature = "testnet")]
	type RelayerOrigin = system::EnsureSigned<Self::IbcAccountId>;
	#[cfg(not(feature = "testnet"))]
	type RelayerOrigin = system::EnsureSignedBy<TechnicalCommitteeMembership, Self::IbcAccountId>;

	type FeeAccount = FeeAccount;
	type CleanUpPacketsPeriod = ConstU32<100>;

	type ServiceChargeOut = IbcIcs20ServiceCharge;
	type FlatFeeAssetId = AssetIdUSDT;
	type FlatFeeAmount = FlatFeeUSDTAmount;
	// type FlatFeeConverter = Pablo
	type FlatFeeConverter = NonFlatFeeConverter<Runtime>;
}

pub mod xcvm_memo_processing {
	use super::*;
	use ::ibc::{
		applications::transfer::error::Error as ICS20Error, core::ics04_channel::packet::Packet,
	};
	use pallet_ibc::ics20::HandleMemo;

	pub struct XcvmMemoHandler<H, T> {
		pub inner: H,
		pub _phantom: core::marker::PhantomData<T>,
	}
	impl<T, H: HandleMemo<T>> HandleMemo<T> for XcvmMemoHandler<H, T>
	where
		T: pallet_ibc::Config + Send + Sync,
		u32: From<<T as frame_system::Config>::BlockNumber>,
		AccountId32: From<<T as frame_system::Config>::AccountId>,
		u128: From<T::AssetId>,
	{
		fn execute_memo(&self, packet: &Packet) -> Result<(), ICS20Error> {
			self.inner.execute_memo(packet)?;
			// TODO: handle XCVM
			Ok(())
		}
	}

	impl<H: Default, T> Default for XcvmMemoHandler<H, T> {
		fn default() -> Self {
			Self { inner: H::default(), _phantom: core::marker::PhantomData }
		}
	}
}
