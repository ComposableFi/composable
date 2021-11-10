#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(trivial_numeric_casts)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create() -> Weight;
	fn deposit() -> Weight;
	fn withdraw() -> Weight;
	fn emergency_shutdown() -> Weight;
	fn start_() -> Weight;
	fn add_surcharge() -> Weight;
	fn claim_surcharge() -> Weight;
	fn delete_tombstoned() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Vault VaultCount (r:1 w:1)
	// Storage: Factory CurrencyCounter (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Allocations (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	fn create() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		(140_947_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	fn withdraw() -> Weight {
		(112_296_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn emergency_shutdown() -> Weight {
		(25_497_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn start_() -> Weight {
		(25_388_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_surcharge() -> Weight {
		(77_802_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_surcharge() -> Weight {
		(70_839_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	fn delete_tombstoned() -> Weight {
		(25_030_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

impl WeightInfo for () {
	// Storage: Vault VaultCount (r:1 w:1)
	// Storage: Factory CurrencyCounter (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Allocations (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	fn create() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		(140_947_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	fn withdraw() -> Weight {
		(112_296_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn emergency_shutdown() -> Weight {
		(25_497_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn start_() -> Weight {
		(25_388_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_surcharge() -> Weight {
		(77_802_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_surcharge() -> Weight {
		(70_839_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	fn delete_tombstoned() -> Weight {
		(25_030_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
