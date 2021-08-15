use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    num::ParseIntError,
    str::FromStr,
};

custom_derive! {
    #[derive(EnumFromStr, Copy, Clone, PartialEq, Eq, Hash, Debug)]
    pub enum Asset {
        BTC,
        ETH,
        LTC,
        DOGE,
        SOL,
        LUNA,
        AAPL,
        BNB,
        TSLA,
        BCH,
        SRM,
        AMZN,
        GOOG,
        NFLX,
        XAU,
        AMC,
        SPY,
        GME,
        GE,
        QQQ,
        USDT,
        USDC,
        GBP,
        EUR,
        USD,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AssetPair(pub Asset, pub Asset);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
#[repr(transparent)]
pub struct AssetIndex(u8);

pub enum AssetIndexError {
    NotANumber(ParseIntError),
    AssetNotFound,
}

lazy_static! {
    /*
      The map of valid asset we are allowed to ask price for.
      We must not swap two indexes.
    */
    pub static ref INDEX_TO_ASSET: HashMap<AssetIndex, Asset> = [
        (0, Asset::BTC),
        (1, Asset::ETH),
        (2, Asset::LTC),
        (3, Asset::DOGE),
        (4, Asset::SOL),
        (5, Asset::LUNA),
        (6, Asset::AAPL),
        (7, Asset::BNB),
        (6, Asset::TSLA),
        (7, Asset::BCH),
        (8, Asset::SRM),
        (9, Asset::AMZN),
        (10, Asset::GOOG),
        (11, Asset::NFLX),
        (12, Asset::XAU),
        (13, Asset::AMC),
        (14, Asset::SPY),
        (15, Asset::GME),
        (16, Asset::GE),
        (17, Asset::QQQ),
        (18, Asset::USDT),
        (19, Asset::USDC),
        (20, Asset::GBP),
        (21, Asset::EUR),
    ]
    .iter()
    .map(|&(i, a)| (AssetIndex(i), a))
    .collect();
    pub static ref ASSET_TO_INDEX: HashMap<Asset, AssetIndex> =
        INDEX_TO_ASSET.iter().map(|(&i, &a)| (a, i)).collect();
    pub static ref VALID_ASSETS: HashSet<Asset> =
        INDEX_TO_ASSET.values().copied().collect();
}

impl FromStr for AssetIndex {
    type Err = AssetIndexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let asset_pair_index =
            AssetIndex(FromStr::from_str(s).map_err(AssetIndexError::NotANumber)?);
        if INDEX_TO_ASSET.contains_key(&asset_pair_index) {
            Ok(asset_pair_index)
        } else {
            Err(AssetIndexError::AssetNotFound)
        }
    }
}

impl AssetPair {
    /*
      We currently only allow X/USD
    */
    pub fn new(x: Asset, y: Asset) -> Option<Self> {
        match (x, y) {
            (Asset::USD, _) => None,
            (_, Asset::USD) => Some(AssetPair(x, y)),
            _ => None,
        }
    }

    pub fn symbol(&self) -> String {
        format!("{:?}/{:?}", self.0, self.1)
    }
}

impl TryFrom<Asset> for AssetIndex {
    type Error = ();
    fn try_from(asset: Asset) -> Result<AssetIndex, Self::Error> {
        ASSET_TO_INDEX.get(&asset).copied().ok_or(())
    }
}

impl TryFrom<AssetIndex> for Asset {
    type Error = ();
    fn try_from(asset_index: AssetIndex) -> Result<Asset, Self::Error> {
        INDEX_TO_ASSET.get(&asset_index).copied().ok_or(())
    }
}
