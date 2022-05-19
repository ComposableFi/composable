//! Traits used in the implementation of the Airdrop pallet.

use frame_support::dispatch::DispatchResult;
use sp_runtime::DispatchError;

/// Contains functions necessary functions for the business logic for managing Airdrops
pub trait AirdropManagement {
	type AccountId;
	type AirdropId;
	type AirdropStart;
	type Balance;
	type Proof;
	type Recipient;
	type RecipientCollection;
	type RemoteAccount;
	type VestingSchedule;

	/// Create a new Airdrop.
	fn create_airdrop(
		creator_id: Self::AccountId,
		start: Option<Self::AirdropStart>,
		schedule: Self::VestingSchedule,
	) -> DispatchResult;

	/// Add one or more recipients to an Airdrop.
	fn add_recipient(
		airdrop_id: Self::AirdropId,
		recipients: Self::RecipientCollection,
	) -> DispatchResult;

	/// Remove a recipient from an Airdrop.
	fn remove_recipient(airdrop_id: Self::AirdropId, recipient: Self::Recipient) -> DispatchResult;

	/// Start an Airdrop.
	fn enable_airdrop(airdrop_id: Self::AirdropId) -> DispatchResult;

	/// Stop an Airdrop.
	fn disable_airdrop(airdrop_id: Self::AirdropId) -> Result<Self::Balance, DispatchError>;

	/// Claim a recipient reward from an Airdrop.
	fn claim(
		airdrop_id: Self::AirdropId,
		remote_account: Self::RemoteAccount,
		reward_account: Self::AccountId,
	) -> Result<Self::Balance, DispatchError>;
}
