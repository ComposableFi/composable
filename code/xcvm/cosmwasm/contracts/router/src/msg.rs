use cw_xcvm_interpreter::msg::ExecuteMsg as InterpreterExecuteMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, CallOrigin, Displayed, Funds, NetworkId, UserOrigin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
	/// Address of the XCVM gateway contract
	pub gateway_address: String,
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
	ExecuteProgram {
		/// The origin of the call.
		call_origin: CallOrigin,
		/// Message to execute in the XCVM interpreter instance
		msg: InterpreterExecuteMsg,
		/// Funds to fund the XCVM interpreter instance
		/// The interpreter is funded prior to execution
		funds: Funds<Displayed<u128>>,
	},
	/// Set a certain bridge security requirement for a specific interpreter even it hasn't
	/// instantiated yet
	SetInterpreterSecurity {
		/// The user origin we initiate this call for.
		user_origin: UserOrigin,
		/// The new bridge security the user is willing to take risk for.
		bridge_security: BridgeSecurity,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {}
