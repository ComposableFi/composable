use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};

use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create_airdrop(x: u32) -> Weight;
	fn add_recipient(x: u32) -> Weight;
	fn remove_recipient(x: u32) -> Weight;
	fn enable_airdrop(x: u32) -> Weight;
	fn disable_airdrop(x: u32) -> Weight;
	fn claim(x: u32) -> Weight;
}

impl WeightInfo for () {
	fn create_airdrop(_x: u32) -> Weight {
		0 as Weight
	}

	fn add_recipient(_x: u32) -> Weight {
		0 as Weight
	}

	fn remove_recipient(_x: u32) -> Weight {
		0 as Weight
	}

	fn enable_airdrop(_x: u32) -> Weight {
		0 as Weight
	}

	fn disable_airdrop(_x: u32) -> Weight {
		0 as Weight
	}

	fn claim(_x: u32) -> Weight {
		0 as Weight
	}
}
