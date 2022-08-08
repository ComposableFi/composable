use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

#[derive(Encode, Decode, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub enum RepaymentStrategy {
	InterestPeriodicallyPrincipalWhenMature,
	PrincipalOnlyWhenMature,
    EmptyStrategy,
}

// This enum is used since we do not want to return an Error in the on_initalize() function.
pub enum RepaymentResult<T: crate::Config> {
	Failed(DispatchError),
	InterestIsPaidInTime(T::Balance),
	InterestIsNotPaidInTime(DispatchError),
	PrincipalAndLastInterestPaymentArePaidBackInTime(T::Balance),
	PrincipalAndLastInterestPaymentAreNotPaidBackInTime(DispatchError),
}


pub mod empty_strategy {
    use crate::Config;
    pub fn apply<T:Config>() -> super::RepaymentResult<T> {
        super::RepaymentResult::Failed("some error".into())
    }
}

// Borrower pays interest regulary, and pays back principal when the loan is mature.
/*
pub mod interest_periodically_principal_when_mature {
	use crate::{
		types::{LoanConfigOf, TimeMeasure},
		Config,
	};
	use frame_support::traits::fungibles::Transfer;

	pub fn apply<T: Config>(
		loan_config: LoanConfigOf<T>,
		current_moment: &TimeMeasure,
		keep_alive: bool,
	) -> super::RepaymentResult<T> {
		let interest_rate =
			match loan_config.get_interest_rate_for_particular_moment(current_moment) {
				Some(interest_rate) => interest_rate,
				None =>
					return super::RepaymentResult::Failed(
						crate::Error::<T>::ThereIsNoSuchMomentInTheLoanPaymentSchedule.into(),
					),
			};
		// Will not overflow since we multiply to interest rate.
		let mut payment_amount = *interest_rate * *loan_config.principal();
		let mut is_principal_payment = false;
		// If it is time to repay principal
		if current_moment == loan_config.last_payment_moment() {
			payment_amount += *loan_config.principal();
			is_principal_payment = true;
		}
		let loan_account_id = loan_config.account_id();
		let market_account_id = loan_config.market_account_id();
		let market_info = crate::Pallet::<T>::get_market_info_via_account_id(&market_account_id);
		let market_info = match market_info {
			Ok(market_info) => market_info,
			Err(error) => return super::RepaymentResult::Failed(error.into()),
		};
		let market_config = market_info.config();
		let borrow_asset_id = market_config.borrow_asset();
		match T::MultiCurrency::transfer(
			*borrow_asset_id,
			loan_account_id,
			market_account_id,
			payment_amount,
			keep_alive,
		) {
			Ok(balance) if is_principal_payment =>
				super::RepaymentResult::PrincipalAndLastInterestPaymentArePaidBackInTime(balance),
			Ok(balance) => super::RepaymentResult::InterestIsPaidInTime(balance),
			Err(error) if is_principal_payment =>
				super::RepaymentResult::PrincipalAndLastInterestPaymentAreNotPaidBackInTime(
					error.into(),
				),
			Err(error) => super::RepaymentResult::InterestIsNotPaidInTime(error.into()),
		}
	}
}

// Borrower pays interest regulary, and pays back principal when the loan is mature.
pub mod interest_and_principal_periodically {
	use crate::{
		types::{LoanConfigOf, TimeMeasure},
		Config,
	};
	use frame_support::traits::fungibles::Transfer;

	pub fn apply<T: Config>(
		loan_config: LoanConfigOf<T>,
		current_moment: &TimeMeasure,
		keep_alive: bool,
	) -> super::RepaymentResult<T> {
		let interest_rate =
			match loan_config.get_interest_rate_for_particular_moment(current_moment) {
				Some(interest_rate) => interest_rate,
				None =>
					return super::RepaymentResult::Failed(
						crate::Error::<T>::ThereIsNoSuchMomentInTheLoanPaymentSchedule.into(),
					),
			};
		// Will not overflow since we multiply to interest rate.
		let mut payment_amount = *interest_rate * *loan_config.principal();
		let mut is_principal_payment = false;
		// If it is time to repay principal
		if current_moment == loan_config.last_payment_moment() {
			payment_amount += *loan_config.principal();
			is_principal_payment = true;
		}
		let loan_account_id = loan_config.account_id();
		let market_account_id = loan_config.market_account_id();
		let market_info = crate::Pallet::<T>::get_market_info_via_account_id(&market_account_id);
		let market_info = match market_info {
			Ok(market_info) => market_info,
			Err(error) => return super::RepaymentResult::Failed(error.into()),
		};
		let market_config = market_info.config();
		let borrow_asset_id = market_config.borrow_asset();
		match T::MultiCurrency::transfer(
			*borrow_asset_id,
			loan_account_id,
			market_account_id,
			payment_amount,
			keep_alive,
		) {
			Ok(balance) if is_principal_payment =>
				super::RepaymentResult::PrincipalAndLastInterestPaymentArePaidBackInTime(balance),
			Ok(balance) => super::RepaymentResult::InterestIsPaidInTime(balance),
			Err(error) if is_principal_payment =>
				super::RepaymentResult::PrincipalAndLastInterestPaymentAreNotPaidBackInTime(
					error.into(),
				),
			Err(error) => super::RepaymentResult::InterestIsNotPaidInTime(error.into()),
		}
	}
}

// Borrower pays back only principal, wheh the loan is mature.
// Fake strategy, just for example.
// TODO: @mikolaichuk: remove this strategy.
pub mod principal_only {
	use crate::{
		types::{LoanConfigOf, TimeMeasure},
		Config,
	};
	use frame_support::traits::fungibles::Transfer;

	pub fn apply<T: Config>(
		loan_config: LoanConfigOf<T>,
		current_moment: &TimeMeasure,
		keep_alive: bool,
	) -> super::RepaymentResult<T> {
		if current_moment != loan_config.last_payment_moment() {
			return super::RepaymentResult::Failed(
				"Error for fake strategy: the moment is not last one.".into(),
			)
		}
		let loan_account_id = loan_config.account_id();
		let market_account_id = loan_config.market_account_id();
		let market_config =
			match crate::Pallet::<T>::get_market_config_via_account_id(&market_account_id) {
				Ok(market_config) => market_config,
				Err(error) => return super::RepaymentResult::Failed(error.into()),
			};

		let borrow_asset_id = market_config.borrow_asset();
		match T::MultiCurrency::transfer(
			*borrow_asset_id,
			loan_account_id,
			market_account_id,
			*loan_config.principal(),
			keep_alive,
		) {
			Ok(balance) =>
				super::RepaymentResult::PrincipalAndLastInterestPaymentArePaidBackInTime(balance),
			Err(error) =>
				super::RepaymentResult::PrincipalAndLastInterestPaymentAreNotPaidBackInTime(
					error.into(),
				),
		}
	}
}
*/
