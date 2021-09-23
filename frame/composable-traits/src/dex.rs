use frame_support::sp_runtime::Perbill;

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
		from_amount: Self::Balance,
		slippage: Perbill,
	) -> Result<Self::Balance, Self::Error>;
}

pub struct TakeResult<BALANCE> {
	pub amount: BALANCE,
	pub total_price: BALANCE,
}

pub trait Orderbook {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;
	type OrderId;

	fn post(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
		price: &Self::Balance,
	) -> Result<Self::OrderId, Self::Error>;

	fn market_sell(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
	) -> Result<Self::OrderId, Self::Error>;

	fn take(
		account: &Self::AccountId,
		orders: impl Iterator<Item=Self::OrderId>,
		up_to: Self::Balance,
	) -> Result<TakeResult<Self::Balance>, Self::Error>;
}
