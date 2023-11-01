use crate::{network::NetworkId, prelude::*};

#[cfg(feature = "cw-storage-plus")]
use cw_storage_plus::{Key, Prefixer};

use crate::shared::Displayed;
use core::ops::Add;
use num::Zero;

#[cfg(feature = "scale")]
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "scale")]
use scale_info::TypeInfo;

use serde::{Deserialize, Serialize};

/// Newtype for CVM assets ID. Must be unique for each asset and must never change.
/// This ID is an opaque, arbitrary type from the CVM protocol and no assumption must be made on
/// how it is computed.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
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

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
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

impl From<(u64, u64)> for Balance {
	fn from(value: (u64, u64)) -> Self {
		Balance { amount: Amount::from(value), is_unit: false }
	}
}

impl From<u128> for Balance {
	fn from(value: u128) -> Self {
		Self { amount: Amount::absolute(value), is_unit: false }
	}
}

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
#[serde(rename_all = "snake_case")]
/// See https://en.wikipedia.org/wiki/Linear_equation#Slope%E2%80%93intercept_form_or_Gradient-intercept_form
pub struct Amount {
	pub intercept: Displayed<u128>,
	pub slope: Displayed<u64>,
}

/// Arithmetic errors.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
pub enum ArithmeticError {
	/// Underflow.
	Underflow,
	/// Overflow.
	Overflow,
	/// Division by zero.
	DivisionByZero,
}

impl From<(u64, u64)> for Amount {
	fn from(value: (u64, u64)) -> Self {
		Self::new(0, (value.0 as u128 * Self::MAX_PARTS as u128 / value.1 as u128) as u64)
	}
}

impl Amount {
	pub const MAX_PARTS: u64 = 1_000_000_000_000_000_000;

	pub const fn new(intercept: u128, slope: u64) -> Self {
		Self { intercept: Displayed(intercept), slope: Displayed(slope) }
	}

	/// An absolute amount
	pub const fn absolute(value: u128) -> Self {
		Self { intercept: Displayed(value), slope: Displayed(0) }
	}

	/// A ratio amount, expressed in parts (x / MAX_PARTS)
	pub const fn ratio(parts: u64) -> Self {
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
			let value =
				value.checked_sub(self.intercept.0.into()).ok_or(ArithmeticError::Underflow)?;
			let value = value
				.checked_mul(self.slope.0.into())
				.ok_or(ArithmeticError::Underflow)?
				.checked_div(Self::MAX_PARTS.into())
				.ok_or(ArithmeticError::Overflow)?;
			let value =
				value.checked_add(self.intercept.0.into()).ok_or(ArithmeticError::Overflow)?;
			value
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
			let value = self.intercept.0;
			let value = value
				.checked_add(
					u128::one()
						.checked_mul(self.slope.0.into())
						.ok_or(ArithmeticError::Overflow)?
						.checked_div(Self::MAX_PARTS.into())
						.ok_or(ArithmeticError::Overflow)?,
				)
				.ok_or(ArithmeticError::Overflow)?;
			let value = value
				.checked_mul(10_u128.pow(decimals as u32))
				.ok_or(ArithmeticError::Overflow)?;
			value
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
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "scale", derive(Encode, Decode, TypeInfo))]
#[repr(transparent)]
pub struct Funds<T = Balance>(pub Vec<(AssetId, T)>);

impl<T> Funds<T> {
	pub fn one<A: Into<T>>(id: AssetId, amount: A) -> Self {
		Self(vec![(id, amount.into())])
	}
}

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

/// see `generate_network_prefixed_id`
pub fn generate_asset_id(network_id: NetworkId, protocol_id: u32, nonce: u64) -> AssetId {
	AssetId::from(generate_network_prefixed_id(network_id, protocol_id, nonce))
}

// `protocol_id` - namespace like thing, default is 0, but can be used for example other consensus
// to create known ahead
/// `nonce` - local consensus atomic number, usually increasing monotonic increment
pub fn generate_network_prefixed_id(network_id: NetworkId, protocol_id: u32, nonce: u64) -> u128 {
	(u128::from(network_id.0) << 96) | (u128::from(protocol_id) << 64) | u128::from(nonce)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn amounts() {
		let amount = Amount::new(0, Amount::MAX_PARTS);
		let result = amount.apply(100).unwrap();
		assert_eq!(result, 100);

		let amount = Amount::new(42, Amount::MAX_PARTS);
		let result = amount.apply(100).unwrap();
		assert_eq!(result, 100);

		let amount = Amount::new(123, 0);
		let result = amount.apply(100).unwrap();
		assert_eq!(result, 100, "seems this is feature to ask more but return what is here");

		let amount = Amount::new(42, 0);
		let result = amount.apply(100).unwrap();
		assert_eq!(result, 42);

		let amount = Amount::new(50, Amount::MAX_PARTS / 10);
		let result = amount.apply(100).unwrap();
		assert_eq!(result, 50 + 5, "percentage of remaining");
	}

	#[test]
	fn devnet() {
		let pica_on_picasso = generate_asset_id(0.into(), 0, 1);
		assert_eq!(pica_on_picasso, 1.into());
		let pica_on_composable = generate_asset_id(1.into(), 0, 1);
		assert_eq!(pica_on_composable, 79228162514264337593543950337.into());
		let pica_on_centauri = generate_asset_id(2.into(), 0, 1);
		assert_eq!(pica_on_centauri, 158456325028528675187087900673.into());
		let pica_on_osmosis = generate_asset_id(3.into(), 0, 1);
		assert_eq!(pica_on_osmosis, 237684487542793012780631851009.into());

		let uosmo_on_centauri = generate_asset_id(2.into(), 0, 2);
		assert_eq!(uosmo_on_centauri, 158456325028528675187087900674.into());
		let uosmo_on_osmosis = generate_asset_id(3.into(), 0, 2);
		assert_eq!(uosmo_on_osmosis, 237684487542793012780631851010.into());

		let dot_on_centauri = generate_asset_id(2.into(), 0, 3);
		assert_eq!(dot_on_centauri, 158456325028528675187087900675.into());
		let dot_on_osmosis = generate_asset_id(3.into(), 0, 3);
		assert_eq!(dot_on_osmosis, 237684487542793012780631851011.into());

		let pica_uosmo_on_osmosis = generate_network_prefixed_id(3.into(), 100, 1);
		assert_eq!(pica_uosmo_on_osmosis, 237684489387467420151587012609);

		let dot_uosmo_on_osmosis = generate_network_prefixed_id(3.into(), 100, 2);
		assert_eq!(dot_uosmo_on_osmosis, 237684489387467420151587012610);
	}
}
