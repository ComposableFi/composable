use composable_traits::currency::DynamicCurrencyId;
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
#[allow(clippy::upper_case_acronyms)] // currencies should be CONSTANT_CASE
pub enum CurrencyId {
	PICA,
	BTC,
	ETH,
	LTC,
	USDT,
	LpToken(u128),
}

impl From<u128> for CurrencyId {
	fn from(id: u128) -> Self {
		match id {
			0 => CurrencyId::PICA,
			1 => CurrencyId::BTC,
			2 => CurrencyId::ETH,
			3 => CurrencyId::LTC,
			4 => CurrencyId::USDT,
			5 => CurrencyId::LpToken(0),
			_ => unreachable!(),
		}
	}
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
