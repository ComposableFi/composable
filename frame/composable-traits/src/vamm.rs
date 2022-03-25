use std::fmt::Debug;

use codec::{FullCodec, MaxEncodedLen};
use frame_support::dispatch::DispatchError;
use scale_info::TypeInfo;

pub trait VirtualAMM {
	// TODO(0xangelo) remove Default trait bound
	/// The unique identifier for a vAMM instance
	type VammId: FullCodec + MaxEncodedLen + TypeInfo + Default;
	/// Parameters for creating and initializing a new vAMM instance. May be used in extrinsic
	/// signatures
	type VammParams: FullCodec + MaxEncodedLen + TypeInfo + Debug + Clone + PartialEq;

	fn create(info: Self::VammParams) -> Result<Self::VammId, DispatchError>;
}
