use crate::dex::Orderbook;

pub trait Liquidate {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;

	fn liquidate(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
	) -> Result<(), Self::Error>;
}

impl<T: Orderbook> Liquidate for T {
	type AssetId = <Self as Orderbook>::AssetId;
	type Balance = <Self as Orderbook>::Balance;
	type AccountId = <Self as Orderbook>::AccountId;
	type Error = <Self as Orderbook>::Error;

	fn liquidate(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
	) -> Result<(), Self::Error> {
		<T as Orderbook>::market_sell(account, asset, want, amount).map(|_| ())
	}
}
