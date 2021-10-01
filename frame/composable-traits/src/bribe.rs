use frame_support::pallet_prelude::*;
use sp_runtime::RuntimeDebug;

pub trait Bribe {
	type Balance;
	type BribeIndex;
	type Conviction;
	type CurrencyId;
	type ReferendumIndex;

	fn create_bribe(
		request: CreateBribeRequest<
			Self::ReferendumIndex,
			Self::Balance,
			Self::Conviction,
			Self::CurrencyId,
		>,
	) -> Result<Self::BribeIndex, DispatchError>;

	fn take_bribe(
		request: TakeBribeRequest<Self::BribeIndex, Self::Balance, Self::Conviction>,
	) -> Result<bool, DispatchError>;
}

/// A request to create a bribe for an (ongoing) referendum.
#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug)]
pub struct CreateBribeRequest<ReferendumIndex, Conviction, Balance, CurrencyId> {
	/// Index of the referendum.
	pub ref_index: ReferendumIndex,
	/// How much to pay bribe-takers in total.
	pub total_reward: Balance,
	/// Requested conviction (e.g. number of blocks to lock tokens for).
	pub requested_conviction: Conviction,
	/// What asset to pay the bribes with.
	pub asset_id: CurrencyId,
	/// Whether the bribe is in favor of the proposal or against it.
	pub is_aye: bool,
}

/// A request to take a bribe and vote for the corresponding referendum.
#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug)]
pub struct TakeBribeRequest<BribeIndex, Balance, Conviction> {
	/// Index of the bribe.
	pub bribe_index: BribeIndex,
	/// How much tokens to lock up for the vote.
	pub tokens: Balance,
	/// How long to lock up the tokens for.
	pub conviction: Conviction,
	// REVIEW: we don't need Vote/aye fields as that will be handled by the pallet
}
