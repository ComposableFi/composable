use sp_runtime::DispatchError;

use crate::loans::PriceStructure;

/// An object from which we can initiate liquidations from.
/// Does not cares if liquidation was completed or not, neither can reasonably provide that
/// information. Off-chain can join relevant ids if needed.
pub trait Liquidation {
	type AssetId;
	type Balance;
	type AccountId;
	type LiquidationId;
	type GroupId;

	/// Initiate a liquidation, this operation should be executed as fast as possible.
	fn liquidate(
		source_account: &Self::AccountId,
		source_asset_id: Self::AssetId,
		source_asset_price: PriceStructure<Self::GroupId, Self::Balance>,
		target_asset_id: Self::AssetId,
		target_account: &Self::AccountId,
		total_amount: Self::Balance,
	) -> Result<Self::LiquidationId, DispatchError>;
}
