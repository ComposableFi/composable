use core::str::FromStr;

use parity_scale_codec::{WrapperTypeEncode, EncodeLike, MaxEncodedLen, Decode, Encode};
use scale_info::TypeInfo;
use thiserror::Error;
use xcm::v3;

#[allow(clippy::large_enum_variant)]
#[derive(Debug,  Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ForeignAssetId {
	Xcm(VersionedMultiLocation),
	IbcIcs20(PrefixedDenom),
}

#[derive(
	Ord, PartialOrd, Debug,  Clone, PartialEq, Eq,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VersionedMultiLocation {
	V3(v3::MultiLocation),
}

impl From<VersionedMultiLocation> for ForeignAssetId {
	fn from(this: VersionedMultiLocation) -> Self {
		Self::Xcm(this)
	}
}

impl From<PrefixedDenom> for ForeignAssetId {
	fn from(this: PrefixedDenom) -> Self {
		Self::IbcIcs20(this)
	}
}

type InnerDenom = ibc_rs_scale::applications::transfer::PrefixedDenom;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(transparent))]
pub struct PrefixedDenom(pub InnerDenom);

#[derive(Debug, Error)]
pub enum Error {
    #[error("TokenTransferError")]
    TokenTransferError,
}

impl FromStr for PrefixedDenom {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Error> {
		InnerDenom::from_str(s)
			.map_err(|x| Error::TokenTransferError)
			.map(Self)
	}
}