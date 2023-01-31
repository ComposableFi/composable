use crate::abstraction::IndexOf;
use alloc::{string::ToString, vec::Vec};
use codec::{Decode, Encode};
use core::ops::Add;
use fixed::{types::extra::U16, FixedU128};
use num::Zero;
use scale_info::TypeInfo;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
pub struct AssetId(pub Displayed<u128>);

impl From<AssetId> for u128 {
	fn from(val: AssetId) -> Self {
		val.0 .0
	}
}

impl From<u128> for AssetId {
	fn from(asset: u128) -> Self {
		AssetId(Displayed(asset))
	}
}

impl From<PICA> for AssetId {
	fn from(_: PICA) -> Self {
		PICA::ID
	}
}

impl From<ETH> for AssetId {
	fn from(_: ETH) -> Self {
		ETH::ID
	}
}

impl From<USDT> for AssetId {
	fn from(_: USDT) -> Self {
		USDT::ID
	}
}

impl From<USDC> for AssetId {
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
// /!\ The order matters and must not be changed, adding a network on the right is safe.
pub type Assets = (InvalidAsset, (PICA, (ETH, (USDT, (USDC, ())))));

/// Type implement network must be part of [`Networks`], otherwise invalid.
pub trait Asset {
	const ID: AssetId;
}

impl Asset for PICA {
	const ID: AssetId = AssetId(Displayed(<Assets as IndexOf<Self, _>>::INDEX as u128));
}

impl Asset for ETH {
	const ID: AssetId = AssetId(Displayed(<Assets as IndexOf<Self, _>>::INDEX as u128));
}

impl Asset for USDT {
	const ID: AssetId = AssetId(Displayed(<Assets as IndexOf<Self, _>>::INDEX as u128));
}

impl Asset for USDC {
	const ID: AssetId = AssetId(Displayed(<Assets as IndexOf<Self, _>>::INDEX as u128));
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
pub struct Balance {
	pub amount: Amount,
	pub is_unit: bool,
}

impl Balance {
	#[inline]
	pub fn new(amount: Amount, is_unit: bool) -> Self {
		Self { amount, is_unit }
	}
}

pub const MAX_PARTS: u128 = 1000000000000000000;

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
/// See https://en.wikipedia.org/wiki/Linear_equation#Slope%E2%80%93intercept_form_or_Gradient-intercept_form
pub struct Amount {
	pub intercept: Displayed<u128>,
	pub slope: Displayed<u128>,
}

impl Amount {
	#[inline]
	pub fn new(intercept: u128, slope: u128) -> Self {
		Self { intercept: Displayed(intercept), slope: Displayed(slope) }
	}

	/// An absolute amount
	#[inline]
	pub fn absolute(value: u128) -> Self {
		Self { intercept: Displayed(value), slope: Displayed(0) }
	}

	/// A ratio amount, expressed in u128 parts (x / u128::MAX)
	#[inline]
	pub fn ratio(parts: u128) -> Self {
		Self { intercept: Displayed(0), slope: Displayed(parts) }
	}

	/// Helper function to see if the amount is absolute
	#[inline]
	pub fn is_absolute(&self) -> bool {
		self.slope.0 == 0
	}

	/// Helper function to see if the amount is ratio
	#[inline]
	pub fn is_ratio(&self) -> bool {
		self.intercept.0 == 0
	}
}

impl Add for Amount {
	type Output = Self;

	#[inline]
	fn add(self, Self { intercept: Displayed(i_1), slope: Displayed(s_1) }: Self) -> Self::Output {
		let Self { intercept: Displayed(i_0), slope: Displayed(s_0) } = self;
		Self {
			intercept: Displayed(i_0.saturating_add(i_1)),
			slope: Displayed(s_0.saturating_add(s_1)),
		}
	}
}

impl Zero for Amount {
	#[inline]
	fn zero() -> Self {
		Self { intercept: Displayed(0), slope: Displayed(0) }
	}

	#[inline]
	fn is_zero(&self) -> bool {
		self == &Self::zero()
	}
}

impl From<u128> for Amount {
	#[inline]
	fn from(x: u128) -> Self {
		Self { intercept: Displayed(x), slope: Displayed(0) }
	}
}

impl Amount {
	/// `f(x) = a(x - b) + b where a = slope / MAX_PARTS, b = intercept`
	#[inline]
	pub fn apply(&self, value: u128) -> u128 {
		let amount = if self.slope.0 == 0 {
			self.intercept.0
		} else {
			FixedU128::<U16>::wrapping_from_num(value)
				.saturating_sub(FixedU128::<U16>::wrapping_from_num(self.intercept.0))
				.saturating_mul(
					FixedU128::<U16>::wrapping_from_num(self.slope.0)
						.saturating_div(FixedU128::<U16>::wrapping_from_num(MAX_PARTS)),
				)
				.wrapping_to_num::<u128>()
				.saturating_add(self.intercept.0)
		};
		u128::min(value, amount)
	}

	/// `f(x) = a + b * 10 ^ decimals where a = intercept, b = slope / MAX_PARTS`
	#[inline]
	pub fn apply_with_decimals(&self, decimals: u8, value: u128) -> u128 {
		let amount = FixedU128::<U16>::wrapping_from_num(self.intercept.0)
			.saturating_add(
				FixedU128::<U16>::wrapping_from_num(self.slope.0)
					.saturating_div(FixedU128::<U16>::wrapping_from_num(MAX_PARTS)),
			)
			.saturating_mul(FixedU128::<U16>::wrapping_from_num(10_u128.pow(decimals as u32)))
			.wrapping_to_num::<u128>();

		u128::min(value, amount)
	}
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct Funds<T = Amount>(pub Vec<(AssetId, T)>);

impl<T> IntoIterator for Funds<T> {
	type Item = <Vec<(AssetId, T)> as IntoIterator>::Item;
	type IntoIter = <Vec<(AssetId, T)> as IntoIterator>::IntoIter;
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<T> Funds<T> {
	#[inline]
	pub fn empty() -> Self {
		Funds(Vec::new())
	}
}

impl<T, U, V> From<Vec<(U, V)>> for Funds<T>
where
	U: Into<AssetId>,
	V: Into<T>,
{
	#[inline]
	fn from(assets: Vec<(U, V)>) -> Self {
		Funds(
			assets
				.into_iter()
				.map(|(asset, amount)| (asset.into(), amount.into()))
				.collect(),
		)
	}
}

impl<T, U, V, const K: usize> From<[(U, V); K]> for Funds<T>
where
	U: Into<AssetId>,
	V: Into<T>,
{
	#[inline]
	fn from(x: [(U, V); K]) -> Self {
		Funds(x.into_iter().map(|(asset, amount)| (asset.into(), amount.into())).collect())
	}
}

impl<T> From<Funds<T>> for Vec<(AssetId, T)> {
	#[inline]
	fn from(Funds(assets): Funds<T>) -> Self {
		assets
	}
}

impl<T> From<Funds<T>> for Vec<(u128, T)> {
	#[inline]
	fn from(Funds(assets): Funds<T>) -> Self {
		assets
			.into_iter()
			.map(|(AssetId(Displayed(asset)), amount)| (asset, amount))
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn asset_ids() {
		assert_eq!(PICA::ID, AssetId::from(1));
		assert_eq!(ETH::ID, AssetId::from(2));
		assert_eq!(USDT::ID, AssetId::from(3));
		assert_eq!(USDC::ID, AssetId::from(4));
	}
}
