//! CurrencyId implementation

use codec::{Encode, Decode};
use sp_runtime::RuntimeDebug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum CurrencyId {
	Token(TokenSymbol),
	LpToken(u128)
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenSymbol {
	PICA,
	LAYR,
	Crowdloan,
}

impl From<u128> for CurrencyId {
	fn from(val: u128) -> Self {
		match val {
			0 => CurrencyId::Token(TokenSymbol::LAYR),
			1 => CurrencyId::Token(TokenSymbol::PICA),
			2 => CurrencyId::Token(TokenSymbol::Crowdloan),
			val => CurrencyId::LpToken(val),
		}
	}
}

impl From<CurrencyId> for u128 {
	fn from(val: CurrencyId) -> Self {
		match val {
			CurrencyId::Token(TokenSymbol::LAYR) => 0,
			CurrencyId::Token(TokenSymbol::PICA) => 1,
			CurrencyId::Token(TokenSymbol::Crowdloan) => 2,
			CurrencyId::LpToken(val) => val,
		}
	}
}
