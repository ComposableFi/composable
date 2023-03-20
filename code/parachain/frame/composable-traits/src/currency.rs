use codec::FullCodec;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_arithmetic::fixed_point::FixedU64;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Zero},
	ArithmeticError, FixedU128, Rational128,
};

use composable_support::math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub};
use sp_std::fmt::Debug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type Exponent = u8;

/// Creates a new asset, compatible with [`MultiCurrency`](https://docs.rs/orml-traits/0.4.0/orml_traits/currency/trait.MultiCurrency.html).
/// The implementor should ensure that a new `CurrencyId` is created and collisions are avoided.
/// Is about Local assets representations. These may differ remotely.
pub trait CurrencyFactory {
	type AssetId;
	type Balance;

	/// permissionless creation of new transferable asset id
	fn create(id: RangeId) -> Result<Self::AssetId, DispatchError>;
	fn reserve_lp_token_id() -> Result<Self::AssetId, DispatchError> {
		Self::create(RangeId::LP_TOKENS)
	}
	/// Given a `u32` ID (within the range of `0` to `u32::MAX`) returns a unique `AssetId` reserved
	/// by Currency Factory for the runtime.
	fn protocol_asset_id_to_unique_asset_id(
		protocol_asset_id: u32,
		range_id: RangeId,
	) -> Result<Self::AssetId, DispatchError>;

	fn unique_asset_id_to_protocol_asset_id(unique_asset_id: Self::AssetId) -> u32;
}

pub trait AssetExistentialDepositInspect {
	type AssetId;
	type Balance;

	/// Given an `asset_id`, returns the existential deposit of an asset in asset currency.
	fn existential_deposit(_asset_id: Self::AssetId) -> Result<Self::Balance, DispatchError> {
		Err(DispatchError::Other("unimplemented!"))
	}
}

pub trait AssetDataMutate {
	type AssetId;
	type Balance;
	fn update_existential_deposit(asset_id: Self::AssetId, ed: Option<Self::Balance>);
}

/// foreign_amount / native_amount
pub type ForeignByNative = Rational64;

