use crate::{dex::Orderbook, loans::DurationSeconds};

pub enum AuctionStepFunction {
	LinearDecrease,
	StairstepExponentialDecrease,
}

/// Auction is done via dexes which act each block. Each block decide if intention was satisfied or not.
/// That information is provided via event subscribes which callback into auction.
/// Assuming liquidity providers to be off our local chain, it means that it is high latency external loop.
pub enum ActionEvent {
	/// success transfer of funds
	Fail,
	CanceledBecauseOfNoPriceMatch,
	CanceledBecauseOfSomeTechnicalReasons,
}

pub struct AuctionOrder<OrderId> {
	pub id: OrderId,
}

pub trait DutchAuction {
	type Error;
	type OrderId;
	type Orderbook: Orderbook;
	type AccountId;
	type AssetId;
	type Balance;

	/// Transfers asset from from provided to auction account.
	/// It is up to caller to check amount he get after auction.
	/// monitors `OrderBook` for possibility to start selling
	/// `account_id` who owns order
	/// `source_account` for specific specific `asset_id` from which `amount` is transferred
	/// onto auction account.
	/// `initial_price` for `total_amount`
	/// `target_account` where to move account after success sell.
	fn start(
		account_id: &Self::AccountId,
		source_asset_id: &Self::AssetId,
		source_account: &Self::AccountId,
		target_asset_id: &Self::AssetId,
		target_account: &Self::AccountId,
		want: &Self::AssetId,
		total_amount: &Self::Balance,
		initial_price: &Self::Balance,
		function: AuctionStepFunction,
	) -> Result<Self::OrderId, Self::Error>;

	/// run existing auctions
	/// if some auctions completed, transfer amount to target account
	/// `now` current time.
	fn run_auctions(now: DurationSeconds) -> Result<(), Self::Error>;

	fn get_auction_state(order: &Self::OrderId) -> Option<AuctionOrder<Self::OrderId>>;

	/// called back from DEX
	fn intention_updated(order: &Self::OrderId, action_event: ActionEvent);
}
