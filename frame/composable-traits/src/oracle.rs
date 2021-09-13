use frame_support::dispatch::DispatchError;

pub trait Oracle {
	type AssetId;
	type Balance;
	type Timestamp;

	/// How much `in` currency should one have to get 1 unit  `of` currency
	/// Error is returned if currency not supported or price information not available.
	/// `in` currency some well known shared (stable) currency.
	/// Consumers may assume can get price of wrapped tokens:
	/// ```python
	/// price(wrap_1(wrap_2(wrap_3((btc))) = price(btc) * stock_dilution_1_ratio * stock_dilution_2_ratio * stock_dilution_3_ratio
	/// ```
	fn get_price(of: &Self::AssetId) -> Result<(Self::Balance, Self::Timestamp), DispatchError>;
}
