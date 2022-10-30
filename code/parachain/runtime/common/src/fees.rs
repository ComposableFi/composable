use crate::{prelude::*, Balance};
use composable_support::math::safe::safe_multiply_by_rational;
use composable_traits::{
	currency::{AssetExistentialDepositInspect, AssetRatioInspect, Rational64},
	rational,
};

use frame_support::{
	traits::{ConstU128, Get},
	weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
};
use num_traits::One;
use primitives::currency::CurrencyId;
use sp_runtime::{helpers_128bit::multiply_by_rational_with_rounding, Perbill};
use sp_std::marker::PhantomData;

pub const NATIVE_EXISTENTIAL_DEPOSIT: NativeBalance = 100_000_000_000;
pub type NativeExistentialDeposit = ConstU128<NATIVE_EXISTENTIAL_DEPOSIT>;

pub struct WeightToFeeConverter;
impl WeightToFeePolynomial for WeightToFeeConverter {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let p = CurrencyId::milli::<Balance>();
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub fn multi_existential_deposits<AssetsRegistry>(_currency_id: &CurrencyId) -> Balance {
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
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>,
>(
	currency_id: &CurrencyId,
) -> Balance {
	AssetsRegistry::existential_deposit(*currency_id)
		.ok()
		.or(WellKnownPriceConverter::existential_deposit(*currency_id))
		.unwrap_or(Balance::MAX)
}

pub struct PriceConverter<AssetsRegistry>(PhantomData<AssetsRegistry>);

pub mod cross_chain_errors {
	pub const ASSET_IS_NOT_PRICEABLE: &str = "Asset is not priceable";
	pub const AMOUNT_OF_ASSET_IS_MORE_THAN_MAX_POSSIBLE: &str =
		"Amount of asset is more than max possible";
}

pub struct WellKnownPriceConverter;

impl WellKnownPriceConverter {
	pub fn get_ratio(asset_id: CurrencyId) -> Option<Rational64> {
		match asset_id {
			CurrencyId::KSM => Some(rational!(375 / 1_000_000)),
			CurrencyId::ibcDOT => Some(rational!(2143 / 1_000_000)),
			CurrencyId::USDT | CurrencyId::USDC => Some(rational!(15 / 1_000_000_000)),
			CurrencyId::aUSD | CurrencyId::kUSD => Some(rational!(15 / 1_000)),
			CurrencyId::PICA => Some(rational!(1 / 1)),
			_ => None,
		}
	}

	pub fn existential_deposit(asset_id: CurrencyId) -> Option<Balance> {
		Self::to_asset_balance(NATIVE_EXISTENTIAL_DEPOSIT, asset_id)
	}

	pub fn to_asset_balance(fee: NativeBalance, asset_id: CurrencyId) -> Option<Balance> {
		Self::get_ratio(asset_id).map(|x| {
			safe_multiply_by_rational(fee, x.numer.into(), x.denom.into()).unwrap_or(Balance::one())
		})
	}
}

pub type NativeBalance = Balance;

impl<AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>>
	frame_support::traits::tokens::BalanceConversion<NativeBalance, CurrencyId, Balance>
	for PriceConverter<AssetsRegistry>
{
	type Error = sp_runtime::DispatchError;

	fn to_asset_balance(
		native_amount: NativeBalance,
		asset_id: CurrencyId,
	) -> Result<Balance, Self::Error> {
		AssetsRegistry::get_ratio(asset_id)
			.and_then(|x| {
				safe_multiply_by_rational(native_amount, x.numer.into(), x.denom.into()).ok()
			})
			.or(WellKnownPriceConverter::to_asset_balance(native_amount, asset_id))
			.ok_or(DispatchError::Other(cross_chain_errors::ASSET_IS_NOT_PRICEABLE))
	}
}

#[cfg(test)]
mod commons_sense {
	use crate::fees::WellKnownPriceConverter;

	use super::*;
	use composable_traits::currency::AssetRatioInspect;
	use frame_support::weights::{constants::WEIGHT_PER_SECOND, WeightToFee};
	use primitives::currency::CurrencyId;

	#[test]
	fn reasonable_fee() {
		let converted = WeightToFeeConverter::weight_to_fee(&WEIGHT_PER_SECOND);
		assert_eq!(converted, 1_158_775_406_000);
	}

	struct Dummy {}
	impl AssetRatioInspect for Dummy {
		type AssetId = CurrencyId;
	}
	impl AssetExistentialDepositInspect for Dummy {
		type AssetId = CurrencyId;
		type Balance = Balance;
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	#[test]
	fn usdt() {
		let converted_static =
			WellKnownPriceConverter::to_asset_balance(1_000_000_000, CurrencyId::USDT).unwrap();
		let converted_dynamic =
			PriceConverter::<Dummy>::to_asset_balance(1_000_000_000, CurrencyId::USDT).unwrap();
		assert_eq!(converted_static, converted_dynamic);
		assert_eq!(converted_static, 15);
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	#[test]
	fn ksm() {
		let converted_static =
			WellKnownPriceConverter::existential_deposit(CurrencyId::KSM).unwrap();
		let converted_dynamic = multi_existential_deposits::<Dummy>(&CurrencyId::KSM);
		assert_eq!(converted_static, converted_dynamic);
		assert_eq!(converted_static, 37500000);
	}
}
