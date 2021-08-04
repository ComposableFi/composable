pub mod pyth;
use serde::Serialize;

use crate::asset::AssetPair;

#[derive(Serialize, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct TimeStamp(pub(crate) i64);

#[derive(Serialize, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Price(pub(crate) u64);

#[derive(Serialize, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Exponent(pub(crate) i32);

#[derive(Serialize, Copy, Clone, Debug)]
pub struct TimeStamped<T> {
    pub value: T,
    pub timestamp: TimeStamp,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Feed {
    Pyth,
}

#[derive(Debug)]
pub enum FeedNotification {
    Opened(Feed, AssetPair),
    Closed(Feed, AssetPair),
    PriceUpdated(Feed, AssetPair, TimeStamped<(Price, Exponent)>),
}
