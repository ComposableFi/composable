use serde::Serialize;
use std::{
	collections::{HashMap, HashSet},
	convert::TryFrom,
	fmt::Display,
	num::ParseIntError,
	str::FromStr,
};

custom_derive! {
	#[derive(EnumFromStr, Copy, Clone, PartialEq, Eq, Hash, Debug)]
	pub enum Asset {
		KSM,
		USDT,
		USDC,
	}
}

pub const VALID_PRICE_QUOTE_ASSETS: &[Asset] = &[Asset::USDT, Asset::USDC];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AssetPair(pub Asset, pub Asset);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize)]
#[repr(transparent)]
pub struct AssetIndex(u128);

impl core::fmt::Display for AssetIndex {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", self.0)
	}
}

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
		(primitives::currency::CurrencyId::KSM.0, Asset::KSM),
		(18_u128, Asset::USDT),
		(19_u128, Asset::USDC),
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
	  We currently only allow X/(USD|Stablecoin)
	*/
	pub fn new(x: Asset, y: Asset) -> Option<Self> {
		match (x, y) {
			(_, y) if VALID_PRICE_QUOTE_ASSETS.contains(&y) => Some(AssetPair(x, y)),
			_ => None,
		}
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

/// A symbol which is the concatenation of two assets.
/// Like BTCUSD, ETHBTC...
pub struct ConcatSymbol(AssetPair);

impl ConcatSymbol {
	#[inline(always)]
	pub fn new(x: AssetPair) -> Self {
		ConcatSymbol(x)
	}
}

impl Display for ConcatSymbol {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let ConcatSymbol(AssetPair(x, y)) = self;
		write!(f, "{:?}{:?}", x, y)
	}
}

/// A symbol which is the concatenation of an two assets with a slash in between.
/// Like BTC/USD, ETH/BTC...
pub struct SlashSymbol(AssetPair);

impl SlashSymbol {
	#[inline(always)]
	pub fn new(x: AssetPair) -> Self {
		SlashSymbol(x)
	}
}

impl Display for SlashSymbol {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let SlashSymbol(AssetPair(x, y)) = self;
		write!(f, "{:?}/{:?}", x, y)
	}
}
