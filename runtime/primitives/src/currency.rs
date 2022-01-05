//! CurrencyId implementation
/// Asset id as if it was deserialized, not necessary exists.
/// We could check asset id during serde, but that:
/// - will make serde setup complicated (need to write and consistently apply static singletons
///   to all places with asset id)
/// - validate will involve at minimum in memory cache call (in worth case db call) during
///   extrinsic invocation
/// - will need to disable this during calls when it is really no need for validation (new
///   currency mapping)
/// - normal path will pay price (validate each time), in instead when fail pays only (like
///   trying to transfer non existing asset id)
/// - we cannot guarantee existence of asset as it may be removed during transaction (so we
///   should make removal exclusive case)
///
/// Given above we stick with possibly wrong asset id passed into API.
///
/// # Assert id pallet design   
/// ```ignore
/// pub trait MaximalConstGet<T> {
///     const VALUE: T;
/// }
/// /// knows existing local assets and how to map them to simple numbers
/// pub trait LocalAssetsRegistry {
///    /// asset id which is exist from now in current block
///    /// valid does not means usable, it can be subject to deletion or not yet approved to be used
///    type AssetId : AssetIdLike + Into<Self::MayBeAssetId>;
///    /// just id after serde
///    type MayBeAssetId : AssetIdLike + From<Self::AssetId>;
///    /// assets which we well know and embedded into `enum`.
///    /// maximal of this is smaller than minimal `OtherAssetId`
///    /// can always convert to valid asset id
///    type WellKnownAssetId : MaximalConstGet<u8> + Into<Self::AssetId> + Into<Self::MayBeAssetId> + Decimals<WellKnownAssetId> + TryFrom<u8>;
///
///    /// Larger than maximal of `WellKnownAssetId` but smaller than minimal `DerivativeAssetId`.
///    type OtherAssetId : MinimalConstGet<Self::WellKnownAssetId> + MaximalConstGet<u128>  + Into<Self::AssetId> + Into<Self::MayBeAssetId>;
///    /// allows to get next asset id
///    /// can consider split out producing assets interface into separate trait
///    type NextOtherAssetId = ErrorNext<OtherAssetId>;
///
///    /// locally diluted derivative and liquidity assets.
///    /// larger than maximal `OtherAssetId`
///    /// `Self::OtherAssetId` may be diluted(derived/wrapped), but only remote.
///    type DerivativeAssetId: MinimalConstGet<Self::OtherAssetId> + Into<Self::AssetId>;
///    /// may consider split out asset producing trait
///    type NextDerivativeAssetId = ErrorNext<Self::DerivativeAssetId>;
///
///    // note: fn to be replaced with Get or traits, just shortcuted here
///  
///    fn try_from<N:From<MayBeAssetId>>(value : N) -> Result<Self::AssetId, DispatchError>;
///    /// one unique native asset id
///    fn native() -> Self::WellKnownAssetId;
///
///    /// really u8, but easy to do math operations
///    /// ORML also has separate trait on Balances to inspect decimals, that is not on type it self
///    fn decimals(asset_id: Self::AssetId) -> u32;
/// }
/// /// read remote paths
/// /// registering is separate trait
/// pub trait RemoteAssetRegistry : LocalAssetsRegistry {
///    fn substrate(asset_id: Self::AssetId) -> Self:XcmPath;
///    fn remote(asset_id: Self::AssetId, network_id:) -> Self::Path;
/// }
/// ```
use codec::{CompactAs, Decode, Encode};
use composable_traits::currency::{DynamicCurrencyId, Exponent, PriceableAsset};
use scale_info::TypeInfo;
use sp_runtime::{ArithmeticError, DispatchError, RuntimeDebug};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::sp_std::ops::Deref;

/// `MayBe`CurrencyId as not each `u128` is valid id.
#[derive(
	Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, CompactAs,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CurrencyId(pub u128);

impl CurrencyId {
	pub const INVALID: CurrencyId = CurrencyId(0);
	pub const PICA: CurrencyId = CurrencyId(1);
	pub const LAYR: CurrencyId = CurrencyId(2);
	pub const CROWD_LOAN: CurrencyId = CurrencyId(3);
	pub const KSM: CurrencyId = CurrencyId(4);

	pub const LOCAL_LP_TOKEN_START: CurrencyId = CurrencyId(u128::MAX / 2);
}

impl PriceableAsset for CurrencyId {
	#[inline]
	fn decimals(self) -> Exponent {
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

/// maps id to junction generic key,
/// unfortunately it is the best way to encode currency id as of now in XCM
#[cfg(feature = "develop")]
impl From<CurrencyId> for xcm::latest::Junction {
	fn from(this: CurrencyId) -> Self {
		xcm::latest::Junction::GeneralKey(this.encode())
	}
}
