use crate::{prelude::*, NetworkId};

#[cfg(feature = "cw-storage-plus")]
use cw_storage_plus::{Key, Prefixer};

use core::ops::Add;
use cosmwasm_std::{Uint128, Uint256};
use num::Zero;
use parity_scale_codec::{Decode, Encode};
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
	Hash,
	Encode,
	Decode,
	TypeInfo,
	Serialize,
	Deserialize,
)]
#[repr(transparent)]
pub struct AssetId(pub Displayed<u128>);

impl core::fmt::Display for AssetId {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		self.0 .0.fmt(fmtr)
	}
}

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

#[cfg(feature = "cw-storage-plus")]
impl<'a> cw_storage_plus::PrimaryKey<'a> for AssetId {
	type Prefix = ();
	type SubPrefix = ();
	type Suffix = u128;
	type SuperSuffix = u128;

	fn key(&self) -> Vec<cw_storage_plus::Key> {
		use cw_storage_plus::IntKey;
		vec![cw_storage_plus::Key::Val128(self.0 .0.to_cw_bytes())]
	}
}

#[cfg(feature = "cw-storage-plus")]
impl<'a> Prefixer<'a> for AssetId {
	fn prefix(&self) -> Vec<Key> {
		use cw_storage_plus::IntKey;
		vec![Key::Val128(self.0 .0.to_cw_bytes())]
	}
}

#[cfg(feature = "cw-storage-plus")]
impl cw_storage_plus::KeyDeserialize for AssetId {
	type Output = <u128 as cw_storage_plus::KeyDeserialize>::Output;

	const KEY_ELEMS: u16 = 1;

	fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
		<u128 as cw_storage_plus::KeyDeserialize>::from_vec(value)
	}

	fn from_slice(value: &[u8]) -> cosmwasm_std::StdResult<Self::Output> {
		<u128 as cw_storage_plus::KeyDeserialize>::from_slice(value)
	}
}

/// A wrapper around a type which is serde-serialised as a string.
///
/// For serde-serialisation to be implemented for the type `T` must implement
/// `Display` and `FromStr` traits.
///
/// ```
/// # use xc_core::Displayed;
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct Foo {
///     value: Displayed<u64>
/// }
///
/// let encoded = serde_json_wasm::to_string(&Foo { value: Displayed(42) }).unwrap();
/// assert_eq!(r#"{"value":"42"}"#, encoded);
///
/// let decoded = serde_json_wasm::from_str::<Foo>(r#"{"value":"42"}"#).unwrap();
/// assert_eq!(Displayed(42), decoded.value);
/// ```
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, TypeInfo)]
#[repr(transparent)]
pub struct Displayed<T>(pub T);

impl<T: Default> Default for Displayed<T> {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<T> parity_scale_codec::WrapperTypeEncode for Displayed<T> {}
impl<T> parity_scale_codec::WrapperTypeDecode for Displayed<T> {
	type Wrapped = T;
}

impl From<Uint128> for Displayed<u128> {
	fn from(value: Uint128) -> Self {
		Self(value.u128())
	}
}

impl From<Displayed<u128>> for Uint128 {
	fn from(value: Displayed<u128>) -> Self {
		value.into()
	}
}

impl<T> core::ops::Deref for Displayed<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T: core::fmt::Display> core::fmt::Display for Displayed<T> {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		self.0.fmt(fmtr)
	}
}

impl<T: core::fmt::Display> Serialize for Displayed<T> {
	fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
		ser.collect_str(&self.0)
	}
}

impl<'de, T: core::str::FromStr> Deserialize<'de> for Displayed<T> {
	fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
		de.deserialize_str(DisplayedVisitor::<T>(Default::default()))
	}
}

/// Serde Visitor helper for deserialising [`Displayed`] type.
struct DisplayedVisitor<V>(core::marker::PhantomData<V>);

impl<'de, T: core::str::FromStr> serde::de::Visitor<'de> for DisplayedVisitor<T> {
	type Value = Displayed<T>;

	fn expecting(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
		fmt.write_str("a string")
	}

	fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
		T::from_str(s)
			.map(Displayed)
			.map_err(|_| serde::de::Error::custom("Parse from string failed"))
	}
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
	pub const fn new(amount: Amount, is_unit: bool) -> Self {
		Self { amount, is_unit }
	}
}

