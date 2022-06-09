pub trait XCVM {
	type AccountId;
	type Input;
	type Output;
	fn execute(who: &Self::AccountId, input: Self::Input) -> Self::Output;
}
