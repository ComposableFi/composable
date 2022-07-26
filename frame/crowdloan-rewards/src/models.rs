use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::types::{EcdsaSignature, EthereumAddress};
use scale_info::TypeInfo;
use sp_runtime::{MultiSignature, RuntimeDebug};

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub struct Reward<Balance, Period> {
	pub(crate) total: Balance,
	pub(crate) claimed: Balance,
	pub(crate) vesting_period: Period,
}

#[derive(Clone, RuntimeDebug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum Proof<AccountId> {
	RelayChain(AccountId, MultiSignature),
	Ethereum(EcdsaSignature),
}

#[derive(Hash, Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum RemoteAccount<AccountId> {
	RelayChain(AccountId),
	Ethereum(EthereumAddress),
}
