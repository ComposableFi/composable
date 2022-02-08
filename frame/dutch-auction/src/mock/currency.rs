use frame_support::parameter_types;

pub type CurrencyId = u128;

pub const PICA: CurrencyId = 0;
pub const BTC: CurrencyId = 1;
pub const USDT: CurrencyId = 2;

parameter_types! {
	pub const NativeAssetId: CurrencyId = 0;
}
