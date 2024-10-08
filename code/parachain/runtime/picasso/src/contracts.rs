use crate::prelude::*;
use ::cosmwasm::pallet_hook::PalletHook;
use common::cosmwasm::CosmwasmToSubstrateAccount;
use composable_traits::cosmwasm::CosmwasmSubstrateError;
use cosmwasm::{
	instrument::CostRules,
	runtimes::vm::{CosmwasmVM, CosmwasmVMError},
	types::{AccountIdOf, ContractLabelOf, ContractTrieIdOf, EntryPoint, PalletContractCodeInfo},
};
use cosmwasm_std::{ContractResult, Event, Response};
use cosmwasm_vm::{
	executor::QueryResponse,
	vm::{VMBase, VmErrorOf},
};
use cosmwasm_vm_wasmi::OwnedWasmiVM;
use cumulus_primitives_core::WeightLimit;
use sp_core::ConstU32;

use sp_runtime::traits::AccountIdConversion;
use xcm::{VersionedMultiAssets, VersionedMultiLocation, VersionedXcm};

use super::*;

parameter_types! {
	pub const CosmwasmPalletId: PalletId = PalletId(*b"cosmwasm");
	pub const ChainId: &'static str = "composable-network-picasso";
	pub const MaxContractLabelSize: u32 = 64;
	pub const MaxContractTrieIdSize: u32 = Hash::len_bytes() as u32;
	pub const MaxInstantiateSaltSize: u32 = 128;
	pub const MaxFundsAssets: u32 = 32;
	pub const CodeTableSizeLimit: u32 = 4096;
	pub const CodeGlobalVariableLimit: u32 = 256;
	pub const CodeParameterLimit: u32 = 128;
	pub const CodeBranchTableSizeLimit: u32 = 256;
	pub const CodeStorageByteDeposit: u32 = 1_000_000;
	pub const ContractStorageByteReadPrice: u32 = 1;
	pub const ContractStorageByteWritePrice: u32 = 1;
	pub WasmCostRules: CostRules<Runtime> = Default::default();
}

impl cosmwasm::Config for Runtime {
	const MAX_FRAMES: u8 = 64;
	type RuntimeEvent = RuntimeEvent;
	type AccountIdExtended = AccountId;
	type PalletId = CosmwasmPalletId;
	type MaxCodeSize = ConstU32<{ 1024 * 1024 }>;
	type MaxInstrumentedCodeSize = ConstU32<{ 2 * 1024 * 1024 }>;
	type MaxMessageSize = ConstU32<{ 64 * 1024 }>;
	type AccountToAddr = common::cosmwasm::CosmwasmToSubstrateAccount;
	type AssetToDenom = common::cosmwasm::CosmwasmToSubstrateAssetId;
	type Balance = Balance;
	type AssetId = CurrencyId;
	type Assets = Assets;
	type NativeAsset = Balances;
	type ChainId = ChainId;
	type MaxContractLabelSize = MaxContractLabelSize;
	type MaxContractTrieIdSize = MaxContractTrieIdSize;
	type MaxInstantiateSaltSize = MaxInstantiateSaltSize;
	type MaxFundsAssets = MaxFundsAssets;

	type CodeTableSizeLimit = CodeTableSizeLimit;
	type CodeGlobalVariableLimit = CodeGlobalVariableLimit;
	type CodeStackLimit = ConstU32<{ u32::MAX }>;

	type CodeParameterLimit = CodeParameterLimit;
	type CodeBranchTableSizeLimit = CodeBranchTableSizeLimit;
	type CodeStorageByteDeposit = CodeStorageByteDeposit;
	type ContractStorageByteReadPrice = ContractStorageByteReadPrice;
	type ContractStorageByteWritePrice = ContractStorageByteWritePrice;

	type WasmCostRules = WasmCostRules;
	type UnixTime = Timestamp;
	type WeightInfo = cosmwasm::weights::SubstrateWeight<Runtime>;
	type IbcRelayerAccount = TreasuryAccount;

	type IbcRelayer = cosmwasm::NoRelayer<Runtime>;

	type PalletHook = Precompiles;

	#[cfg(feature = "testnet")]
	type UploadWasmOrigin = system::EnsureSigned<Self::AccountId>;

	#[cfg(feature = "testnet")]
	type ExecuteWasmOrigin = system::EnsureSigned<Self::AccountId>;

	// really need to do EnsureOnOf<Sudo::key, >
	#[cfg(not(feature = "testnet"))]
	type UploadWasmOrigin = system::EnsureSignedBy<TechnicalCommitteeMembership, Self::AccountId>;

