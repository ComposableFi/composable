use crate::state::UserId;
use cw_xcvm_interpreter::msg::ExecuteMsg as InterpreterExecuteMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{Bridge, BridgeSecurity, Displayed, Funds, NetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	/// Address of the XCVM registry contract
	pub registry_address: String,
	/// Address of the XCVM interpreter contract code
	pub interpreter_code_id: u64,
	/// Network ID of this network
	pub network_id: NetworkId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	/// Run an XCVM program on the XCVM interpreter instance
	/// Creates a new one if there is no instance.
	Run {
		/// Origin network ID
		network_id: NetworkId,
		/// Origin user ID. (Caller)
		user_id: UserId,
		/// Message to execute in the XCVM interpreter instance
		interpreter_execute_msg: InterpreterExecuteMsg,
		/// Funds to fund the XCVM interpreter instance
		/// The interpreter is funded prior to execution
		funds: Funds<Displayed<u128>>,
		/// The bridge that is used to call the router
		bridge: Bridge,
	},
	/// Set a certain bridge security requirement for a specific interpreter even it hasn't
	/// instantiated yet
	SetInterpreterSecurity {
		network_id: NetworkId,
		user_id: UserId,
		bridge_security: BridgeSecurity,
	},
	/// Register a bridge. The caller needs to be admin for this.
	RegisterBridge { bridge: Bridge },
	/// Unregister a bridge. The caller needs to be admin for this.
	UnregisterBridge { bridge: Bridge },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
