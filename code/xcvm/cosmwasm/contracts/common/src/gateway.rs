use cosmwasm_schema::cw_serde;
use cw_xc_utils::DefaultXCVMProgram;
use xc_core::{CallOrigin, Displayed, Funds, InterpreterOrigin, NetworkId};

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.gateway";

/// Version of IBC channels used by the gateway.
pub const IBC_VERSION: &str = "xcvm-v0";

#[cw_serde]
pub struct InstantiateMsg {
	/// Address of the XCVM registry contract
	pub registry_address: String,
	/// Address of the XCVM interpreter contract code
	pub interpreter_code_id: u64,
	/// Network ID of this network
	pub network_id: NetworkId,
	/// The admin which is allowed to update the bridge list.
	pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
	IbcSetNetworkChannel {
		network_id: NetworkId,
		channel_id: String,
	},

	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram {
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
	},

	/// Request to execute a program on behalf of given user.
	///
	/// This can only be sent by trusted contract.  The message is
	ExecuteProgramPrivileged {
		/// The origin of the call.
		call_origin: CallOrigin,
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
	},

	/// Message sent from interpreter trying to spawn program on another
	/// network.
	Bridge(BridgeMsg),
}

/// Definition of a program to be executed including its context.
#[cw_serde]
pub struct ExecuteProgramMsg {
	/// The program salt.
	pub salt: Vec<u8>,
	/// The program.
	pub program: DefaultXCVMProgram,
	/// Assets to fund the XCVM interpreter instance
	/// The interpreter is funded prior to execution
	pub assets: Funds<Displayed<u128>>,
}

#[cw_serde]
pub struct BridgeMsg {
	pub interpreter_origin: InterpreterOrigin,
	pub network_id: NetworkId,
	pub execute_program: ExecuteProgramMsg,
}
