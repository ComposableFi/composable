use crate::{
    asset::AssetPairHash,
    feed::{Exponent, Price, TimeStamped},
};
use std::collections::HashMap;

pub type PriceCacheEntry = TimeStamped<(Price, Exponent)>;

pub type PriceCache = HashMap<AssetPairHash, PriceCacheEntry>;
