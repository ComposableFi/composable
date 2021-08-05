use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    hash::Hasher,
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
pub struct AssetPair(Asset, Asset);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
#[repr(transparent)]
pub struct AssetPairHash(u64);

pub enum AssetPairHashError {
    NotANumber(ParseIntError),
    AssetNotFound,
}

lazy_static! {
    pub static ref VALID_ASSETPAIRS: Vec<AssetPair> = {
        [
            Asset::BTC,
            Asset::ETH,
            Asset::LTC,
            Asset::DOGE,
            Asset::SOL,
            Asset::LUNA,
            Asset::AAPL,
            Asset::BNB,
            Asset::TSLA,
            Asset::BCH,
            Asset::SRM,
            Asset::AMZN,
            Asset::GOOG,
            Asset::NFLX,
            Asset::XAU,
            Asset::AMC,
            Asset::SPY,
            Asset::GME,
            Asset::GE,
            Asset::QQQ,
            Asset::USDT,
            Asset::USDC,
            Asset::GBP,
            Asset::EUR,
        ]
        .iter()
        .map(|&x| AssetPair(x, Asset::USD))
        .collect()
    };
    pub static ref ASSETPAIR_HASHES: HashMap<AssetPair, AssetPairHash> =
        VALID_ASSETPAIRS.iter().map(|&x| (x, x.hash())).collect();
    pub static ref ASSETPAIR_HASHES_VALUES: HashSet<AssetPairHash> =
        ASSETPAIR_HASHES.iter().map(|(_, &h)| h).collect();
}

impl FromStr for AssetPairHash {
    type Err = AssetPairHashError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let asset_pair_hash =
            AssetPairHash(FromStr::from_str(s).map_err(AssetPairHashError::NotANumber)?);
        if ASSETPAIR_HASHES_VALUES.contains(&asset_pair_hash) {
            Ok(asset_pair_hash)
        } else {
            Err(AssetPairHashError::AssetNotFound)
        }
    }
}

impl AssetPair {
    pub(crate) fn new(x: Asset, y: Asset) -> Self {
        AssetPair(x, y)
    }

    pub fn hash(&self) -> AssetPairHash {
        // not secure but we only need this for indexing
        let mut hasher = fnv::FnvHasher::default();
        hasher.write(self.symbol().as_bytes());
        AssetPairHash(hasher.finish())
    }

    pub fn symbol(&self) -> String {
        format!("{:?}/{:?}", self.0, self.1)
    }
}
