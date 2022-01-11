use frame_support::dispatch::Weight;

pub trait WeightInfo {
    fn add_liqudation_strategy() -> Weight {
        10000
    }
}