use primitives::currency::CurrencyId;
use std::{
	collections::{HashMap, HashSet},
	convert::TryFrom,
	fmt::Display,
};

custom_derive! {
	#[derive(EnumFromStr, Copy, Clone, PartialEq, Eq, Hash, Debug)]
	pub enum Asset {
		KSM,
		PICA,
		USDT,
		USDC,
	}
}

pub const VALID_PRICE_QUOTE_ASSETS: &[Asset] = &[Asset::USDT, Asset::USDC];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AssetPair(pub Asset, pub Asset);

// NOTE: sure it is better move it into primitives
lazy_static! {
	/*
	  The map of valid asset we are allowed to ask price for.
	  We must not swap two indexes.
	*/
	pub static ref INDEX_TO_ASSET: HashMap<CurrencyId, Asset> = [
		(CurrencyId::KSM, Asset::KSM),
		(CurrencyId::PICA, Asset::PICA),
		(CurrencyId::USDT, Asset::USDT),
		(CurrencyId::USDC, Asset::USDC),
	]
	.into_iter()
	.collect();

	pub static ref ASSET_TO_INDEX: HashMap<Asset, CurrencyId> =
		INDEX_TO_ASSET.iter().map(|(&i, &a)| (a, i)).collect();

	pub static ref VALID_ASSETS: HashSet<Asset> =
		INDEX_TO_ASSET.values().copied().collect();
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

impl TryFrom<Asset> for CurrencyId {
	type Error = ();
	fn try_from(asset: Asset) -> Result<CurrencyId, Self::Error> {
		ASSET_TO_INDEX.get(&asset).copied().ok_or(())
	}
}

impl TryFrom<CurrencyId> for Asset {
	type Error = ();
	fn try_from(currency_index: CurrencyId) -> Result<Asset, Self::Error> {
		INDEX_TO_ASSET.get(&currency_index).copied().ok_or(())
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
