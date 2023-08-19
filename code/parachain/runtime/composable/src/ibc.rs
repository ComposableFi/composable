use crate::prelude::*;
use ::ibc::core::{
	ics24_host::identifier::PortId,
	ics26_routing::context::{Module, ModuleId},
};
use common::{
	fees::{IbcIcs20FeePalletId, IbcIcs20ServiceCharge},
	governance::native::EnsureRootOrOneThirdNativeTechnical,
};
use pallet_ibc::{
	ics20::{IbcMemoHandler, MODULE_ID_STR, PORT_ID_STR},
	light_client_common::RelayChain,
	routing::ModuleRouter,
	DenomToAssetId, IbcAssetIds, IbcAssets,
};
use sp_core::ConstU64;
use sp_runtime::{AccountId32, DispatchError, Either};

use hex_literal::hex;
use pallet_ibc::ics20_fee::NonFlatFeeConverter;

use super::*;

#[allow(clippy::derivable_impls)]
impl Default for Runtime {
	fn default() -> Self {
		Self {}
	}
}

use common::ibc::ForeignIbcIcs20Assets;
pub struct IbcDenomToAssetIdConversion;

impl DenomToAssetId<Runtime> for IbcDenomToAssetIdConversion {
	type Error = DispatchError;

	fn from_denom_to_asset_id(denom: &str) -> Result<CurrencyId, Self::Error> {
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

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Dummy;

impl ModuleRouter for Dummy {
	fn get_route_mut(&mut self, _module_id: &ModuleId) -> Option<&mut dyn Module> {
		None
	}

	fn has_route(_module_id: &ModuleId) -> bool {
		false
	}

	fn lookup_module_by_port(_port_id: &PortId) -> Option<ModuleId> {
		None
	}
}

#[derive(
	Debug, codec::Encode, Clone, codec::Decode, PartialEq, Eq, scale_info::TypeInfo, Default,
)]
pub struct MemoMessage;
extern crate alloc;
impl alloc::string::ToString for MemoMessage {
	fn to_string(&self) -> String {
		Default::default()
	}
}

impl core::str::FromStr for MemoMessage {
	type Err = ();

	fn from_str(_s: &str) -> Result<Self, Self::Err> {
		Ok(Default::default())
	}
}

parameter_types! {
	pub const GRANDPA: pallet_ibc::LightClientProtocol = pallet_ibc::LightClientProtocol::Grandpa;
	pub const IbcTriePrefix : &'static [u8] = b"ibc/";
	// converted from 63yg1BAWeUQG7WgpZNqbPrreo9HCoWKUcFqswfNz3TjpKHiL using https://www.shawntabrizi.com/substrate-js-utilities/
	pub FeeAccount: <Runtime as pallet_ibc::Config>::AccountIdConversion = ibc_primitives::IbcAccount(AccountId32::from(hex!("9fed34f0114500f263d074e91ac4b1ef6b11b2e09fa4684dfe4bce07f94ab603")));

}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Router {
	ics20: pallet_ibc::ics20::memo::Memo<
		Runtime,
		pallet_ibc::ics20_fee::Ics20ServiceCharge<Runtime, pallet_ibc::ics20::IbcModule<Runtime>>,
	>,
}

impl ModuleRouter for Router {
	fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module> {
		match module_id.as_ref() {
			MODULE_ID_STR => Some(&mut self.ics20),
			&_ => None,
		}
	}

	fn has_route(module_id: &ModuleId) -> bool {
		matches!(module_id.as_ref(), MODULE_ID_STR)
	}

	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId> {
		match port_id.as_str() {
			PORT_ID_STR => ModuleId::from_str(MODULE_ID_STR).ok(),
			_ => None,
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
	type MinimumConnectionDelay = ConstU64<1>;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type WeightInfo = weights::pallet_ibc::WeightInfo<Self>;
	type AdminOrigin = EnsureRootOrOneThirdNativeTechnical;
	type FreezeOrigin = EnsureRootOrOneThirdNativeTechnical;
	type SpamProtectionDeposit = SpamProtectionDeposit;
	type IbcAccountId = Self::AccountId;
	type TransferOrigin = system::EnsureSigned<Self::IbcAccountId>;
	#[cfg(feature = "testnet")]
	type RelayerOrigin = system::EnsureSigned<Self::IbcAccountId>;
	#[cfg(not(feature = "testnet"))]
	type RelayerOrigin = system::EnsureSignedBy<TechnicalCommitteeMembership, Self::IbcAccountId>;
	type HandleMemo = IbcMemoHandler<(), Runtime>;
	type MemoMessage = alloc::string::String;
	type IsReceiveEnabled = ConstBool<true>;
	type IsSendEnabled = ConstBool<true>;
	type SubstrateMultihopXcmHandler = pallet_multihop_xcm_ibc::Pallet<Runtime>;
	type FeeAccount = FeeAccount;
	type CleanUpPacketsPeriod = ConstU32<100>;
	type ServiceChargeOut = IbcIcs20ServiceCharge;
	type FlatFeeAssetId = AssetIdUSDT;
	type FlatFeeAmount = FlatFeeUSDTAmount;
	type FlatFeeConverter = NonFlatFeeConverter<Runtime>;
}
