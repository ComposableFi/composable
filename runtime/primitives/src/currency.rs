//! CurrencyId implementation
use codec::{CompactAs, Decode, Encode};
use composable_traits::currency::{DynamicCurrencyId, Exponent};
use core::ops::Div;
use scale_info::TypeInfo;
use sp_runtime::{ArithmeticError, DispatchError, RuntimeDebug};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::sp_std::ops::Deref;


impl CurrencyId {
	pub const INVALID: CurrencyId = CurrencyId(0);
	pub const PICA: CurrencyId = CurrencyId(1);
	pub const LAYR: CurrencyId = CurrencyId(2);
	pub const CROWD_LOAN: CurrencyId = CurrencyId(3);
	pub const KSM: CurrencyId = CurrencyId(4);
	pub const LOCAL_LP_TOKEN_START: CurrencyId = CurrencyId(u128::MAX / 2);

	#[inline(always)]
	pub fn decimals(&self) -> Exponent {
		12
	}
	pub fn unit<T: From<u64>>(&self) -> T {
		T::from(10_u64.pow(self.decimals()))
	}
	pub fn milli<T: From<u64> + Div<Output = T>>(&self) -> T {
		self.unit::<T>() / T::from(1000_u64)
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

/// maps id to junction generic key,
/// unfortunately it is the best way to encode currency id as of now in XCM
#[cfg(feature = "develop")]
impl From<CurrencyId> for xcm::latest::Junction {
	fn from(this: CurrencyId) -> Self {
		xcm::latest::Junction::GeneralKey(this.encode())
	}
}
