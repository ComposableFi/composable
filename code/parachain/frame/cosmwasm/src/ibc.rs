use crate::{
	entrypoint::EntryPointCaller,
	runtimes::wasmi::{CodeValidation, CosmwasmVM, CosmwasmVMError},
	version::Version,
	AccountIdOf, CodeIdToInfo, Config, ContractMessageOf, Error, Pallet,
};
use alloc::{format, string::String};
use alloc::string::{String, ToString};

use cosmwasm_vm::{
    executor::{
        ibc::{ IbcChannelOpenInput, 
		}
	}
};
use cosmwasm_vm::executor::{cosmwasm_call_serialize, ibc::{IbcChannelOpen, IbcChannelOpenInput}, ExecuteInput};
use sp_std::{marker::PhantomData, str::FromStr};


use crate::runtimes::wasmi::InitialStorageMutability;
use frame_support::{
	ensure,
	traits::{Get, UnixTime},
	RuntimeDebug,
};
use ibc::core::{
	ics04_channel::{
		channel::{Counterparty, Order},
		error::Error as IbcError,
		Version as IbcVersion,
	},
	ics24_host::identifier::{ChannelId, ConnectionId, PortId},
	ics26_routing::context::{Module as IbcModule, ModuleId, ModuleOutputBuilder},
};

use ibc_primitives::{IbcHandler, SendPacketData};
use pallet_ibc::routing::ModuleRouter as IbcModuleRouter;

const PORT_PREFIX: &str = "wasm";
>>>>>>> draft

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
		vm: &mut CosmwasmVM<T>,
		channel_id: String,
		data: cosmwasm_minimal_std::Binary,
		_timeout: cosmwasm_minimal_std::ibc::IbcTimeout,
	) -> Result<(), CosmwasmVMError<T>> {
		let port_id =
			PortId::from_str(&Self::do_ibc_contract_port(vm.contract_address.as_ref().clone()))
				.expect("address is port; qed");
		let channel_id = ChannelId::from_str(&channel_id)
			.map_err(|_| CosmwasmVMError::<T>::Ibc("unsupported channel name".to_string()))?;

		T::IbcRelayer::send_packet(SendPacketData {
			data: data.to_vec(),
			timeout_timestamp_offset: ((T::UnixTime::now().as_secs() + 36) -
				T::UnixTime::now().as_secs()) *
				1_000_000,
			timeout_height_offset: 0,
			channel_id,
			port_id,
		})
		.map_err(|_| Error::<T>::Unsupported.into())
	}

	pub(crate) fn do_ibc_close_channel(
		_vm: &mut CosmwasmVM<T>,
		_channel_id: String,
	) -> Result<(), CosmwasmVMError<T>> {
		Err(Error::<T>::Unsupported.into())
	}

<<<<<<< HEAD
	pub(crate) fn do_compute_ibc_contract_port(address: AccountIdOf<T>) -> String {
		format!("wasm.{}", Pallet::<T>::account_to_cosmwasm_addr(address))
=======
	pub(crate) fn do_ibc_contract_port(address: AccountIdOf<T>) -> String {
		format!("{}.{}", PORT_PREFIX, Pallet::<T>::account_to_cosmwasm_addr(address))
	}
}

#[derive(Default, RuntimeDebug, Eq, PartialEq, Clone)]
pub struct Router<T: Config> {
	_marker: PhantomData<T>,
}

impl<T: Config> Router<T> {
	fn parse_address_part(port_id: &PortId) -> Result<&str, IbcError> {
		let port_id = port_id.as_str();
		let mut prefix_address = port_id.split('.');
		ensure!(
			prefix_address.next() == Some(PORT_PREFIX),
			IbcError::implementation_specific(format!(
				"port should be prefixed with `{}.`",
				PORT_PREFIX
			))
		);
		let address = prefix_address
			.next()
			.ok_or_else(|| IbcError::implementation_specific("wrong port_id".to_string()))?;
		ensure!(
			prefix_address.next() == None,
			IbcError::implementation_specific("wrong port_id".to_string())
		);
		Ok(address)
	}
}

