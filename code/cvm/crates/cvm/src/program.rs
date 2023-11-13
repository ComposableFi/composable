use crate::prelude::*;

#[cfg(feature = "scale")]
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "scale")]
use scale_info::TypeInfo;

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "scale", derive(TypeInfo, Encode, Decode))]
#[serde(rename_all = "snake_case")]
pub struct Program<Instructions> {
	/// In JSON, hex encoded identifiers to identify the program off chain (for example in
	/// indexer).
	#[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
	#[cfg_attr(feature = "json-schema", schemars(schema_with = "String::json_schema"))]
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub tag: Vec<u8>,
	pub instructions: Instructions,
}
