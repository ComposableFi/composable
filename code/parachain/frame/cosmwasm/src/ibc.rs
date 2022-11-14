use crate::{
	entrypoint::EntryPointCaller,
	runtimes::wasmi::{CodeValidation, CosmwasmVM, CosmwasmVMError},
	version::Version,
	AccountIdOf, CodeIdToInfo, Config, ContractMessageOf, Error, Pallet,
};
use alloc::{
	format,
	string::{String, ToString},
};

use cosmwasm_minimal_std::{
	ibc::{IbcChannelOpenMsg, IbcOrder},
	ContractResult,
};
use cosmwasm_vm::executor::{cosmwasm_call_serialize, ibc::IbcChannelOpen, ExecuteInput};
use cosmwasm_vm_wasmi::WasmiVM;
use sp_std::{marker::PhantomData, str::FromStr};

use crate::runtimes::wasmi::InitialStorageMutability;
use frame_support::{
	ensure,
	traits::{Get, UnixTime},
	weights::Weight,
	RuntimeDebug,
};
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, Amount, PrefixedCoin, PrefixedDenom},
	core::{
		ics04_channel::{
			channel::{Counterparty, Order},
			error::Error as IbcError,
			Version as IbcVersion,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{Module as IbcModule, ModuleId, ModuleOutputBuilder},
	},
	signer::Signer as IbcSigner,
};

use ibc_primitives::{IbcHandler, SendPacketData};
use pallet_ibc::routing::ModuleRouter as IbcModuleRouter;

const PORT_PREFIX: &str = "wasm";

impl<T: Config> Pallet<T> {
	/// Check whether a contract export the mandatory IBC functions and is consequently IBC capable.
	pub(crate) fn do_check_ibc_capability(module: &parity_wasm::elements::Module) -> bool {
		CodeValidation::new(module)
			.validate_exports(Version::<T>::IBC_EXPORTS)
			.map(|_| true)
			.unwrap_or(false)
	}

	pub(crate) fn do_ibc_transfer(
		vm: &mut CosmwasmVM<T>,
		channel_id: String,
		to_address: String,
		amount: cosmwasm_minimal_std::Coin,
		_timeout: cosmwasm_minimal_std::ibc::IbcTimeout,
	) -> Result<(), CosmwasmVMError<T>> {
		let channel_id = ChannelId::from_str(channel_id.as_ref())
			.map_err(|_| <CosmwasmVMError<T>>::Ibc("channel name is not valid".to_string()))?;
		let address: cosmwasm_minimal_std::Addr = vm.contract_address.clone().into();

		let port_id = PortId::from_str(address.as_str())
			.expect("all pallet instanced contract addresses are valid port names; qwe");

		let msg = MsgTransfer {
			source_port: port_id,
			source_channel: channel_id,
			token: PrefixedCoin {
				amount: Amount::from(amount.amount as u64),
				denom: PrefixedDenom::from_str(amount.denom.as_ref()).unwrap(),
			},
			sender: IbcSigner::from_str(address.as_str()).expect("address is signer; qed"),
			receiver: IbcSigner::from_str(to_address.as_str()).map_err(|_| <CosmwasmVMError<T>>::Ibc(format!("receiver is wrong")))?,
			timeout_height: todo!("after timeout will have pub interface"),
			timeout_timestamp: todo!("above"),
		};

		T::IbcRelayer::send_transfer(msg)
			.map_err(|err| <CosmwasmVMError<T>>::Ibc(format!("failed to send amount")))
	}

	pub(crate) fn do_ibc_send_packet(
		vm: &mut CosmwasmVM<T>,
		channel_id: String,
		data: cosmwasm_minimal_std::Binary,
		_timeout: cosmwasm_minimal_std::ibc::IbcTimeout,
	) -> Result<(), CosmwasmVMError<T>> {
		let port_id = PortId::from_str(&Self::do_compute_ibc_contract_port(
			vm.contract_address.as_ref().clone(),
		))
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
		vm: &mut CosmwasmVM<T>,
		channel_id: String,
	) -> Result<(), CosmwasmVMError<T>> {
		let _channel_id = ChannelId::from_str(channel_id.as_ref())
			.map_err(|_| <CosmwasmVMError<T>>::Ibc("channel name is not valid".to_string()))?;
		let address: cosmwasm_minimal_std::Addr = vm.contract_address.clone().into();

		let _port_id = PortId::from_str(address.as_str())
			.expect("all pallet instanced contract addresses are valid port names; qwe");
		/// https://github.com/ComposableFi/centauri/issues/115
		Err(Error::<T>::Unsupported.into())
	}

	pub(crate) fn do_compute_ibc_contract_port(address: AccountIdOf<T>) -> String {
		format!("wasm.{}", Pallet::<T>::account_to_cosmwasm_addr(address))
	}
}

#[derive(Default, RuntimeDebug, Eq, PartialEq, Clone)]
pub struct Router<T: Config> {
	_marker: PhantomData<T>,
}

impl<T: Config> Router<T> {
	fn port_to_address(port_id: &PortId) -> Result<<T as Config>::AccountIdExtended, IbcError> {
		let address_part = Self::parse_address_part(port_id)?;
		let address =
			<Pallet<T>>::cosmwasm_addr_to_account(address_part.to_string()).map_err(|_| {
				IbcError::implementation_specific("contract for port not found".to_string())
			})?;
		Ok(address)
	}

