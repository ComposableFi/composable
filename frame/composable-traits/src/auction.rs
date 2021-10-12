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
pub enum AuctionState<DexOrderId> {
	AuctionStarted,
	AuctionOnDex(DexOrderId),
	AuctionEndedSuccessfully,
	/// like DEX does not support asset now or halted
	AuctionFatalFailed,
	/// so if for some reason system loop is not properly set, than will get timeout
	AuctionTimeFailed,
}

impl<DexOrderId> Default for AuctionState<DexOrderId> {
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
	/// The number of seconds until the price reach zero.
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

/// An object from which we can initiate a dutch auction.
// see example of it in clip.sol of makerdao
pub trait DutchAuction {
	type OrderId;
	type Orderbook: Orderbook;
	type AccountId;
	type AssetId;
	type Balance;
	type Order;

	/// Transfer the asset from the provided account to the auction account.
	/// The caller is responsible for checking the price at which the auction executed (not known in
	/// advance of course).
	///
	/// Description.
	///
	/// * `account_id`: the order owner.
	/// * `source_account`: the account from which we extract the `amount` of `source_asset_id`
	///   from.
	/// * `source_asset_id`: the asset we are interested to trade for `target_asset_id`.
	/// * `target_account`: the beneficiary of the order.
	/// * `total_amount`: the amount of `source_asset_id`.
	/// * `initial_price`: the initial price for `total_amount`.
	#[allow(clippy::too_many_arguments)]
	fn start(
		account_id: &Self::AccountId,
		source_asset_id: Self::AssetId,
		source_account: &Self::AccountId,
		target_asset_id: Self::AssetId,
		target_account: &Self::AccountId,
		total_amount: Self::Balance,
		initial_price: Self::Balance,
		function: AuctionStepFunction,
	) -> Result<Self::OrderId, DispatchError>;

	/// run existing auctions
	/// if some auctions completed, transfer amount to target account
	/// `now` current time.
	fn run_auctions(now: Timestamp) -> DispatchResult;

	fn get_auction_state(order: &Self::OrderId) -> Option<Self::Order>;

	/// called back from DEX
	fn intention_updated(
		order: &Self::OrderId,
		action_event: AuctionExchangeCallback,
	) -> DispatchResult;
}
