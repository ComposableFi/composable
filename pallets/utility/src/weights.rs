use frame_support::weights::Weight;
use sp_std::marker::PhantomData; //PhantomData takes runtime as input | https://docs.rs/sp-std/3.0.0/sp_std/marker/struct.PhantomData.html

pub trait WeightInfo {
    fn batch(c: u32) -> Weight;
    fn as_derivative() -> Weight;
    fn batch_all(c: u32) -> Weight;
    fn batch_info(c: u32) -> Weight;  //Check final weight without sending anything
}

pub struct MyWeight<T>(PhantomData<T>); // use this with runtime as input
impl<T: frame_system::Config> WeightInfo for MyWeight<T> {
    fn batch(c: u32) -> Weight {
        (20779000 as Weight)
            // Standard Error: 1_000
            .saturating_add((1_080_000 as Weight).saturating_mul(c as Weight))
    }
    fn as_derivative() -> Weight {
        3994000 as Weight
    }
    fn batch_all(c: u32) -> Weight {
        (22183000 as Weight)
            // Standard Error: 1_000
            .saturating_add((1_506_000 as Weight).saturating_mul(c as Weight))
    }

    fn batch_info(c: u32) -> Weight {
        (22183000 as Weight)
            // Standard Error: 1_000
            .saturating_add((1_206_000 as Weight).saturating_mul(c as Weight))
    }

}
