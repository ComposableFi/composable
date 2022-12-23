//! CurrencyId implementation
use codec::{CompactAs, Decode, Encode, MaxEncodedLen};
use composable_support::validation::Validate;
use composable_traits::{assets::Asset, currency::Exponent, xcm::assets::XcmAssetLocation};
use core::{fmt::Display, ops::Div, str::FromStr};
use frame_support::WeakBoundedVec;
use scale_info::TypeInfo;
use sp_runtime::{
	sp_std::{ops::Deref, vec::Vec},
	RuntimeDebug,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::topology;
use xcm::latest::prelude::*;

/// Trait used to write generalized code over well know currencies
/// We use const to allow for match on these
/// Allows to have reuse of code amids runtime and cross relay transfers in future.
// TODO: split CurrencyId for runtimes - one for DOT and one for KSM
pub trait WellKnownCurrency {
	// works well with patterns unlike impl trait `associated consts cannot be referenced in
	// patterns`
	const NATIVE: Self;
	/// usually we expect running with relay,
	/// but if  not, than degenerative case would be this equal to `NATIVE`
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
	Hash,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CurrencyId(pub u128);

impl FromStr for CurrencyId {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		u128::from_str(s).map(CurrencyId).map_err(|_| ())
	}
}

impl Display for CurrencyId {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let CurrencyId(id) = self;
		write!(f, "{}", id)
	}
}

impl WellKnownCurrency for CurrencyId {
	const NATIVE: CurrencyId = CurrencyId::PICA;
	const RELAY_NATIVE: CurrencyId = CurrencyId::KSM;
}

