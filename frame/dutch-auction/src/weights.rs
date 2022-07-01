#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]

use frame_support::{pallet_prelude::Weight, traits::Get};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_configuration() -> Weight;
	fn ask() -> Weight;
	fn take() -> Weight;
	fn liquidate() -> Weight;
	fn xcm_sell() -> Weight;
	fn known_overhead_for_on_finalize() -> Weight;
}

/// Weight functions for `dutch_auction`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DutchAuction Configurations (r:0 w:1)
	fn add_configuration() -> Weight {
		(8_434_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DutchAuction OrdersIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: DutchAuction SellOrders (r:0 w:1)
	fn ask() -> Weight {
		(36_854_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: DutchAuction SellOrders (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: DutchAuction Takes (r:1 w:1)
	fn take() -> Weight {
		(21_406_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DutchAuction SellOrders (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn liquidate() -> Weight {
		(33_038_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: DutchAuction Configurations (r:1 w:0)
	// Storage: DutchAuction OrdersIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: DutchAuction LocalOrderIdToRemote (r:0 w:1)
	// Storage: DutchAuction SellOrders (r:0 w:1)
	fn xcm_sell() -> Weight {
		(44_295_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: DutchAuction Takes (r:2 w:1)
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: DutchAuction SellOrders (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: DutchAuction LocalOrderIdToRemote (r:1 w:1)
	fn known_overhead_for_on_finalize() -> Weight {
		(37_057_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}
