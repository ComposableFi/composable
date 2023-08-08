use crate::prelude::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Program<Instructions> {
	/// If JSON, than hex encoded non prefixed lower case string.
	#[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
	#[cfg_attr(feature = "std", schemars(schema_with = "String::json_schema"))]
	pub tag: Vec<u8>,
	pub instructions: Instructions,
}
