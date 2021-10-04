use sp_runtime::Permill;

use crate::dex::Orderbook;

pub trait Liquidate {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;
	type LiquidationId;

	fn initiate_liquidation(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
	) -> Result<Self::LiquidationId, Self::Error>;
	fn is_liquidation_completed(liquidation_id: &Self::LiquidationId) -> bool;
}

impl<T: Orderbook> Liquidate for T {
	type AssetId = <Self as Orderbook>::AssetId;
	type Balance = <Self as Orderbook>::Balance;
	type AccountId = <Self as Orderbook>::AccountId;
	type Error = <Self as Orderbook>::Error;
	type LiquidationId = <Self as Orderbook>::OrderId;

	fn initiate_liquidation(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
	) -> Result<Self::LiquidationId, Self::Error> {
		<T as Orderbook>::market_sell(account, asset, want, amount, Permill::from_perthousand(0))
	}
	fn is_liquidation_completed(liquidation_id: &Self::LiquidationId) -> bool {
		<T as Orderbook>::is_order_executed(liquidation_id)
	}
}
