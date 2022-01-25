use composable_traits::currency::DynamicCurrencyId;
use frame_support::parameter_types;
use scale_info::TypeInfo;
use sp_runtime::{ArithmeticError, DispatchError};

#[derive(
	PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Clone, codec::Encode, codec::Decode, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
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
	pub const NativeAssetId: CurrencyId = CurrencyId::PICA;
}
