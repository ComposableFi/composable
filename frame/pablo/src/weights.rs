#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn create() -> Weight;
	fn create_lbp() -> Weight;
	fn add_liquidity() -> Weight;
	fn add_liquidity_lbp() -> Weight;
	fn remove_liquidity() -> Weight;
	fn remove_liquidity_lbp() -> Weight;
	fn buy() -> Weight;
	fn sell() -> Weight;
	fn swap() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create() -> Weight {10_000}
	fn create_lbp() -> Weight {10_000}
	fn add_liquidity() -> Weight {10_000}
	fn add_liquidity_lbp() -> Weight {10_000}
	fn remove_liquidity() -> Weight {10_000}
	fn remove_liquidity_lbp() -> Weight {10_000}
	fn buy() -> Weight {10_000}
	fn sell() -> Weight {10_000}
	fn swap() -> Weight {10_000}
}
