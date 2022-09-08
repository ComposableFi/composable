use crate::{types::AccruedInterest, *};
use composable_support::math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub};
use composable_traits::{
	defi::*,
	lending::{
		math::{self, *},
		BorrowAmountOf, Lending, TotalDebtWithInterest,
	},
	time::{DurationSeconds, Timestamp, SECONDS_PER_YEAR_NAIVE},
};
use frame_support::traits::fungibles::{Inspect, InspectHold, Mutate};
use sp_runtime::{
	traits::Zero, ArithmeticError, DispatchError, FixedPointNumber, FixedU128, Percent,
};

impl<T: Config> Pallet<T> {
	pub(crate) fn do_total_borrowed_from_market_excluding_interest(
		market_id: &MarketId,
	) -> Result<T::Balance, DispatchError> {
		let debt_token =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		// total amount of debt *interest* owned by the market
		let total_debt_interest =
			<T as Config>::MultiCurrency::balance(debt_token, &Self::account_id(market_id));

		let total_issued = <T as Config>::MultiCurrency::total_issuance(debt_token);
		let total_amount_borrowed_from_market = total_issued.safe_sub(&total_debt_interest)?;

		Ok(total_amount_borrowed_from_market)
	}

	pub(crate) fn do_total_interest(market_id: &MarketId) -> Result<T::Balance, DispatchError> {
		let debt_token =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		// total amount of debt *interest* owned by the market
		let total_debt_interest =
			<T as Config>::MultiCurrency::balance(debt_token, &Self::account_id(market_id));

		Ok(total_debt_interest)
	}

	pub(crate) fn do_accrue_interest(
		market_id: &MarketId,
		now: Timestamp,
	) -> Result<(), DispatchError> {
		// we maintain original borrow principals intact on hold,
		// but accrue total borrow balance by adding to market debt balance
		// when user pays loan back, we reduce marked accrued debt
		// so no need to loop over each account -> scales to millions of users
		let total_borrowed_from_market_excluding_interest =
			Self::total_borrowed_from_market_excluding_interest(market_id)?;
		let total_available_to_be_borrowed = Self::total_available_to_be_borrowed(market_id)?;

		let utilization_ratio = Self::calculate_utilization_ratio(
			total_available_to_be_borrowed,
			total_borrowed_from_market_excluding_interest,
		)?;

		let delta_time = now
			.checked_sub(LastBlockTimestamp::<T>::get())
			.ok_or(ArithmeticError::Underflow)?;

		let borrow_index =
			BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;
		let debt_asset_id =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		let accrued_interest = Markets::<T>::try_mutate(market_id, |market_config| {
			let market_config = market_config.as_mut().ok_or(Error::<T>::MarketDoesNotExist)?;

			Self::accrue_interest_internal::<InterestRateModel>(
				utilization_ratio,
				&mut market_config.interest_rate_model,
				borrow_index,
				delta_time,
				total_borrowed_from_market_excluding_interest,
			)
		})?;

		// overwrites
		BorrowIndex::<T>::insert(market_id, accrued_interest.new_borrow_index);
		<T as Config>::MultiCurrency::mint_into(
			debt_asset_id,
			&Self::account_id(market_id),
			accrued_interest.accrued_increment,
		)?;

		Ok(())
	}

	pub(crate) fn do_calculate_utilization_ratio(
		cash: T::Balance,
		borrows: T::Balance,
	) -> Result<Percent, DispatchError> {
		Ok(math::calculate_utilization_ratio(
			LiftedFixedBalance::saturating_from_integer(cash.into()),
			LiftedFixedBalance::saturating_from_integer(borrows.into()),
		)?)
	}

	pub(crate) fn do_total_debt_with_interest(
		market_id: &MarketId,
		account: &T::AccountId,
	) -> Result<TotalDebtWithInterest<BorrowAmountOf<Self>>, DispatchError> {
		let debt_token =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		// Self::get_assets_for_market()?;
		match DebtIndex::<T>::get(market_id, account) {
			Some(account_interest_index) => {
				let market_interest_index =
					BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

				let account_principal =
					<T as Config>::MultiCurrency::balance_on_hold(debt_token, account);

				if account_principal.is_zero() {
					Ok(TotalDebtWithInterest::NoDebt)
				} else {
					// REVIEW
					let account_principal =
						LiftedFixedBalance::saturating_from_integer(account_principal.into());
					// principal * (market index / debt index)
					let index_ratio = market_interest_index.safe_div(&account_interest_index)?;

					let balance = account_principal
						.safe_mul(&index_ratio)?
						// TODO: Balance should be u128 eventually
						.checked_mul_int(1_u64)
						.ok_or(ArithmeticError::Overflow)?;
					Ok(TotalDebtWithInterest::Amount(balance.into()))
				}
			},
			None => Ok(TotalDebtWithInterest::NoDebt),
		}
	}
	/// ```python
	/// delta_interest_rate = delta_time / period_interest_rate
	/// debt_delta = debt_principal * delta_interest_rate
	/// new_accrued_debt = accrued_debt + debt_delta
	/// total_debt = debt_principal + new_accrued_debt
	/// ```
	pub(crate) fn accrue_interest_internal<I: InterestRate>(
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
			.ok_or(Error::<T>::CannotCalculateBorrowRate)?;

		// borrow_rate * index * delta_time / SECONDS_PER_YEAR_NAIVE + index
		let borrow_rate_delta = borrow_rate
			.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
			.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR_NAIVE))?;

		let new_borrow_index =
			borrow_rate_delta.safe_mul(&borrow_index)?.safe_add(&borrow_index)?;

		let accrued_increment = total_borrows
			.safe_mul(&borrow_rate_delta)?
			.checked_mul_int(1_u64)
			.ok_or(ArithmeticError::Overflow)?
			.into();

		Ok(AccruedInterest { accrued_increment, new_borrow_index })
	}
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
		.ok_or(Error::<T>::CannotCalculateBorrowRate)
		.map_err(Into::into)
}