impl<T: Config + Send + Sync> IbcModule for Router<T> {
	fn on_chan_open_init(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &IbcVersion,
		// weight_limit: Weight,
	) -> Result<(), IbcError> {
		let address_part = Self::parse_address_part(port_id)?;

		let address =
			<Pallet<T>>::cosmwasm_addr_to_account(address_part.to_string()).map_err(|_| {
				IbcError::implementation_specific("contract for port not found".to_string())
			})?;

		let contract_info = <Pallet<T>>::contract_info(&address).map_err(|_| {
			IbcError::implementation_specific("contract for desired port not found".to_string())
		})?;

		let ibc_capable = <CodeIdToInfo<T>>::get(contract_info.code_id)
			.expect("all contract have code because of RC; qed")
			.ibc_capable;

		ensure!(
			ibc_capable,
			IbcError::implementation_specific("contract is not IBC capable".to_string())
		);

		let message = {
			use cosmwasm_minimal_std::ibc::{IbcChannel, IbcChannelOpenMsg, IbcEndpoint, IbcOrder};
			IbcChannelOpenMsg::OpenInit {
				channel: IbcChannel {
					endpoint: IbcEndpoint {
						channel_id: channel_id.to_string(),
						port_id: port_id.to_string(),
					},
					counterparty_endpoint: IbcEndpoint {
						port_id: counterparty.port_id.to_string(),
						channel_id: counterparty.channel_id.expect("channel").to_string(),
					},
					order: match order {
						Order::None => unimplemented!("bridge: team what we should do with it? it is neither by spec nor cosmwasm knows this (Order::OrderedAllowTimeout  is in spec)"),
						Order::Unordered => IbcOrder::Unordered,
						Order::Ordered => IbcOrder::Unordered,
					},
					version: version.to_string(),
					connection_id: connection_hops[0].to_string(),
				},
			}
		};

		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut vm = <Pallet<T>>::cosmwasm_new_vm(
			&mut vm,
			T::IbcRelayerAccount::get(),
			address,
			contract_info,
			Default::default(),
		)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;

		cosmwasm_call_serialize::<IbcChannelOpenInput, _, _>(&mut vm, &message);
		Ok(())
	}

	fn on_chan_open_try(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_order: Order,
		_connection_hops: &[ConnectionId],
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_counterparty: &Counterparty,
		_version: &IbcVersion,
		_counterparty_version: &IbcVersion,
	) -> Result<IbcVersion, IbcError> {
		Err(IbcError::implementation_specific("unimplemented!".to_string()))
	}
}

impl<T: Config + Send + Sync + Default> IbcModuleRouter for Router<T> {
	fn get_route_mut(
		&mut self,
		module_id: &impl core::borrow::Borrow<ModuleId>,
	) -> Option<&mut dyn IbcModule> {
		if module_id.borrow() == &ModuleId::from_str("cosmwasm").expect("constant") {
			return Some(self)
		}

		None
	}

	fn has_route(module_id: &impl sp_std::borrow::Borrow<ModuleId>) -> bool {
		module_id.borrow() == &ModuleId::from_str("cosmwasm").expect("constant")
	}

	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId> {
		let address_part = Self::parse_address_part(port_id).ok()?;

		let address = <Pallet<T>>::cosmwasm_addr_to_account(address_part.to_string())
			.map_err(|_| {
				IbcError::implementation_specific("contract for port not found".to_string())
			})
			.ok()?;

		let contract_info = <Pallet<T>>::contract_info(&address)
			.map_err(|_| {
				IbcError::implementation_specific("contract for desired port not found".to_string())
			})
			.ok()?;

		let ibc_capable = <CodeIdToInfo<T>>::get(contract_info.code_id)
			.expect("all contract have code because of RC; qed")
			.ibc_capable;

		Some(ModuleId::from_str("cosmwasm").expect("constant"))
>>>>>>> draft
	}
}
