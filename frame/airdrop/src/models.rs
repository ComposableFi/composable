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
	/// Total number of recipients
	pub(crate) total_recipients: u32,
	/// Amount of the `total_funds` already claimed.
	pub(crate) claimed_funds: Balance,
	/// Starting block of the Airdrop.
	pub(crate) start: Option<Moment>,
	/// The minimum time, in blocks, between recipient claims.
	pub(crate) schedule: Moment,
	/// Set `true` if an airdrop has been explicitly disabled.
	pub(crate) disabled: bool,
}

/// Funds, and related information, to be claimed by an Airdrop recipient.
#[derive(Encode, Decode, PartialEq, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub struct RecipientFund<Balance, Period> {
	/// Total funds committed for this recipient.
	pub(crate) total: Balance,
	/// Amount of the `total` this recipient has claimed.
	pub(crate) claimed: Balance,
	/// The minimum time, in blocks, between recipient claims.
	pub(crate) vesting_period: Period,
	/// If claims by this user will be funded by an external pool.
	pub(crate) funded_claim: bool,
}

/// Current State of an [`Airdrop`](Airdrop).
#[derive(Debug, Encode, Decode, PartialEq, Copy, Clone, TypeInfo, MaxEncodedLen)]
pub enum AirdropState {
	/// The Airdrop has been created but has not started.
	Created,
	/// The Airdrop has started. Recipients can claim funds.
	Enabled,
	/// The Airdrop has ended. Recipients can **NOT** claim funds.
	Disabled,
}

/// Proof that a remote account owns a local recipient account.
#[derive(Clone, RuntimeDebug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum Proof<AccountId> {
	RelayChain(AccountId, MultiSignature),
	Ethereum(EcdsaSignature),
}

/// Remote account that is associated with a local account.
#[derive(Hash, Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum RemoteAccount<AccountId> {
	RelayChain(AccountId),
	Ethereum(EthereumAddress),
}
