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
#[repr(i32)]
pub enum BridgeSecurity {
	None,
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
#[repr(u8)]
pub enum BridgeProtocol {
	IBC,
	XCM,
	// TODO(aeryz): Spec defines (OTP bytes BridgeSecurity), is this "bytes",
	// is used to identify the protocol?
	OTP,
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
pub struct Bridge {
	pub protocol: BridgeProtocol,
	pub security: BridgeSecurity,
}
