#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> lending::weights::WeightInfo for WeightInfo<T> {
        fn create_new_market() -> Weight {
            (96_881_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(5 as Weight))
                .saturating_add(T::DbWeight::get().writes(11 as Weight))
        }
        fn deposit_collateral() -> Weight {
            123_789_000_u64
                .saturating_add(T::DbWeight::get().reads(6_u64))
                .saturating_add(T::DbWeight::get().writes(4_u64))
        }
        fn withdraw_collateral() -> Weight {
            138_802_000_u64
                .saturating_add(T::DbWeight::get().reads(10_u64))
                .saturating_add(T::DbWeight::get().writes(3_u64))
        }
        fn borrow() -> Weight {
            (332_730_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(19 as Weight))
                .saturating_add(T::DbWeight::get().writes(9 as Weight))
        }
        fn repay_borrow() -> Weight {
            (209_694_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(13 as Weight))
                .saturating_add(T::DbWeight::get().writes(6 as Weight))
        }
        fn now() -> Weight {
            (4_744_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
        }
        fn accrue_interest() -> Weight {
            (76_626_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(8 as Weight))
                .saturating_add(T::DbWeight::get().writes(1 as Weight))
        }
        fn account_id() -> Weight {
            (3_126_000 as Weight)
        }
        fn available_funds() -> Weight {
            (16_450_000 as Weight).saturating_add(T::DbWeight::get().reads(2 as Weight))
        }
        fn handle_withdrawable() -> Weight {
            (20_716_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(2 as Weight))
                .saturating_add(T::DbWeight::get().writes(1 as Weight))
        }
        fn handle_depositable() -> Weight {
            (40_066_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(3 as Weight))
                .saturating_add(T::DbWeight::get().writes(1 as Weight))
        }
        fn handle_must_liquidate() -> Weight {
            (38_744_000 as Weight)
                .saturating_add(T::DbWeight::get().reads(3 as Weight))
                .saturating_add(T::DbWeight::get().writes(1 as Weight))
        }

        fn liquidate(positions_count: Weight) -> Weight {
            10000 * positions_count
        }
}