use frame_support::parameter_types;

pub type CurrencyId = u128;

#[allow(dead_code)]
pub const INVALID: CurrencyId = 0;
pub const PICA: CurrencyId = 1;
pub const KUSD: CurrencyId = 129;

parameter_types! {
	pub const NativeAssetId: CurrencyId = 1;
}
