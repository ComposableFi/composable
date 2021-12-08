//pub type VotingOf<T> = VotingOf<T>;
use sp_runtime::DispatchError;

//pub type VotingOf<T> = <T as Democracy>::VotingOf;

pub trait Democracy {
	type AccountId;
	type Balance;
	type ReferendumIndex;
	type Vote;
	type BlockNumber;

	fn vote(account: Self::AccountId, ref_index: Self::ReferendumIndex, vote: Self::Vote);
	fn exists_and_is_ongoing(index: Self::ReferendumIndex) -> bool;

	#[allow(clippy::type_complexity)]
	fn count_votes(
		account: Self::AccountId,
	) -> Result<Voting<Self::Balance, Self::AccountId, Self::BlockNumber>, DispatchError>;
}

pub struct Voting<Balance, AccountId, BlockNumber> {
	pub balance: Balance,
	pub accountid: AccountId,
	pub blocknumber: BlockNumber,
}
