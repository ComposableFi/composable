use sp_runtime::{DispatchError, Permill};

use crate::dex::Orderbook;

/// An object from which we can initiate liquidations from.
pub trait Liquidation {
	type AssetId;
	type Balance;
	type AccountId;
	type LiquidationId;

	/// Initiate a liquidation, this operation should be executed as fast as possible.
	fn liquidate(
		source_account: &Self::AccountId,
		source_asset_id: Self::AssetId,
		source_asset_price: Self::Balance,
		target_asset_id: Self::AssetId,
		target_account: &Self::AccountId,
		total_amount: Self::Balance,
	) -> Result<Self::LiquidationId, DispatchError>;

	/// Determine whether a liquidation has been completed.
	fn has_been_liquidated(liquidation_id: &Self::LiquidationId) -> bool;
}

impl<T: Orderbook> Liquidation for T {
	type AssetId = <Self as Orderbook>::AssetId;
	type Balance = <Self as Orderbook>::Balance;
	type AccountId = <Self as Orderbook>::AccountId;
	type LiquidationId = <Self as Orderbook>::OrderId;

	fn liquidate(
		source_account: &Self::AccountId,
		source_asset_id: Self::AssetId,
		_source_asset_price: Self::Balance,
		target_asset_id: Self::AssetId,
		_target_account: &Self::AccountId,
		total_amount: Self::Balance,
	) -> Result<Self::LiquidationId, DispatchError> {
		<T as Orderbook>::market_sell(
			source_account,
			source_asset_id,
			target_asset_id,
			total_amount,
			Permill::from_perthousand(0),
		)
	}

	fn has_been_liquidated(liquidation_id: &Self::LiquidationId) -> bool {
		<T as Orderbook>::is_order_executed(liquidation_id)
	}
}
