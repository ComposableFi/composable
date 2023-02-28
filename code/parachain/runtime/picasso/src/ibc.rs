use ::ibc::core::{
	ics24_host::identifier::PortId,
	ics26_routing::context::{Module, ModuleId},
};
use pallet_ibc::{
	light_client_common::RelayChain, routing::ModuleRouter, DenomToAssetId, IbcAssetIds, IbcAssets,
	IbcDenoms,
};
use sp_core::ConstU64;
use sp_runtime::{DispatchError, Either};
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
		let denom_bytes = denom.as_bytes().to_vec();
		if let Some(id) = IbcDenoms::<Runtime>::get(&denom_bytes) {
			return Ok(id)
		}

		// will be decided in next prs for composable <-> picasso ibc and/or pallets-assets updates
		// merge
		if denom == &alloc::format!("transfer/channel-0/{:}", CurrencyId::ibcxcDOT.0) {
			IbcDenoms::<Runtime>::insert(denom_bytes, CurrencyId::ibcxcDOT);
			return Ok(CurrencyId::ibcxcDOT)
		}

		Err(DispatchError::Other("IbcDenomToAssetIdConversion: denom not found"))
	}

	fn from_asset_id_to_denom(id: CurrencyId) -> Option<String> {
		IbcAssetIds::<Runtime>::get(id).and_then(|denom| String::from_utf8(denom).ok())
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
}

use pallet_ibc::ics20::Ics20RateLimiter;

pub struct ConstantAny;

impl Ics20RateLimiter for ConstantAny {
	fn allow(
		msg: &pallet_ibc::ics20::Ics20TransferMsg,
		_flow_type: pallet_ibc::ics20::FlowType,
	) -> Result<(), ()> {
		// one DOT/PICA, so so for USDT not safe, but we do not yet do it
		if msg.token.amount.as_u256() <= ::ibc::bigint::U256::from(10_u64.pow(12)) {
			return Ok(())
		}
		Err(())
	}
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
	type Router = ();
	type MinimumConnectionDelay = ConstU64<1>;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type WeightInfo = weights::ibc::WeightInfo<Self>;
	type AdminOrigin = EnsureRootOrOneThirdNativeTechnical;
	type FreezeOrigin = EnsureRootOrOneThirdNativeTechnical;
	type SpamProtectionDeposit = SpamProtectionDeposit;
	type IbcAccountId = Self::AccountId;
	type TransferOrigin = EnsureSignedBy<ReleaseMembership, Self::IbcAccountId>;
	type RelayerOrigin = EnsureSignedBy<TechnicalCommitteeMembership, Self::IbcAccountId>;
	type HandleMemo = ();
	type MemoMessage = MemoMessage;
	type Ics20RateLimiter = ConstantAny;
}
