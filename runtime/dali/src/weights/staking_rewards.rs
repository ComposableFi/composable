use std::marker::PhantomData;
use frame_support::dispatch::Weight;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_staking_rewards::weights::WeightInfo for WeightInfo<T> {
    fn create_reward_pool() -> Weight {
        10_000
    }
}
