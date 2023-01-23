use crate::{
	runtimes::vm::{CodeValidation, CosmwasmVMError, CosmwasmVMShared, InitialStorageMutability},
	types::{AccountIdOf, DefaultCosmwasmVM},
	version::Version,
	CodeIdToInfo, Config, Pallet,
};
use alloc::{
	format,
	string::{String, ToString},
};

use cosmwasm_vm::{
	cosmwasm_std::{
		Addr, ContractResult, Env, IbcAcknowledgement, IbcChannel, IbcChannelCloseMsg,
		IbcChannelConnectMsg, IbcChannelOpenMsg, IbcEndpoint, IbcOrder, IbcPacket, IbcPacketAckMsg,
		IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcTimeout, MessageInfo,
	},
	executor::{
		cosmwasm_call_serialize,
		ibc::{
			IbcChannelCloseCall, IbcChannelConnectCall, IbcChannelOpenCall, IbcPacketAckCall,
			IbcPacketReceiveCall, IbcPacketTimeoutCall,
		},
		AllocateCall, CosmwasmCallInput, CosmwasmCallWithoutInfoInput, DeallocateCall,
		DeserializeLimit, ExecutorError, HasInfo, ReadLimit, Unit,
	},
	has::Has,
	input::Input,
	memory::{ReadWriteMemory, ReadableMemoryErrorOf, WritableMemoryErrorOf},
	system::cosmwasm_system_entrypoint_serialize,
	vm::{VMBase, VmErrorOf, VmInputOf, VmOutputOf},
};
use cosmwasm_vm_wasmi::WasmiVM;
use sp_runtime::SaturatedConversion;

