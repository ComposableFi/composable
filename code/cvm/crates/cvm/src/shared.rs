use crate::{asset::*, prelude::*};

#[cfg(feature = "cosmwasm")]
use cosmwasm_std::{from_json, to_json_binary, Binary, StdResult};

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};

pub type Salt = Vec<u8>;
/// absolute amounts
pub type XcFunds = Vec<(AssetId, Displayed<u128>)>;
// like `XcFunds`, but allow relative(percentages) amounts. Similar to assets filters in XCM
pub type XcBalanceFilter = crate::asset::Balance;
pub type XcFundsFilter = Funds<XcBalanceFilter>;
pub type XcInstruction = crate::instruction::Instruction<Vec<u8>, XcAddr, XcFundsFilter>;
pub type XcPacket = crate::packet::Packet<XcProgram>;
pub type XcProgram = crate::program::Program<Vec<XcInstruction>>;

#[cfg(feature = "cosmwasm")]
pub fn encode_base64<T: Serialize>(x: &T) -> StdResult<String> {
	Ok(to_json_binary(x)?.to_base64())
}

#[cfg(feature = "cosmwasm")]
pub fn decode_base64<S: AsRef<str>, T: DeserializeOwned>(encoded: S) -> StdResult<T> {
	from_json::<T>(&Binary::from_base64(encoded.as_ref())?)
}

/// A wrapper around any address in canonical form
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Deref, serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
pub struct XcAddr(Vec<u8>);

#[cfg(feature = "cosmwasm")]
impl core::fmt::Debug for XcAddr {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Debug::fmt(&self.0, fmtr)
	}
}

#[cfg(feature = "cosmwasm")]
impl From<XcAddr> for Vec<u8> {
	fn from(addr: XcAddr) -> Self {
		addr.0
	}
}

#[cfg(feature = "cosmwasm")]
impl From<Vec<u8>> for XcAddr {
	fn from(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}
}

#[cfg(feature = "cosmwasm")]
impl From<Binary> for XcAddr {
	fn from(bytes: Binary) -> Self {
		Self(bytes.0)
	}
}

#[cfg(all(feature = "cosmwasm", feature = "scale"))]
impl parity_scale_codec::Encode for XcAddr {
	fn size_hint(&self) -> usize {
		self.as_slice().size_hint()
	}

	fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, dest: &mut T) {
		self.as_slice().encode_to(dest)
	}
}

#[cfg(all(feature = "cosmwasm", feature = "scale"))]
impl parity_scale_codec::Decode for XcAddr {
	fn decode<I: parity_scale_codec::Input>(
		input: &mut I,
	) -> Result<Self, parity_scale_codec::Error> {
		Vec::<u8>::decode(input).map(|vec| Self(CanonicalAddr(Binary(vec))))
	}
}

#[cfg(all(feature = "cosmwasm", feature = "scale"))]
impl scale_info::TypeInfo for XcAddr {
	type Identity = <[u8] as scale_info::TypeInfo>::Identity;
	fn type_info() -> scale_info::Type {
		<[u8] as scale_info::TypeInfo>::type_info()
	}
}

/// A wrapper around a type which is serde-serialised as a string.
///
/// For serde-serialisation to be implemented for the type `T` must implement
/// `Display` and `FromStr` traits.
///
/// ```
/// # use cvm::shared::Displayed;
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
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "scale", derive(scale_info::TypeInfo))]
#[derive(
	Copy,
	Clone,
	Default,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
	derive_more::Deref,
	derive_more::From,
)]
#[repr(transparent)]
pub struct Displayed<T>(pub T);

impl<T: FromStr> FromStr for Displayed<T> {
	type Err = <T as FromStr>::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		T::from_str(s).map(Displayed)
	}
}

#[cfg(feature = "scale")]
impl<T> parity_scale_codec::WrapperTypeEncode for Displayed<T> {}
#[cfg(feature = "scale")]
impl<T> parity_scale_codec::WrapperTypeDecode for Displayed<T> {
	type Wrapped = T;
}

impl<T: core::fmt::Display> core::fmt::Display for Displayed<T> {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Display::fmt(&self.0, fmtr)
	}
}

impl<T: core::fmt::Display> core::fmt::Debug for Displayed<T> {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Display::fmt(&self.0, fmtr)
	}
}

impl<T: core::fmt::Display> serde::Serialize for Displayed<T> {
	fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
		ser.collect_str(&self.0)
	}
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Displayed<T>
where
	T: core::str::FromStr,
	<T as core::str::FromStr>::Err: core::fmt::Display,
{
	fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
		de.deserialize_str(DisplayedVisitor::<T>(Default::default()))
	}
}

/// Serde Visitor helper for deserialising [`Displayed`] type.
#[cfg(feature = "serde")]
struct DisplayedVisitor<V>(core::marker::PhantomData<V>);

#[cfg(feature = "serde")]
impl<'de, T> serde::de::Visitor<'de> for DisplayedVisitor<T>
where
	T: core::str::FromStr,
	<T as core::str::FromStr>::Err: core::fmt::Display,
{
	type Value = Displayed<T>;

	fn expecting(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
		fmt.write_str("a string")
	}

	fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
		T::from_str(s).map(Displayed).map_err(E::custom)
	}
}

macro_rules! impl_conversions {
	($(Displayed<$inner:ty> => $other:ty),*) => {
		$(
		impl From<Displayed<$inner>> for $other {
			fn from(value: Displayed<$inner>) -> Self {
				<$other>::from(value.0)
			}
		}
			)*
	};

	($($other:ty = Displayed<$inner:ty>),*) => {
		$(
		impl From<$other> for Displayed<$inner> {
			fn from(value: $other) -> Self {
				Self(<$inner>::from(value))
			}
		}

		impl From<Displayed<$inner>> for $other {
			fn from(value: Displayed<$inner>) -> Self {
				<$other>::from(value.0)
			}
		}
			)*
	};
}

#[cfg(feature = "proto")]
impl prost::Message for Displayed<u64> {
	fn encoded_len(&self) -> usize {
		self.0.encoded_len()
	}

	fn clear(&mut self) {
		self.0.clear()
	}

	fn encode_raw<B>(&self, buf: &mut B)
	where
		B: prost::bytes::BufMut,
		Self: Sized,
	{
		self.0.encode_raw(buf)
	}

	fn merge_field<B>(
		&mut self,
		tag: u32,
		wire_type: prost::encoding::WireType,
		buf: &mut B,
		ctx: prost::encoding::DecodeContext,
	) -> Result<(), prost::DecodeError>
	where
		B: prost::bytes::Buf,
		Self: Sized,
	{
		self.0.merge_field(tag, wire_type, buf, ctx)
	}
}

// Due to Rust orphan rules it’s not possible to make generic `impl<T>
// From<Displayed<T>> for T` so we’re defining common conversions explicitly.
impl_conversions!(Displayed<u128> => u128, Displayed<u64> => u64);

#[cfg(feature = "cosmwasm")]
impl_conversions!(cosmwasm_std::Uint128 = Displayed<u128>,
                  cosmwasm_std::Uint64 = Displayed<u64>);

#[cfg(feature = "proto")]
impl_conversions!(crate::proto::pb::common::Uint128 = Displayed<u128>);

impl<T: core::cmp::PartialEq> core::cmp::PartialEq<T> for Displayed<T> {
	fn eq(&self, rhs: &T) -> bool {
		self.0 == *rhs
	}
}
