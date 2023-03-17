use crate::{prelude::*, Balance};
use composable_support::math::safe::safe_multiply_by_rational;
use composable_traits::currency::{AssetRatioInspect, Rational64};

use composable_traits::currency::AssetExistentialDepositInspect;
use frame_support::{
	dispatch::marker::PhantomData,
	traits::ConstU128,
	weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
};
use primitives::currency::CurrencyId;
use sp_runtime::Perbill;

pub const NATIVE_EXISTENTIAL_DEPOSIT: NativeBalance = 100_000_000_000;
pub type NativeExistentialDeposit = ConstU128<NATIVE_EXISTENTIAL_DEPOSIT>;

pub struct WeightToFeeConverter;
impl WeightToFeePolynomial for WeightToFeeConverter {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let p = CurrencyId::milli::<Balance>();
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub fn multi_existential_deposits<AssetsRegistry, ForeignToNative>(
	_currency_id: &CurrencyId,
) -> Balance {
	// ISSUE:
	// Running benchmarks with non zero multideposit leads to fail in 3rd party pallet.
	// It is not clearly why it happens.pub const BaseXcmWeight: Weight = 100_000_000;
	// 2022-03-14 20:50:19 Running Benchmark: collective.set_members 2/1 1/1
	// Error:
	//   0: Invalid input: Account cannot exist with the funds that would be given
	use num_traits::Zero;
	Balance::zero()
}

#[cfg(not(feature = "runtime-benchmarks"))]
pub fn multi_existential_deposits<
	AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>
		+ BalanceConversion<NativeBalance, CurrencyId, Balance>,
	ForeignToNative: ForeignToNativePriceConverter,
>(
	currency_id: &CurrencyId,
) -> Balance {
	AssetsRegistry::existential_deposit(*currency_id).unwrap_or_else(|_| {
		AssetsRegistry::to_asset_balance(NATIVE_EXISTENTIAL_DEPOSIT, *currency_id)
			.unwrap_or(Balance::MAX)
	})
}

pub struct PriceConverter<AssetsRegistry, ForeignToNative>(
	PhantomData<(AssetsRegistry, ForeignToNative)>,
);

pub mod cross_chain_errors {
	pub const ASSET_PRICE_NOT_FOUND: &str = "Asset price not found";
	pub const AMOUNT_OF_ASSET_IS_MORE_THAN_MAX_POSSIBLE: &str =
		"Amount of asset is more than max possible";
}

pub trait ForeignToNativePriceConverter {
	fn get_ratio(asset_id: CurrencyId) -> Option<Rational64>;
	fn existential_deposit(asset_id: CurrencyId) -> Option<Balance> {
		Self::to_asset_balance(NATIVE_EXISTENTIAL_DEPOSIT, asset_id)
	}

	fn to_asset_balance(balance: NativeBalance, asset_id: CurrencyId) -> Option<Balance> {
		Self::get_ratio(asset_id).and_then(|ratio| {
			safe_multiply_by_rational(balance, ratio.n.into(), ratio.d.into()).ok()
		})
	}
}

pub type NativeBalance = Balance;

impl<
		AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>,
		ForeignToNative: ForeignToNativePriceConverter,
	> frame_support::traits::tokens::BalanceConversion<NativeBalance, CurrencyId, Balance>
	for PriceConverter<AssetsRegistry, ForeignToNative>
{
	type Error = sp_runtime::DispatchError;

	fn to_asset_balance(
		native_amount: NativeBalance,
		asset_id: CurrencyId,
	) -> Result<Balance, Self::Error> {
		AssetsRegistry::get_ratio(asset_id)
			.and_then(|x| safe_multiply_by_rational(native_amount, x.n().into(), x.d().into()).ok())
			.or_else(|| ForeignToNative::to_asset_balance(native_amount, asset_id))
			.ok_or(DispatchError::Other(cross_chain_errors::ASSET_PRICE_NOT_FOUND))
	}
}

#[cfg(test)]
mod commons_sense {
	use composable_traits::rational;
	struct Dummy {}
	impl ForeignToNativePriceConverter for Dummy {
		fn get_ratio(asset_id: CurrencyId) -> Option<Rational64> {
			match asset_id {
				CurrencyId::USDT | CurrencyId::USDC => Some(rational!(15 / 1_000_000_000)),
				CurrencyId::KSM => Some(rational!(375 / 1_000_000)),
				_ => None,
			}
		}
	}

	use super::*;
	use composable_traits::currency::AssetRatioInspect;
	use frame_support::weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight, WeightToFee};
	use primitives::currency::CurrencyId;

	#[test]
	fn reasonable_fee() {
		let converted =
			WeightToFeeConverter::weight_to_fee(&Weight::from_ref_time(WEIGHT_REF_TIME_PER_SECOND));
		assert_eq!(converted, 1_010_366_358_000);
	}

	impl AssetRatioInspect for Dummy {
		type AssetId = CurrencyId;
	}

	impl AssetExistentialDepositInspect for Dummy {
		type AssetId = CurrencyId;

		type Balance = Balance;

		fn existential_deposit(asset_id: Self::AssetId) -> Result<Self::Balance, DispatchError> {
			<Self as ForeignToNativePriceConverter>::existential_deposit(asset_id)
				.ok_or(DispatchError::Other("Dummy can't find"))
		}
	}

	impl BalanceConversion<Balance, CurrencyId, Balance> for Dummy {
		type Error = DispatchError;

		fn to_asset_balance(
			balance: Balance,
			asset_id: CurrencyId,
		) -> Result<Balance, Self::Error> {
			<Self as ForeignToNativePriceConverter>::to_asset_balance(balance, asset_id)
				.ok_or(DispatchError::Other("Dummy can't find"))
		}
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	#[test]
	fn usdt() {
		let converted_static = <Dummy as ForeignToNativePriceConverter>::to_asset_balance(
			1_000_000_000,
			CurrencyId::USDT,
		)
		.unwrap();
		let converted_dynamic =
			PriceConverter::<Dummy, Dummy>::to_asset_balance(1_000_000_000, CurrencyId::USDT)
				.unwrap();
		assert_eq!(converted_static, converted_dynamic);
		assert_eq!(converted_static, 15);
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	#[test]
	fn ksm() {
		let converted_static =
			<Dummy as ForeignToNativePriceConverter>::existential_deposit(CurrencyId::KSM).unwrap();
		let converted_dynamic = multi_existential_deposits::<Dummy, Dummy>(&CurrencyId::KSM);
		assert_eq!(converted_static, converted_dynamic);
		assert_eq!(converted_static, 37500000);
	}
}
