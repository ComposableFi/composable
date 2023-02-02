use crate::{runtimes::vm::CosmwasmVM, types::*, Config, Error};
use cosmwasm_vm::{
	cosmwasm_std::{ContractResult, QueryResponse, Response},
	vm::{VMBase, VmErrorOf},
};
use cosmwasm_vm_wasmi::OwnedWasmiVM;

/// A hook for pallets into the VM. Used to call substrate pallets from a CosmWasm contract.
pub trait PalletHook<T: Config> {
	/// Return hardcoded contract informations for a precompiled contract.
	fn info(
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
	fn execute<'a>(
		vm: &mut OwnedWasmiVM<CosmwasmVM<'a, T>>,
		entrypoint: EntryPoint,
		message: &[u8],
	) -> Result<
		ContractResult<Response<<OwnedWasmiVM<CosmwasmVM<'a, T>> as VMBase>::MessageCustom>>,
		VmErrorOf<OwnedWasmiVM<CosmwasmVM<'a, T>>>,
	>;

	/// Hook into a contract query.
	fn query<'a>(
		vm: &mut OwnedWasmiVM<CosmwasmVM<'a, T>>,
		message: &[u8],
	) -> Result<ContractResult<QueryResponse>, VmErrorOf<OwnedWasmiVM<CosmwasmVM<'a, T>>>>;
}

/// Default implementation, acting as identity (unhooked).
impl<T: Config> PalletHook<T> for () {
	fn info(
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

	fn execute<'a>(
		_vm: &mut OwnedWasmiVM<CosmwasmVM<'a, T>>,
		_entrypoint: EntryPoint,
		_message: &[u8],
	) -> Result<
		ContractResult<Response<<OwnedWasmiVM<CosmwasmVM<'a, T>> as VMBase>::MessageCustom>>,
		VmErrorOf<OwnedWasmiVM<CosmwasmVM<'a, T>>>,
	>
where {
		Err(Error::<T>::Unsupported.into())
	}

	fn query<'a>(
		_vm: &mut OwnedWasmiVM<CosmwasmVM<'a, T>>,
		_: &[u8],
	) -> Result<ContractResult<QueryResponse>, VmErrorOf<OwnedWasmiVM<CosmwasmVM<'a, T>>>> {
		Err(Error::<T>::Unsupported.into())
	}
}

impl<AccountId, Hash, Label, TrieId> PalletContractCodeInfo<AccountId, Hash, Label, TrieId>
where
	AccountId: Clone,
	Hash: Default,
	TrieId: Default,
{
	pub fn new(account_id: AccountId, ibc_capable: bool, label: Label) -> Self {
		PalletContractCodeInfo {
			code: CodeInfo::<AccountId, Hash> {
				// When this is used for an actual Pallet, we would use the Pallet's AccountId
				creator: account_id.clone(),
				// Not applicable to Pallet, so we use default()
				pristine_code_hash: Default::default(),
				// Not applicable since we use native gas metering
				instrumentation_version: u16::MAX,
				// Not applicable to Pallet, so we use the max
				refcount: u32::MAX,
				// A pallet can choose wether to be IBC capable
				ibc_capable,
			},
			contract: ContractInfo {
				// Pallets don't need a code ID, but we do not want to clash with CosmWasm
				// contracts so we pick u64::MAX
				code_id: u64::MAX,
				// We have no storage
				trie_id: Default::default(),
				// When this is used for an actual Pallet, we would use the Pallet's AccountId
				instantiator: account_id.clone(),
				// When this is used for an actual Pallet, we would use Some(the Pallet's
				// AccountId)
				admin: Some(account_id),
				// When this is used for an actual Pallet, we would use "pallet-PALLET_NAME"
				label,
			},
		}
	}
}
