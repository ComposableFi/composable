use codec::{Decode, Encode, MaxEncodedLen};
use composable_traits::vamm::VirtualAMM;
use scale_info::TypeInfo;

pub struct Vamm;
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, PartialEq)]
pub struct VammParams;

impl VirtualAMM for Vamm {
	type VammId = u64;
	type VammParams = VammParams;
}
