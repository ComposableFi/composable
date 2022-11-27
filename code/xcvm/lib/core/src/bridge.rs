use crate::{NetworkId, UserOrigin};
use alloc::vec::Vec;
use codec::{Decode, Encode};
use core::cmp::Ordering;
use cosmwasm_std::Addr;
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

// Keep the ordering explicit.
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
		match self {
			BridgeProtocol::IBC => Ok(()),
			BridgeProtocol::XCM => Ok(()),
			BridgeProtocol::OTP { security: otp_security, .. } if *otp_security >= security =>
				Ok(()),
			_ => Err(()),
		}
	}
}

/// The Origin that executed the XCVM operation.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum CallOrigin {
	Remote { protocol: BridgeProtocol, relayer: Addr, user_origin: UserOrigin },
	Local { user: Addr },
}

impl CallOrigin {
	/// Extract the user from a [`CallOrigin`].
	/// No distinction is done for local or remote user in this case.
	pub fn user(&self, current_network: NetworkId) -> UserOrigin {
		match self {
			CallOrigin::Remote { user_origin, .. } => user_origin.clone(),
			CallOrigin::Local { user } =>
				UserOrigin { network_id: current_network, user_id: user.as_bytes().to_vec().into() },
		}
	}

	/// The relayer.
	pub fn relayer(&self) -> &Addr {
		match self {
			CallOrigin::Remote { relayer, .. } => relayer,
			CallOrigin::Local { user } => user,
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
