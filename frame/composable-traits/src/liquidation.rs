use sp_runtime::DispatchError;

use crate::defi::{DeFiEngine, Sell};

/// An object from which we can initiate liquidations from.
/// Does not cares if liquidation was completed or not, neither can reasonably provide that
/// information. Off-chain can join relevant ids if needed.
/// `configuration` - optional list of liquidations strategies
pub trait Liquidation: DeFiEngine {
	type OrderId;
	type LiquidationStrategyId;

	/// Initiate a liquidation, this operation should be executed as fast as possible.
	fn liquidate(
		from_to: &Self::AccountId,
		order: Sell<Self::MayBeAssetId, Self::Balance>,
		configuration: sp_std::vec::Vec<Self::LiquidationStrategyId>,
	) -> Result<Self::OrderId, DispatchError>;
}
