//! CurrencyId implementation
use codec::{CompactAs, Decode, Encode, EncodeLike, MaxEncodedLen, WrapperTypeEncode};
use composable_support::validation::Validate;
use composable_traits::{assets::Asset, currency::Exponent};

use scale_info::TypeInfo;
use sp_runtime::{
	sp_std::{ops::Deref, vec::Vec},
	DispatchError, RuntimeDebug,
};

use crate::prelude::*;

use serde::{Deserialize, Serialize};

use xcm::{latest::prelude::*, v3};

pub trait WellKnownCurrency {
	const NATIVE: CurrencyId;
	const RELAY_NATIVE: CurrencyId;

	fn local_to_remote(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			id if id == Self::NATIVE => Some(MultiLocation::here()),
			id if id == Self::RELAY_NATIVE => Some(MultiLocation::parent()),
			_ => None,
		}
	}

	fn remote_to_local(id: MultiLocation) -> Option<CurrencyId> {
		match id {
			MultiLocation { parents: 0, interior: Junctions::Here } => Some(Self::NATIVE),
			MultiLocation { parents: 1, interior: Junctions::Here } => Some(Self::RELAY_NATIVE),
			_ => None,
		}
	}
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
	Serialize,
	Deserialize,
)]
#[repr(transparent)]
#[serde(transparent)]
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

#[macro_export]
macro_rules! list_assets {
	(
		$(
			$(#[$attr:meta])*
			pub const $NAME:ident: CurrencyId = CurrencyId($id:literal $(, $decimals:expr )? );
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

		pub fn remote_decimals_for_local(id: CurrencyId) -> Option<Exponent> {
            match id {
				$(
						$( CurrencyId::$NAME => $decimals, )?
				)*
				_ => None,
			}
		}

		pub fn list_assets() -> Vec<Asset<CurrencyId, u128, VersionedMultiLocation>> {
			[
				$(Asset {
					id: CurrencyId::$NAME,
					name: Some(stringify!($NAME).as_bytes().to_vec()),
					symbol: Some(stringify!($NAME).as_bytes().to_vec()),
					ratio: None,
					decimals: Self::remote_decimals_for_local(CurrencyId::$NAME).unwrap_or(Self::decimals()),
					foreign_id: None,
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
		/// Native from Picasso
		pub const PICA: CurrencyId = CurrencyId(
			1
		);
		///  Native from Composable
		pub const LAYR: CurrencyId = CurrencyId(2);
		///  Native from Composable
		pub const COMPOSABLE_LAYR: CurrencyId = CurrencyId(79228162514264337593543950338);

		/// Kusama native token
		pub const KSM: CurrencyId = CurrencyId(
			4
		);

		// From Picasso
		pub const PBLO: CurrencyId = CurrencyId(
			5
		);

		/// DOT from Polkadot
		pub const DOT: CurrencyId = CurrencyId(6, None);
		pub const stDOT: CurrencyId = CurrencyId(7, None);
		pub const COMPOSABLE_DOT: CurrencyId = CurrencyId(79228162514264337593543950342);

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
			101
		);
		/// BIFROST BNC
		pub const BNC: CurrencyId = CurrencyId(102, None);
		/// BIFROST vKSM
		pub const vKSM: CurrencyId = CurrencyId(103, None);
		/// Moonriver MOVR
		pub const MOVR: CurrencyId = CurrencyId(104, None);

		/// Karura stable coin(Acala Dollar), not native.
		pub const kUSD: CurrencyId = CurrencyId(
			129
		);

		/// Statemine USDT
		pub const USDT: CurrencyId = CurrencyId(
			130,
			Some(6)
		);

		/// Statemint USDT
		pub const USDTP: CurrencyId = CurrencyId(
			140,
			Some(6)
		);

		pub const USDC: CurrencyId = CurrencyId(131, None);
		/// Wrapped BTC
		pub const wBTC: CurrencyId = CurrencyId(132, None);
		/// Wrapped ETH
		pub const wETH: CurrencyId = CurrencyId(133, None);

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

#[allow(clippy::large_enum_variant)]
#[derive(RuntimeDebug, Decode, Encode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ForeignAssetId {
	Xcm(VersionedMultiLocation),
	IbcIcs20(PrefixedDenom),
}

#[derive(
	Ord, PartialOrd, RuntimeDebug, Decode, Encode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VersionedMultiLocation {
	#[codec(index = 3)]
	V3(v3::MultiLocation),
}

impl From<VersionedMultiLocation> for ForeignAssetId {
	fn from(this: VersionedMultiLocation) -> Self {
		Self::Xcm(this)
	}
}

impl From<PrefixedDenom> for ForeignAssetId {
	fn from(this: PrefixedDenom) -> Self {
		Self::IbcIcs20(this)
	}
}

type InnerDenom = ibc_rs_scale::applications::transfer::PrefixedDenom;

#[derive(Debug, Decode, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(transparent))]
pub struct PrefixedDenom(pub InnerDenom);

impl FromStr for PrefixedDenom {
	type Err = DispatchError;
	fn from_str(s: &str) -> Result<Self, DispatchError> {
		InnerDenom::from_str(s)
			.map_err(|_| DispatchError::Other("PrefixedDenom parse failed"))
			.map(Self)
	}
}

impl WrapperTypeEncode for PrefixedDenom {}
impl EncodeLike for PrefixedDenom {}
impl core::ops::Deref for PrefixedDenom {
	type Target = InnerDenom;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<InnerDenom> for PrefixedDenom {
	fn from(this: InnerDenom) -> Self {
		Self(this)
	}
}

impl MaxEncodedLen for PrefixedDenom {
	fn max_encoded_len() -> usize {
		2048
	}
}