use frame_support::{ensure, traits::Get, RuntimeDebug};
use ibc::{
	applications::transfer::{Amount, PrefixedCoin, PrefixedDenom},
	core::{
		ics04_channel::{
			channel::{Counterparty, Order},
			error::Error as IbcError,
			Version as IbcVersion,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{
			Module as IbcModule, ModuleCallbackContext, ModuleId, ModuleOutputBuilder,
		},
	},
	signer::Signer as IbcSigner,
};
use sp_std::{marker::PhantomData, str::FromStr};

use ibc_primitives::{HandlerMessage, IbcHandler};
use pallet_ibc::routing::ModuleRouter as IbcModuleRouter;

const PORT_PREFIX: &str = "wasm";

trait IbcHandlerExtended<C: Config> {
	fn get_relayer_account() -> AccountIdOf<C>;
}

impl<T: IbcHandler<AccountIdOf<C>>, C: Config> IbcHandlerExtended<C> for T {
	fn get_relayer_account() -> AccountIdOf<C> {
		C::IbcRelayerAccount::get()
	}
}

impl<T: Config> Pallet<T> {
	/// Check whether a contract export the mandatory IBC functions and is consequently IBC capable.
	pub(crate) fn do_check_ibc_capability(module: &parity_wasm::elements::Module) -> bool {
		CodeValidation::new(module)
			.validate_exports(Version::<T>::IBC_EXPORTS)
			.map(|_| true)
			.unwrap_or(false)
	}

	pub(crate) fn do_ibc_transfer(
		vm: &mut DefaultCosmwasmVM<T>,
		channel_id: String,
		to_address: String,
		amount: cosmwasm_vm::cosmwasm_std::Coin,
		timeout: cosmwasm_vm::cosmwasm_std::IbcTimeout,
	) -> Result<(), CosmwasmVMError<T>> {
		let channel_id = ChannelId::from_str(channel_id.as_ref())
			.map_err(|_| <CosmwasmVMError<T>>::Ibc("channel name is not valid".to_string()))?;
		let address: cosmwasm_vm::cosmwasm_std::Addr = vm.contract_address.clone().into();

		let _port_id = PortId::from_str(address.as_str())
			.expect("all pallet instanced contract addresses are valid port names; qwe");

		let msg = HandlerMessage::<AccountIdOf<T>>::Transfer {
			channel_id,
			coin: PrefixedCoin {
				// TODO: Amount from centauri should not have a From<u64> instance.
				// https://app.clickup.com/t/20465559/XCVM-241?comment=1190198806
				amount: Amount::from(amount.amount.u128().saturated_into::<u64>()),
				denom: PrefixedDenom::from_str(amount.denom.as_ref()).map_err(|_| {
					<CosmwasmVMError<T>>::Ibc("provided asset is not IBC compatible".to_string())
				})?,
			},
			from: vm.contract_address.clone().into_inner(),
			timeout: ibc_primitives::Timeout::Absolute {
				timestamp: timeout.timestamp().map(|t| t.nanos()),
				height: timeout.block().map(|b| b.height),
			},
			to: IbcSigner::from_str(to_address.as_ref())
				.map_err(|_| <CosmwasmVMError<T>>::Ibc("bad ".to_string()))?,
		};

		T::IbcRelayer::handle_message(msg)
			.map_err(|_err| <CosmwasmVMError<T>>::Ibc("failed to send amount".to_string()))
	}

	pub(crate) fn do_ibc_send_packet(
		vm: &mut DefaultCosmwasmVM<T>,
		channel_id: String,
		data: cosmwasm_vm::cosmwasm_std::Binary,
		timeout: cosmwasm_vm::cosmwasm_std::IbcTimeout,
	) -> Result<(), CosmwasmVMError<T>> {
		let port_id = PortId::from_str(&Self::do_compute_ibc_contract_port(
			vm.contract_address.as_ref().clone(),
		))
		.expect("address is port; qed");
		let channel_id = ChannelId::from_str(&channel_id)
			.map_err(|_| CosmwasmVMError::<T>::Ibc("unsupported channel name".to_string()))?;

		T::IbcRelayer::handle_message(HandlerMessage::SendPacket {
			data: data.to_vec(),
			timeout: ibc_primitives::Timeout::Absolute {
				timestamp: timeout.timestamp().map(|t| t.nanos()),
				height: timeout.block().map(|b| b.height),
			},
			channel_id,
			port_id,
		})
		.map_err(|_| CosmwasmVMError::<T>::Ibc("failed to send packet".to_string()))
	}

	pub(crate) fn do_ibc_close_channel(
		vm: &mut DefaultCosmwasmVM<T>,
		channel_id: String,
	) -> Result<(), CosmwasmVMError<T>> {
		let channel_id = ChannelId::from_str(channel_id.as_ref())
			.map_err(|_| <CosmwasmVMError<T>>::Ibc("channel name is not valid".to_string()))?;
		let address: cosmwasm_vm::cosmwasm_std::Addr = vm.contract_address.clone().into();

		let port_id = PortId::from_str(address.as_str())
			.expect("all pallet instanced contract addresses are valid port names; qwe");

		T::IbcRelayer::handle_message(HandlerMessage::CloseChannel { channel_id, port_id })
			.map_err(|_| CosmwasmVMError::<T>::Ibc("failed to close channel".to_string()))
	}

	pub(crate) fn do_compute_ibc_contract_port(address: AccountIdOf<T>) -> String {
		format!("wasm.{}", Pallet::<T>::account_to_cosmwasm_addr(address))
	}
}

#[derive(RuntimeDebug, Eq, PartialEq, Clone)]
pub struct Router<T: Config> {
	_marker: PhantomData<T>,
}

impl<T: Config> Default for Router<T> {
	fn default() -> Self {
		Self { _marker: <_>::default() }
	}
}

struct MapBinary(sp_std::vec::Vec<u8>);

impl AsRef<[u8]> for MapBinary {
	fn as_ref(&self) -> &[u8] {
		&self.0[..]
	}
}
impl ibc::core::ics26_routing::context::Acknowledgement for MapBinary {}

struct VmPerContract<T: Config> {
	pub runtime: CosmwasmVMShared,
	pub address: T::AccountIdExtended,
}

impl<T: Config> VmPerContract<T> {
	pub fn instance(&mut self) -> Result<WasmiVM<DefaultCosmwasmVM<T>>, IbcError> {
		<Router<T>>::relayer_executor(&mut self.runtime, self.address.clone())
	}
}

impl<T: Config> Router<T> {
	fn port_to_address(port_id: &PortId) -> Result<<T as Config>::AccountIdExtended, IbcError> {
		let address_part = Self::parse_address_part(port_id)?;
		let address =
			<Pallet<T>>::cosmwasm_addr_to_account(address_part.to_string()).map_err(|_| {
				IbcError::implementation_specific(
					"was not able to convert port to contract address".to_string(),
				)
			})?;
		Ok(address)
	}

	fn relayer_executor(
		vm: &mut CosmwasmVMShared,
		address: T::AccountIdExtended,
	) -> Result<WasmiVM<DefaultCosmwasmVM<T>>, IbcError> {
		let executor = <Pallet<T>>::cosmwasm_new_vm(
			vm,
			<T::IbcRelayer as IbcHandlerExtended<T>>::get_relayer_account(),
			address,
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
		let contract_info = <Pallet<T>>::contract_info(address).map_err(|_| {
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
			prefix_address.next().is_none(),
			IbcError::implementation_specific("wrong port_id".to_string())
		);
		Ok(address)
	}

	fn create(address: T::AccountIdExtended) -> Result<VmPerContract<T>, IbcError> {
		let gas = u64::MAX;
		let vm = {
			let runtime =
				<Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);

			VmPerContract { runtime, address }
		};
		Ok(vm)
	}

	fn execute<I, M, V>(vm: &mut V, message: M) -> Result<I::Output, IbcError>
	where
		// unfortunately, here is reason
		// 1. Rust fails to decided that VM is V (which it is)
		// 2. also without unsafe rust cannot do struct which borrows mut 2 of its items (without
		// callbacks) 3. so in order to reuse calls either need to build entrypoint.rs like wrappers for
		// each call 4. but here just closed `cosmwasm_vm + pallet-cosmwasm` on functional level (helped
		// rust type inference)
		M: serde::Serialize,
		I: Input + HasInfo,
		I::Output: serde::de::DeserializeOwned + ReadLimit + DeserializeLimit,
		V: cosmwasm_vm::vm::VM + ReadWriteMemory + Has<Env> + Has<MessageInfo> + VMBase,
		<V as VMBase>::Error: sp_std::fmt::Debug,
		V::Pointer: for<'x> TryFrom<VmOutputOf<'x, V>, Error = VmErrorOf<V>>,
		for<'x> Unit: TryFrom<VmOutputOf<'x, V>, Error = VmErrorOf<V>>,
		for<'x> VmInputOf<'x, V>: TryFrom<AllocateCall<V::Pointer>, Error = VmErrorOf<V>>
			+ TryFrom<DeallocateCall<V::Pointer>, Error = VmErrorOf<V>>
			+ TryFrom<CosmwasmCallInput<'x, V::Pointer, I>, Error = VmErrorOf<V>>
			+ TryFrom<CosmwasmCallWithoutInfoInput<'x, V::Pointer, I>, Error = VmErrorOf<V>>,
		VmErrorOf<V>:
			From<ReadableMemoryErrorOf<V>> + From<WritableMemoryErrorOf<V>> + From<ExecutorError>,
	{
		cosmwasm_call_serialize::<I, V, M>(vm, &message)
			.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))
	}

	fn on_recv_packet_internal(
		&self,
		_output: &mut ModuleOutputBuilder,
		packet: &ibc::core::ics04_channel::packet::Packet,
		relayer: &pallet_ibc::Signer,
	) -> Result<sp_std::vec::Vec<u8>, IbcError> {
		let address = Self::port_to_address(&packet.destination_port)?;

		let message = IbcPacketReceiveMsg::new(
			IbcPacket::new(
				packet.data.clone(),
				IbcEndpoint {
					port_id: packet.source_port.to_string(),
					channel_id: packet.source_channel.to_string(),
				},
				IbcEndpoint {
					port_id: packet.destination_port.to_string(),
					channel_id: packet.destination_channel.to_string(),
				},
				packet.sequence.into(),
				IbcTimeout::with_both(
					to_cosmwasm_timeout_block(packet.timeout_height),
					to_cosmwasm_timestamp(packet.timeout_timestamp),
				),
			),
			Addr::unchecked(relayer.to_string()),
		);
		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address)?;
		let (data, _) = cosmwasm_system_entrypoint_serialize::<
			IbcPacketReceiveCall,
			_,
			IbcPacketReceiveMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;
		let _remaining = vm.gas.remaining();
		Ok(data.expect("there is always data from contract; qed").0)
	}
}

