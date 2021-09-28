pub enum AuctionStepFunction {}

pub trait DutchAuction {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;
	type OrderId;

	/// transfers asset from `account` to auction account
	/// stores order locally
	/// monitors `OrderBook` for possibility to start selling
	fn start(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
		initial_price: &Self::Balance,
		target_account: &Self::AccountId,
		function: AuctionStepFunction,
	) -> Result<Self::OrderId, Self::Error>;

	/// run existing auctions
	/// if some auctions completed, transfer amount to target account
	fn run_auctions() -> Result<(), Self::Error>;

	// cancel, iterate..
}
