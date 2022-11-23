use crate::{Displayed, Funds, UserOrigin};
use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Packet<Program> {
	/// The user that originated the first XCVM call.
	pub user_origin: UserOrigin,
	/// The salt associated with the program.
	pub salt: Vec<u8>,
	/// The protobuf encoded program.
	pub program: Program,
	/// The assets that were attached to the program.
	pub assets: Funds<Displayed<u128>>,
}
