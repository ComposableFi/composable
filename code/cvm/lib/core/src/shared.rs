use crate::{prelude::*, AssetId};
use cosmwasm_std::{from_binary, to_binary, Api, Binary, CanonicalAddr, StdError, StdResult};
use serde::{de::DeserializeOwned, Serialize};

pub type Salt = Vec<u8>;
/// absolute amounts
pub type XcFunds = Vec<(AssetId, Displayed<u128>)>;
// like `XcFunds`, but allow relative(percentages) amounts. Similar to assets filters in XCM
pub type XcBalanceFilter = crate::asset::Amount;
pub type XcFundsFilter = crate::Funds<XcBalanceFilter>;
pub type XcInstruction = crate::Instruction<Vec<u8>, XcAddr, XcFundsFilter>;
pub type XcPacket = crate::Packet<XcProgram>;
pub type XcProgram = crate::Program<VecDeque<XcInstruction>>;

pub fn encode_base64<T: Serialize>(x: &T) -> StdResult<String> {
	Ok(to_binary(x)?.to_base64())
}

pub fn decode_base64<S: AsRef<str>, T: DeserializeOwned>(encoded: S) -> StdResult<T> {
	from_binary::<T>(&Binary::from_base64(encoded.as_ref())?)
}

/// A wrapper around any address on any chain.
/// Similar to `ibc_rs::Signer`(multi encoding), but not depend on ibc code bloat.
/// Unlike parity MultiLocation/Account32/Account20 which hard codes enum into code.
/// Better send canonical address to each chain for performance,
/// But it will also decode/reencode best effort.
/// Inner must be either base64 or hex encoded or contain only characters from these.
/// Added with helper per chain to get final address to use.
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[cfg_attr(
	feature = "scale",
	derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo)
)]
#[derive(
	Clone,
	PartialEq,
	Eq,
	Hash,
	derive_more::Deref,
	derive_more::From,
	derive_more::Into,
	serde::Deserialize,
	serde::Serialize,
)]
#[into(owned, ref, ref_mut)]
#[repr(transparent)]
pub struct XcAddr(String);

impl From<XcAddr> for Vec<u8> {
	fn from(value: XcAddr) -> Self {
		value.0.into_bytes()
	}
}

impl TryFrom<Vec<u8>> for XcAddr {
	type Error = StdError;

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		Ok(Self(String::from_utf8(value)?))
	}
}

impl XcAddr {
	/// idea that whatever user plugs into, it works, really for adoption
	/// sure for Ethereum he must plug exact binary address, but for others it's just a string
	pub fn encode_cosmwasm(&self, api: &dyn Api) -> Result<String, StdError> {
		if let Ok(addr) = Binary::from_base64(&self.0) {
			if let Ok(addr) = api.addr_humanize(&CanonicalAddr(addr)) {
				return Ok(addr.into_string())
			}
		}
		if let Ok((_, addr, _)) = bech32_no_std::decode(&self.0) {
			use bech32_no_std::FromBase32;
			if let Ok(addr) = Vec::from_base32(&addr) {
				if let Ok(addr) = api.addr_humanize(&CanonicalAddr(Binary(addr))) {
					return Ok(addr.into_string())
				}
			}
		}

		// here we will do CW on Substrate if that will be needed, but not prio
		Err(StdError::generic_err("Failed to ensure XcAddr encoding"))
	}
}

impl core::fmt::Display for XcAddr {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Display::fmt(&self.0, fmtr)
	}
}

impl core::fmt::Debug for XcAddr {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::Debug::fmt(&self.0, fmtr)
	}
}

/// A wrapper around a type which is serde-serialised as a string.
///
/// For serde-serialisation to be implemented for the type `T` must implement
/// `Display` and `FromStr` traits.
///
/// ```
/// # use xc_core::shared::Displayed;
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
#[derive(
	Copy,
	Clone,
	Default,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
	scale_info::TypeInfo,
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

impl<T> parity_scale_codec::WrapperTypeEncode for Displayed<T> {}
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
struct DisplayedVisitor<V>(core::marker::PhantomData<V>);

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

// impl prost::Message for Displayed<u64> {
//     fn encoded_len(&self) -> usize {
//         self.0.encoded_len()
//     }

//     fn clear(&mut self) {
//         self.0.clear()
//     }

// 	fn encode_raw<B>(&self, buf: &mut B)
// 		where
// 			B: prost::bytes::BufMut,
// 			Self: Sized {
// 		self.0.encode_raw(buf)
// 	}

// 	fn merge_field<B>(
// 			&mut self,
// 			tag: u32,
// 			wire_type: prost::encoding::WireType,
// 			buf: &mut B,
// 			ctx: prost::encoding::DecodeContext,
// 		) -> Result<(), prost::DecodeError>
// 		where
// 			B: prost::bytes::Buf,
// 			Self: Sized {
// 		self.0.merge_field(tag, wire_type, buf, ctx)
// 	}
// }

// Due to Rust orphan rules it’s not possible to make generic `impl<T>
// From<Displayed<T>> for T` so we’re defining common conversions explicitly.
impl_conversions!(Displayed<u128> => u128, Displayed<u64> => u64);

#[cfg(feature = "cosmwasm")]
impl_conversions!(cosmwasm_std::Uint128 = Displayed<u128>,
                  cosmwasm_std::Uint64 = Displayed<u64>);
impl_conversions!(crate::proto::pb::common::Uint128 = Displayed<u128>);

impl<T: core::cmp::PartialEq> core::cmp::PartialEq<T> for Displayed<T> {
	fn eq(&self, rhs: &T) -> bool {
		self.0 == *rhs
	}
}
