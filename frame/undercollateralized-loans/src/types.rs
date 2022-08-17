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

pub(crate) type PossiblePaymentsOutcomes<T> = Vec<PossiblePaymentOutcome<<T as frame_system::Config>::AccountId>>;

// This enum is used for off-chain payment checking procedure.
#[derive(Encode, Decode, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub enum PossiblePaymentOutcome<AccountId> {
	RegularPaymentMaySucceed(AccountId),
	LastPaymentMaySucceed(AccountId),
    // We assume that payment is failed if it is not possible to transfer money from borrower account 
    // to loan account on the moment of checking.
    PaymentFailed(AccountId),
}

#[derive(Encode, Decode, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub enum PaymentOutcome<AccountId> {
	RegularPaymentSucceed(AccountId),
	LastPaymentSucceed(AccountId),
	PaymentFailed(AccountId),
}

#[derive(Encode, Decode)]
pub struct LoanId(pub [u8; 8]);

impl TypeId for LoanId {
	const TYPE_ID: [u8; 4] = *b"loan";
}
