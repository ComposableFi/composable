use crate::{
	dex::Orderbook,
	loans::{DurationSeconds, Timestamp},
};
use frame_support::pallet_prelude::*;
use sp_runtime::Permill;

#[derive(Decode, Encode, Clone)]
pub enum AuctionStepFunction {
	/// default - direct pass through to dex without steps, just to satisfy defaults and reasonably
	/// for testing
	LinearDecrease(LinearDecrease),
	StairstepExponentialDecrease(StairstepExponentialDecrease),
}

impl Default for AuctionStepFunction {
	fn default() -> Self {
		Self::LinearDecrease(Default::default())
	}
}

#[derive(Decode, Encode, Clone, PartialEq)]
pub enum AuctionState {
	AuctionStarted,
	AuctionEndedSuccessfully,
	/// like DEX does not support asset now or halted
	AuctionFatalFailed,
	/// so if for some reason system loop is not properly set, than will get timeout
	AuctionTimeFailed,
}

impl Default for AuctionState {
	fn default() -> Self {
		Self::AuctionStarted
	}
}

/// Auction is done via dexes which act each block. Each block decide if intention was satisfied or
/// not. That information is provided via event subscribes which callback into auction.
/// Assuming liquidity providers to be off our local chain, it means that it is high latency
/// external loop.
pub enum AuctionExchangeCallback {
	/// success transfer of funds
	Success,
	/// some technical fail of transaction, can issue new one
	RetryFail,
	/// cannot retry within current state of system, like assets are not supported
	FatalFail,
}

#[derive(Default, Decode, Encode, Clone)]
pub struct LinearDecrease {
	/// Seconds after auction start when the price reaches zero
	pub total: DurationSeconds,
}

#[derive(Default, Decode, Encode, Clone)]
pub struct StairstepExponentialDecrease {
	// Length of time between price drops
	pub step: DurationSeconds,
	// Per-step multiplicative factor, usually more than 50%, mostly closer to 100%, but not 100%.
	// Drop per unit of `step`.
	pub cut: Permill,
}

/// see example of it in clip.sol of makerdao
pub trait DutchAuction {
	type Error;
	type OrderId;
	type Orderbook: Orderbook;
	type AccountId;
	type AssetId;
	type Balance;
	type Order;

	/// Transfers asset from from provided to auction account.
	/// It is up to caller to check amount he get after auction.
	/// monitors `OrderBook` for possibility to start selling
	/// `account_id` who owns order
	/// `source_account` for specific specific `asset_id` from which `amount` is transferred
	/// onto auction account.
	/// `initial_price` for `total_amount`
	/// `target_account` where to move account after success sell.
	#[allow(clippy::too_many_arguments)]
	fn start(
		account_id: &Self::AccountId,
		source_asset_id: &Self::AssetId,
		source_account: &Self::AccountId,
		target_asset_id: &Self::AssetId,
		target_account: &Self::AccountId,
		total_amount: &Self::Balance,
		initial_price: &Self::Balance,
		function: AuctionStepFunction,
	) -> Result<Self::OrderId, Self::Error>;

	/// run existing auctions
	/// if some auctions completed, transfer amount to target account
	/// `now` current time.
	fn run_auctions(now: Timestamp) -> Result<(), Self::Error>;

	fn get_auction_state(order: &Self::OrderId) -> Option<Self::Order>;

	/// called back from DEX
	fn intention_updated(order: &Self::OrderId, action_event: AuctionExchangeCallback);
}
