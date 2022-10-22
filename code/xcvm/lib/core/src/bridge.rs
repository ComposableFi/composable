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
pub enum BridgeSecurity {
	Local = 0,
	Deterministic = 1,
	Probabilistic = 2,
	Optimistic = 3,
}

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
pub enum MessageOrigin {
	Local,
	IBC,
	XCM,
	// TODO(aeryz): Spec defines (OTP bytes BridgeSecurity), is this "bytes",
	// is used to identify the protocol?
	OTP(BridgeSecurity),
}

impl MessageOrigin {
	pub fn assert_security(&self, bridge_security: BridgeSecurity) -> Result<(), BridgeSecurity> {
		let security = self.security();
		if bridge_security <= security {
			Ok(())
		} else {
			Err(security)
		}
	}

	pub fn security(&self) -> BridgeSecurity {
		match self {
			MessageOrigin::Local => BridgeSecurity::Local,
			MessageOrigin::IBC | MessageOrigin::XCM => BridgeSecurity::Deterministic,
			MessageOrigin::OTP(bridge_security) => *bridge_security,
		}
	}
}
