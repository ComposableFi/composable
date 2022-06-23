use primitives::currency::CurrencyId;

#[allow(non_snake_case)]
pub const fn AssetId(id: u128) -> AssetId {
	CurrencyId(id)
}

pub type AssetId = CurrencyId;
pub const PICA: AssetId = CurrencyId::PICA;
pub const USDC: AssetId = CurrencyId::USDC;
pub const BTC: AssetId = AssetId(2000);
pub const LAYR: AssetId = CurrencyId::LAYR;
pub const DOT: AssetId = AssetId(4000);
pub const ETH: AssetId = AssetId(5000);

pub const ASSETS_WITH_USDC: [AssetId; 5] = [PICA, USDC, BTC, LAYR, DOT];
pub const ASSETS: [AssetId; 4] = [PICA, BTC, LAYR, DOT];
