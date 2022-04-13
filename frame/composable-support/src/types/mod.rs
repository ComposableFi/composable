//! Definitions for types used throughout the Composable Rust project

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Raw ethereum address.
#[derive(
	Hash, Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
)]
pub struct EthereumAddress(pub [u8; 20]);

#[cfg(feature = "std")]
impl frame_support::Serialize for EthereumAddress {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let hex: String = rustc_hex::ToHex::to_hex(&self.0[..]);
		serializer.serialize_str(&format!("0x{}", hex))
	}
}

#[cfg(feature = "std")]
impl<'de> frame_support::Deserialize<'de> for EthereumAddress {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let base_string = String::deserialize(deserializer)?;
		// strip_prefix instead of trim_start_matches because strip_prefix only removes
		// whereas trim_start_matches removes the prefix as many times at it appears
		// (i.e. "11foo".trim_start_matches("1") == "foo", not "1foo")
		let s = base_string.strip_prefix("0x").unwrap_or(&base_string);
		if s.len() != 40 {
			return Err(frame_support::serde::de::Error::custom(
				"Bad length of Ethereum address (should be 42 including '0x')",
			))
		}
		let raw: Vec<u8> = rustc_hex::FromHex::from_hex(s)
			.map_err(|e| frame_support::serde::de::Error::custom(format!("{:?}", e)))?;
		let mut r = Self::default();
		r.0.copy_from_slice(&raw);
		Ok(r)
	}
}
