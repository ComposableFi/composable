

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn transfer() -> Weight;
	fn transfer_native() -> Weight;
	fn force_transfer() -> Weight;
	fn force_transfer_native() -> Weight;
	fn transfer_all() -> Weight;
/*
      fn can_reserve() -> Weights;
      fn slash_reserved() -> Weights;                
      fn reserved_balance() -> Weights;
      fn reserve() -> Weights;
      fn unreserve() -> Weights;
      fn repatriate_reserved() -> Weights;           
	fn destroy(c: u32, s: u32, a: u32, ) -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn force_transfer() -> Weight;
	fn freeze() -> Weight;
	fn thaw() -> Weight;
	fn freeze_asset() -> Weight;
	fn thaw_asset() -> Weight;
	fn transfer_ownership() -> Weight;
	fn set_team() -> Weight;
	fn set_metadata(n: u32, s: u32, ) -> Weight;
	fn clear_metadata() -> Weight;
	fn force_set_metadata(n: u32, s: u32, ) -> Weight;
	fn force_clear_metadata() -> Weight;
	fn force_asset_status() -> Weight;
	fn approve_transfer() -> Weight;
	fn transfer_approved() -> Weight;
	fn cancel_approval() -> Weight;
	fn force_cancel_approval() -> Weight;
*/
}


pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Assets Asset (r:1 w:1)
//	fn force_create() -> Weight {
//		(21_378_000 as Weight)
//			.saturating_add(T::DbWeight::get().reads(1 as Weight))
//			.saturating_add(T::DbWeight::get().writes(1 as Weight))
//	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
//	fn mint() -> Weight {
//		(47_913_000 as Weight)
//			.saturating_add(T::DbWeight::get().reads(2 as Weight))
//			.saturating_add(T::DbWeight::get().writes(2 as Weight))
//	}
	// Storage: Assets Asset (r:1 w:1)
//	fn burn() -> Weight {
//		(55_759_000 as Weight)
//			.saturating_add(T::DbWeight::get().reads(2 as Weight))
//			.saturating_add(T::DbWeight::get().writes(2 as Weight))
//	}


	fn transfer_all() -> Weight {
		(83_205_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}

	// Storage: Assets Asset (r:1 w:1)
	fn transfer() -> Weight {
		(83_205_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn transfer_native() -> Weight {
		(70_665_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		(81_458_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}

	fn force_transfer_native() -> Weight {
		(81_458_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}

}

// For backwards compatibility and tests
impl WeightInfo for () {


	fn transfer_native() -> Weight {
		(83_205_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn force_transfer_native() -> Weight {
		(83_205_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn transfer_all() -> Weight {
		(83_205_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}

	// Storage: Assets Asset (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		(83_205_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		(81_458_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}


}