pub trait AssetRatioInspect {
	type AssetId;
	fn get_ratio(_asset_id: Self::AssetId) -> Option<ForeignByNative> {
		None
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit<T>(PhantomData<T>);

impl<T> AssetRatioInspect for Unit<T> {
	type AssetId = T;
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct RangeId(u32);

impl RangeId {
	pub const LP_TOKENS: RangeId = RangeId(0);
	pub const TOKENS: RangeId = RangeId(1);
	pub const FOREIGN_ASSETS: RangeId = RangeId(2);
	pub const IBC_ASSETS: RangeId = RangeId(3);
	pub const FNFT_ASSETS: RangeId = RangeId(4);
	pub const XTOKEN_ASSETS: RangeId = RangeId(5);

	pub fn inner(&self) -> u32 {
		self.0
	}
}

impl From<u32> for RangeId {
	fn from(i: u32) -> Self {
		RangeId(i)
	}
}

/// Local presentation of asset information.
/// Most pallets do not need it.
pub trait LocalAssets<MayBeAssetId> {
	/// decimals of of big unit over minimal unit.
	/// orml also has separate trait on Balances to inspect decimals, that is not on type it self
	fn decimals(currency_id: MayBeAssetId) -> Result<Exponent, DispatchError>;

	/// Amount which humans operate as `1` usually.
	/// Amount is probably priceable by Oracles.
	/// Amount reasonably higher than minimal tradeable amount or minimal trading step on DEX.
	fn unit<T: From<u64>>(currency_id: MayBeAssetId) -> Result<T, DispatchError> {
		let exponent = Self::decimals(currency_id)?;
		Ok(10_u64.checked_pow(exponent as u32).ok_or(ArithmeticError::Overflow)?.into())
	}
}

/// when we store assets in native form to chain in smallest units or for mock in tests
impl<MayBeAssetId> LocalAssets<MayBeAssetId> for () {
	fn decimals(_currency_id: MayBeAssetId) -> Result<Exponent, DispatchError> {
		Ok(12)
	}
}

// FIXME(hussein-aitlahcen): this trait already exist under frame_support, named Balance
pub trait BalanceLike:
	AtLeast32BitUnsigned
	+ FullCodec
	+ Copy
	+ Default
	+ Debug
	+ MaybeSerializeDeserialize
	+ MaxEncodedLen
	+ TypeInfo
{
}
impl<T> BalanceLike for T where
	T: AtLeast32BitUnsigned
		+ FullCodec
		+ Copy
		+ Default
		+ Debug
		+ MaybeSerializeDeserialize
		+ MaxEncodedLen
		+ TypeInfo
{
}

/// limited counted number trait which maximal number is more than `u64`,  but not more than `u128`,
/// so inner type is either u64 or u128 with helpers for producing `ArithmeticError`s instead of
/// `Option`s.
pub trait MathBalance:
	PartialOrd
	+ Zero
	+ SafeAdd
	+ SafeDiv
	+ SafeMul
	+ SafeSub
	+ Into<u128>
	+ TryFrom<u128>
	+ From<u64>
	+ Copy
{
}
impl<
		T: PartialOrd
			+ Zero
			+ SafeAdd
			+ SafeDiv
			+ SafeMul
			+ SafeSub
			+ Into<u128>
			+ TryFrom<u128>
			+ From<u64>
			+ Copy,
	> MathBalance for T
{
}

pub trait AssetIdLike = FullCodec + MaxEncodedLen + Copy + Eq + PartialEq + Debug + TypeInfo;

#[derive(
	RuntimeDebug,
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Rational64 {
	pub n: u64,
	pub d: u64,
}

pub trait RationalLike<const N: u64, const D: u64> {
	fn new() -> Self;
	const CHECK: () = if D == 0 {
		panic!("denominator cannot be zero")
	};
}

#[macro_export]
macro_rules! rational {
	($n:literal / $d: literal) => {
		<composable_traits::currency::Rational64 as composable_traits::currency::RationalLike<
			$n,
			$d,
		>>::new()
	};
}

// const struct version of https://paritytech.github.io/substrate/master/src/sp_core/lib.rs.html#422
// so that it can be used in runtime config
// ands its private
macro_rules! impl_const_get {
	($name:ident, $t:ty) => {
		#[doc = "Const getter for a basic type."]
		#[derive(RuntimeDebug)]
		pub struct $name<const T: $t>;
		impl<const T: $t> Get<$t> for $name<T> {
			fn get() -> $t {
				T
			}
		}
		impl<const T: $t> Get<Option<$t>> for $name<T> {
			fn get() -> Option<$t> {
				Some(T)
			}
		}
		impl<const T: $t> TypedGet for $name<T> {
			type Type = $t;
			fn get() -> $t {
				T
			}
		}
	};
}

impl_const_get!(ConstRational64, Rational64);

impl<const N: u64, const D: u64> RationalLike<N, D> for Rational64 {
	fn new() -> Self {
		Self::from_unchecked(N, D)
	}
}

impl Rational64 {
	pub const fn from(n: u64, d: u64) -> Self {
		Self::from_unchecked(n, d.max(1))
	}

	pub const fn from_unchecked(n: u64, d: u64) -> Self {
		Self { n, d }
	}

	pub const fn one() -> Self {
		Rational64::from(1, 1)
	}

	pub const fn zero() -> Self {
		Rational64::from(0, 1)
	}

	pub const fn n(&self) -> u64 {
		self.n
	}

	pub const fn d(&self) -> u64 {
		self.d
	}
}

impl const From<Rational64> for FixedU128 {
	fn from(this: Rational64) -> Self {
		Self::from_rational(this.n.into(), this.d.into())
	}
}

impl const From<(u64, u64)> for Rational64 {
	fn from(this: (u64, u64)) -> Self {
		{
			let n = this.0;
			let d = this.1;
			Rational64 { n, d }
		}
	}
}

impl const From<Rational64> for FixedU64 {
	fn from(this: Rational64) -> Self {
		Self::from_rational(this.n.into(), this.d.into())
	}
}

impl From<Rational64> for Rational128 {
	fn from(this: Rational64) -> Self {
		Self::from(this.n.into(), this.d.into())
	}
}
