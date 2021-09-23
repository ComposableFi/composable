use frame_support::dispatch::DispatchError;

pub trait Oracle {
	type AssetId;
	type Balance;
	type Timestamp;

	/// How much `in` currency should one have to get 1 unit  `of` currency
	/// Error is returned if currency not supported or price information not available.
	/// `in` currency some well known shared (stable) currency.
	/// Implementation ensure that a LP token price can be resolved as long as the base asset price
	/// is resolvable.
	///```haskell
	/// price (Base USDT) = 100
	/// price (Base BTC) = 5000000
	/// price (Vaulted base stock_dilution_rate) = price base * stock_dilution_rate
	/// ```
	fn get_price(of: &Self::AssetId) -> Result<(Self::Balance, Self::Timestamp), DispatchError>;
}
