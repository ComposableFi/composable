use codec::{Decode, Encode};
use frame_support::sp_runtime::Perbill;
use sp_runtime::{DispatchError, Permill};

/// Describes a simple exchanges which does not allow advanced configurations such as slippage.
pub trait SimpleExchange {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;

	/// Obtains the current price for a given asset, possibly routing through multiple markets.
	fn price(asset_id: Self::AssetId) -> Option<Self::Balance>;

	/// Exchange `amount` of `from` asset for `to` asset. The maximum price paid for the `to` asset
	/// is `SimpleExchange::price * (1 + slippage)`
	fn exchange(
		from: Self::AssetId,
		from_account: Self::AccountId,
		to: Self::AssetId,
		to_account: Self::AccountId,
		to_amount: Self::Balance,
		slippage: Perbill,
	) -> Result<Self::Balance, Self::Error>;
}

pub struct TakeResult<BALANCE> {
	pub amount: BALANCE,
	pub total_price: BALANCE,
}

pub struct SellOrder<OrderId, AccountId> {
	pub id: OrderId,
	/// account holding sell order amount.
	/// if it becomes empty or non existing, and there was no direct call from seller to cancel
	/// order, it means amount was sold
	pub account: AccountId,
}

#[derive(Encode, Decode)]
pub enum Price<GroupId, Balance> {
	Preferred(GroupId, Balance),
	Both { preferred_id: GroupId, preferred_price: Balance, any_price: Balance },
	Any(Balance),
}

impl<GroupId, Balance> Price<GroupId, Balance> {
	pub fn new_any(price: Balance) -> Self {
		Self::Any(price)
	}
}

/// see for examples:
/// - https://github.com/galacticcouncil/Basilisk-node/blob/master/pallets/exchange/src/lib.rs
/// - https://github.com/Polkadex-Substrate/polkadex-aura-node/blob/master/pallets/polkadex/src/lib.rs
/// expected that failed exchanges are notified by events.
pub trait Orderbook {
	type AssetId;
	type Balance;
	type AccountId;
	type OrderId;
	type GroupId;

	/// sell. exchanges specified amount of asset to other at specific price
	/// `source_price` price per unit
	/// `amm_slippage` set to zero to avoid AMM sell
	/// for remote auction we should  have sent some random to make sure we have idempotent request
	fn post(
		account_from: &Self::AccountId,
		asset: Self::AssetId,
		want: Self::AssetId,
		source_amount: Self::Balance,
		source_price: Price<Self::GroupId, Self::Balance>,
		amm_slippage: Permill,
	) -> Result<SellOrder<Self::OrderId, Self::AccountId>, DispatchError>;

	/// updates same existing order with new price
	/// to avoid overpay, use `take` with `up_to` price
	fn patch(
		order_id: Self::OrderId,
		price: Price<Self::GroupId, Self::Balance>,
	) -> Result<(), DispatchError>;

	/// sell. exchanges specified amount of asset to other at market price.
	fn market_sell(
		account: &Self::AccountId,
		asset: Self::AssetId,
		want: Self::AssetId,
		amount: Self::Balance,
		amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError>;

	/// ask to take order. get not found error if order never existed or was removed. got conflict
	/// error if order still on chain but was executed. please subscribe to events dispatched or
	/// check your balance or check blockchain history to validate your won the order.
	fn ask(
		account: &Self::AccountId,
		orders: impl Iterator<Item = Self::OrderId>,
		up_to: Self::Balance,
	) -> Result<(), DispatchError>;
}
