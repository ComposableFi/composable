use crate::{
	runtimes::wasmi::{CodeValidation, CosmwasmVM, CosmwasmVMError},
	version::Version,
	AccountIdOf, Config, Error, Pallet,
};
<<<<<<< HEAD
use alloc::{format, string::String};
=======
use alloc::string::String;
>>>>>>> feat(cosmwasm): on query: do return ibc port for ibc capable contracts

impl<T: Config> Pallet<T> {
	/// Check whether a contract export the mandatory IBC functions and is consequently IBC capable.
	pub(crate) fn do_check_ibc_capability(module: &parity_wasm::elements::Module) -> bool {
		CodeValidation::new(module)
			.validate_exports(Version::<T>::IBC_EXPORTS)
			.map(|_| true)
			.unwrap_or(false)
	}

	pub(crate) fn do_ibc_transfer(
		_vm: &mut CosmwasmVM<T>,
		_channel_id: String,
		_to_address: String,
		_amount: cosmwasm_minimal_std::Coin,
		_timeout: cosmwasm_minimal_std::ibc::IbcTimeout,
	) -> Result<(), CosmwasmVMError<T>> {
		Err(Error::<T>::Unsupported.into())
	}

	pub(crate) fn do_ibc_send_packet(
		_vm: &mut CosmwasmVM<T>,
		_channel_id: String,
		_data: cosmwasm_minimal_std::Binary,
		_timeout: cosmwasm_minimal_std::ibc::IbcTimeout,
	) -> Result<(), CosmwasmVMError<T>> {
		Err(Error::<T>::Unsupported.into())
	}

	pub(crate) fn do_ibc_close_channel(
		_vm: &mut CosmwasmVM<T>,
		_channel_id: String,
	) -> Result<(), CosmwasmVMError<T>> {
		Err(Error::<T>::Unsupported.into())
	}

	pub(crate) fn do_compute_ibc_contract_port(address: AccountIdOf<T>) -> String {
		format!("wasm.{}", Pallet::<T>::account_to_cosmwasm_addr(address))
	}
}
