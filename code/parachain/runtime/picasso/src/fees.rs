use crate::*;
use crate::prelude::*;
use common::fees::{ForeignToNativePriceConverter, PriceConverter};
use composable_traits::{currency::Rational64, rational};
use sp_core::ConstU8;
use transaction_payment::{Multiplier, TargetedFeeAdjustment};
use primitives::currency::CurrencyId;

pub struct WellKnownForeignToNativePriceConverter;
impl ForeignToNativePriceConverter for WellKnownForeignToNativePriceConverter {
	fn get_ratio(asset_id: CurrencyId) -> Option<Rational64> {
		match asset_id {
			CurrencyId::KSM => Some(rational!(375 / 1_000_000)),
			CurrencyId::DOT => Some(rational!(2143 / 1_000_000)),
			CurrencyId::USDT | CurrencyId::USDC => Some(rational!(15 / 1_000_000_000)),
			CurrencyId::kUSD => Some(rational!(15 / 1_000)),
			CurrencyId::PICA => Some(rational!(1 / 1)),
			CurrencyId::PBLO => Some(rational!(1 / 1)),
			CurrencyId::KSM_USDT_LPT => Some(rational!(1 / 1_000_000_000)),
			CurrencyId::PICA_USDT_LPT => Some(rational!(1 / 1_000_000_000)),
			CurrencyId::PICA_KSM_LPT => Some(rational!(1 / 1_000_000_000)),
			_ => None,
		}
	}
}

pub type FinalPriceConverter =
	PriceConverter<crate::AssetsRegistry, WellKnownForeignToNativePriceConverter>;

parameter_types! {
	/// 1 milli-pica/byte should be fine
	pub TransactionByteFee: Balance = CurrencyId::milli();

	// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly. This low value causes changes to occur slowly over time.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero` in integration_tests.rs.
	/// This value is currently only used by pallet-transaction-payment as an assertion that the
	/// next multiplier is always > min value.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_u128);
}

impl transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction =
		transaction_payment::CurrencyAdapter<Balances, StakingPot<Runtime, NativeTreasury>>;
	type WeightToFee = WeightToFeeConverter;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
}
pub type AssetsPaymentHeader = asset_tx_payment::ChargeAssetTxPayment<Runtime>;
pub struct TransferToTreasuryOrDrop;
impl asset_tx_payment::HandleCredit<AccountId, Tokens> for TransferToTreasuryOrDrop {
	fn handle_credit(credit: fungibles::CreditOf<AccountId, Tokens>) {
		let _ =
			<Tokens as fungibles::Balanced<AccountId>>::resolve(&TreasuryAccount::get(), credit);
	}
}

impl asset_tx_payment::Config for Runtime {
	type Fungibles = Tokens;
	type OnChargeAssetTransaction =
		asset_tx_payment::FungiblesAdapter<FinalPriceConverter, TransferToTreasuryOrDrop>;

	type UseUserConfiguration = ConstBool<true>;

	type WeightInfo = weights::asset_tx_payment::WeightInfo<Runtime>;

	type ConfigurationOrigin = EnsureRootOrTwoThirdNativeCouncil;

	type ConfigurationExistentialDeposit = NativeExistentialDeposit;

	type PayableCall = RuntimeCall;

	type Lock = Assets;

	type BalanceConverter = FinalPriceConverter;
}


