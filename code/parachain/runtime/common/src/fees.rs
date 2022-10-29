use crate::Balance;
use frame_support::weights::{
	constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
	WeightToFeePolynomial,
};
use primitives::currency::CurrencyId;
use sp_runtime::Perbill;

parameter_types! {
	pub NativeExistentialDeposit: Balance = native_existential_deposit();
}


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

/// Given a `currency_id`, returns the existential deposit of a MultiAsset in the native asset.
/// Returns `Balance::MAX` as the existential deposit if unable to get an existential deposit
/// for the given `currency_id`, this will prune unknown asset balances.
#[cfg(not(feature = "runtime-benchmarks"))]
pub fn multi_existential_deposits<
	AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>,
>(
	currency_id: &CurrencyId,
) -> Balance {
	AssetsRegistry::existential_deposit(*currency_id)
		.and_then(|ed| PriceConverter::<AssetsRegistry>::get_price_inverse(*currency_id, ed))
		.unwrap_or(match *currency_id {
			CurrencyId::PICA => native_existential_deposit(),
			// PICA: 0.1 or 100_000_000_000
			CurrencyId::PBLO => 100_000_000_000,
			// USDT: 100_000_000_000 * 1_000_000 / 67_000_000_000_000 = 1492 + 36/67
			CurrencyId::USDT => 1492,
			// //TODO: KAR: ?
			CurrencyId::KAR => 100_000_000_000,
			// kUSD: 100_000_000_000 / 67 = 1_492_537_313 + 29/67
			CurrencyId::kUSD => 1_492_537_313,
			// KSM: 100_000_000_000 / 2667 = 37_495_314 + 229/2667
			CurrencyId::KSM => 37_495_314,
			// TODO: BNC: ?
			CurrencyId::BNC => 100_000_000_000,
			// TODO: vKSM: ?
			CurrencyId::vKSM => 100_000_000_000,
			// TODO: MOVR: ?
			CurrencyId::MOVR => 100_000_000_000,
			// Unknown: Prune unknown balances
			_ => Balance::MAX,
		})
}



#[derive(Default)]
pub struct PriceConverter<AssetsRegistry>(PhantomData<AssetsRegistry>);

pub mod cross_chain_errors {
	pub const ASSET_IS_NOT_PRICEABLE: &str = "Asset is not priceable";
	pub const AMOUNT_OF_ASSET_IS_MORE_THAN_MAX_POSSIBLE: &str =
		"Amount of asset is more than max possible";
}

impl<AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>>
	frame_support::traits::tokens::BalanceConversion<Balance, CurrencyId, Balance>
	for PriceConverter<AssetsRegistry>
{
	type Error = sp_runtime::DispatchError;

	fn to_asset_balance(balance: Balance, asset_id: CurrencyId) -> Result<Balance, Self::Error> {
		Self::get_price_inverse(asset_id, balance)
	}
}

impl<AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>> MinimalOracle
	for PriceConverter<AssetsRegistry>
{
	type AssetId = CurrencyId;
	type Balance = Balance;
	fn get_price_inverse(
		asset_id: Self::AssetId,
		native_amount: Self::Balance,
	) -> Result<Self::Balance, sp_runtime::DispatchError> {
		match asset_id {
			CurrencyId::PICA => Ok(native_amount),
			_ =>
				if let Some(ratio) = AssetsRegistry::get_ratio(asset_id) {
					let amount = Ratio::from_inner(native_amount);
					if let Some(payment) = ratio.checked_mul(&amount) {
						Ok(payment.into_inner())
					} else {
						Err(DispatchError::Other(
							cross_chain_errors::AMOUNT_OF_ASSET_IS_MORE_THAN_MAX_POSSIBLE,
						))
					}
				} else if asset_id == CurrencyId::KSM {
					Ok(native_amount / 2667)
				} else if asset_id == CurrencyId::kUSD {
					Ok(native_amount / 67)
				} else if asset_id == CurrencyId::USDT || asset_id == CurrencyId::USDC {
					Ok(native_amount / 67_000_000)
				} else {
					Err(DispatchError::Other(cross_chain_errors::ASSET_IS_NOT_PRICEABLE))
				},
		}
	}
}

//  cannot be zero as in benches it fails Invalid input: InsufficientBalance
fn native_existential_deposit() -> Balance {
	100 * CurrencyId::milli::<Balance>()
}


#[cfg(test)]
mod commons_sence {
	use super::WeightToFeeConverter;
	use frame_support::weights::{
		constants::{WEIGHT_PER_SECOND},
		WeightToFee,
	};

	#[test]
	fn reasonable_fee() {
		let converted = WeightToFeeConverter::weight_to_fee(&WEIGHT_PER_SECOND);
		assert_eq!(converted, 1_158_775_406_000);
	}
}
