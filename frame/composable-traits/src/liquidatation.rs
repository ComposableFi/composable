use codec::Codec;
use frame_support::{pallet_prelude::*, sp_runtime::Permill};

#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct LiquidationConfig {
	pub liquidation_fee: Permill,
}

pub trait Liquidation {
    type PairId: core::cmp::Ord;
    type AccountId: core::cmp::Ord;
    type Error;
    type Balance;

	fn liquidate(pair: Self::PairId, borrower : &Self::AccountId) -> Result<(), Self::Error>;
	fn calculate_liquidation_fee(amount : Self::Balance, config : &LiquidationConfig) ->
		Self::Balance;
}
