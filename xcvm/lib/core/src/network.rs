use crate::abstraction::IndexOf;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

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
pub struct NetworkID(pub u8);

impl From<u8> for NetworkID {
	fn from(x: u8) -> Self {
		NetworkID(x)
	}
}

impl From<Picasso> for NetworkID {
	fn from(_: Picasso) -> Self {
		Picasso::ID
	}
}

impl From<Ethereum> for NetworkID {
	fn from(_: Ethereum) -> Self {
		Ethereum::ID
	}
}

impl From<Juno> for NetworkID {
	fn from(_: Juno) -> Self {
		Juno::ID
	}
}

/// The index 0 must not be used for safety purpose, we hence introduce an invalid network at this
/// index.
pub struct InvalidNetwork;
/// Composable Picasso (Kusama parachain)
pub struct Picasso;
/// Ethereum mainnet
pub struct Ethereum;
/// Juno (Cosmos) mainnet
pub struct Juno;

/// List of networks supported by XCVM.
/// The order matter and must not be changed, adding a network on the right is safe.
pub type Networks = (InvalidNetwork, (Picasso, (Ethereum, (Juno, ()))));

/// Type implement network must be part of [`Networks`], otherwise invalid.
pub trait Network {
	const ID: NetworkID;
	type EncodedCall;
}

impl Network for Picasso {
	const ID: NetworkID = NetworkID(<Networks as IndexOf<Self, _>>::INDEX);
	type EncodedCall = Vec<u8>;
}

impl Network for Ethereum {
	const ID: NetworkID = NetworkID(<Networks as IndexOf<Self, _>>::INDEX);
	type EncodedCall = Vec<u8>;
}

impl Network for Juno {
	const ID: NetworkID = NetworkID(<Networks as IndexOf<Self, _>>::INDEX);
	type EncodedCall = Vec<u8>;
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn network_ids() {
		assert_eq!(Picasso::ID, NetworkID(1u8));
		assert_eq!(Ethereum::ID, NetworkID(2u8));
		assert_eq!(Juno::ID, NetworkID(3u8));
	}
}
