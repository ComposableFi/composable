use codec::FullCodec;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::{AtLeast32BitUnsigned, Zero};
use sp_std::fmt::Debug;

use crate::math::SafeArithmetic;

/// really u8, but easy to do math operations
pub type Exponent = u32;

/* NOTE(hussein-aitlahcen):
 I initially added a generic type to index into the generatable sub-range but realised it was
 overkill. Perhaps it will be required later if we want to differentiate multiple sub-ranges
 (possibly making a sub-range constant, e.g. using a constant currency id for a pallet expecting
 currency ids to be generated).
 The implementor should ensure that a new `DynamicCurrency` is created and collisions are
 avoided.
*/
/// A currency we can generate given that we have a previous currency.
pub trait DynamicCurrencyId
where
	Self: Sized,
{
	fn next(self) -> Result<Self, DispatchError>;
}

/// Creates a new asset, compatible with [`MultiCurrency`](https://docs.rs/orml-traits/0.4.0/orml_traits/currency/trait.MultiCurrency.html).
/// The implementor should ensure that a new `CurrencyId` is created and collisions are avoided.
/// Is about Local assets representations. These may differ remotely.
pub trait CurrencyFactory<CurrencyId> {
	fn create() -> Result<CurrencyId, DispatchError>;
}

/// Local presentation of asset information.
/// Most pallets do not need it.
pub trait LocalAssets<MayBeAssetId> {
	/// decimals of of big unit over minimal unit.
	/// orml also has separate trait on Balances to inspect decimals, that is not on type it self
	fn decimals(currency_id: MayBeAssetId) -> Result<Exponent, DispatchError>;

	/// Amount which humans operate as `1` usually.
	/// Amount is probably priceable by Oracles.
	/// Amount resonably higher than minimal tradeable amount or minial trading step on DEX.
	fn unit<T: From<u64>>(currency_id: MayBeAssetId) -> Result<T, DispatchError> {
		let exponent = Self::decimals(currency_id)?;
		Ok(10_u64.pow(exponent).into())
	}
}

/// when we store assets in native form to chain in smallest units or for mock in tests
impl<MayBeAssetId> LocalAssets<MayBeAssetId> for () {
	fn decimals(_currency_id: MayBeAssetId) -> Result<Exponent, DispatchError> {
		Ok(0)
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
impl<
		T: AtLeast32BitUnsigned
			+ FullCodec
			+ Copy
			+ Default
			+ Debug
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo,
	> BalanceLike for T
{
}

/// limited counted number trait which maximal number is more than `u64`,  but not more than `u128`,
/// so inner type is either u64 or u128 with helpers for producing `ArithmeticError`s instead of
/// `Option`s.
pub trait MathBalance:
	PartialOrd + Zero + SafeArithmetic + Into<u128> + TryFrom<u128> + From<u64> + Copy
{
}
impl<T: PartialOrd + Zero + SafeArithmetic + Into<u128> + TryFrom<u128> + From<u64> + Copy>
	MathBalance for T
{
}

// hack to imitate type alias until it is in stable
// named with like implying it is`like` is is necessary to be `AssetId`, but may be not enough (if
// something is `AssetIdLike` than it is not always asset)

// FIXME(hussein-aitlahcen): this trait already exists in frame_support, named `AssetId`
pub trait AssetIdLike:
	FullCodec + MaxEncodedLen + Copy + Eq + PartialEq + Debug + TypeInfo
{
}
impl<T: FullCodec + MaxEncodedLen + Copy + Eq + PartialEq + Debug + TypeInfo> AssetIdLike for T {}