	fn relayer_executor(
		vm: &mut crate::runtimes::wasmi::CosmwasmVMShared,
		address: <T as Config>::AccountIdExtended,
		contract_info: crate::types::ContractInfo<
			<T as Config>::AccountIdExtended,
			frame_support::BoundedVec<u8, <T as Config>::MaxContractLabelSize>,
			frame_support::BoundedVec<u8, <T as Config>::MaxContractTrieIdSize>,
		>,
	) -> Result<WasmiVM<CosmwasmVM<T>>, IbcError> {
		let mut executor = <Pallet<T>>::cosmwasm_new_vm(
			vm,
			T::IbcRelayerAccount::get(),
			address,
			contract_info,
			Default::default(),
		)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;
		Ok(executor)
	}

	fn to_ibc_contract(
		address: &<T as Config>::AccountIdExtended,
	) -> Result<
		crate::types::ContractInfo<
			<T as Config>::AccountIdExtended,
			frame_support::BoundedVec<u8, <T as Config>::MaxContractLabelSize>,
			frame_support::BoundedVec<u8, <T as Config>::MaxContractTrieIdSize>,
		>,
		IbcError,
	> {
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
		Ok(contract_info)
	}

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
		let address = Self::port_to_address(port_id)?;

		let contract_info = Self::to_ibc_contract(&address)?;

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
					order: map_order(order),
					version: version.to_string(),
					connection_id: connection_hops[0].to_string(),
				},
			}
		};

		let gas = Weight::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address, contract_info)?;
		cosmwasm_call_serialize::<IbcChannelOpen, WasmiVM<CosmwasmVM<T>>, IbcChannelOpenMsg>(
			&mut executor,
			&message,
		)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))
		.map(|x| match x.0 {
			ContractResult::Ok(_) => Ok(()),
			ContractResult::Err(err) =>
				Err(IbcError::implementation_specific(format!("{:?}", err))),
		})??;
		let _remaining = vm.gas.remaining();
		Ok(())
	}

	fn on_chan_open_try(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &IbcVersion,
		counterparty_version: &IbcVersion,
	) -> Result<IbcVersion, IbcError> {
		let order = map_order(order);
		let address = Self::port_to_address(port_id)?;
		let contract_info = Self::to_ibc_contract(&address)?;

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
					order,
					version: version.to_string(),
					connection_id: connection_hops[0].to_string(),
				},
			}
		};

		let gas = Weight::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address, contract_info)?;
		let result = cosmwasm_call_serialize::<
			IbcChannelOpen,
			WasmiVM<CosmwasmVM<T>>,
			IbcChannelOpenMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))
		.map(|x| match x.0 {
			ContractResult::Ok(version) => Ok(version),
			ContractResult::Err(err) =>
				Err(IbcError::implementation_specific(format!("{:?}", err))),
		})??
		.map(|x| IbcVersion::new(x.version.to_string()))
		.unwrap_or(version.clone());
		let _remaining = vm.gas.remaining();
		Ok(result)
	}

	fn on_chan_open_ack(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_port_id: &PortId,
		_channel_id: &ChannelId,
		_counterparty_version: &IbcVersion,
	) -> Result<(), IbcError> {
		Ok(())
	}

	fn on_chan_open_confirm(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_port_id: &PortId,
		_channel_id: &ChannelId,
	) -> Result<(), IbcError> {
		Ok(())
	}

	fn on_chan_close_init(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_port_id: &PortId,
		_channel_id: &ChannelId,
	) -> Result<(), IbcError> {
		Ok(())
	}

	fn on_chan_close_confirm(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_port_id: &PortId,
		_channel_id: &ChannelId,
	) -> Result<(), IbcError> {
		Ok(())
	}

	fn on_recv_packet(
		&self,
		_output: &mut ModuleOutputBuilder,
		_packet: &ibc::core::ics04_channel::packet::Packet,
		_relayer: &pallet_ibc::Signer,
	) -> ibc::core::ics26_routing::context::OnRecvPacketAck {
		ibc::core::ics26_routing::context::OnRecvPacketAck::Nil(Box::new(|_| Ok(())))
	}

	fn on_acknowledgement_packet(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_packet: &ibc::core::ics04_channel::packet::Packet,
		_acknowledgement: &ibc::core::ics04_channel::msgs::acknowledgement::Acknowledgement,
		_relayer: &pallet_ibc::Signer,
	) -> Result<(), IbcError> {
		Ok(())
	}

	fn on_timeout_packet(
		&mut self,
		_output: &mut ModuleOutputBuilder,
		_packet: &ibc::core::ics04_channel::packet::Packet,
		_relayer: &pallet_ibc::Signer,
	) -> Result<(), IbcError> {
		Ok(())
	}
}

fn map_order(order: Order) -> IbcOrder {
	match order {
						    Order::None => unimplemented!("bridge: team what we should do with it? it is neither by spec nor cosmwasm knows this (Order::OrderedAllowTimeout  is in spec)"),
						    Order::Unordered => IbcOrder::Unordered,
						    Order::Ordered => IbcOrder::Unordered,
					    }
}

impl<T: Config + Send + Sync + Default> IbcModuleRouter for Router<T> {
	fn get_route_mut(
		&mut self,
		module_id: &impl core::borrow::Borrow<ModuleId>,
	) -> Option<&mut dyn IbcModule> {
		if module_id.borrow() == &into_module_id::<T>() {
			return Some(self)
		}

		None
	}

	fn has_route(module_id: &impl sp_std::borrow::Borrow<ModuleId>) -> bool {
		module_id.borrow() == &into_module_id::<T>()
	}

	fn lookup_module_by_port(port_id: &PortId) -> Option<ModuleId> {
		let address = Self::port_to_address(port_id).ok()?;
		let _ = Self::to_ibc_contract(&address).ok()?;
		Some(into_module_id::<T>())
	}
}

fn into_module_id<T: Config + Send + Sync + Default>() -> ModuleId {
	ModuleId::from_str(&String::from_utf8_lossy(&T::PalletId::get().0[..])).expect("constant")
}
