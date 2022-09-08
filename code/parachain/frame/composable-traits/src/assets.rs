//! Interfaces to managed assets
use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::collections::vec::bounded::BiBoundedVec;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BasicAssetMetadata {
	pub symbol: BiBoundedVec<u8, 1, 8>,
	pub name: BiBoundedVec<u8, 1, 32>,
}

impl BasicAssetMetadata {
	pub fn try_from(symbol: &[u8], name: &[u8]) -> Option<Self> {
		Some(Self {
			symbol: BiBoundedVec::try_from(symbol.to_vec()).ok()?,
			name: BiBoundedVec::try_from(name.to_vec()).ok()?,
		})
	}
}

#[derive(Decode, Encode, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Asset {
	pub name: Vec<u8>,
	pub id: u64,
}
