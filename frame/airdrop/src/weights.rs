use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};

use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create_airdrop() -> Weight;
	fn add_recipient(x: u32) -> Weight;
	fn remove_recipient() -> Weight;
	fn enable_airdrop() -> Weight;
	fn disable_airdrop() -> Weight;
	fn claim(x: u32) -> Weight;
}

impl WeightInfo for () {
	fn create_airdrop() -> Weight {
		0 as Weight
	}

	fn add_recipient(_x: u32) -> Weight {
		0 as Weight
	}

	fn remove_recipient() -> Weight {
		0 as Weight
	}

	fn enable_airdrop() -> Weight {
		0 as Weight
	}

	fn disable_airdrop() -> Weight {
		0 as Weight
	}

	fn claim(_x: u32) -> Weight {
		0 as Weight
	}
}
