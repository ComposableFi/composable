/// Bounded collections types. For inputs of extrinsics, you'll want these 99% of the time.
pub mod bounded;

/// Sorted collection types, useful for keeping data in a valid state through the type system.
pub mod sorted;

pub use bounded::BoundedSortedVec;
pub use sorted::SortedVec;
