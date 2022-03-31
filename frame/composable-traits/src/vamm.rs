use std::fmt::Debug;

use codec::{FullCodec, MaxEncodedLen};
use frame_support::dispatch::DispatchError;
use scale_info::TypeInfo;
use sp_runtime::FixedPointNumber;

pub trait VirtualAMM {
	/// The unique identifier for a vAMM instance
	type VammId: FullCodec + MaxEncodedLen + TypeInfo;
	/// Parameters for creating and initializing a new vAMM instance. May be used in extrinsic
	/// signatures
	type VammParams: FullCodec + MaxEncodedLen + TypeInfo + Debug + Clone + PartialEq;
	/// Signed fixed point number implementation
	type Decimal: FixedPointNumber;

	/// Create a new virtual AMM and return its id
	fn create(info: Self::VammParams) -> Result<Self::VammId, DispatchError>;

	/// Compute the time-weighted average price of a virtual AMM
	fn get_twap(vamm: &Self::VammId) -> Result<Self::Decimal, DispatchError>;
}
