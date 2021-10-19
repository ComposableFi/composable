use frame_support::{dispatch::DispatchError, pallet_prelude::*};

use crate::currency::PriceableAsset;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
pub struct Price<PriceValue, BlockNumber> {
	pub price: PriceValue,
	pub block: BlockNumber,
}

/// An object that is able to provide an asset price.
/// Important: the current price-feed is providing prices in USDT only.
pub trait Oracle {
	type AssetId: PriceableAsset;
	type Balance: From<u64>;
	type Timestamp;

	/// Quote the `amount` of `asset` in USDT cent.
	/// Error is returned if `asset` not supported or price information not available.

	/// Assuming we have a price `p` for an unit (not smallest) of `asset` in USDT cents.
	/// Let `k` be the number of decimals for `asset`.
	/// The price of an amount `a` of the smallest possible unit of `asset` is:
	/// p * a / 10^k
	/// e.g. for BTC, the price is expressed for 1 BTC, but the amount is in sats:
	/// 1 BTC = 10^8 sats
	/// get_price(BTC, 1_00000000) = price(1BTC) * 1_00000000 / 10^8 = $50000

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

	/// Check whether the provided `asset` is supported (a.k.a. a price can be computed) by the
	/// oracle.
	fn is_supported(asset: Self::AssetId) -> Result<bool, DispatchError> {
		Self::get_price(asset, asset.unit()).map(|_| true)
	}
}
