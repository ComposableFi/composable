use crate::prelude::*;

use crate::abstraction::IndexOf;
use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

/// The interpreter origin, composite of a user origin and a salt.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
pub struct InterpreterOrigin {
	pub user_origin: UserOrigin,
	#[serde(with = "hex")]
	#[cfg_attr(feature = "std", schemars(with = "String"))]
	pub salt: Vec<u8>,
}

impl Display for InterpreterOrigin {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let salt = hex::encode(self.salt.as_slice());
		core::write!(fmtr, "{}-{salt}", self.user_origin)
	}
}

/// The origin of a user, which consist of the composite, origin network and origin network user id.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
pub struct UserOrigin {
	pub network_id: NetworkId,
	pub user_id: UserId,
}

impl Display for UserOrigin {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::write!(fmtr, "{}-{}", self.network_id, self.user_id)
	}
}

/// Arbitrary `User` type that represent the identity of a user on a given network, usually a public
/// key.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct UserId(
	#[serde(with = "hex")]
	#[cfg_attr(feature = "std", schemars(with = "String"))]
	pub Vec<u8>,
);

impl Display for UserId {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		hex::encode(self.0.as_slice()).fmt(fmtr)
	}
}

impl From<Vec<u8>> for UserId {
	fn from(x: Vec<u8>) -> Self {
		Self(x)
	}
}

impl From<UserId> for Vec<u8> {
	fn from(UserId(x): UserId) -> Self {
		x
	}
}

impl AsRef<[u8]> for UserId {
	fn as_ref(&self) -> &[u8] {
		self.0.as_ref()
	}
}

/// Newtype for XCVM networks ID. Must be unique for each network and must never change.
/// This ID is an opaque, arbitrary type from the XCVM protocol and no assumption must be made on
/// how it is computed.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Debug,
	Encode,
	Decode,
	TypeInfo,
	Serialize,
	Deserialize,
)]
#[repr(transparent)]
pub struct NetworkId(pub u32);

impl Display for NetworkId {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		<u32 as Display>::fmt(&self.0, f)
	}
}

impl From<u32> for NetworkId {
	fn from(x: u32) -> Self {
		NetworkId(x)
	}
}

impl From<NetworkId> for u32 {
	fn from(network_id: NetworkId) -> Self {
		network_id.0
	}
}

impl From<Picasso> for NetworkId {
	fn from(_: Picasso) -> Self {
		Picasso::ID
	}
}

impl From<Ethereum> for NetworkId {
	fn from(_: Ethereum) -> Self {
		Ethereum::ID
	}
}

impl From<Centauri> for NetworkId {
	fn from(_: Centauri) -> Self {
		Centauri::ID
	}
}

/// The index 0 must not be used for safety purpose, we hence introduce an invalid network at this
/// index.
pub struct InvalidNetwork;
/// Composable Picasso (Kusama parachain)
pub struct Picasso;
/// Centauri (Cosmos) mainnet
pub struct Centauri;
/// Ethereum mainnet
pub struct Ethereum;
pub struct CosmosHub;
pub struct Osmosis;
pub struct Composable;

/// List of networks supported by XCVM.
// /!\ The order matters and must not be changed, adding a network on the right is safe.
pub type Networks = (InvalidNetwork, (Picasso, (Centauri, (Ethereum, ()))));

/// Type implement network must be part of [`Networks`], otherwise invalid.
pub trait Network {
	const ID: NetworkId;
	type EncodedCall;
}

impl Network for Picasso {
	const ID: NetworkId = NetworkId(<Networks as IndexOf<Self, _>>::INDEX);
	type EncodedCall = Vec<u8>;
}

impl Network for Ethereum {
	const ID: NetworkId = NetworkId(<Networks as IndexOf<Self, _>>::INDEX);
	type EncodedCall = Vec<u8>;
}

impl Network for Centauri {
	const ID: NetworkId = NetworkId(<Networks as IndexOf<Self, _>>::INDEX);
	type EncodedCall = Vec<u8>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn network_ids() {
		assert_eq!(Picasso::ID, NetworkId(1));
		assert_eq!(Centauri::ID, NetworkId(2));
		assert_eq!(Ethereum::ID, NetworkId(3));
	}

	#[test]
	fn test_serialisation() {
		#[track_caller]
		fn check<T>(want: &str, value: T)
		where
			T: for<'a> Deserialize<'a> + Serialize + core::fmt::Debug + PartialEq,
		{
			let serialised = serde_json_wasm::to_string(&value).unwrap();
			assert_eq!(want, serialised);
			let deserialised = serde_json_wasm::from_str::<T>(&serialised).unwrap();
			assert_eq!(value, deserialised);
		}

		check("42", NetworkId(42));
		check("\"616c696365\"", UserId(b"alice".to_vec()));
	}
}
