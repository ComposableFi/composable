pub mod pyth;
use serde::{Deserialize, Serialize};

use crate::asset::AssetPair;

#[derive(Serialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct TimeStamp(pub(crate) i64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Price(pub(crate) u64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Exponent(pub(crate) i32);

#[derive(Serialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct TimeStamped<T> {
    pub value: T,
    pub timestamp: TimeStamp,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Feed {
    Pyth,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum FeedNotification {
    Opened(Feed, AssetPair),
    Closed(Feed, AssetPair),
    PriceUpdated(Feed, AssetPair, TimeStamped<(Price, Exponent)>),
}