	#[cfg(not(feature = "testnet"))]
	type ExecuteWasmOrigin = frame_support::traits::EitherOfDiverse<
		system::EnsureSignedBy<TechnicalCommitteeMembership, Self::AccountId>,
		system::EnsureSignedBy<ReleaseMembership, Self::AccountId>,
	>;
}

pub struct Precompiles;

impl PalletHook<Runtime> for Precompiles {
	fn info(
		contract_address: &AccountIdOf<Runtime>,
	) -> Option<
		PalletContractCodeInfo<
			AccountIdOf<Runtime>,
			ContractLabelOf<Runtime>,
			ContractTrieIdOf<Runtime>,
		>,
	> {
		let dex: AccountIdOf<Runtime> = PabloPalletId::get().into_account_truncating();
		let xcm = PolkadotXcm::check_account();
		match contract_address {
			address if address == &dex => Some(PalletContractCodeInfo::new(
				dex,
				false,
				PabloPalletId::get().0.to_vec().try_into().unwrap_or_default(),
			)),
			address if address == &xcm => Some(PalletContractCodeInfo::new(
				xcm,
				false,
				// they did not made pallet to be public
				PalletId(*b"py/xcmch").0.to_vec().try_into().unwrap_or_default(),
			)),
			_ => None,
		}
	}

	fn execute<'a>(
		vm: &mut OwnedWasmiVM<CosmwasmVM<'a, Runtime>>,
		entrypoint: EntryPoint,
		message: &[u8],
	) -> Result<
		ContractResult<Response<<OwnedWasmiVM<CosmwasmVM<'a, Runtime>> as VMBase>::MessageCustom>>,
		VmErrorOf<OwnedWasmiVM<CosmwasmVM<'a, Runtime>>>,
	> {
		let contract_address = vm.0.data().contract_address.clone().into_inner();
		log::info!(
			"{:?}{:?}{:?}",
			&contract_address,
			&entrypoint,
			String::from_utf8_lossy(message)
		);
		let dex: AccountIdOf<Runtime> = PabloPalletId::get().into_account_truncating();
		let xcm = PolkadotXcm::check_account();
		match contract_address {
			address if address == dex => {
				let message: composable_traits::dex::ExecuteMsg =
					serde_json_wasm::from_slice(message)
						.map_err(|_| CosmwasmVMError::ExecuteDeserialize)?;

				let result = common::dex::DexPrecompile::<Pablo>::execute(
					vm.0.data().cosmwasm_message_info.sender.as_str(),
					message,
				)
				.map_err(|_| CosmwasmVMError::<Runtime>::Precompile);

				match result {
					Ok(result) => Ok(ContractResult::Ok(result)),
					Err(err) => Ok(ContractResult::Err(alloc::format!("{:?}", err))),
				}
			},
			address if address == xcm => {
				let message: xc_core::transport::xcm::ExecuteMsg =
					serde_json_wasm::from_slice(message)
						.map_err(|_| CosmwasmVMError::ExecuteDeserialize)?;

				let result = XcmPrecompile::execute(
					vm.0.data().cosmwasm_message_info.sender.as_str(),
					message,
				)
				.map_err(|_| CosmwasmVMError::<Runtime>::Precompile);

				match result {
					Ok(result) => Ok(ContractResult::Ok(result)),
					Err(err) => Ok(ContractResult::Err(alloc::format!("{:?}", err))),
				}
			},
			_ => Err(CosmwasmVMError::ContractNotFound),
		}
	}

	fn run<'a>(
		_vm: &mut OwnedWasmiVM<CosmwasmVM<'a, Runtime>>,
		_entrypoint: EntryPoint,
		_message: &[u8],
	) -> Result<Vec<u8>, VmErrorOf<OwnedWasmiVM<CosmwasmVM<'a, Runtime>>>> {
		Err(CosmwasmVMError::ContractNotFound)
	}

	fn query<'a>(
		vm: &mut OwnedWasmiVM<CosmwasmVM<'a, Runtime>>,
		message: &[u8],
	) -> Result<ContractResult<QueryResponse>, VmErrorOf<OwnedWasmiVM<CosmwasmVM<'a, Runtime>>>> {
		let contract_address = vm.0.data().contract_address.clone().into_inner();
		log::error!("{:?}{:?}", &contract_address, String::from_utf8_lossy(message));
		let dex: AccountIdOf<Runtime> = PabloPalletId::get().into_account_truncating();
		match contract_address {
			address if address == dex => {
				let message: composable_traits::dex::QueryMsg =
					serde_json_wasm::from_slice(message)
						.map_err(|_| CosmwasmVMError::ExecuteDeserialize)?;
				let result = common::dex::DexPrecompile::<Pablo>::query(
					vm.0.data().cosmwasm_message_info.sender.as_str(),
					message,
				)
				.map_err(|_| CosmwasmVMError::<Runtime>::Precompile);
				match result {
					Ok(ok) => Ok(ContractResult::Ok(ok)),
					Err(err) => Ok(ContractResult::Err(alloc::format!("{:?}", err))),
				}
			},
			_ => Err(CosmwasmVMError::ContractNotFound),
		}
	}
}

