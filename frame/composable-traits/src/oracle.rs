use frame_support::dispatch::DispatchError;

pub trait Oracle {
	type AssetId;
	type Balance;
	type Timestamp;

	/// how much `in` currency should one have to get 1 unit  `of` currency
	/// error is returned if currency not supported or price information not available.
	/// `in` currency some well known shared (stable) currency
	fn get_price(of: &Self::AssetId) -> Result<(Self::Balance, Self::Timestamp), DispatchError>;
}
