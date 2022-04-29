#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> dex_router::WeightInfo for WeightInfo<T> {
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

    fn add_liquidity() -> Weight {
        10_000
    }

    fn remove_liquidity() -> Weight {
        10_000
    }
}
