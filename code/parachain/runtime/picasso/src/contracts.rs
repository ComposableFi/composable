use crate::prelude::*;
use ::cosmwasm::pallet_hook::PalletHook;
use cosmwasm::{
	instrument::CostRules,
	runtimes::vm::{CosmwasmVM, CosmwasmVMError},
	types::{AccountIdOf, ContractLabelOf, ContractTrieIdOf, EntryPoint, PalletContractCodeInfo},
};
use cosmwasm_std::{ContractResult, Response};
use cosmwasm_vm::{
	executor::QueryResponse,
	vm::{VMBase, VmErrorOf},
};
use cosmwasm_vm_wasmi::OwnedWasmiVM;
use sp_core::ConstU32;

use sp_runtime::traits::AccountIdConversion;

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
	type Assets = AssetsTransactorRouter;
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

		match contract_address {
			address if address == &dex => Some(PalletContractCodeInfo::new(
				dex,
				false,
				PabloPalletId::get().0.to_vec().try_into().unwrap_or_default(),
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
