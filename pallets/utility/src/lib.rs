use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};


// Weights ready to use
pub trait WeightInfo {
	fn batch(c: u32, ) -> Weight;
	fn as_derivative() -> Weight;
	fn batch_all(c: u32, ) -> Weight;
}

// Rest of utilities


