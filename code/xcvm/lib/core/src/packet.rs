use crate::{Displayed, Funds, UserOrigin};
use alloc::{vec, vec::Vec};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct XCVMAck(u8);

impl XCVMAck {
	pub const KO: XCVMAck = XCVMAck(0);
	pub const OK: XCVMAck = XCVMAck(1);
	pub fn into_vec(self) -> Vec<u8> {
		self.into()
	}
	pub fn value(self) -> u8 {
		self.0
	}
}

impl From<XCVMAck> for Vec<u8> {
	fn from(XCVMAck(x): XCVMAck) -> Self {
		vec![x]
	}
}

impl TryFrom<&[u8]> for XCVMAck {
	type Error = ();
	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		match value {
			[0] => Ok(XCVMAck::KO),
			[1] => Ok(XCVMAck::OK),
			_ => Err(()),
		}
	}
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Packet<Program> {
	/// The interpreter that was the origin of this packet.
	pub interpreter: Vec<u8>,
	/// The user that originated the first XCVM call.
	pub user_origin: UserOrigin,
	/// The salt associated with the program.
	pub salt: Vec<u8>,
	/// The protobuf encoded program.
	pub program: Program,
	/// The assets that were attached to the program.
	pub assets: Funds<Displayed<u128>>,
}
