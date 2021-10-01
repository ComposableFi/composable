use frame_support::pallet_prelude::*;
use sp_runtime::RuntimeDebug;

pub trait Bribe {
	type Balance;
	type BribeIndex;
	type CurrencyId;
	type ReferendumIndex;
	type Votes;

	fn create_bribe(
		request: CreateBribeRequest<
			Self::ReferendumIndex,
			Self::Balance,
			Self::CurrencyId,
			Self::Votes,
		>,
	) -> Result<Self::BribeIndex, DispatchError>;

	fn take_bribe(
		request: TakeBribeRequest<Self::BribeIndex, Self::Votes>,
	) -> Result<bool, DispatchError>;
}

/// A request to create a bribe for an (ongoing) referendum.
#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug)]
pub struct CreateBribeRequest<ReferendumIndex, Balance, CurrencyId, Votes> {
	/// Index of the referendum.
	pub ref_index: ReferendumIndex,
	/// How much to pay bribe-takers in total.
	pub total_reward: Balance,
	/// What asset to pay the bribes with.
	pub asset_id: CurrencyId,
	/// Requested votes (e.g. number of tokens and the lock period).
	pub requested_votes: Votes,
	/// Whether the bribe is in favor of the proposal or against it.
	pub is_aye: bool,
}

/// A request to take a bribe and vote for the corresponding referendum.
#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug)]
pub struct TakeBribeRequest<BribeIndex, Votes> {
	/// Index of the bribe.
	pub bribe_index: BribeIndex,
	/// A product of token amount and lock period.
	pub votes: Votes,
}
