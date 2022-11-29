use crate::UserOrigin;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use core::cmp::Ordering;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

/// Security associated with a bridge.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
#[repr(u8)]
pub enum BridgeSecurity {
	Insecure = 0,
	Optimistic = 1,
	Probabilistic = 2,
	Deterministic = 3,
}

impl PartialOrd for BridgeSecurity {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(match (self, other) {
			(BridgeSecurity::Insecure, BridgeSecurity::Insecure) => Ordering::Equal,
			(BridgeSecurity::Insecure, BridgeSecurity::Deterministic) => Ordering::Less,
			(BridgeSecurity::Insecure, BridgeSecurity::Probabilistic) => Ordering::Less,
			(BridgeSecurity::Insecure, BridgeSecurity::Optimistic) => Ordering::Less,
			(BridgeSecurity::Deterministic, BridgeSecurity::Insecure) => Ordering::Greater,
			(BridgeSecurity::Deterministic, BridgeSecurity::Deterministic) => Ordering::Equal,
			(BridgeSecurity::Deterministic, BridgeSecurity::Probabilistic) => Ordering::Greater,
			(BridgeSecurity::Deterministic, BridgeSecurity::Optimistic) => Ordering::Greater,
			(BridgeSecurity::Probabilistic, BridgeSecurity::Insecure) => Ordering::Greater,
			(BridgeSecurity::Probabilistic, BridgeSecurity::Deterministic) => Ordering::Less,
			(BridgeSecurity::Probabilistic, BridgeSecurity::Probabilistic) => Ordering::Equal,
			(BridgeSecurity::Probabilistic, BridgeSecurity::Optimistic) => Ordering::Greater,
			(BridgeSecurity::Optimistic, BridgeSecurity::Insecure) => Ordering::Greater,
			(BridgeSecurity::Optimistic, BridgeSecurity::Deterministic) => Ordering::Less,
			(BridgeSecurity::Optimistic, BridgeSecurity::Probabilistic) => Ordering::Less,
			(BridgeSecurity::Optimistic, BridgeSecurity::Optimistic) => Ordering::Equal,
		})
	}
}

/// Unique identity of a bridge in a given chain, usually a public key.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
pub struct BridgeId(Vec<u8>);

/// Protocol used to bridge call/funds.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum BridgeProtocol {
	IBC,
	XCM,
	OTP { id: BridgeId, security: BridgeSecurity },
}

impl BridgeProtocol {
	/// Ensure that a protocol enforce the minimum requested security.
	pub fn ensure_security(&self, security: BridgeSecurity) -> Result<(), ()> {
		let has = match (self, security) {
			(BridgeProtocol::IBC, _) => true,
			(BridgeProtocol::XCM, _) => true,
			(BridgeProtocol::OTP { security: otp_security, .. }, security) =>
				*otp_security >= security,
		};
		if has {
			Ok(())
		} else {
			Err(())
		}
	}
}

/// The Origin that executed the XCVM operation.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
pub enum CallOrigin {
	Remote { protocol: BridgeProtocol, relayer: Vec<u8>, user_origin: UserOrigin },
	Local { user_origin: UserOrigin },
}

impl CallOrigin {
	/// Extract the user from a [`CallOrigin`].
	/// No distinction is done for local or remote user in this case.
	pub fn user(&self) -> &UserOrigin {
		match self {
			CallOrigin::Remote { user_origin, .. } => user_origin,
			CallOrigin::Local { user_origin } => user_origin,
		}
	}

	/// Ensure that the call origin meet the security requirement.
	/// If the call is originating from the local chain, it is considered trusted.
	pub fn ensure_security(&self, security: BridgeSecurity) -> Result<(), ()> {
		match self {
			CallOrigin::Remote { protocol, .. } => protocol.ensure_security(security),
			CallOrigin::Local { .. } => Ok(()),
		}
	}
}
