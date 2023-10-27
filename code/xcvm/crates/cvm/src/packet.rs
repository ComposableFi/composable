use crate::prelude::*;

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
#[serde(rename_all = "snake_case")]
pub struct Packet<Program> {
	/// The interpreter that was the origin of this packet.
	#[serde(with = "hex")]
	#[cfg_attr(feature = "json-schema", schemars(with = "String"))]
	pub interpreter: Vec<u8>,
	/// The user that originated the first XCVM call.
	pub user_origin: crate::network::UserOrigin,
	/// The salt associated with the program.
	#[serde(with = "hex")]
	#[cfg_attr(feature = "json-schema", schemars(with = "String"))]
	pub salt: Vec<u8>,
	/// The protobuf encoded program.
	pub program: Program,
	/// The assets that were attached to the program.
	pub assets: crate::asset::Funds<crate::shared::Displayed<u128>>,
}
