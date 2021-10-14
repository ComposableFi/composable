use frame_support::{dispatch::DispatchError, pallet_prelude::*};

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Price<PriceValue, BlockNumber> {
	pub price: PriceValue,
	pub block: BlockNumber,
}

pub trait Oracle {
	type AssetId;
	type Balance;
	type Timestamp;

	/// Quote the `amount` of `asset` in USDT cent.
	/// Error is returned if `asset` not supported or price information not available.
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
	fn get_price(
		asset: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Price<Self::Balance, Self::Timestamp>, DispatchError>;
}
