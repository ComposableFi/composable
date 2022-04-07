use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::types::EthereumAddress;
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
