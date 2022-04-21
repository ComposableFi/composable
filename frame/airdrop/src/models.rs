use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::types::{EcdsaSignature, EthereumAddress};
use scale_info::TypeInfo;
use sp_runtime::{MultiSignature, RuntimeDebug};

/// A single Airdrop.
#[derive(Encode, Decode, PartialEq, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub struct Airdrop<AccountId, Balance, Moment> {
	/// Creator of the Airdrop.
	pub(crate) creator: AccountId,
	/// Total funds committed to the Airdrop.
	pub(crate) total_funds: Balance,
	/// Amount of the `total_funds` already claimed.
	pub(crate) claimed_funds: Balance,
	/// Starting block of the Airdrop.
	pub(crate) start: Moment,
	/// The mimimum time, in blocks, between recipient claims.
	pub(crate) schedule: Moment,
}

/// Funds, and related information, to be claimed by an Airdrop recipient.
#[derive(Encode, Decode, PartialEq, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub struct RecipientFund<Balance, Period> {
	/// Total funds committed for this recipient.
	pub(crate) total: Balance,
	/// Amount of the `total` this recipient has claimed.
	pub(crate) claimed: Balance,
	/// The mimimum time, in blocks, between recipient claims.
	pub(crate) vesting_period: Period,
	/// If claims by this user will be funded by an external pool.
	pub(crate) funded_claim: bool,
}

/// Proof that a remote account owns a local recipient account.
#[derive(Clone, RuntimeDebug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum Proof<AccountId> {
	RelayChain(AccountId, MultiSignature),
	Ethereum(EcdsaSignature),
}

/// Remote account that is associated with a local account.
#[derive(Hash, Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum RemoteAccount<AccountId> {
	RelayChain(AccountId),
	Ethereum(EthereumAddress),
}
