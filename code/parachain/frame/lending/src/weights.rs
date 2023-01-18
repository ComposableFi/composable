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
		Weight::from_ref_time(96_881_000_u64)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(11_u64))
	}
	// same as vaults deposit plus 1 more read
	fn vault_deposit() -> Weight {
		Weight::from_ref_time(140_947_000_u64)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// same as vaults withdraw plus 1 more read
	fn vault_withdraw() -> Weight {
		Weight::from_ref_time(112_296_000_u64)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn deposit_collateral() -> Weight {
		Weight::from_ref_time(123_789_000_u64)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	fn withdraw_collateral() -> Weight {
		Weight::from_ref_time(138_802_000_u64)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn borrow() -> Weight {
		Weight::from_ref_time(332_730_000_u64)
			.saturating_add(RocksDbWeight::get().reads(19_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	fn repay_borrow() -> Weight {
		Weight::from_ref_time(209_694_000_u64)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	fn liquidate(b: u32) -> Weight {
		Weight::from_ref_time(25_879_000_u64)
			.saturating_add(Weight::from_ref_time(7_877_000_u64).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
	}
	fn now() -> Weight {
		Weight::from_ref_time(4_744_000_u64).saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn accrue_interest(_x: u32) -> Weight {
		Weight::from_ref_time(76_626_000_u64)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn account_id() -> Weight {
		Weight::from_ref_time(3_126_000_u64)
	}
	fn available_funds() -> Weight {
		Weight::from_ref_time(16_450_000_u64).saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	fn handle_withdrawable() -> Weight {
		Weight::from_ref_time(20_716_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn handle_depositable() -> Weight {
		Weight::from_ref_time(40_066_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn handle_must_liquidate() -> Weight {
		Weight::from_ref_time(38_744_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
