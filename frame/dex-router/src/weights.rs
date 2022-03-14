#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
  fn update_route() -> Weight;
  fn exchange() -> Weight;
  fn sell() -> Weight;
  fn buy() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn update_route() -> Weight {
        10_000
    }

    fn buy() -> Weight  {
        10_000
    }

    fn sell() -> Weight {
        10_000
    }

    fn exchange() -> Weight {
        10_000
    }
}
