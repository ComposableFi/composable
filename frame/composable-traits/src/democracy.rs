pub trait Democracy {
	type AccountId;
	type Balance;
	type ReferendumIndex;
	type Vote;

	fn vote(account: Self::AccountId, ref_index: Self::ReferendumIndex, vote: Self::Vote);
	fn exists_and_is_ongoing(index: Self::ReferendumIndex) -> bool;
}
