use std::fmt::Debug;

use codec::{FullCodec, MaxEncodedLen};
use scale_info::TypeInfo;

pub trait VirtualAMM {
	/// The unique identifier for a vAMM instance
	type VammId: FullCodec + MaxEncodedLen + TypeInfo;
	/// Parameters for creating and initializing a new vAMM instance. May be used in extrinsic
	/// signatures
	type VammParams: FullCodec + MaxEncodedLen + TypeInfo + Debug + Clone + PartialEq;
}
