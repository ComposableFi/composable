pub mod config;

pub use config::*;

use crate::prelude::*;

use crate::{
	transport::ibc::XcMessageData, AssetId, CallOrigin, Displayed, Funds, InterpreterOrigin,
	NetworkId,
};

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.gateway";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	Config(ConfigSubMsg),

	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram {
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
		tip: Addr,
	},

	/// Request to execute a program on behalf of given user.
	///
	/// This can only be sent by trusted contract.  The message is
	ExecuteProgramPrivileged {
		/// The origin of the call.
		call_origin: CallOrigin,
		/// Program to execute.
		execute_program: ExecuteProgramMsg,

		tip: Addr,
	},

	/// Message sent from interpreter trying to spawn program on another
	/// network.
	BridgeForward(BridgeForwardMsg),

	/// simple permissionless message which produce xcvm program to test routes
	Shortcut(ShortcutSubMsg),

	/// executed by host as part of memo handling
	MessageHook(XcMessageData),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ShortcutSubMsg {
	Transfer {
		/// assets from there
		asset_id: AssetId,
		amount: Uint128,
		/// target network, can hope over several networks
		/// if route is stored in state
		network: NetworkId,
		/// by default receiver is this
		receiver: Option<String>,
	},
}

/// Definition of a program to be executed including its context.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct ExecuteProgramMsg {
	/// The program salt.
	pub salt: Vec<u8>,
	/// The program.
	pub program: crate::shared::DefaultXCVMProgram,
	/// Assets to fund the XCVM interpreter instance
	/// The interpreter is funded prior to execution
	pub assets: Funds<Displayed<u128>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct BridgeForwardMsg {
	pub interpreter_origin: InterpreterOrigin,
	/// target network
	pub to: NetworkId,
	pub msg: ExecuteProgramMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema, QueryResponses))]
pub enum QueryMsg {
	/// Returns [`AssetReference`] for an asset with given id.
	#[cfg_attr(feature = "std", returns(LookupResponse))]
	LookupAsset { asset_id: AssetId },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct LookupResponse {
	pub reference: AssetItem,
}