impl From<u128> for Balance {
	fn from(value: u128) -> Self {
		Balance { amount: Amount::absolute(value), is_unit: false }
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

/// Arithmetic errors.
#[derive(Eq, PartialEq, Clone, Copy, Encode, Decode, Debug, TypeInfo, Serialize, Deserialize)]
pub enum ArithmeticError {
	/// Underflow.
	Underflow,
	/// Overflow.
	Overflow,
	/// Division by zero.
	DivisionByZero,
}

impl Amount {
	pub const MAX_PARTS: u128 = 1000000000000000000;

	pub const fn new(intercept: u128, slope: u128) -> Self {
		Self { intercept: Displayed(intercept), slope: Displayed(slope) }
	}

	/// An absolute amount
	pub const fn absolute(value: u128) -> Self {
		Self { intercept: Displayed(value), slope: Displayed(0) }
	}

	/// A ratio amount, expressed in u128 parts (x / MAX_PARTS)
	pub const fn ratio(parts: u128) -> Self {
		Self { intercept: Displayed(0), slope: Displayed(parts) }
	}

	/// Helper function to see if the amount is absolute
	pub const fn is_absolute(&self) -> bool {
		self.slope.0 == 0
	}

	/// Helper function to see if the amount is ratio
	pub const fn is_ratio(&self) -> bool {
		self.intercept.0 == 0
	}

	/// Everything mean that we move 100% of whats left.
	pub const fn everything() -> Self {
		Self::ratio(Self::MAX_PARTS)
	}

	/// `f(x) = a(x - b) + b where a = slope / MAX_PARTS, b = intercept`
	pub fn apply(&self, value: u128) -> Result<u128, ArithmeticError> {
		if value.is_zero() {
			return Ok(0)
		}
		let amount = if self.slope.0.is_zero() {
			self.intercept.0
		} else if self.slope.0 == Self::MAX_PARTS {
			value
		} else {
			let value = Uint256::from(value)
				.checked_sub(self.intercept.0.into())
				.map_err(|_| ArithmeticError::Underflow)?;
			let value = value
				.checked_multiply_ratio(self.slope.0, Self::MAX_PARTS)
				.map_err(|_| ArithmeticError::Overflow)?;
			let value = value
				.checked_add(self.intercept.0.into())
				.map_err(|_| ArithmeticError::Overflow)?;
			Uint128::try_from(value).map_err(|_| ArithmeticError::Overflow)?.u128()
		};
		Ok(u128::min(value, amount))
	}

	/// `f(x) = (a + b) * 10 ^ decimals where a = intercept, b = slope / MAX_PARTS`
	pub fn apply_with_decimals(&self, decimals: u8, value: u128) -> Result<u128, ArithmeticError> {
		if value.is_zero() {
			return Ok(0)
		}
		let unit = 10_u128.checked_pow(decimals as u32).ok_or(ArithmeticError::Overflow)?;
		let amount = if self.slope.0.is_zero() {
			self.intercept.0.checked_mul(unit).ok_or(ArithmeticError::Overflow)?
		} else if self.slope.0 == Self::MAX_PARTS {
			value
		} else {
			let value = Uint256::from(self.intercept.0);
			let value = value
				.checked_add(
					Uint256::one()
						.checked_multiply_ratio(self.slope.0, Self::MAX_PARTS)
						.map_err(|_| ArithmeticError::Overflow)?,
				)
				.map_err(|_| ArithmeticError::Overflow)?;
			let value = value
				.checked_mul(Uint256::from(10_u128.pow(decimals as u32)))
				.map_err(|_| ArithmeticError::Overflow)?;
			Uint128::try_from(value).map_err(|_| ArithmeticError::Overflow)?.u128()
		};
		Ok(u128::min(value, amount))
	}
}

impl Add for Amount {
	type Output = Self;

	fn add(self, Self { intercept: Displayed(i_1), slope: Displayed(s_1) }: Self) -> Self::Output {
		let Self { intercept: Displayed(i_0), slope: Displayed(s_0) } = self;
		Self {
			intercept: Displayed(i_0.saturating_add(i_1)),
			slope: Displayed(s_0.saturating_add(s_1)),
		}
	}
}

impl Zero for Amount {
	fn zero() -> Self {
		Self { intercept: Displayed(0), slope: Displayed(0) }
	}

	fn is_zero(&self) -> bool {
		self == &Self::zero()
	}
}

impl From<u128> for Amount {
	fn from(x: u128) -> Self {
		Self::absolute(x)
	}
}

/// a set of assets with non zero balances
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct Funds<T = Balance>(pub Vec<(AssetId, T)>);

impl<T> Default for Funds<T> {
	fn default() -> Self {
		Self(Vec::new())
	}
}

impl<T> IntoIterator for Funds<T> {
	type Item = <Vec<(AssetId, T)> as IntoIterator>::Item;
	type IntoIter = <Vec<(AssetId, T)> as IntoIterator>::IntoIter;
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<T, U, V> From<Vec<(U, V)>> for Funds<T>
where
	U: Into<AssetId>,
	V: Into<T>,
{
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
	fn from(Funds(assets): Funds<T>) -> Self {
		assets
	}
}

impl<T> From<Funds<T>> for Vec<(u128, T)> {
	fn from(Funds(assets): Funds<T>) -> Self {
		assets
			.into_iter()
			.map(|(AssetId(Displayed(asset)), amount)| (asset, amount))
			.collect()
	}
}

pub fn generate_asset_id(network_id: NetworkId, protocol_id: u32, nonce: u64) -> AssetId {
	[
		network_id.0.to_be_bytes().to_vec(),
		protocol_id.to_be_bytes().to_vec(),
		nonce.to_be_bytes().to_vec(),
	]
	.concat()
	.try_into()
	.map(u128::from_be_bytes)
	.expect("[u8; 8] + bytes(u64) = [u8; 16]")
	.into()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn devnet() {
		let pica = generate_asset_id(0.into(), 0, 1);
		assert_eq!(pica, 1.into());
		let pica = generate_asset_id(1.into(), 0, 1);
		assert_eq!(pica, 79228162514264337593543950337.into());	
		let pica = generate_asset_id(2.into(), 0, 1);
		assert_eq!(pica, 158456325028528675187087900673.into());	
		let pica = generate_asset_id(3.into(), 0, 1);
		assert_eq!(pica, 237684487542793012780631851009.into());		

		let atom = generate_asset_id(2.into(), 0, 2);
		assert_eq!(atom, 158456325028528675187087900674.into());	
		let atom = generate_asset_id(3.into(), 0, 2);
		assert_eq!(atom, 237684487542793012780631851010.into());		
	}
}
