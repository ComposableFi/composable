use frame_support::{dispatch::DispatchError, pallet_prelude::*};
use sp_std::vec::Vec;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Price<PriceValue, BlockNumber> {
	pub price: PriceValue,
	pub block: BlockNumber,
}

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
	/// data Currency = USDT | BTC
	/// data Asset = Base Currency | Vaulted Asset Int
	///
	/// price :: Asset -> Int
	/// price (Base USDT) = 100
	/// price (Base BTC) = 5000000
	/// price (Vaulted base stock_dilution_rate) = price base * stock_dilution_rate
	/// ```
	fn get_price(of: Self::AssetId)
		-> Result<Price<Self::Balance, Self::Timestamp>, DispatchError>;

	fn get_twap(
		of: Self::AssetId,
		weighting: Vec<Self::Balance>,
	) -> Result<Self::Balance, DispatchError>;
}
