use codec::{Codec, Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// https://github.com/interlay/interbtc/blob/a7c0e69ac041176a2531bafb1c4e35cbc2f7e192/crates/oracle/rpc/runtime-api/src/lib.rs#L10
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub struct SafeRpcWrapper<T: SafeRpcWrapperType>(
	#[cfg_attr(feature = "std", serde(serialize_with = "serialize_to_hex"))]
	#[cfg_attr(feature = "std", serde(deserialize_with = "deserialize_from_hex"))]
	pub T,
);

pub trait SafeRpcWrapperType
where
	Self: sp_std::fmt::LowerHex + FromHexStr + Codec,
{
}

impl<T> SafeRpcWrapperType for T where T: sp_std::fmt::LowerHex + FromHexStr + Codec {}

pub trait FromHexStr: sp_std::marker::Sized {
	type Err: sp_std::fmt::Display;

	fn from_hex_str(src: &str) -> sp_std::result::Result<Self, Self::Err>;
}

#[derive(Debug)]
pub enum FromHexStrErr {
	No0xPrefix,
	ParseIntError(sp_std::num::ParseIntError),
}

impl sp_std::fmt::Display for FromHexStrErr {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		match self {
			FromHexStrErr::No0xPrefix => f.write_str("No `0x` prefix"),
			FromHexStrErr::ParseIntError(parse_int_error) =>
				f.write_fmt(format_args!("{}", parse_int_error)),
		}
	}
}

impl FromHexStr for u128 {
	type Err = FromHexStrErr;

	fn from_hex_str(src: &str) -> sp_std::result::Result<Self, Self::Err> {
		match src.strip_prefix("0x") {
			Some(stripped) =>
				u128::from_str_radix(stripped, 16).map_err(FromHexStrErr::ParseIntError),
			None => Err(FromHexStrErr::No0xPrefix),
		}
	}
}

#[cfg(feature = "std")]
fn serialize_to_hex<S: Serializer, T: SafeRpcWrapperType>(
	t: &T,
	serializer: S,
) -> Result<S::Ok, S::Error> {
	serializer.serialize_str(&format!("{:#x}", t))
}

#[cfg(feature = "std")]
fn deserialize_from_hex<'de, D: Deserializer<'de>, T: SafeRpcWrapperType>(
	deserializer: D,
) -> Result<T, D::Error> {
	use serde::de::Error;
	let hex_string = String::deserialize(deserializer)?;

	T::from_hex_str(&hex_string).map_err(|err| {
		D::Error::custom(format!(
			"Unable to parse as 0x-prefixed hex string: {} (error: {})",
			hex_string, err
		))
	})
}

// TODO: tests?
