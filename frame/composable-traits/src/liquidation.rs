use sp_runtime::Permill;

use crate::dex::Orderbook;

pub trait Liquidate {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;
	type LiquidationId;

	fn initiate_liquidation(
		source_account: &Self::AccountId,
		source_asset_id: &Self::AssetId,
		source_asset_price: &Self::Balance,
		target_asset_id: &Self::AssetId,
		target_account: &Self::AccountId,
		total_amount: &Self::Balance,
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
		source_account: &Self::AccountId,
		source_asset_id: &Self::AssetId,
		_source_asset_price: &Self::Balance,
		target_asset_id: &Self::AssetId,
		_target_account: &Self::AccountId,
		total_amount: &Self::Balance,
	) -> Result<Self::LiquidationId, Self::Error> {
		<T as Orderbook>::market_sell(
			source_account,
			source_asset_id,
			target_asset_id,
			total_amount,
			Permill::from_perthousand(0),
		)
	}
	fn is_liquidation_completed(liquidation_id: &Self::LiquidationId) -> bool {
		<T as Orderbook>::is_order_executed(liquidation_id)
	}
}