fn to_cosmwasm_timeout_block(
	ibc::core::ics02_client::height::Height { revision_number, revision_height }: ibc::core::ics02_client::height::Height,
) -> cosmwasm_vm::cosmwasm_std::IbcTimeoutBlock {
	cosmwasm_vm::cosmwasm_std::IbcTimeoutBlock {
		revision: revision_number,
		height: revision_height,
	}
}

fn to_cosmwasm_timestamp(
	timestamp: ibc::timestamp::Timestamp,
) -> cosmwasm_vm::cosmwasm_std::Timestamp {
	cosmwasm_vm::cosmwasm_std::Timestamp::from_nanos(timestamp.nanoseconds())
}

impl<T: Config + Send + Sync> IbcModule for Router<T> {
	fn on_chan_open_init(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &IbcVersion,
		_relayer: &IbcSigner,
		// weight_limit: Weight, https://github.com/ComposableFi/centauri/issues/129
	) -> Result<(), IbcError> {
		let address = Self::port_to_address(port_id)?;

		let message = {
			IbcChannelOpenMsg::OpenInit {
				channel: IbcChannel::new(
					IbcEndpoint {
						channel_id: channel_id.to_string(),
						port_id: port_id.to_string(),
					},
					IbcEndpoint {
						port_id: counterparty.port_id.to_string(),
						channel_id: counterparty.channel_id.expect("channel").to_string(),
					},
					map_order(order)?,
					version.to_string(),
					connection_hops
						.get(0)
						.expect("by spec there is at least one connection; qed")
						.to_string(),
				),
			}
		};

		let mut vm = Self::create(address)?;
		let mut instance = vm.instance()?;
		contract_to_result(
			Self::execute::<IbcChannelOpenCall, IbcChannelOpenMsg, WasmiVM<DefaultCosmwasmVM<T>>>(
				&mut instance,
				message,
			)?
			.0,
		)?;

		Ok(())
	}

