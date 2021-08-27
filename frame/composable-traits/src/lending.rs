




/// basic lending
pub trait Lending  {
	type AccountId: core::cmp::Ord;
	type AssetId;
	type Error;
	type PairId : AccountId;
	type Balance;

	fn take_loan(
		pair: PairId,
		to: &Self::AccountId,
		borrowed_amount: &Self::Balance
	) -> Result<(), Self::Error>;

	fn pay_back_loan(
		pair: PairId,
		from: &Self::AccountId,
		to: &Self::AccountId,
	) -> Result<(), Self::Error>;

	/// gets per year interest rate
	fn get_interest_rate(
		pair: PairId,
	) ->

	// assumption that user will deposit borrow and collateral assets via vault
	// fn add_collateral(
	// 	pair: PairId,
	// 	from: &Self::AccountId,
	// 	borrowed_amount: &Self::Balance
	// ) -> Result<(), Self::Error>;
}
