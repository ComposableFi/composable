use std::fmt::Debug;

use codec::{FullCodec, MaxEncodedLen};
use scale_info::TypeInfo;

pub trait VirtualAMM {
	type VammId: FullCodec + MaxEncodedLen + TypeInfo;
	type VammParams: FullCodec + MaxEncodedLen + TypeInfo + Debug + Clone + PartialEq;
}
