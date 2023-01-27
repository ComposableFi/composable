use crate::{runtimes::vm::ExportRequirement, types::DefaultCosmwasmVM, Config};
use core::marker::PhantomData;
use cosmwasm_vm::{
	executor::{
		ibc::{
			IbcChannelCloseCall, IbcChannelConnectCall, IbcChannelOpenCall, IbcPacketAckCall,
			IbcPacketReceiveCall, IbcPacketTimeoutCall,
		},
		AllocateCall, AsFunctionName, DeallocateCall, ExecuteCall, InstantiateCall, MigrateCall,
		QueryCall, ReplyCall,
	},
	vm::VmMessageCustomOf,
};

pub struct Version<T>(PhantomData<T>);
impl<T: Config> Version<T> {
	/// Default module from where a CosmWasm import functions.
	pub const ENV_MODULE: &'static str = "env";

	/// Arbitrary function name for gas instrumentation.
	/// A contract is not allowed to import this function from the above [`ENV_MODULE`].
	pub const ENV_GAS: &'static str = "gas";

	/// V1 exports, verified w.r.t https://github.com/CosmWasm/cosmwasm/#exports
	/// The format is (required, function_name, function_signature)
	pub const EXPORTS: &'static [(
		ExportRequirement,
		&'static str,
		&'static [parity_wasm::elements::ValueType],
	)] = &[
		// We support v1+
		(
			ExportRequirement::Mandatory,
			// 	extern "C" fn interface_version_8() -> () {}
			"interface_version_8",
			&[],
		),
		// Memory related exports.
		(
			ExportRequirement::Mandatory,
			AllocateCall::<()>::NAME,
			// extern "C" fn allocate(size: usize) -> u32;
			&[parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Mandatory,
			DeallocateCall::<()>::NAME,
			// extern "C" fn deallocate(pointer: u32);
			&[parity_wasm::elements::ValueType::I32],
		),
		// Contract execution exports.
		(
			ExportRequirement::Mandatory,
			InstantiateCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			// extern "C" fn instantiate(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
			&[
				parity_wasm::elements::ValueType::I32,
				parity_wasm::elements::ValueType::I32,
				parity_wasm::elements::ValueType::I32,
			],
		),
		(
			ExportRequirement::Mandatory,
			ExecuteCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			// extern "C" fn execute(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
			&[
				parity_wasm::elements::ValueType::I32,
				parity_wasm::elements::ValueType::I32,
				parity_wasm::elements::ValueType::I32,
			],
		),
		(
			ExportRequirement::Mandatory,
			QueryCall::NAME,
			// extern "C" fn query(env_ptr: u32, msg_ptr: u32) -> u32;
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Optional,
			MigrateCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			// extern "C" fn migrate(env_ptr: u32, msg_ptr: u32) -> u32;
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Optional,
			ReplyCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			// extern "C" fn reply(env_ptr: u32, msg_ptr: u32) -> u32;
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
	];

	/// IBC callback a contract must export to be considered IBC capable:
	/// extern "C" fn ibc_channel_open(env_ptr: u32, msg_ptr: u32) -> u32;
	/// extern "C" fn ibc_channel_connect(env_ptr: u32, msg_ptr: u32) -> u32;
	/// extern "C" fn ibc_channel_close(env_ptr: u32, msg_ptr: u32) -> u32;
	/// extern "C" fn ibc_packet_receive(env_ptr: u32, msg_ptr: u32) -> u32;
	/// extern "C" fn ibc_packet_ack(env_ptr: u32, msg_ptr: u32) -> u32;
	/// extern "C" fn ibc_packet_timeout(env_ptr: u32, msg_ptr: u32) -> u32;
	pub const IBC_EXPORTS: &'static [(
		ExportRequirement,
		&'static str,
		&'static [parity_wasm::elements::ValueType],
	)] = &[
		(
			ExportRequirement::Mandatory,
			IbcChannelOpenCall::NAME,
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Mandatory,
			IbcChannelConnectCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Mandatory,
			IbcChannelCloseCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Mandatory,
			IbcPacketReceiveCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Mandatory,
			IbcPacketAckCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
		(
			ExportRequirement::Mandatory,
			IbcPacketTimeoutCall::<VmMessageCustomOf<DefaultCosmwasmVM<T>>>::NAME,
			&[parity_wasm::elements::ValueType::I32, parity_wasm::elements::ValueType::I32],
		),
	];
}