#[macro_export]
macro_rules! list_assets {
	(
		$(
			$(#[$attr:meta])*
			pub const $NAME:ident: CurrencyId = CurrencyId($id:literal $(, $location:expr $(, $decimals:expr )?)? );
		)*
	) => {
		$(
			$(#[$attr])*
			pub const $NAME: CurrencyId = CurrencyId($id);
		)*

		pub fn native_asset_name(id: u128) -> Result<&'static str, &'static str> {
			match id {
				$($id => Ok(stringify!($NAME)),)*
				_ => Err("Invalid native asset")
			}
		}

		pub fn to_native_id(name: &str) -> Result<CurrencyId, &'static str> {
			match name {
				$(stringify!($NAME) => Ok(CurrencyId::$NAME),)*
				_ => Err("Invalid native asset")
			}
		}

		pub fn local_to_xcm_reserve(id: CurrencyId) -> Option<xcm::latest::MultiLocation> {
            match id {
				$(
					$( CurrencyId::$NAME => $location, )?
				)*
				_ => None,
			}
		}

		pub fn remote_decimals_for_local(id: CurrencyId) -> Option<Exponent> {
            match id {
				$(
					$(
						$( CurrencyId::$NAME => $decimals, )?
					)?
				)*
				_ => None,
			}
		}

		pub fn xcm_reserve_to_local(remote_id: xcm::latest::MultiLocation) -> Option<CurrencyId> {
			use lazy_static::lazy_static;
			use sp_std::collections::btree_map::BTreeMap;

			lazy_static! {
				static ref XCM_ASSETS: BTreeMap<Vec<u8>, CurrencyId> = {
					let mut map = BTreeMap::new();
					$(
						$(
							let xcm_id: Option<xcm::latest::MultiLocation> = $location;
							if let Some(xcm_id) = xcm_id {
								map.insert(xcm_id.encode(), CurrencyId::$NAME);
							}
						)?
					)*
					map
				};
			}
			XCM_ASSETS.get(&remote_id.encode()).map(|x| *x)
		}

		pub fn list_assets() -> Vec<Asset<u128, XcmAssetLocation>> {
			[
				$(Asset {
					id: CurrencyId::$NAME.0 as u128,
					name: Some(stringify!($NAME).as_bytes().to_vec()),
					ratio: None,
					decimals: Self::remote_decimals_for_local(CurrencyId::$NAME).unwrap_or(Self::decimals()),
					foreign_id: Self::local_to_xcm_reserve(CurrencyId::$NAME).map(XcmAssetLocation::new),
					existential_deposit: 0_u128,
				},)*
			]
			.to_vec()
		}
	}
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
impl CurrencyId {
	pub const INVALID: CurrencyId = CurrencyId(0);

	list_assets! {
		// Native Tokens (1 - 100)
		/// Runtime native token Kusama
		pub const PICA: CurrencyId = CurrencyId(
			1,
			Some(topology::this::LOCAL)
		);
		/// Runtime native token Polkadot
		pub const LAYR: CurrencyId = CurrencyId(2);

		/// Kusama native token
		pub const KSM: CurrencyId = CurrencyId(
			4,
			Some(MultiLocation::parent())
		);
		pub const PBLO: CurrencyId = CurrencyId(
			5,
			Some(MultiLocation {
				parents: 0,
				interior: X1(GeneralIndex(5)),
			})
		);

		pub const ibcDOT: CurrencyId = CurrencyId(6, None);

		pub const KSM_USDT_LPT: CurrencyId = CurrencyId(105, None);
		pub const PICA_USDT_LPT: CurrencyId = CurrencyId(106, None);
		pub const PICA_KSM_LPT: CurrencyId = CurrencyId(107, None);

		/// Staked asset xPICA Token
		pub const xPICA: CurrencyId = CurrencyId(1001, None);
		/// Staked asset xLAYR Token
		pub const xLAYR: CurrencyId = CurrencyId(1002, None);

		/// Staked asset xPBLO Token
		pub const xPBLO: CurrencyId = CurrencyId(1005, None);

		// fNFT Collection IDs (2001 - 100_000_000_000)
		/// PICA Stake fNFT Collection
		pub const PICA_STAKE_FNFT_COLLECTION: CurrencyId = CurrencyId(2001, None);
		/// PBLO Stake fNFT Collection
		pub const PBLO_STAKE_FNFT_COLLECTION: CurrencyId = CurrencyId(2005, None);

		// Non-Native Tokens (101 - 1000)
		/// Karura KAR
		pub const KAR: CurrencyId = CurrencyId(
			101,
			Some(MultiLocation {
				parents: 1,
				interior: X2(
					Parachain(topology::karura::ID),
					GeneralKey(WeakBoundedVec::force_from(
						topology::karura::KAR_KEY.to_vec(),
						None,
					)),
				),
			})
		);
		/// BIFROST BNC
		pub const BNC: CurrencyId = CurrencyId(102, None);
		/// BIFROST vKSM
		pub const vKSM: CurrencyId = CurrencyId(103, None);
		/// Moonriver MOVR
		pub const MOVR: CurrencyId = CurrencyId(104, None);

		/// Karura stable coin(Acala Dollar), not native.
		pub const kUSD: CurrencyId = CurrencyId(
			129,
			Some(MultiLocation {
				parents: 1,
				interior: X2(
					Parachain(topology::karura::ID),
					GeneralKey(WeakBoundedVec::force_from(
						topology::karura::AUSD_KEY.to_vec(),
						None,
					)),
				),
			})
		);

		/// Statemine USDT
		pub const USDT: CurrencyId = CurrencyId(
			130,
			Some(MultiLocation {
				parents: 1,
				interior: X3(
					Parachain(topology::common_good_assets::ID),
					PalletInstance(topology::common_good_assets::ASSETS),
					GeneralIndex(topology::common_good_assets::USDT),
				),
			}),
			Some(6)
		);
		pub const USDC: CurrencyId = CurrencyId(131, None);
		/// Wrapped BTC
		pub const wBTC: CurrencyId = CurrencyId(132, None);
		/// Wrapped ETH
		pub const wETH: CurrencyId = CurrencyId(133, None);

		// Staked asset xTokens (1001 - 2000)

		/// Staked asset xKSM Token
		pub const xKSM: CurrencyId = CurrencyId(1004, None);
	}

	#[inline(always)]
	pub const fn decimals() -> Exponent {
		12
	}

	pub fn unit<T: From<u64>>() -> T {
		T::from(10_u64.pow(Self::decimals().into()))
	}

	pub fn milli<T: From<u64> + Div<Output = T>>() -> T {
		Self::unit::<T>() / T::from(1000_u64)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, TypeInfo)]
pub struct ValidateCurrencyId;

impl Validate<CurrencyId, ValidateCurrencyId> for ValidateCurrencyId {
	fn validate(input: CurrencyId) -> Result<CurrencyId, &'static str> {
		if input != CurrencyId::INVALID {
			Ok(input)
		} else {
			Err("Invalid Currency")
		}
	}
}

impl Validate<u64, ValidateCurrencyId> for ValidateCurrencyId {
	fn validate(input: u64) -> Result<u64, &'static str> {
		if input != 0_u64 {
			Ok(input)
		} else {
			Err("Invalid Currency")
		}
	}
}

impl Validate<u128, ValidateCurrencyId> for ValidateCurrencyId {
	fn validate(input: u128) -> Result<u128, &'static str> {
		if input != 0_u128 {
			Ok(input)
		} else {
			Err("Invalid Currency")
		}
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

impl From<CurrencyId> for xcm::latest::Junction {
	fn from(this: CurrencyId) -> Self {
		xcm::latest::Junction::GeneralIndex(this.0)
	}
}

#[cfg(test)]
mod common_sense {
	use super::*;

	#[test]
	fn no_wrong_map() {
		assert_eq!(
			CurrencyId::xcm_reserve_to_local(MultiLocation {
				parents: 1,
				interior: X3(Parachain(1000), PalletInstance(50), GeneralIndex(666))
			}),
			None
		);
	}

	#[test]
	fn one_right_map() {
		let decimals = CurrencyId::remote_decimals_for_local(
			CurrencyId::xcm_reserve_to_local(MultiLocation {
				parents: 1,
				interior: X3(Parachain(1000), PalletInstance(50), GeneralIndex(1984)),
			})
			.unwrap(),
		)
		.unwrap();

		assert_eq!(decimals, 6);
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