/// refactoring will be when request would be to do Composable,
/// in this case need to generalize on Runtime and make account converter generic too
/// and also when we would need proxy usage (as account delegation in cosmos)
struct XcmPrecompile {}
impl XcmPrecompile {
	pub fn execute(
		sender: &str,
		msg: xc_core::transport::xcm::ExecuteMsg,
	) -> Result<Response, CosmwasmSubstrateError> {
		use codec::Decode;
		use sp_runtime::traits::Convert;
		use xc_core::transport::xcm::ExecuteMsg::*;
		match msg {
			Send { dest, message } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConvert)
					.map(RuntimeOrigin::signed)?;
				PolkadotXcm::send(
					who,
					VersionedMultiLocation::decode(&mut dest.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedXcm::<()>::decode(&mut message.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
				)
				.map_err(|_| CosmwasmSubstrateError::Xcm)?;
				Ok(Response::new().add_event(Event::new("xcm.sent")))
			},
			ReserveTransferAssets { dest, beneficiary, assets, fee_asset_item } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConvert)
					.map(RuntimeOrigin::signed)?;
				PolkadotXcm::reserve_transfer_assets(
					who,
					VersionedMultiLocation::decode(&mut dest.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiLocation::decode(&mut beneficiary.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiAssets::decode(&mut assets.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					fee_asset_item,
				)
				.map_err(|_| CosmwasmSubstrateError::Xcm)?;
				Ok(Response::new().add_event(Event::new("xcm.reserve_transferred_assets")))
			},
			Execute { message, max_weight } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConvert)
					.map(RuntimeOrigin::signed)?;
				PolkadotXcm::execute(
					who,
					VersionedXcm::<RuntimeCall>::decode(&mut message.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					Weight::from_parts(max_weight, 0),
				)
				.map_err(|_| CosmwasmSubstrateError::Xcm)?;
				Ok(Response::new().add_event(Event::new("xcm.executed")))
			},
			TeleportAssets { dest, beneficiary, assets, fee_asset_item } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConvert)
					.map(RuntimeOrigin::signed)?;
				PolkadotXcm::teleport_assets(
					who,
					VersionedMultiLocation::decode(&mut dest.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiLocation::decode(&mut beneficiary.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiAssets::decode(&mut assets.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					fee_asset_item,
				)
				.map_err(|_| CosmwasmSubstrateError::Xcm)?;
				Ok(Response::new().add_event(Event::new("xcm.teleported_assets")))
			},
			LimitedReserveTransferAssets {
				dest,
				beneficiary,
				assets,
				fee_asset_item,
				weight_limit,
			} => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConvert)
					.map(RuntimeOrigin::signed)?;
				PolkadotXcm::limited_reserve_transfer_assets(
					who,
					VersionedMultiLocation::decode(&mut dest.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiLocation::decode(&mut beneficiary.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiAssets::decode(&mut assets.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					fee_asset_item,
					if weight_limit == 0 {
						WeightLimit::Unlimited
					} else {
						WeightLimit::Limited(Weight::from_parts(weight_limit, 0))
					},
				)
				.map_err(|_| CosmwasmSubstrateError::Xcm)?;
				Ok(Response::new().add_event(Event::new("xcm.limited_reserve_transferred_assets")))
			},
			LimitedTeleportAssets { dest, beneficiary, assets, fee_asset_item, weight_limit } => {
				let who = CosmwasmToSubstrateAccount::convert(sender.to_string())
					.map_err(|_| CosmwasmSubstrateError::AccountConvert)
					.map(RuntimeOrigin::signed)?;
				PolkadotXcm::limited_teleport_assets(
					who,
					VersionedMultiLocation::decode(&mut dest.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiLocation::decode(&mut beneficiary.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					VersionedMultiAssets::decode(&mut assets.0.as_ref())
						.map_err(|_| CosmwasmSubstrateError::Xcm)?
						.into(),
					fee_asset_item,
					if weight_limit == 0 {
						WeightLimit::Unlimited
					} else {
						WeightLimit::Limited(Weight::from_parts(weight_limit, 0))
					},
				)
				.map_err(|_| CosmwasmSubstrateError::Xcm)?;
				Ok(Response::new().add_event(Event::new("xcm.limited_teleported_assets")))
			},
		}
	}
}
