use crate::defi::DeFiEngine;
use frame_support::pallet_prelude::*;
pub use configs::*;

pub mod configs;

pub trait UndercollateralizedLoans: DeFiEngine {
	type BlockNumber: Clone + Eq + PartialEq;
	type LiquidationStrategyId: Clone + Eq + PartialEq;
	type Percent: Clone + Eq + PartialEq;
	type VaultId: Clone + Eq + PartialEq;
	type RepaymentStrategy: Clone;
    type TimeMeasure;

	fn create_market(
		manager: Self::AccountId,
		input: MarketInput<
			Self::AccountId,
			Self::MayBeAssetId,
			Self::BlockNumber,
			Self::LiquidationStrategyId,
		>,
		keep_alive: bool,
	) -> Result<
		MarketInfo<
			Self::AccountId,
			Self::MayBeAssetId,
			Self::BlockNumber,
			Self::LiquidationStrategyId,
			Self::VaultId,
		>,
		DispatchError,
	>;

	fn create_loan(
		input: LoanInput<
			Self::AccountId,
			Self::Balance,
			Self::BlockNumber,
		    Self::TimeMeasure,	
            Self::Percent,
			Self::RepaymentStrategy,
		>,
	) -> Result<
		LoanConfig<
			Self::AccountId,
			Self::Balance,
			Self::BlockNumber,
			Self::Percent,
			Self::RepaymentStrategy,
		>,
		DispatchError,
	>;

	fn borrow(
		borrower_account_id: Self::AccountId,
		loan_account_id: Self::AccountId,
		keep_alive: bool,
	) -> Result<
		LoanInfo<
			Self::AccountId,
			Self::Balance,
			Self::BlockNumber,
			Self::Percent,
			Self::RepaymentStrategy,
		>,
		DispatchError,
	>;

	fn market_account_id<S: Encode>(postfix: S) -> Self::AccountId;

	fn loan_account_id<S: Encode>(postfix: S) -> Self::AccountId;

	fn is_borrower_account_whitelisted(
		borrower_account_id_ref: &Self::AccountId,
		market_id_ref: &Self::AccountId,
	) -> Result<bool, DispatchError>;
}
