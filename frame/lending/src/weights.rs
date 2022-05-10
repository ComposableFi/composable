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
		(96_881_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
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
		(332_730_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(19 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	fn repay_borrow() -> Weight {
		(209_694_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn liquidate(b: u32) -> Weight {
		(25_879_000 as Weight)
			.saturating_add((7_877_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
	}
	fn now() -> Weight {
		(4_744_000 as Weight).saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn accrue_interest(_x: u32) -> Weight {
		(76_626_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn account_id() -> Weight {
		(3_126_000 as Weight)
	}
	fn available_funds() -> Weight {
		(16_450_000 as Weight).saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	fn handle_withdrawable() -> Weight {
		(20_716_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn handle_depositable() -> Weight {
		(40_066_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn handle_must_liquidate() -> Weight {
		(38_744_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
