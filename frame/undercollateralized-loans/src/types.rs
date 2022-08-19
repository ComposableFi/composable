use crate::Config;
use composable_traits::{
	defi::DeFiComposableConfig,
	undercollateralized_loans::{LoanConfig, LoanInput, MarketConfig, MarketInfo, MarketInput},
};
use frame_support::pallet_prelude::*;
use sp_core::TypeId;

// Seconds from the Unix epoche.
// use i64 since NaiveDateTime timestamp is i64.
pub(crate) type Timestamp = i64;

// Type to count created entities such as markets and loans.
// Counters values arem used to generate loans' and markets' accounts ids.
pub(crate) type Counter = u128;

pub(crate) type MarketInputOf<T> = MarketInput<
	<T as frame_system::Config>::AccountId,
	<T as DeFiComposableConfig>::MayBeAssetId,
	<T as frame_system::Config>::BlockNumber,
	<T as Config>::LiquidationStrategyId,
>;

pub(crate) type LoanInputOf<T> = LoanInput<
	<T as frame_system::Config>::AccountId,
	<T as DeFiComposableConfig>::Balance,
	Timestamp,
>;

pub(crate) type MarketInfoOf<T> = MarketInfo<
	<T as frame_system::Config>::AccountId,
	<T as DeFiComposableConfig>::MayBeAssetId,
	<T as frame_system::Config>::BlockNumber,
	<T as Config>::LiquidationStrategyId,
	<T as Config>::VaultId,
>;

pub(crate) type MarketConfigOf<T> = MarketConfig<
	<T as frame_system::Config>::AccountId,
	<T as DeFiComposableConfig>::MayBeAssetId,
	<T as frame_system::Config>::BlockNumber,
	<T as Config>::VaultId,
>;

pub(crate) type LoanConfigOf<T> = LoanConfig<
	<T as frame_system::Config>::AccountId,
	<T as DeFiComposableConfig>::MayBeAssetId,
	<T as DeFiComposableConfig>::Balance,
	Timestamp,
>;

pub(crate) type PaymentOutcomeOf<T> =
	PaymentOutcome<<T as DeFiComposableConfig>::Balance, LoanConfigOf<T>, Timestamp>;

pub(crate) type PaymentsOutcomes<T> = Vec<PaymentOutcomeOf<T>>;

pub(crate) type PaymentOf<T> =
	Payment<<T as DeFiComposableConfig>::Balance, LoanConfigOf<T>, Timestamp>;

// This structure is used to simplify holding of payment info.
#[derive(Encode, Decode, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct Payment<Balance, LoanConfig, Timestamp> {
	pub loan_config: LoanConfig,
	pub amount: Balance,
	pub timestamp: Timestamp,
}

// Used to treat payments outcomes in off-chain and on-chain payments checking procedures.
#[derive(Encode, Decode, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub enum PaymentOutcome<Balance, LoanConfig, Timestamp> {
	RegularPaymentSucceed(Payment<Balance, LoanConfig, Timestamp>),
	LastPaymentSucceed(Payment<Balance, LoanConfig, Timestamp>),
	// We assume that payment is failed if it is not possible to transfer money from borrower
	// account to loan account on the moment of off-chain checking.
	PaymentFailed(Payment<Balance, LoanConfig, Timestamp>),
}

// Used for loan's account ids generation.
#[derive(Encode, Decode)]
pub struct LoanId(pub [u8; 8]);

impl TypeId for LoanId {
	const TYPE_ID: [u8; 4] = *b"loan";
}
