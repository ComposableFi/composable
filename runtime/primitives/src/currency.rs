//! CurrencyId implementation
use codec::{CompactAs, Decode, Encode, MaxEncodedLen};
use composable_traits::currency::Exponent;
use composable_traits::assets::Asset;
use core::ops::Div;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

use composable_support::rpc_helpers::FromHexStr;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::sp_std::ops::Deref;

/// Trait used to write generalized code over well know currencies
/// We use const to allow for match on these
/// Allows to have reuse of code amids runtime and cross relay transfers in future.
// TODO: split CurrenyId for runtimes - one for DOT and one for KSM
pub trait WellKnownCurrency {
	// works well with pattnrs unlike impl trait `associated consts cannot be referenced in
	// patterns`
	const NATIVE: Self;
	/// usually we expect running with relay,
	/// but if  not, than degenrative case would be this equal to `NATIVE`
	const RELAY_NATIVE: Self;
}

#[derive(
	Encode,
	Decode,
	MaxEncodedLen,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	TypeInfo,
	CompactAs,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CurrencyId(pub u128);

impl WellKnownCurrency for CurrencyId {
	const NATIVE: CurrencyId = CurrencyId::PICA;
	const RELAY_NATIVE: CurrencyId = CurrencyId::KSM;
}

impl CurrencyId {
	// NOTE: Make sure to update list_assets when adding or removing assets
	pub const INVALID: CurrencyId = CurrencyId(0);
	pub const PICA: CurrencyId = CurrencyId(1);
	pub const LAYR: CurrencyId = CurrencyId(2);
	pub const CROWD_LOAN: CurrencyId = CurrencyId(3);

	/// Kusama native token
	pub const KSM: CurrencyId = CurrencyId(4);

	/// Karura stable coin(Karura Dollar), not native.
	#[allow(non_upper_case_globals)]
	pub const kUSD: CurrencyId = CurrencyId(129);

	#[inline(always)]
	pub const fn decimals() -> Exponent {
		12
	}
	pub fn unit<T: From<u64>>() -> T {
		T::from(10_u64.pow(Self::decimals()))
	}
	pub fn milli<T: From<u64> + Div<Output = T>>() -> T {
		Self::unit::<T>() / T::from(1000_u64)
	}
	
	#[cfg(feature="std")]
	pub fn list_assets() -> Vec<Asset> {
	   vec![
			Asset{
               id: CurrencyId::PICA.0,
			   name: "PICA".to_string(),
		    },
			Asset{
				id: CurrencyId::LAYR.0,
				name: "LAYR".to_string(),
			},
			Asset{
				id: CurrencyId::CROWD_LOAN.0,
				name: "CROWD_LOAN".to_string(),
			},
			Asset{
				id: CurrencyId::KSM.0,
				name: "KSM".to_string(),
			},
			Asset{
				id: CurrencyId::kUSD.0,
				name: "kUSD".to_string(),
			}
		]
	}
}

impl FromHexStr for CurrencyId {
	type Err = <u128 as FromHexStr>::Err;

	fn from_hex_str(src: &str) -> core::result::Result<Self, Self::Err> {
		u128::from_hex_str(src).map(CurrencyId)
	}
}

impl core::fmt::LowerHex for CurrencyId {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::LowerHex::fmt(&self.0, f)
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

mod ops {
	use super::CurrencyId;
	use core::ops::{Add, Mul};
	use sp_runtime::traits::{Bounded, CheckedAdd, CheckedMul, One, Saturating, Zero};

	impl Add for CurrencyId {
		type Output = Self;

		fn add(self, rhs: Self) -> Self::Output {
			CurrencyId(self.0.add(rhs.0))
		}
	}

	impl Mul for CurrencyId {
		type Output = CurrencyId;

		fn mul(self, rhs: Self) -> Self::Output {
			CurrencyId(self.0.mul(rhs.0))
		}
	}

	impl CheckedAdd for CurrencyId {
		fn checked_add(&self, v: &Self) -> Option<Self> {
			Some(CurrencyId(self.0.checked_add(v.0)?))
		}
	}

	impl CheckedMul for CurrencyId {
		fn checked_mul(&self, v: &Self) -> Option<Self> {
			Some(CurrencyId(self.0.checked_mul(v.0)?))
		}
	}

	impl Zero for CurrencyId {
		fn zero() -> Self {
			CurrencyId(0)
		}

		fn is_zero(&self) -> bool {
			self.0.is_zero()
		}
	}

	impl One for CurrencyId {
		fn one() -> Self {
			CurrencyId(u128::one())
		}
	}

	impl Bounded for CurrencyId {
		fn min_value() -> Self {
			CurrencyId(u128::min_value())
		}

		fn max_value() -> Self {
			CurrencyId(u128::max_value())
		}
	}

	impl Saturating for CurrencyId {
		fn saturating_add(self, rhs: Self) -> Self {
			self.0.saturating_add(rhs.0).into()
		}

		fn saturating_sub(self, rhs: Self) -> Self {
			<u128 as Saturating>::saturating_sub(self.0, rhs.0).into()
		}

		fn saturating_mul(self, rhs: Self) -> Self {
			<u128 as Saturating>::saturating_mul(self.0, rhs.0).into()
		}

		fn saturating_pow(self, exp: usize) -> Self {
			<u128 as Saturating>::saturating_pow(self.0, exp).into()
		}
	}
}
