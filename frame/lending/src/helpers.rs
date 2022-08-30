
mod borrow;
mod collateral;
mod liquidation;
mod market;
mod price;
mod offchain_workers;

use crate::{types::AccruedInterest, *};

use crate::{
	types::InitializeBlockCallCounters,
};
use composable_support::{
	math::safe::{SafeAdd, SafeDiv, SafeMul},
};
use composable_traits::{
	defi::{
		LiftedFixedBalance, MoreThanOneFixedU128,
		OneOrMoreFixedU128,
	},
	lending::{
		math::InterestRate, Lending,
	},
	time::{DurationSeconds, SECONDS_PER_YEAR_NAIVE},
	vault::{FundsAvailability, StrategicVault, Vault},
};
use frame_support::{
	storage::{with_transaction, TransactionOutcome},
	traits::{
		fungibles::{Inspect},
		UnixTime,
	},
};
use sp_runtime::{
	ArithmeticError, DispatchError, FixedPointNumber, FixedU128, Percent,
};


// crate-public helper functions
impl<T: Config> Pallet<T> {
	pub(crate) fn initialize_block(block_number: T::BlockNumber) -> InitializeBlockCallCounters {
		let mut call_counters = InitializeBlockCallCounters::default();
		let _ = with_transaction(|| {
			let now = Self::now();
			call_counters.now += 1;

			let mut errors = Markets::<T>::iter()
				.map(|(market_id, config)| {
					call_counters.read_markets += 1;
					Self::accrue_interest(&market_id, now)?;
					call_counters.accrue_interest += 1;
					let market_account = Self::account_id(&market_id);
					call_counters.account_id += 1;
					// NOTE(hussein-aitlahcen):
					// It would probably be more perfomant to handle theses
					// case while borrowing/repaying.
					//
					// I don't know whether we would face any issue by doing that.
					//
					// borrow:
					//  - withdrawable = transfer(vault->market) + transfer(market->user)
					//  - depositable = error(not enough borrow asset) // vault asking for reserve
					//    to be fullfilled
					//  - mustliquidate = error(market is closing)
					// repay:
					// 	- (withdrawable || depositable || mustliquidate) = transfer(user->market) +
					//    transfer(market->vault)
					//
					// The intermediate transfer(vault->market) while borrowing would
					// allow the vault to update the strategy balance (market = borrow vault
					// strategy).
					match Self::available_funds(&config, &market_account)? {
						FundsAvailability::Withdrawable(balance) => {
							Self::handle_withdrawable(&config, &market_account, balance)?;
							call_counters.handle_withdrawable += 1;
						},
						FundsAvailability::Depositable(balance) => {
							Self::handle_depositable(&config, &market_account, balance)?;
							call_counters.handle_depositable += 1;
						},
						FundsAvailability::MustLiquidate => {
							Self::handle_must_liquidate(&config, &market_account)?;
							call_counters.handle_must_liquidate += 1;
						},
					}

					call_counters.available_funds += 1;

					Result::<(), DispatchError>::Ok(())
				})
				.filter_map(|r| match r {
					Ok(_) => None,
					Err(err) => Some(err),
				})
				.peekable();

			if errors.peek().is_none() {
				LastBlockTimestamp::<T>::put(now);
				TransactionOutcome::Commit(Ok(1000))
			} else {
				errors.for_each(|e| {
					log::error!(
						"This should never happen, could not initialize block!!! {:#?} {:#?}",
						block_number,
						e
					)
				});
				TransactionOutcome::Rollback(Err(DispatchError::Other(
					"failed to initialize block",
				)))
			}
		});
		call_counters
	}

	pub(crate) fn now() -> u64 {
		T::UnixTime::now().as_secs()
	}

	pub(crate) fn available_funds(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
	) -> Result<FundsAvailability<T::Balance>, DispatchError> {
		<T::Vault as StrategicVault>::available_funds(&config.borrow_asset_vault, market_account)
	}

