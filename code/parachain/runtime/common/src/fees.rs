use crate::{prelude::*, Balance};
use composable_traits::currency::AssetRatioInspect;

use composable_traits::currency::AssetExistentialDepositInspect;
use frame_support::{
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
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>
		+ BalanceConversion<NativeBalance, CurrencyId, Balance>,
>(
	currency_id: &CurrencyId,
) -> Balance {
	AssetsRegistry::existential_deposit(*currency_id).unwrap_or_else(|_| {
		AssetsRegistry::to_asset_balance(NATIVE_EXISTENTIAL_DEPOSIT, *currency_id)
			.unwrap_or(Balance::MAX)
	})
}

pub mod cross_chain_errors {
	pub const ASSET_PRICE_NOT_FOUND: &str = "Asset price not found";
	pub const AMOUNT_OF_ASSET_IS_MORE_THAN_MAX_POSSIBLE: &str =
		"Amount of asset is more than max possible";
}

pub type NativeBalance = Balance;

#[cfg(test)]
mod commons_sense {
	use super::*;
	use composable_traits::currency::AssetRatioInspect;
	use frame_support::weights::{constants::WEIGHT_PER_SECOND, WeightToFee};
	use primitives::currency::CurrencyId;

	#[test]
	fn reasonable_fee() {
		let converted = WeightToFeeConverter::weight_to_fee(&WEIGHT_PER_SECOND);
		assert_eq!(converted, 1_010_366_358_000);
	}

	struct Dummy {}
	impl AssetRatioInspect for Dummy {
		type AssetId = CurrencyId;
	}
	impl AssetExistentialDepositInspect for Dummy {
		type AssetId = CurrencyId;
		type Balance = Balance;
	}
}
