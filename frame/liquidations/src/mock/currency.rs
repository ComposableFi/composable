use frame_support::parameter_types;

pub type CurrencyId = u128;

pub const PICA: CurrencyId = 1;
pub const KSM: CurrencyId = 4;
pub const KUSD: CurrencyId = 129;

parameter_types! {
	pub const NativeAssetId: CurrencyId = 0;
}
