use ::cosmwasm::pallet_hook::PalletHook;
use cosmwasm::{
	instrument::CostRules,
	runtimes::vm::{CosmwasmVM, CosmwasmVMError},
	types::{AccountIdOf, ContractLabelOf, ContractTrieIdOf, EntryPoint, PalletContractCodeInfo},
};
use cosmwasm_vm::{
	cosmwasm_std::{ContractResult, Response},
	executor::QueryResponse,
	vm::{VMBase, VmErrorOf},
};
use cosmwasm_vm_wasmi::OwnedWasmiVM;
use sp_core::ConstU32;

use sp_runtime::traits::AccountIdConversion;

use super::*;

/// Native <-> Cosmwasm account mapping
pub struct AccountToAddr;

impl Convert<alloc::string::String, Result<AccountId, ()>> for AccountToAddr {
	fn convert(a: alloc::string::String) -> Result<AccountId, ()> {
		let account =
			ibc_primitives::runtime_interface::ss58_to_account_id_32(&a).map_err(|_| ())?;
		Ok(account.into())
	}
}

impl Convert<AccountId, alloc::string::String> for AccountToAddr {
	fn convert(a: AccountId) -> alloc::string::String {
		let account = ibc_primitives::runtime_interface::account_id_to_ss58(a.into(), 49);
		String::from_utf8_lossy(account.as_slice()).to_string()
	}
}

impl Convert<Vec<u8>, Result<AccountId, ()>> for AccountToAddr {
	fn convert(a: Vec<u8>) -> Result<AccountId, ()> {
		Ok(<[u8; 32]>::try_from(a).map_err(|_| ())?.into())
	}
}

/// Native <-> Cosmwasm asset mapping
pub struct AssetToDenom;

impl Convert<alloc::string::String, Result<CurrencyId, ()>> for AssetToDenom {
	fn convert(currency_id: alloc::string::String) -> Result<CurrencyId, ()> {
		core::str::FromStr::from_str(&currency_id).map_err(|_| ())
	}
}

impl Convert<CurrencyId, alloc::string::String> for AssetToDenom {
	fn convert(CurrencyId(currency_id): CurrencyId) -> alloc::string::String {
		alloc::format!("{}", currency_id)
	}
}

parameter_types! {
	pub const CosmwasmPalletId: PalletId = PalletId(*b"cosmwasm");
	pub const ChainId: &'static str = "composable-network-picasso";
	pub const MaxInstrumentedCodeSize: u32 = 1024 * 1024;
	pub const MaxContractLabelSize: u32 = 64;
	pub const MaxContractTrieIdSize: u32 = Hash::len_bytes() as u32;
	pub const MaxInstantiateSaltSize: u32 = 128;
	pub const MaxFundsAssets: u32 = 32;
	pub const CodeTableSizeLimit: u32 = 4096;
	pub const CodeGlobalVariableLimit: u32 = 256;
	pub const CodeParameterLimit: u32 = 128;
	pub const CodeBranchTableSizeLimit: u32 = 256;

	// TODO: benchmark for proper values
	pub const CodeStorageByteDeposit: u32 = 1_000_000;
	pub const ContractStorageByteReadPrice: u32 = 1;
	pub const ContractStorageByteWritePrice: u32 = 1;
	pub WasmCostRules: CostRules<Runtime> = Default::default();
}

impl cosmwasm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdExtended = AccountId;
	type PalletId = CosmwasmPalletId;
	type MaxFrames = ConstU32<64>;
	type MaxCodeSize = ConstU32<{ 512 * 1024 }>;
	type MaxInstrumentedCodeSize = MaxInstrumentedCodeSize;

	#[cfg(feature = "testnet")]
	type MaxMessageSize = ConstU32<{ 128 * 1024 }>;

	#[cfg(not(feature = "testnet"))]
	type MaxMessageSize = ConstU32<{ 32 * 1024 }>;

	type AccountToAddr = AccountToAddr;

	type AssetToDenom = AssetToDenom;

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
		log::error!(
			"{:?}{:?}{:?}",
			&contract_address,
			&entrypoint,
			String::from_utf8_lossy(message)
		);
		let dex: AccountIdOf<Runtime> = PabloPalletId::get().into_account_truncating();
		match contract_address {
			address if address == dex => {
				let message: cw_dex_router::msg::ExecuteMsg =
					serde_json::from_slice(message).map_err(|_| CosmwasmVMError::ExecuteDeserialize)?;
				match message {
					cw_dex_router::msg::ExecuteMsg::Swap { in_asset, min_receive, pool_id } => {
						//<Pablo>::do_swap(contract_address, pool_id, in_asset, min_receive, keep_alive)
						let in_asset = AssetToDenom::convert(in_asset.denom).map_err(|_| CosmwasmVMError::AssetConversion)?;
						<Pablo>::do_swap(contract_address, pool_id, in_asset, min_receive, true)
							.unwrap();
						todo!()
					},
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
				panic!()
			},
			_ => Err(CosmwasmVMError::ContractNotFound),
		}
	}
}
