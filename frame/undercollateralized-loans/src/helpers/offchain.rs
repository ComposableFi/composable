use crate::{
	types::{PaymentsOutcomes, Timestamp},
	Call, Config, Pallet,
};
use frame_support::traits::Get;
use frame_system::offchain::SubmitTransaction;

// Off-chain methods are collected here to avoid to be called from on-chain context.
impl<T: Config> Pallet<T> {
	pub(crate) fn sync_offchain_worker(today: Timestamp) {
		let payments_possible_outcomes = Self::collect_possible_payments_outcomes(today);
		Self::submit_unsigned_process_payments_transcations(payments_possible_outcomes);
		let expired_loans_accounts_ids = Self::collect_non_activated_expired_loans(today);
		Self::submit_unsigned_remove_loans_transcations(expired_loans_accounts_ids);
	}

	// Submit unsigned transaction to process checked payments.
	fn submit_unsigned_process_payments_transcations(
		possible_payment_outcomes: PaymentsOutcomes<T>,
	) {
		// Split possible outcomes vector to avoid heavy transactions.
		// Unwrapped since CheckPaymentsBatchSize is u32.
		// Safe since we do not suppose compile node for 8-bit or 16-bit targets.
		let size = T::CheckPaymentsBatchSize::get()
			.try_into()
			.expect("This method does not panic.");
		for chunk in possible_payment_outcomes.chunks(size) {
			let call = Call::<T>::process_checked_payments { outcomes: chunk.to_vec() };
			SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
                .map_or_else(|_| log::error!("Unable to submit process_checked_payments() unsigned transaction for the following payment outcomes: {:?}.", chunk.to_vec()),|_|());
		}
	}

	fn collect_possible_payments_outcomes(today: Timestamp) -> PaymentsOutcomes<T> {
		// Retrive collection of loans(loans' accounts ids) which have to be paid today.
		// If nothing found we get empty set.
		let mut possible_payments_outcomes = vec![];
		let loans_accounts_ids = crate::ScheduleStorage::<T>::try_get(today).unwrap_or_default();
		for loan_account_id in loans_accounts_ids {
			let loan_config = match Self::get_loan_config_via_account_id(&loan_account_id) {
				Ok(loan_config) => loan_config,
				Err(error) => {
					log::error!("Error: {:?}", error);
					continue
				},
			};
			// Collect possible payment outcomes.
			if let Some(possible_payment_outcome) = Self::treat_payment(&loan_config, today) {
				possible_payments_outcomes.push(possible_payment_outcome)
			}
		}
		possible_payments_outcomes
	}

	fn submit_unsigned_remove_loans_transcations(non_activated_expired_loans: Vec<T::AccountId>) {
		// Split loans accounts ids vector to avoid heavy transactions.
		// Unwrapped since CheckNonActivatedLoansBatchSize is u32.
		// Safe since we do not suppose compile node for 8-bit or 16-bit targets.
		let size = T::CheckNonActivatedLoansBatchSize::get()
			.try_into()
			.expect("This method does not panic.");
		for chunk in non_activated_expired_loans.chunks(size) {
			let call = Call::<T>::remove_loans { loans_accounts_ids: chunk.to_vec() };
			SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
                .map_or_else(|_| log::error!("Unable to submit remove_loans() unsigned transaction for the following loans: {:?}.", chunk.to_vec()),|_|());
		}
	}

	// Collect expired non-activated loans accounts ids.
	// Expired non-activated loans are loans which were not activated by borrower before first
	// payment date.
	pub(crate) fn collect_non_activated_expired_loans(today: Timestamp) -> Vec<T::AccountId> {
		let mut expired_loans_accounts_ids = vec![];
		for non_active_loan_account_id in crate::NonActiveLoansStorage::<T>::iter_keys() {
			match Self::get_loan_config_via_account_id(&non_active_loan_account_id) {
				Ok(loan_config) if today > *loan_config.first_payment_moment() =>
					expired_loans_accounts_ids.push(loan_config.account_id().clone()),
				Ok(_) => (),
				// If loan is marked as non-active but not presented in the loans' storage,
				// we remove it's id from the non-activated loans ids storage.
				Err(_) => crate::NonActiveLoansStorage::<T>::remove(non_active_loan_account_id),
			}
		}
		expired_loans_accounts_ids
	}
}
