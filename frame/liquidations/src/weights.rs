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
		(3_127_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Liquidations Strategies (r:2 w:0)
	// Storage: DutchAuction OrdersIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: DutchAuction SellOrders (r:0 w:1)
	fn sell(x: u32) -> Weight {
		(43_980_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((1_758_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
