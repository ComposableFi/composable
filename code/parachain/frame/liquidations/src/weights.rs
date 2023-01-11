#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(trivial_numeric_casts)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{pallet_prelude::Weight, traits::Get};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_liquidation_strategy() -> Weight;
	fn sell(vector_length: u32) -> Weight;
}

/// Weight functions for `liquidations`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Liquidations StrategyIndex (r:1 w:1)
	// Storage: Liquidations Strategies (r:0 w:1)
	fn add_liquidation_strategy() -> Weight {
		Weight::from_ref_time(3_127_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Liquidations Strategies (r:2 w:0)
	// Storage: DutchAuction OrdersIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: DutchAuction SellOrders (r:0 w:1)
	fn sell(x: u32) -> Weight {
		Weight::from_ref_time(43_980_000 as u64)
			// Standard Error: 27_000
			.saturating_add(Weight::from_ref_time(1_758_000 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}
