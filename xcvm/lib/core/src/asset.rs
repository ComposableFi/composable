use alloc::{collections::BTreeMap, string::ToString};
use codec::{Decode, Encode};
use fixed::{types::extra::U16, FixedU128};
use scale_info::TypeInfo;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::abstraction::IndexOf;

/// Newtype for XCVM assets ID. Must be unique for each asset and must never change.
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
#[repr(transparent)]
pub struct AssetID(pub u32);

impl From<AssetID> for u32 {
	fn from(val: AssetID) -> Self {
		val.0
	}
}

impl From<u32> for AssetID {
	fn from(asset: u32) -> Self {
		AssetID(asset)
	}
}

impl From<PICA> for AssetID {
	fn from(_: PICA) -> Self {
		PICA::ID
	}
}

impl From<ETH> for AssetID {
	fn from(_: ETH) -> Self {
		ETH::ID
	}
}

impl From<USDT> for AssetID {
	fn from(_: USDT) -> Self {
		USDT::ID
	}
}

impl From<USDC> for AssetID {
	fn from(_: USDC) -> Self {
		USDC::ID
	}
}

pub struct InvalidAsset;
pub struct PICA;
pub struct ETH;
pub struct USDT;
pub struct USDC;

/// List of XCVM compatible assets.
/// The order matter and must not be changed, adding a network on the right is safe.
pub type Assets = (InvalidAsset, (PICA, (ETH, (USDT, (USDC, ())))));

/// Type implement network must be part of [`Networks`], otherwise invalid.
pub trait Asset {
	const ID: AssetID;
}

impl Asset for PICA {
	const ID: AssetID = AssetID(<Assets as IndexOf<Self, _>>::INDEX as u32);
}

impl Asset for ETH {
	const ID: AssetID = AssetID(<Assets as IndexOf<Self, _>>::INDEX as u32);
}

impl Asset for USDT {
	const ID: AssetID = AssetID(<Assets as IndexOf<Self, _>>::INDEX as u32);
}

impl Asset for USDC {
	const ID: AssetID = AssetID(<Assets as IndexOf<Self, _>>::INDEX as u32);
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct Displayed<T>(
	#[serde(bound(serialize = "T: core::fmt::Display"))]
	#[serde(serialize_with = "serialize_as_string")]
	#[serde(bound(deserialize = "T: core::str::FromStr"))]
	#[serde(deserialize_with = "deserialize_from_string")]
	pub T,
);

fn serialize_as_string<S: Serializer, T: core::fmt::Display>(
	t: &T,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	serializer.serialize_str(&t.to_string())
}

fn deserialize_from_string<'de, D: Deserializer<'de>, T: core::str::FromStr>(
	deserializer: D,
) -> Result<T, D::Error> {
	let s = alloc::string::String::deserialize(deserializer)?;
	s.parse::<T>().map_err(|_| serde::de::Error::custom("Parse from string failed"))
}

impl<T> From<T> for Displayed<T> {
	fn from(x: T) -> Self {
		Displayed(x)
	}
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Amount {
	Fixed(Displayed<u128>),
	Ratio(u32),
}

impl From<u128> for Amount {
	fn from(x: u128) -> Self {
		Amount::Fixed(Displayed(x))
	}
}

impl Amount {
	pub fn apply(&self, value: u128) -> u128 {
		match self {
			Amount::Fixed(Displayed(x)) => *x,
			Amount::Ratio(x) => FixedU128::<U16>::from_num(value)
				.saturating_mul(FixedU128::<U16>::from_num(*x as u128).saturating_div(FixedU128::<
					U16,
				>::from_num(
					u32::MAX
				)))
				.to_num(),
		}
	}
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct Funds<T = Amount>(pub BTreeMap<AssetID, T>);

impl Funds {
	pub fn empty() -> Self {
		Funds(BTreeMap::new())
	}
}

impl<U, V> From<BTreeMap<U, V>> for Funds
where
	U: Into<AssetID>,
	V: Into<Amount>,
{
	fn from(assets: BTreeMap<U, V>) -> Self {
		Funds(
			assets
				.into_iter()
				.map(|(asset, amount)| (asset.into(), amount.into()))
				.collect(),
		)
	}
}

impl<U, V, const K: usize> From<[(U, V); K]> for Funds
where
	U: Into<AssetID>,
	V: Into<Amount>,
{
	fn from(x: [(U, V); K]) -> Self {
		Funds(x.into_iter().map(|(asset, amount)| (asset.into(), amount.into())).collect())
	}
}

impl From<Funds> for BTreeMap<u32, Amount> {
	fn from(Funds(assets): Funds) -> Self {
		assets.into_iter().map(|(AssetID(asset), amount)| (asset, amount)).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn asset_ids() {
		assert_eq!(PICA::ID, AssetID(1));
		assert_eq!(ETH::ID, AssetID(2));
		assert_eq!(USDT::ID, AssetID(3));
		assert_eq!(USDC::ID, AssetID(4));
	}
}
