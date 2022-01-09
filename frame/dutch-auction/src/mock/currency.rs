use composable_traits::currency::{DynamicCurrencyId, PriceableAsset};
use frame_support::parameter_types;
use scale_info::TypeInfo;
use sp_runtime::{ArithmeticError, DispatchError};

#[derive(
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Copy,
	Clone,
	codec::Encode,
	codec::Decode,
	serde::Serialize,
	serde::Deserialize,
	TypeInfo,
)]
#[allow(clippy::upper_case_acronyms)] // Currencies should be in CONSTANT_CASE
pub enum CurrencyId {
	PICA,
	BTC,
	ETH,
	LTC,
	USDT,
	LpToken(u128),
}

impl Default for CurrencyId {
	fn default() -> Self {
		CurrencyId::PICA
	}
}

impl PriceableAsset for CurrencyId {
	fn decimals(&self) -> composable_traits::currency::Exponent {
		match self {
			CurrencyId::PICA => 0,
			CurrencyId::BTC => 8,
			CurrencyId::ETH => 18,
			CurrencyId::LTC => 8,
			CurrencyId::USDT => 2,
			CurrencyId::LpToken(_) => 0,
		}
	}
}

impl DynamicCurrencyId for CurrencyId {
	fn next(self) -> Result<Self, DispatchError> {
		match self {
			CurrencyId::LpToken(x) => Ok(CurrencyId::LpToken(
				x.checked_add(1).ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)),
			_ => unreachable!(),
		}
	}
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: CurrencyId = CurrencyId::PICA;
}
