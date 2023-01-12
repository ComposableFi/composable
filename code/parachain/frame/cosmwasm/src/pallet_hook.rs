use crate::{runtimes::vm::CosmwasmVM, types::*, Config, Error};
use cosmwasm_vm::{
	cosmwasm_std::{ContractResult, QueryResponse, Response},
	vm::{VMBase, VmErrorOf},
};
use cosmwasm_vm_wasmi::WasmiVM;

/// A hook for pallets into the VM. Used to call substrate pallets from a CosmWasm contract.
pub trait PalletHook<T: Config> {
	/// Return hardcoded contract informations for a precompiled contract.
	fn precompiled_info(
		contract_address: &AccountIdOf<T>,
	) -> Option<
		PalletContractCodeInfo<
			AccountIdOf<T>,
			CodeHashOf<T>,
			ContractLabelOf<T>,
			ContractTrieIdOf<T>,
		>,
	>;

	/// Hook into a contract call.
	fn precompiled_execute<'a>(
		vm: &mut WasmiVM<CosmwasmVM<'a, T>>,
		entrypoint: EntryPoint,
		message: &[u8],
	) -> Result<
		ContractResult<Response<<WasmiVM<CosmwasmVM<'a, T>> as VMBase>::MessageCustom>>,
		VmErrorOf<WasmiVM<CosmwasmVM<'a, T>>>,
	>;

	/// Hook into a contract query.
	fn precompiled_query<'a>(
		vm: &mut WasmiVM<CosmwasmVM<'a, T>>,
		message: &[u8],
	) -> Result<ContractResult<QueryResponse>, VmErrorOf<WasmiVM<CosmwasmVM<'a, T>>>>;
}

/// Default implementation, acting as identity (unhooked).
impl<T: Config> PalletHook<T> for () {
	fn precompiled_info(
		_: &AccountIdOf<T>,
	) -> Option<
		PalletContractCodeInfo<
			AccountIdOf<T>,
			CodeHashOf<T>,
			ContractLabelOf<T>,
			ContractTrieIdOf<T>,
		>,
	> {
		None
	}

	fn precompiled_execute<'a>(
		_vm: &mut WasmiVM<CosmwasmVM<'a, T>>,
		_entrypoint: EntryPoint,
		_message: &[u8],
	) -> Result<
		ContractResult<Response<<WasmiVM<CosmwasmVM<'a, T>> as VMBase>::MessageCustom>>,
		VmErrorOf<WasmiVM<CosmwasmVM<'a, T>>>,
	>
where {
		Err(Error::<T>::Unsupported.into())
	}

	fn precompiled_query<'a>(
		_vm: &mut WasmiVM<CosmwasmVM<'a, T>>,
		_: &[u8],
	) -> Result<ContractResult<QueryResponse>, VmErrorOf<WasmiVM<CosmwasmVM<'a, T>>>> {
		Err(Error::<T>::Unsupported.into())
	}
}
