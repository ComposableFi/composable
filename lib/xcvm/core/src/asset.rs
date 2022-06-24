use alloc::{collections::BTreeMap, string::ToString};
use codec::{Decode, Encode};
use fixed::{types::extra::U16, FixedU128};
use scale_info::TypeInfo;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
pub struct XCVMAsset(pub u32);

impl XCVMAsset {
	pub const PICA: XCVMAsset = XCVMAsset(1);
	pub const ETH: XCVMAsset = XCVMAsset(2);
	pub const USDT: XCVMAsset = XCVMAsset(3);
	pub const USDC: XCVMAsset = XCVMAsset(4);

	pub const UST: XCVMAsset = XCVMAsset(0xDEADC0DE);
}

impl From<XCVMAsset> for u32 {
	fn from(val: XCVMAsset) -> Self {
		val.0
	}
}

impl From<u32> for XCVMAsset {
	fn from(asset: u32) -> Self {
		XCVMAsset(asset)
	}
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
					u32::MAX as u128,
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
pub struct XCVMTransfer(pub BTreeMap<XCVMAsset, Amount>);

impl XCVMTransfer {
	pub fn empty() -> Self {
		XCVMTransfer(BTreeMap::new())
	}
}

impl<U, V> From<BTreeMap<U, V>> for XCVMTransfer
where
	U: Into<XCVMAsset>,
	V: Into<Amount>,
{
	fn from(assets: BTreeMap<U, V>) -> Self {
		XCVMTransfer(
			assets
				.into_iter()
				.map(|(asset, amount)| (asset.into(), amount.into()))
				.collect(),
		)
	}
}

impl<U, V, const K: usize> From<[(U, V); K]> for XCVMTransfer
where
	U: Into<XCVMAsset>,
	V: Into<Amount>,
{
	fn from(x: [(U, V); K]) -> Self {
		XCVMTransfer(x.into_iter().map(|(asset, amount)| (asset.into(), amount.into())).collect())
	}
}

impl From<XCVMTransfer> for BTreeMap<u32, Amount> {
	fn from(XCVMTransfer(assets): XCVMTransfer) -> Self {
		assets.into_iter().map(|(XCVMAsset(asset), amount)| (asset, amount)).collect()
	}
}
