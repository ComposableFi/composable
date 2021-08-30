pub trait Oracle {
	type AssetId;
	type Balance;
	type Timestamp;

	/// how much `in` currency should one have to get 1 unit  `of` currency
	/// None is returned if currency not supported or price information not available.
	/// `in` currency some well known shared (stable) currency
	fn get_price(of: Self::AssetId) -> Option<(Self::Balance, Self::Timestamp)>;
}
