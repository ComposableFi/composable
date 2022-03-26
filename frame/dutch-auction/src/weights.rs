#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]

use frame_support::{pallet_prelude::Weight, traits::Get};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn ask() -> Weight;
	fn take() -> Weight;
	fn liquidate() -> Weight;
	fn known_overhead_for_on_finalize() -> Weight;
}

/// Weight functions for `dutch_auction`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DutchAuction OrdersIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: DutchAuction SellOrders (r:0 w:1)
	fn ask() -> Weight {
		(185_212_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: DutchAuction SellOrders (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: DutchAuction Takes (r:1 w:1)
	fn take() -> Weight {
		(140_074_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DutchAuction SellOrders (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn liquidate() -> Weight {
		(150_867_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: DutchAuction Takes (r:2 w:1)
	// Storage: DutchAuction SellOrders (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	fn known_overhead_for_on_finalize() -> Weight {
		(195_022_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
