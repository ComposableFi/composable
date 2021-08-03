pub mod pyth;
use serde::Serialize;

use crate::asset::AssetPair;

#[derive(Serialize, Copy, Clone, Debug)]
pub struct TimeStamp(i64);

#[derive(Serialize, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Price(u64);

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
    PriceUpdated(Feed, AssetPair, TimeStamped<Price>),
}
