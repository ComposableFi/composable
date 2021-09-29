pub trait Democracy {
	type AccountId;
	type Balance;
	type Account;
	type ReferendumIndex;
	type Vote;

	fn vote(account: Self::Account, ref_index: Self::ReferendumIndex, vote: Self::Vote);
	fn exists_and_is_ongoing(index: Self::ReferendumIndex) -> bool;
}
