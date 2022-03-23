use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{MultiSignature, RuntimeDebug};

#[derive(Encode, Decode, PartialEq, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub struct Reward<Balance, Period> {
	pub(crate) total: Balance,
	pub(crate) claimed: Balance,
	pub(crate) vesting_period: Period,
}

#[derive(Clone, RuntimeDebug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum Proof<AccountId> {
	RelayChain(AccountId, MultiSignature),
	Ethereum(EcdsaSignature),
}

#[derive(Hash, Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum RemoteAccount<AccountId> {
	RelayChain(AccountId),
	Ethereum(EthereumAddress),
}

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