	fn on_chan_open_try(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		order: Order,
		connection_hops: &[ConnectionId],
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty: &Counterparty,
		version: &IbcVersion,
		_counterparty_version: &IbcVersion,
		_relayer: &IbcSigner,
	) -> Result<IbcVersion, IbcError> {
		let address = Self::port_to_address(port_id)?;

		let message = {
			IbcChannelOpenMsg::OpenInit {
				channel: IbcChannel::new(
					IbcEndpoint {
						channel_id: channel_id.to_string(),
						port_id: port_id.to_string(),
					},
					IbcEndpoint {
						port_id: counterparty.port_id.to_string(),
						channel_id: counterparty.channel_id.expect("one may not have OpenTry without remote channel id by protocol; qed").to_string(),
					},
					map_order(order)?,
					version.to_string(),
					connection_hops.get(0).expect("by spec there is at least one connection; qed").to_string(),
				),
			}
		};

		let mut vm = Self::create(address)?;
		let mut instance = vm.instance()?;
		let result = contract_to_result(
			Self::execute::<IbcChannelOpenCall, IbcChannelOpenMsg, WasmiVM<DefaultCosmwasmVM<T>>>(
				&mut instance,
				message,
			)?
			.0,
		)?
		.map(|x| IbcVersion::new(x.version))
		.unwrap_or_else(|| version.clone());
		let _remaining = vm.runtime.gas.remaining();
		Ok(result)
	}

	fn on_chan_open_ack(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		counterparty_version: &IbcVersion,
		_relayer: &IbcSigner,
	) -> Result<(), IbcError> {
		let metadata = ctx
			.channel_end(&(port_id.clone(), *channel_id))
			.expect("callback provides only existing connection port pairs; qed");
		let address = Self::port_to_address(port_id)?;
		let message = IbcChannelConnectMsg::OpenAck {
			channel: map_channel(port_id, channel_id, metadata)?,
			counterparty_version: counterparty_version.to_string(),
		};
		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address)?;
		let (_data, _events) = cosmwasm_system_entrypoint_serialize::<
			IbcChannelConnectCall,
			_,
			IbcChannelConnectMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;

