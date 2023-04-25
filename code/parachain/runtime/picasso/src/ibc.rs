pub(crate) use ::ibc::{
	applications::transfer::{MODULE_ID_STR, PORT_ID_STR},
	core::{
		ics24_host::identifier::PortId,
		ics26_routing::context::{Module, ModuleId},
	},
};
use common::{
	fees::{IbcIcs20FeePalletId, IbcIcs20ServiceCharge},
	ibc::MinimumConnectionDelaySeconds,
};
use frame_support::traits::EitherOf;
use hex_literal::hex;
pub(crate) use pallet_ibc::{
	light_client_common::RelayChain, routing::ModuleRouter, DenomToAssetId, IbcAssetIds, IbcAssets,
};
use sp_core::ConstU64;
use sp_runtime::{AccountId32, DispatchError, Either};
use system::EnsureSignedBy;

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

	fn from_denom_to_asset_id(denom: &String) -> Result<CurrencyId, Self::Error> {
		log::debug!("converting denom {} to currency id", denom);
		let denom_bytes = denom.as_bytes().to_vec();
		if let Some(id) = IbcDenoms::<Runtime>::get(&denom_bytes) {
			log::debug!("converted {:} to currency id", &id);
			return Ok(id)
		}

		// +1 nonce
		let asset_id =
			<currency_factory::Pallet<Runtime> as CurrencyFactoryT>::create(RangeId::IBC_ASSETS)?;

		let asset_id = <assets_transactor_router::Pallet<Runtime> as composable_traits::assets::CreateAsset>
		::create_local_asset([0;8], asset_id.0 as u64,
			AssetInfo {
				name: None,
				symbol: None,
				decimals: None,
				// we do not have sufficient assets, nor setting zero is good
				// so set at least some
				existential_deposit: 10_000,
				ratio: Some(rational!(1/1)),
		})?;
		log::info!("registered {:} asset id for {:} denom", &asset_id, &denom);
		IbcDenoms::<Runtime>::insert(denom_bytes.clone(), asset_id);
		IbcAssetIds::<Runtime>::insert(asset_id, denom_bytes);

		Ok(asset_id)
	}

	fn from_asset_id_to_denom(id: CurrencyId) -> Option<String> {
		IbcAssetIds::<Runtime>::get(id).and_then(|denom| String::from_utf8(denom).ok())
	}

	fn ibc_assets(start_key: Option<Either<CurrencyId, u32>>, limit: u64) -> IbcAssets<CurrencyId> {
		let mut iterator = match start_key {
			None => IbcAssetIds::<Runtime>::iter().skip(0),
			Some(Left(asset_id)) => {
				let raw_key = asset_id.encode();
				IbcAssetIds::<Runtime>::iter_from(raw_key).skip(0)
			},
			Some(Right(offset)) => IbcAssetIds::<Runtime>::iter().skip(offset as usize),
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
	// converted from 5xMXcPsD9B9xDMvLyNBLmn9uhK7sTXTfubGVTZmXwVJmTVWa using https://www.shawntabrizi.com/substrate-js-utilities/
	pub FeeAccount: <Runtime as pallet_ibc::Config>::AccountIdConversion = ibc_primitives::IbcAccount(AccountId32::from(hex!("a72ef3ce1ecd46163bc5e23fd3e6a4623d9717c957fb59001a5d4cb949150f28")));
}

use pallet_ibc::ics20::Ics20RateLimiter;

pub struct ConstantAny;

impl Ics20RateLimiter for ConstantAny {
	fn allow(
		msg: &pallet_ibc::ics20::Ics20TransferMsg,
		_flow_type: pallet_ibc::ics20::FlowType,
	) -> Result<(), ()> {
		// one DOT/PICA, so for USDT not safe, but we do not yet do it
		if msg.token.amount.as_u256() <= ::ibc::bigint::U256::from(10_u64.pow(12)) {
			return Ok(())
		}
		Err(())
	}
}

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
	type ServiceCharge = IbcIcs20ServiceCharge;
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
	type Fungibles = AssetsTransactorRouter;
	type ExpectedBlockTime = ConstU64<SLOT_DURATION>;
	type Router = Router;
	type MinimumConnectionDelay = MinimumConnectionDelaySeconds;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type WeightInfo = weights::ibc::WeightInfo<Self>;
	type SpamProtectionDeposit = SpamProtectionDeposit;
	type IbcAccountId = Self::AccountId;
	type HandleMemo = ();
	type MemoMessage = MemoMessage;
	type Ics20RateLimiter = ConstantAny;
	type IsReceiveEnabled = ConstBool<true>;
	type IsSendEnabled = ConstBool<true>;

	type AdminOrigin = EnsureRootOrOneThirdNativeTechnical;
	type FreezeOrigin = EnsureRootOrOneThirdNativeTechnical;

	#[cfg(feature = "testnet")]
	type TransferOrigin = system::EnsureSigned<Self::IbcAccountId>;
	#[cfg(feature = "testnet")]
	type RelayerOrigin = system::EnsureSigned<Self::IbcAccountId>;

	#[cfg(not(feature = "testnet"))]
	type TransferOrigin = EitherOf<
		EnsureSignedBy<ReleaseMembership, Self::IbcAccountId>,
		EnsureSignedBy<TechnicalCommitteeMembership, Self::IbcAccountId>,
	>;
	#[cfg(not(feature = "testnet"))]
	type RelayerOrigin = EnsureSignedBy<TechnicalCommitteeMembership, Self::IbcAccountId>;

	type FeeAccount = FeeAccount;
}
