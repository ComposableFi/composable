use composable_traits::currency::{DynamicCurrencyId, PriceableAsset};
use frame_support::parameter_types;
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
pub enum MockCurrencyId {
	PICA,
	BTC,
	ETH,
	LTC,
	USDT,
	LpToken(u128),
}

/// seems need unifying type which stores and checks asset id boundaries, so not sure about design
/// yet
// pub trait ConstGet<T> {
// 	const VALUE: T;
// }

// /// knows existing local assets and how to map them to simple numbers
// pub trait LocalAssetsRegistry {
// 	type AssetId : AssetIdLike;
// 	/// assets which we well know and embedded into enum.
// 	/// maximal of this is smaller than minimal `Assets`
// 	type WellKnownAssetId : ConstGet<u8> + Into<Self::AssetId>;
// 	/// Larger than maximal of `WellKnownAssetId` but smaller than minimal `DerivativeAssetId`.
// 	type OtherAssetId : ConstGet<u128>  + Into<Self::AssetId>;
// 	/// locally diluted derivative and liquidity assets.
// 	/// larger than maximal `OtherAssetId`
// 	type DerivativeAssetId: ConstGet<u128> + Into<Self::AssetId>;
// 	fn try_from<N:Into<u128>>(number : N) -> Result<Self::AssetId, DispatchError>;
// 	fn native() -> Self::WellKnownAssetId;
// }

impl Default for MockCurrencyId {
	fn default() -> Self {
		MockCurrencyId::PICA
	}
}

impl PriceableAsset for MockCurrencyId {
	fn decimals(self) -> composable_traits::currency::Exponent {
		match self {
			MockCurrencyId::PICA => 0,
			MockCurrencyId::BTC => 8,
			MockCurrencyId::ETH => 18,
			MockCurrencyId::LTC => 8,
			MockCurrencyId::USDT => 2,
			MockCurrencyId::LpToken(_) => 0,
		}
	}
}

impl DynamicCurrencyId for MockCurrencyId {
	fn next(self) -> Result<Self, DispatchError> {
		match self {
			MockCurrencyId::LpToken(x) => Ok(MockCurrencyId::LpToken(
				x.checked_add(1).ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)),
			_ => unreachable!(),
		}
	}
}

parameter_types! {
	pub const MaxStrategies: usize = 255;
	pub const NativeAssetId: MockCurrencyId = MockCurrencyId::PICA;
}
