use codec::{Encode, Decode};
use frame_support::RuntimeDebug;
use sp_runtime::DispatchError;
use scale_info::TypeInfo;

#[derive(Encode, Decode, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub enum RepaymentStrategy {
    InterestPeriodicallyPrincipalWhenMature,
    PrincipalOnlyWhenMature,
}

// This enum is used since we do not want to return an Error in the on_initalize() function.
pub enum RepaymentResult<T: crate::Config> {
    Failed(DispatchError),
    InterestIsPaidInTime(T::Balance), 
    InterestIsNotPaidInTime(DispatchError),
    PrincipalAndLastInterestPaymentArePaidBackInTime(T::Balance),
    PrincipalAndLastInterestPaymentAreNotPaidBackInTime(DispatchError),
}

/// Borrower pays interest with payment frequency, and pays back principal when the loan is mature.
pub mod interest_periodically_principal_when_mature_strategy {
    use crate::{Config, types};
    use frame_support::traits::fungibles::Transfer;
    use composable_support::math::safe::SafeAdd;

        pub fn apply<T: Config>(loan_info: types::LoanInfoOf<T>, current_block_number: T::BlockNumber, keep_alive: bool) -> super::RepaymentResult<T> {
            // TODO: @mikolaichuk: Move most demanded fileds retrieving into a separate function.
            let loan_config = loan_info.config().clone();
            // Will not overflow since we multiply to percent.
            let mut payment_amount = *loan_config.interest() * *loan_config.principal();
            let mut is_principal_payment = false; 
            // If it is time to repay principal  
            if current_block_number == *loan_info.end_block() {
                payment_amount += *loan_config.principal();
                is_principal_payment = true; 
            }
            let loan_account_id = loan_config.account_id();
            let market_account_id = loan_config.market_account_id();
            // TODO: @mikolaichuk: Are the cases when we can get error here? 
            let market_info = crate::Pallet::<T>::get_market_info_via_account_id(&market_account_id);
            // TODO: @mikolaichuk: Check if this can be done in more idiomatic way. 
            let market_info = match market_info {
                Ok(market_info) => market_info,
                Err(error)  => return super::RepaymentResult::Failed(error.into()),
            };
            let market_config = market_info.config();
            let borrow_asset_id = market_config.borrow_asset();
            let next_payment_block_number = match current_block_number.safe_add(loan_config.payment_frequency()) {
                Err(error) => return super::RepaymentResult::Failed(error.into()), 
                Ok(block_number) if block_number > *loan_info.end_block() => return super::RepaymentResult::Failed(crate::Error::<T>::CurrentBlockNumberExceedsFinalBlockNumberForTheLoan.into()), 
                Ok(block_number) => block_number,
            };
            // Update payments schedule.
            crate::PaymentsScheduleStorage::<T>::mutate(next_payment_block_number, |loans_accounts_set| {
			    loans_accounts_set.insert(loan_account_id.clone());
		    });
            match T::MultiCurrency::transfer(*borrow_asset_id, loan_account_id, market_account_id, payment_amount, keep_alive) {
                Ok(balance) if is_principal_payment => super::RepaymentResult::PrincipalAndLastInterestPaymentArePaidBackInTime(balance),
                Ok(balance) => super::RepaymentResult::InterestIsPaidInTime(balance),
                Err(error) if is_principal_payment => super::RepaymentResult::PrincipalAndLastInterestPaymentAreNotPaidBackInTime(error.into()),
                Err(error) => super::RepaymentResult::InterestIsNotPaidInTime(error.into()),  
            }
            
    }
}
/// Borrower pays back only principal, wheh the loan is mature.
/// Fake strategy, just for example.
pub mod principal_only_fake_strategy {
    use crate::{types, Config};
    use frame_support::traits::fungibles::Transfer;

        pub fn apply<T: Config>(loan_info: types::LoanInfoOf<T>, keep_alive: bool) -> super::RepaymentResult<T> {
            let loan_config = loan_info.config().clone();
            let loan_account_id = loan_config.account_id();
            let market_account_id = loan_config.market_account_id();
            let market_info = crate::Pallet::<T>::get_market_info_via_account_id(&market_account_id);
            let market_info = match market_info {
                Ok(market_info) => market_info,
                Err(error)  => return super::RepaymentResult::Failed(error.into()),
            };
            let market_config = market_info.config();
            let borrow_asset_id = market_config.borrow_asset();
            match T::MultiCurrency::transfer(*borrow_asset_id, loan_account_id, market_account_id, *loan_config.principal(), keep_alive) {
                Ok(balance) => super::RepaymentResult::PrincipalAndLastInterestPaymentArePaidBackInTime(balance),
                Err(error)  => super::RepaymentResult::PrincipalAndLastInterestPaymentAreNotPaidBackInTime(error.into()),
            }
        }
}