		Ok(())
	}

	fn on_chan_open_confirm(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		_relayer: &IbcSigner,
	) -> Result<(), IbcError> {
		let metadata = ctx
			.channel_end(&(port_id.clone(), *channel_id))
			.expect("callback provides only existing connection port pairs; qed");
		let address = Self::port_to_address(port_id)?;
		let message = IbcChannelConnectMsg::OpenConfirm {
			channel: map_channel(port_id, channel_id, metadata)?,
		};
		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address)?;
		let (_data, _events) = cosmwasm_system_entrypoint_serialize::<
			IbcChannelConnectCall,
			_,
			IbcChannelConnectMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;
		Ok(())
	}

	fn on_chan_close_init(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		_relayer: &IbcSigner,
	) -> Result<(), IbcError> {
		let metadata = ctx
			.channel_end(&(port_id.clone(), *channel_id))
			.expect("callback provides only existing connection port pairs; qed");
		let address = Self::port_to_address(port_id)?;
		let message = IbcChannelCloseMsg::CloseInit {
			channel: IbcChannel::new(
				IbcEndpoint { port_id: port_id.to_string(), channel_id: channel_id.to_string() },
				map_endpoint(&metadata),
				map_order(metadata.ordering)?,
				metadata.version.to_string(),
				metadata
					.connection_hops
					.get(0)
					.expect("at least one connection should exists; qed")
					.to_string(),
			),
		};
		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address)?;
		let (_data, _events) = cosmwasm_system_entrypoint_serialize::<
			IbcChannelCloseCall,
			_,
			IbcChannelCloseMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;
		Ok(())
	}

	fn on_chan_close_confirm(
		&mut self,
		ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		port_id: &PortId,
		channel_id: &ChannelId,
		_relayer: &IbcSigner,
	) -> Result<(), IbcError> {
		let metadata = ctx
			.channel_end(&(port_id.clone(), *channel_id))
			.expect("callback provides only existing connection port pairs; qed");
		let address = Self::port_to_address(port_id)?;
		let message = IbcChannelCloseMsg::CloseConfirm {
			channel: map_channel(port_id, channel_id, metadata)?,
		};
		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address)?;
		let (_data, _events) = cosmwasm_system_entrypoint_serialize::<
			IbcChannelCloseCall,
			_,
			IbcChannelCloseMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;
		Ok(())
	}

	fn on_recv_packet(
		&self,
		_ctx: &dyn ModuleCallbackContext,
		output: &mut ModuleOutputBuilder,
		packet: &ibc::core::ics04_channel::packet::Packet,
		relayer: &pallet_ibc::Signer,
	) -> Result<(), IbcError> {
		match self.on_recv_packet_internal(output, packet, relayer) {
			Ok(_) => Ok(()), // so unlike Go IBC impl we cannot send data in acknowledgement...
			Err(err) => Err(IbcError::implementation_specific(format!("{:?}", err))),
		}
	}

	fn on_acknowledgement_packet(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		packet: &ibc::core::ics04_channel::packet::Packet,
		acknowledgement: &ibc::core::ics04_channel::msgs::acknowledgement::Acknowledgement,
		relayer: &pallet_ibc::Signer,
	) -> Result<(), IbcError> {
		let address = Self::port_to_address(&packet.source_port)?;

		let message = IbcPacketAckMsg::new(
			IbcAcknowledgement::new(acknowledgement.clone().into_bytes()),
			IbcPacket::new(
				packet.data.clone(),
				IbcEndpoint {
					port_id: packet.source_port.to_string(),
					channel_id: packet.source_channel.to_string(),
				},
				IbcEndpoint {
					port_id: packet.source_port.to_string(),
					channel_id: packet.source_channel.to_string(),
				},
				packet.sequence.into(),
				IbcTimeout::with_both(
					to_cosmwasm_timeout_block(packet.timeout_height),
					to_cosmwasm_timestamp(packet.timeout_timestamp),
				),
			),
			Addr::unchecked(relayer.to_string()),
		);

		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address)?;
		let (_data, _events) = cosmwasm_system_entrypoint_serialize::<
			IbcPacketAckCall,
			WasmiVM<_>,
			IbcPacketAckMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;
		let _remaining = vm.gas.remaining();
		Ok(())
	}

	fn on_timeout_packet(
		&mut self,
		_ctx: &dyn ModuleCallbackContext,
		_output: &mut ModuleOutputBuilder,
		packet: &ibc::core::ics04_channel::packet::Packet,
		relayer: &pallet_ibc::Signer,
	) -> Result<(), IbcError> {
		let address = Self::port_to_address(&packet.source_port)?;

		let message = IbcPacketTimeoutMsg::new(
			IbcPacket::new(
				packet.data.clone(),
				IbcEndpoint {
					port_id: packet.source_port.to_string(),
					channel_id: packet.source_channel.to_string(),
				},
				IbcEndpoint {
					port_id: packet.source_port.to_string(),
					channel_id: packet.source_channel.to_string(),
				},
				packet.sequence.into(),
				IbcTimeout::with_both(
					to_cosmwasm_timeout_block(packet.timeout_height),
					to_cosmwasm_timestamp(packet.timeout_timestamp),
				),
			),
			Addr::unchecked(relayer.to_string()),
		);

		let gas = u64::MAX;
		let mut vm = <Pallet<T>>::do_create_vm_shared(gas, InitialStorageMutability::ReadWrite);
		let mut executor = Self::relayer_executor(&mut vm, address)?;
		let (_data, _events) = cosmwasm_system_entrypoint_serialize::<
			IbcPacketTimeoutCall,
			WasmiVM<DefaultCosmwasmVM<T>>,
			IbcPacketTimeoutMsg,
		>(&mut executor, &message)
		.map_err(|err| IbcError::implementation_specific(format!("{:?}", err)))?;
		let _remaining = vm.gas.remaining();
		Ok(())
	}
}

