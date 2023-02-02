use crate::{
	pallet_hook::PalletHook,
	runtimes::vm::{ContractBackend, CosmwasmVMError, CosmwasmVMShared},
	types::*,
	Config, Pallet,
};
use alloc::vec::Vec;
use core::marker::PhantomData;
use cosmwasm_vm::{
	cosmwasm_std::Coin,
	executor::{cosmwasm_call, AsFunctionName},
	system::{
		cosmwasm_system_entrypoint_hook, cosmwasm_system_run_hook, CosmwasmCallVM,
		CosmwasmDynamicVM, StargateCosmwasmCallVM,
	},
	vm::VmErrorOf,
};
use cosmwasm_vm_wasmi::OwnedWasmiVM;
use wasmi::AsContext;

/// Generic ready-to-call state for all input types
pub struct DispatchableCall<I, O, T: Config> {
	pub sender: AccountIdOf<T>,
	pub contract: AccountIdOf<T>,
	pub entrypoint: EntryPoint,
	pub output: O,
	pub marker: PhantomData<I>,
}

/// Dispatch state for all `Input`s
impl<I, O, T: Config> DispatchableCall<I, O, T> {
	/// Start a cosmwasm transaction by calling an entrypoint.
	/// This is used for running the top level messages and will return output O
	/// when succesful.
	///
	/// * `shared` - Shared state of the Cosmwasm VM.
	/// * `funds` - Funds to be transferred before execution.
	/// * `message` - Message to be passed to the entrypoint.
	pub(crate) fn top_level_call(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: ContractMessageOf<T>,
	) -> Result<O, CosmwasmVMError<T>>
	where
		for<'x> OwnedWasmiVM<DefaultCosmwasmVM<'x, T>>:
			CosmwasmCallVM<I> + CosmwasmDynamicVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<OwnedWasmiVM<DefaultCosmwasmVM<'x, T>>>:
			From<CosmwasmVMError<T>> + Into<CosmwasmVMError<T>>,
		I: AsFunctionName,
	{
		Pallet::<T>::top_level_dispatch(
			shared,
			self.entrypoint,
			self.sender,
			self.contract,
			funds,
			|mut vm| {
				cosmwasm_system_entrypoint_hook::<I, _>(&mut vm, &message, |vm, message| {
					match vm.0.as_context().data().contract_runtime {
						ContractBackend::CosmWasm { .. } =>
							cosmwasm_call::<I, _>(vm, message).map(Into::into),
						ContractBackend::Pallet =>
							T::PalletHook::execute(vm, self.entrypoint, message),
					}
				})
				.map_err(Into::into)
			},
		)?;
		Ok(self.output)
	}

	/// Continue the execution by running an entrypoint. This is used for running
	/// submessages.
	///
	/// * `shared` - Shared state of the Cosmwasm VM.
	/// * `funds` - Funds to be transferred before execution.
	/// * `message` - Message to be passed to the entrypoint.
	/// * `event_handler` - Event handler that is passed by the VM.
	pub(crate) fn sub_call(
		self,
		shared: &mut CosmwasmVMShared,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, CosmwasmVMError<T>>
	where
		for<'x> OwnedWasmiVM<DefaultCosmwasmVM<'x, T>>:
			CosmwasmCallVM<I> + CosmwasmDynamicVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<OwnedWasmiVM<DefaultCosmwasmVM<'x, T>>>:
			From<CosmwasmVMError<T>> + Into<CosmwasmVMError<T>>,
	{
		// Call `cosmwasm_call` to transfer funds and create the vm instance before
		// calling the callback.
		Pallet::<T>::sub_level_dispatch(
			shared,
			self.sender,
			self.contract,
			funds,
			// `cosmwasm_system_run` is called instead of `cosmwasm_system_entrypoint` here
			// because here, we want to continue running the transaction with the given
			// entrypoint
			|mut vm| {
				cosmwasm_system_run_hook::<I, _>(&mut vm, message, event_handler, |vm, message| {
					match vm.0.as_context().data().contract_runtime {
						ContractBackend::CosmWasm { .. } =>
							cosmwasm_call::<I, _>(vm, message).map(Into::into),
						ContractBackend::Pallet =>
							T::PalletHook::execute(vm, self.entrypoint, message),
					}
				})
				.map_err(Into::into)
			},
		)
	}
}
