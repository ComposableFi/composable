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
