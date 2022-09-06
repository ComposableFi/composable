use codec::Encode;
use sp_runtime::DispatchError;
use sp_std::vec::*;

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

/// generic transaction which can target any pallet and any method in any parachain (local or
/// remote)
/// so it must be encoded in format with widest possible values to incorporate some chains we do
/// now (similar on how XCM is modelled)
#[derive(Encode)]
pub struct XcmLiquidation<AssetId> {
	pallet: u8,
	method: u8,
	order: Sell<AssetId, u128>,
	strategy: Vec<u128>,
}

impl<AssetId> XcmLiquidation<AssetId> {
	pub fn new(pallet: u8, method: u8, order: Sell<AssetId, u128>, strategy: Vec<u128>) -> Self {
		Self { pallet, method, order, strategy }
	}
}
