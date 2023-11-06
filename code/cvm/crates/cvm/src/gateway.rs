use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram {
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
		tip: String,
	},
}