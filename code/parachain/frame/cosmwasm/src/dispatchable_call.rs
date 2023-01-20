use crate::{
	pallet_hook::PalletHook,
	runtimes::vm::{ContractBackend, CosmwasmVMError, CosmwasmVMShared},
	types::*,
	Config, Pallet,
};
use alloc::vec::Vec;

use core::marker::PhantomData;
use cosmwasm_vm::{
	cosmwasm_std::{Binary, Coin, Event as CosmwasmEvent},
	executor::{cosmwasm_call, AsFunctionName},
	system::{
		cosmwasm_system_entrypoint_hook, cosmwasm_system_run_hook, CosmwasmCallVM,
		CosmwasmDynamicVM, StargateCosmwasmCallVM,
	},
	vm::VmErrorOf,
};
use cosmwasm_vm_wasmi::WasmiVM;

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
	///
	/// * `shared` - Shared state of the Cosmwasm VM.
	/// * `funds` - Funds to be transferred before execution.
	/// * `message` - Message to be passed to the entrypoint.
	pub(crate) fn call(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: ContractMessageOf<T>,
	) -> Result<O, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<DefaultCosmwasmVM<'x, T>>:
			CosmwasmCallVM<I> + CosmwasmDynamicVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<WasmiVM<DefaultCosmwasmVM<'x, T>>>:
			From<CosmwasmVMError<T>> + Into<CosmwasmVMError<T>>,
		I: AsFunctionName,
	{
		let entrypoint = self.entrypoint;
		self.call_internal(shared, funds, |vm| {
			cosmwasm_system_entrypoint_hook::<I, _>(vm, &message, |vm, message| {
				match vm.0.contract_runtime {
					ContractBackend::CosmWasm { .. } =>
						cosmwasm_call::<I, _>(vm, message).map(Into::into),
					ContractBackend::Pallet => T::PalletHook::execute(vm, entrypoint, message),
				}
			})
			.map_err(Into::into)
		})
	}

	fn call_internal<F>(
		self,
		shared: &mut CosmwasmVMShared,
		funds: FundsOf<T>,
		message: F,
	) -> Result<O, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<DefaultCosmwasmVM<'x, T>>:
			CosmwasmCallVM<I> + CosmwasmDynamicVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<WasmiVM<DefaultCosmwasmVM<'x, T>>>: Into<CosmwasmVMError<T>>,
		F: for<'x> FnOnce(
			&'x mut WasmiVM<DefaultCosmwasmVM<'x, T>>,
		) -> Result<(Option<Binary>, Vec<CosmwasmEvent>), CosmwasmVMError<T>>,
	{
		Pallet::<T>::do_extrinsic_dispatch(
			shared,
			self.entrypoint,
			self.sender,
			self.contract,
			funds,
			|vm| message(vm).map_err(Into::into),
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
	pub(crate) fn continue_run(
		self,
		shared: &mut CosmwasmVMShared,
		funds: Vec<Coin>,
		message: &[u8],
		event_handler: &mut dyn FnMut(cosmwasm_vm::cosmwasm_std::Event),
	) -> Result<Option<cosmwasm_vm::cosmwasm_std::Binary>, CosmwasmVMError<T>>
	where
		for<'x> WasmiVM<DefaultCosmwasmVM<'x, T>>:
			CosmwasmCallVM<I> + CosmwasmDynamicVM<I> + StargateCosmwasmCallVM,
		for<'x> VmErrorOf<WasmiVM<DefaultCosmwasmVM<'x, T>>>:
			From<CosmwasmVMError<T>> + Into<CosmwasmVMError<T>>,
	{
		// Call `cosmwasm_call` to transfer funds and create the vm instance before
		// calling the callback.
		Pallet::<T>::cosmwasm_call(
			shared,
			self.sender,
			self.contract,
			funds,
			// `cosmwasm_system_run` is called instead of `cosmwasm_system_entrypoint` here
			// because here, we want to continue running the transaction with the given
			// entrypoint
			|vm| {
				cosmwasm_system_run_hook::<I, _>(vm, message, event_handler, |vm, message| match vm
					.0
					.contract_runtime
				{
					ContractBackend::CosmWasm { .. } =>
						cosmwasm_call::<I, _>(vm, message).map(Into::into),
					ContractBackend::Pallet => T::PalletHook::execute(vm, self.entrypoint, message),
				})
				.map_err(Into::into)
			},
		)
	}
}
