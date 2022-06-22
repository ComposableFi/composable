//! Definitions for types used throughout the Composable Rust project

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Cosmos AccAddress type
///
/// Cosmos supports both secp256k1 & secp256r1 for transaction authentication.
/// Public Keys for both are in the ECDSA 33-byte compressed format.
#[derive(
	Clone, Copy, Decode, Encode, Eq, Hash, MaxEncodedLen, PartialEq, RuntimeDebug, TypeInfo,
)]
pub enum CosmosAddress {
	/// Address length will be 20 bytes long
	Secp256k1([u8; 33]),
	/// Address length will be 32 bytes long
	Secp256r1([u8; 33]),
}

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

#[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo)]
pub struct CosmosEcdsaSignature(pub [u8; 64]);

impl PartialEq for CosmosEcdsaSignature {
	fn eq(&self, other: &Self) -> bool {
		self.0[..] == other.0[..]
	}
}

impl sp_std::fmt::Debug for CosmosEcdsaSignature {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		write!(f, "CosmosEcdsaSignature({:?})", &self.0[..])
	}
}

/// Struct representing an Elliptic Curve Signature
#[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo)]
pub struct EcdsaSignature(pub [u8; 65]);

impl PartialEq for EcdsaSignature {
	fn eq(&self, other: &Self) -> bool {
		self.0[..] == other.0[..]
	}
}

impl sp_std::fmt::Debug for EcdsaSignature {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		write!(f, "EcdsaSignature({:?})", &self.0[..])
	}
}

impl From<CosmosEcdsaSignature> for EcdsaSignature {
	fn from(item: CosmosEcdsaSignature) -> Self {
        let mut sig: [u8; 65] = [0; 65];

        sig.copy_from_slice(&item.0);

        EcdsaSignature(sig)
		// let mut sig = item.0.to_vec();
		// sig.push(0);
		// let mut signature: [u8; 65] = [0; 65];
		// signature.copy_from_slice(sig.as_slice());
		// EcdsaSignature(signature)
	}
}
