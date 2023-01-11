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
	fn create_market() -> Weight;
	fn vault_deposit() -> Weight;
	fn vault_withdraw() -> Weight;
	fn deposit_collateral() -> Weight;
	fn withdraw_collateral() -> Weight;
	fn borrow() -> Weight;
	fn repay_borrow() -> Weight;
	fn liquidate(b: u32) -> Weight;
	fn now() -> Weight;
	fn accrue_interest(x: u32) -> Weight;
	fn account_id() -> Weight;
	fn available_funds() -> Weight;
	fn handle_withdrawable() -> Weight;
	fn handle_depositable() -> Weight;
	fn handle_must_liquidate() -> Weight;
}

impl WeightInfo for () {
	fn create_market() -> Weight {
		Weight::from_ref_time(96_881_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(11 as u64))
	}
	// same as vaults deposit plus 1 more read
	fn vault_deposit() -> Weight {
		Weight::from_ref_time(140_947_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(10 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// same as vaults withdraw plus 1 more read
	fn vault_withdraw() -> Weight {
		Weight::from_ref_time(112_296_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn deposit_collateral() -> Weight {
		123_789_000_u64
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn withdraw_collateral() -> Weight {
		138_802_000_u64
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn borrow() -> Weight {
		Weight::from_ref_time(332_730_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(19 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	fn repay_borrow() -> Weight {
		Weight::from_ref_time(209_694_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn liquidate(b: u32) -> Weight {
		Weight::from_ref_time(25_879_000 as u64)
			.saturating_add(Weight::from_ref_time(7_877_000 as u64).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
	}
	fn now() -> Weight {
		(4_744_000 as u64).saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	fn accrue_interest(_x: u32) -> Weight {
		Weight::from_ref_time(76_626_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn account_id() -> Weight {
		Weight::from_ref_time(3_126_000 as u64)
	}
	fn available_funds() -> Weight {
		(16_450_000 as u64).saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	fn handle_withdrawable() -> Weight {
		Weight::from_ref_time(20_716_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn handle_depositable() -> Weight {
		Weight::from_ref_time(40_066_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn handle_must_liquidate() -> Weight {
		Weight::from_ref_time(38_744_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
