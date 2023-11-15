use crate::{asset::Funds, prelude::*};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram(ExecuteProgramMsg),
}

/// Definition of a program to be executed including its context.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct ExecuteProgramMsg<Assets = Option<Funds<crate::shared::Displayed<u128>>>> {
	/// The program salt.
	/// If JSON, than hex encoded non prefixed lower case string.
	/// If not specified, uses no salt.
	#[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
	#[cfg_attr(feature = "json-schema", schemars(schema_with = "String::json_schema"))]
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub salt: Vec<u8>,
	/// The program.
	pub program: crate::shared::XcProgram,
	/// Assets to fund the CVM interpreter instance.
	/// The interpreter is funded prior to execution.
	/// If None, 100% of received funds go to interpreter.
	pub assets: Assets,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub tip: Option<String>,
}
