use crate::pallet::Config;
use composable_traits::defi::DeFiComposableConfig;
use frame_support::pallet_prelude::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::FixedU128;
use sp_std::{
	fmt::{Debug, Display},
	str::FromStr,
};

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

impl InitializeBlockCallCounters {
	pub(crate) fn calculate_weight<T: Config>(&self) -> Weight {
		use crate::weights::WeightInfo;
		let mut weight: Weight = 0;
		let one_read = T::DbWeight::get().reads(1);
		weight += u64::from(self.now) * <T as Config>::WeightInfo::now();
		weight += u64::from(self.read_markets) * one_read;
		weight += u64::from(self.accrue_interest) * <T as Config>::WeightInfo::accrue_interest(1);
		weight += u64::from(self.account_id) * <T as Config>::WeightInfo::account_id();
		weight += u64::from(self.available_funds) * <T as Config>::WeightInfo::available_funds();
		weight +=
			u64::from(self.handle_withdrawable) * <T as Config>::WeightInfo::handle_withdrawable();
		weight +=
			u64::from(self.handle_depositable) * <T as Config>::WeightInfo::handle_depositable();
		weight += u64::from(self.handle_must_liquidate) *
			<T as Config>::WeightInfo::handle_must_liquidate();
		weight
	}
}

pub type MarketIdInner = u32;

#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct MarketId(
	// to allow pattern matching in tests outside of this crate
	#[cfg(test)] pub MarketIdInner,
	#[cfg(not(test))] pub(crate) MarketIdInner,
);

impl MarketId {
	pub fn new(i: u32) -> Self {
		Self(i)
	}
}

impl FromStr for MarketId {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		const ERROR: &str = "Parse MarketId error";
		u128::from_str(s)
			.map_err(|_| ERROR)
			.and_then(|id| id.try_into().map(MarketId).map_err(|_| ERROR))
	}
}

impl Display for MarketId {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let MarketId(id) = self;
		write!(f, "{}", id)
	}
}

pub(crate) struct MarketAssets<T: DeFiComposableConfig> {
	/// The borrow asset for the market.
	pub(crate) borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,
	/// The debt token/ debt marker for the market.
	pub(crate) debt_asset: <T as DeFiComposableConfig>::MayBeAssetId,
}

#[derive(Debug, PartialEqNoBound)]
pub(crate) struct AccruedInterest<T: Config> {
	pub(crate) accrued_increment: T::Balance,
	pub(crate) new_borrow_index: FixedU128,
}
