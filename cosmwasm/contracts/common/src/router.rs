use crate::shared::BridgeMsg;
use cw_xcvm_utils::DefaultXCVMProgram;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, CallOrigin, Displayed, Funds, InterpreterOrigin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
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
    /// Set a certain bridge security requirement for a specific interpreter even it hasn't
    /// instantiated yet
    SetInterpreterSecurity {
        /// The interpreter origin we initiate this call for.
        interpreter_origin: InterpreterOrigin,
        /// The new bridge security the user is willing to take risk for.
        bridge_security: BridgeSecurity,
    },
    BridgeForward {
        /// The message we want to forward to the bridge gateway.
        msg: BridgeMsg,
    },
}
