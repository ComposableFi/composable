//! CurrencyId implementation

use codec::{Decode, Encode};
use composable_traits::currency::{DynamicCurrencyId, Exponent, PriceableAsset};
use scale_info::TypeInfo;
use sp_runtime::{ArithmeticError, DispatchError, RuntimeDebug};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::sp_std::ops::Deref;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CurrencyId(u128);

impl CurrencyId {
	pub const INVALID: CurrencyId = CurrencyId(0);
	pub const PICA: CurrencyId = CurrencyId(1);
	pub const LAYR: CurrencyId = CurrencyId(2);
	pub const CROWD_LOAN: CurrencyId = CurrencyId(3);
	pub const LOCAL_LP_TOKEN_START: CurrencyId = CurrencyId(u128::MAX / 2);
}

impl PriceableAsset for CurrencyId {
	#[inline]
	fn smallest_unit_exponent(self) -> Exponent {
		match self {
			// NOTE(hussein-aitlahcen): arbitrary, can we please determine this in the PR?
			CurrencyId::PICA => 8,
			CurrencyId::LAYR => 8,
			CurrencyId::CROWD_LOAN => 8,
			_ => 0,
		}
	}
}

// NOTE(hussein-aitlahcen): we could add an index to DynamicCurrency to differentiate sub-ranges
// This implementation is only valid if the initial value used to step using next is
// LOCAL_LP_TOKEN_START
impl DynamicCurrencyId for CurrencyId {
	#[inline]
	fn next(self) -> Result<Self, sp_runtime::DispatchError> {
		let CurrencyId(x) = self;
		let y = x.checked_add(1).ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?;
		Ok(CurrencyId(y))
	}
}

impl Default for CurrencyId {
	#[inline]
	fn default() -> Self {
		CurrencyId::INVALID
	}
}

impl Deref for CurrencyId {
	type Target = u128;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<CurrencyId> for u128 {
	#[inline]
	fn from(id: CurrencyId) -> Self {
		id.0
	}
}

impl From<u128> for CurrencyId {
	#[inline]
	fn from(raw: u128) -> Self {
		CurrencyId(raw)
	}
}
