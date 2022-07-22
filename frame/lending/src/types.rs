use crate::pallet::Config;
use composable_traits::{defi::DeFiComposableConfig, lending::CreateInput};
use frame_support::pallet_prelude::*;
use sp_runtime::FixedU128;
use sp_std::fmt::Debug;

/// Used to count the calls in [`Pallet::initialize_block`]. Each field corresponds to a
/// function call to count.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct InitializeBlockCallCounters {
	pub(crate) now: u32,
	pub(crate) read_markets: u32,
	pub(crate) accrue_interest: u32,
	pub(crate) account_id: u32,
	pub(crate) available_funds: u32,
	pub(crate) handle_withdrawable: u32,
	pub(crate) handle_depositable: u32,
	pub(crate) handle_must_liquidate: u32,
}

pub type MarketId = u32;

// REVIEW: Maybe move this to `models::market_index`?
// TODO: Rename to `MarketId`.
#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq, MaxEncodedLen, TypeInfo)]
#[repr(transparent)]
pub struct MarketIndex(
	#[cfg(test)] // to allow pattern matching in tests outside of this crate
	pub MarketId,
	#[cfg(not(test))] pub(crate) MarketId,
);

impl MarketIndex {
	pub fn new(i: u32) -> Self {
		Self(i)
	}
}

pub(crate) struct MarketAssets<T: DeFiComposableConfig> {
	/// The borrow asset for the market.
	pub(crate) borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,
	/// The debt token/ debt marker for the market.
	pub(crate) debt_asset: <T as DeFiComposableConfig>::MayBeAssetId,
}

/// A convenience wrapper around [`CreateInput`] for `T: Config`.
pub type CreateInputOf<T> = CreateInput<
	<T as Config>::LiquidationStrategyId,
	<T as DeFiComposableConfig>::MayBeAssetId,
	<T as frame_system::Config>::BlockNumber,
>;

#[derive(Debug, PartialEqNoBound)]
pub(crate) struct AccruedInterest<T: Config> {
	pub(crate) accrued_increment: T::Balance,
	pub(crate) new_borrow_index: FixedU128,
}
