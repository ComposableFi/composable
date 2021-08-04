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

pub type AssetPair = (Asset, Asset);

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
        .map(|&x| (x, Asset::USD))
        .collect()
    };
    pub static ref ASSETPAIR_HASHES: HashMap<AssetPair, AssetPairHash> =
        VALID_ASSETPAIRS.iter().map(|&x| (x, to_hash(&x))).collect();
    pub static ref VALID_ASSETPAIR_HASHES: HashSet<AssetPairHash> =
        ASSETPAIR_HASHES.iter().map(|(_, &h)| h).collect();
}

impl FromStr for AssetPairHash {
    type Err = AssetPairHashError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let asset_pair_hash =
            AssetPairHash(FromStr::from_str(s).map_err(|e| AssetPairHashError::NotANumber(e))?);
        if VALID_ASSETPAIR_HASHES.contains(&asset_pair_hash) {
            Ok(asset_pair_hash)
        } else {
            Err(AssetPairHashError::AssetNotFound)
        }
    }
}

pub fn to_hash(asset_pair: &AssetPair) -> AssetPairHash {
    // not secure but we only need this for indexing
    let mut hasher = fnv::FnvHasher::default();
    hasher.write(to_symbol(asset_pair).as_bytes());
    AssetPairHash(hasher.finish())
}

pub fn to_symbol((x, y): &AssetPair) -> String {
    format!("{:?}/{:?}", x, y)
}
