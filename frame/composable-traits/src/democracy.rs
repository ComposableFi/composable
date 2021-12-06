//pub type VotingOf<T> = VotingOf<T>;
use sp_runtime::DispatchError;

pub trait Democracy {
	type AccountId;
	type Balance;
	type ReferendumIndex;
	type Vote;
	type VotingOf;

	fn vote(account: Self::AccountId, ref_index: Self::ReferendumIndex, vote: Self::Vote);
	fn exists_and_is_ongoing(index: Self::ReferendumIndex) -> bool;

	fn count_votes(account: Self::AccountId) -> Result<Self::VotingOf, DispatchError>;
}