	pub(crate) fn handle_withdrawable(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
		balance: T::Balance,
	) -> Result<(), DispatchError> {
		<T::Vault as StrategicVault>::withdraw(&config.borrow_asset_vault, market_account, balance)
	}

	pub(crate) fn handle_depositable(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
		balance: T::Balance,
	) -> Result<(), DispatchError> {
		let asset_id = <T::Vault as Vault>::asset_id(&config.borrow_asset_vault)?;
		let balance =
			<T as Config>::MultiCurrency::reducible_balance(asset_id, market_account, false)
				.min(balance);
		<T::Vault as StrategicVault>::deposit(&config.borrow_asset_vault, market_account, balance)
	}

	pub(crate) fn handle_must_liquidate(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
	) -> Result<(), DispatchError> {
		let asset_id = <T::Vault as Vault>::asset_id(&config.borrow_asset_vault)?;
		let balance =
			<T as Config>::MultiCurrency::reducible_balance(asset_id, market_account, false);
		<T::Vault as StrategicVault>::deposit(&config.borrow_asset_vault, market_account, balance)
	}
}

// various helper functions

/// given collateral information, how much of borrow asset can get?
pub fn swap(
	collateral_balance: &LiftedFixedBalance,
	collateral_price: &LiftedFixedBalance,
	collateral_factor: &MoreThanOneFixedU128,
) -> Result<LiftedFixedBalance, ArithmeticError> {
	collateral_balance.safe_mul(collateral_price)?.safe_div(collateral_factor)
}

/// ```python
/// delta_interest_rate = delta_time / period_interest_rate
/// debt_delta = debt_principal * delta_interest_rate
/// new_accrued_debt = accrued_debt + debt_delta
/// total_debt = debt_principal + new_accrued_debt
/// ```
pub(crate) fn accrue_interest_internal<T: Config, I: InterestRate>(
	utilization_ratio: Percent,
	interest_rate_model: &mut I,
	borrow_index: OneOrMoreFixedU128,
	delta_time: DurationSeconds,
	total_borrows: T::Balance,
) -> Result<AccruedInterest<T>, DispatchError> {
	let total_borrows: FixedU128 =
		FixedU128::checked_from_integer(Into::<u128>::into(total_borrows))
			.ok_or(ArithmeticError::Overflow)?;

	let borrow_rate = interest_rate_model
		.get_borrow_rate(utilization_ratio)
		.ok_or(Error::<T>::BorrowRateDoesNotExist)?;

	// borrow_rate * index * delta_time / SECONDS_PER_YEAR_NAIVE + index
	let borrow_rate_delta = borrow_rate
		.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
		.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR_NAIVE))?;

	let new_borrow_index = borrow_rate_delta.safe_mul(&borrow_index)?.safe_add(&borrow_index)?;

	let accrued_increment = total_borrows
		.safe_mul(&borrow_rate_delta)?
		.checked_mul_int(1_u64)
		.ok_or(ArithmeticError::Overflow)?
		.into();

	Ok(AccruedInterest { accrued_increment, new_borrow_index })
}

/// Retrieve the current interest rate for the given `market_id`.
#[cfg(test)]
pub fn current_interest_rate<T: Config>(
	market_id: MarketIdInner,
) -> Result<composable_traits::defi::Rate, DispatchError> {
	let market_id = MarketId::new(market_id);
	let total_borrowed_from_market_excluding_interest =
		Pallet::<T>::total_borrowed_from_market_excluding_interest(&market_id)?;
	let total_available_to_be_borrowed = Pallet::<T>::total_available_to_be_borrowed(&market_id)?;
	let utilization_ratio = Pallet::<T>::calculate_utilization_ratio(
		total_available_to_be_borrowed,
		total_borrowed_from_market_excluding_interest,
	)?;

	Markets::<T>::try_get(market_id)
		.map_err(|_| Error::<T>::MarketDoesNotExist)?
		.interest_rate_model
		.get_borrow_rate(utilization_ratio)
		.ok_or(Error::<T>::BorrowRateDoesNotExist)
		.map_err(Into::into)
}
