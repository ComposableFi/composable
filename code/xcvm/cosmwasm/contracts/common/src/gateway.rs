use cw_xc_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xc_core::{CallOrigin, Displayed, Funds, InterpreterOrigin, NetworkId};

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.gateway";

/// Version of IBC channels used by the gateway.
pub const IBC_VERSION: &str = "xcvm-v0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	IbcSetNetworkChannel {
		network_id: NetworkId,
		channel_id: String,
	},

	ExecuteProgram {
		/// The program salt.
		salt: Vec<u8>,
		/// The program.
		program: DefaultXCVMProgram,
		/// Assets to fund the XCVM interpreter instance
		/// The interpreter is funded prior to execution
		assets: Funds<Displayed<u128>>,
	},

	/// Run an XCVM program on the XCVM interpreter instance
	/// Creates a new one if there is no instance.
	ExecuteProgramPrivileged {
		/// The origin of the call.
		call_origin: CallOrigin,
		/// The program salt.
		salt: Vec<u8>,
		/// The program.
		program: DefaultXCVMProgram,
		/// Assets to fund the XCVM interpreter instance
		/// The interpreter is funded prior to execution
		assets: Funds<Displayed<u128>>,
	},

	/// Message sent from interpreter trying to spawn program on another
	/// network.
	Bridge(BridgeMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BridgeMsg {
	pub interpreter_origin: InterpreterOrigin,
	pub network_id: NetworkId,
	pub salt: Vec<u8>,
	pub program: DefaultXCVMProgram,
	pub assets: Funds<Displayed<u128>>,
}
