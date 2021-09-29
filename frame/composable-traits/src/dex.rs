use frame_support::sp_runtime::Perbill;
use sp_runtime::Permill;

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

/// see for examples:
/// - https://github.com/galacticcouncil/Basilisk-node/blob/master/pallets/exchange/src/lib.rs
/// - https://github.com/Polkadex-Substrate/polkadex-aura-node/blob/master/pallets/polkadex/src/lib.rs
pub trait Orderbook {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;
	type OrderId;

	/// sell. exchanges specified amount of asset to other at specific price
	/// `source_price` price per unit
	/// `amm_slippage` set to zero to avoid AMM sell
	fn post(
		account_from: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		source_amount: &Self::Balance,
		source_price: &Self::Balance,
		amm_slippage : Permill,
	) -> Result<Self::OrderId, Self::Error>;

	/// sell. exchanges specified amount of asset to other at market price.
	fn market_sell(
		account: &Self::AccountId,
		asset: &Self::AssetId,
		want: &Self::AssetId,
		amount: &Self::Balance,
		amm_slippage : Permill,
	) -> Result<Self::OrderId, Self::Error>;

	/// buy
	fn take(
		account: &Self::AccountId,
		orders: impl Iterator<Item = Self::OrderId>,
		up_to: Self::Balance,
	) -> Result<TakeResult<Self::Balance>, Self::Error>;
}