fn map_channel(
	port_id: &PortId,
	channel_id: &ChannelId,
	metadata: ibc::core::ics04_channel::channel::ChannelEnd,
) -> Result<IbcChannel, IbcError> {
	Ok(IbcChannel::new(
		IbcEndpoint { port_id: port_id.to_string(), channel_id: channel_id.to_string() },
		map_endpoint(&metadata),
		map_order(metadata.ordering)?,
		metadata.version.to_string(),
		metadata
			.connection_hops
			.get(0)
			.expect("at least one connection should exists; qed")
			.to_string(),
	))
}

fn map_endpoint(metadata: &ibc::core::ics04_channel::channel::ChannelEnd) -> IbcEndpoint {
	IbcEndpoint {
		port_id: metadata.remote.port_id.to_string(),
		channel_id: metadata
			.remote
			.channel_id
			.expect("if callback was from counter party, then it has channel; qed")
			.to_string(),
	}
}

fn contract_to_result<T>(result: ContractResult<T>) -> Result<T, IbcError> {
	match result {
		ContractResult::Ok(ok) => Ok(ok),
		ContractResult::Err(err) => Err(IbcError::implementation_specific(err)),
	}
}

fn map_order(order: Order) -> Result<IbcOrder, IbcError> {
	match order {
		Order::Unordered => Ok(IbcOrder::Unordered),
		Order::Ordered => Ok(IbcOrder::Ordered),
	}
}

impl<T: Config + Send + Sync + Default> IbcModuleRouter for Router<T> {
	fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn IbcModule> {
		if module_id == &into_module_id::<T>() {
			return Some(self)
		}

		None
	}

	fn has_route(module_id: &ModuleId) -> bool {
		module_id == &into_module_id::<T>()
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

pub struct NoRelayer<T> {
	_marker: sp_std::marker::PhantomData<T>,
}

impl<T: Config> ibc_primitives::IbcHandler<AccountIdOf<T>> for NoRelayer<T> {
	fn latest_height_and_timestamp(
		_port_id: &PortId,
		_channel_id: &ChannelId,
	) -> Result<(ibc::Height, ibc::timestamp::Timestamp), ibc_primitives::Error> {
		Err(ibc_primitives::Error::Other { msg: Some("not supported".to_string()) })
	}

	fn handle_message(_msg: HandlerMessage<AccountIdOf<T>>) -> Result<(), ibc_primitives::Error> {
		Err(ibc_primitives::Error::Other { msg: Some("not supported".to_string()) })
	}

	fn write_acknowledgement(
		_packet: &ibc::core::ics04_channel::packet::Packet,
		_ack: sp_std::vec::Vec<u8>,
	) -> Result<(), ibc_primitives::Error> {
		Err(ibc_primitives::Error::Other { msg: Some("not supported".to_string()) })
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn create_client() -> Result<ibc::core::ics24_host::identifier::ClientId, ibc_primitives::Error>
	{
		Err(ibc_primitives::Error::Other { msg: Some("not supported".to_string()) })
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn create_connection(
		_client_id: ibc::core::ics24_host::identifier::ClientId,
		_connection_id: ConnectionId,
	) -> Result<(), ibc_primitives::Error> {
		Err(ibc_primitives::Error::Other { msg: Some("not supported".to_string()) })
	}
}
