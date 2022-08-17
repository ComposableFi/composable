use crate::{
	types::{
		Timestamp, PossiblePaymentOutcome, PossiblePaymentsOutcomes,
	},
	Config, Pallet, Call
};
use frame_system::offchain::SubmitTransaction;
use frame_support::traits::Get;

// Off-chain methods are collected here to avoid to be called from on-chain context. 
impl<T: Config> Pallet<T> {
	pub(crate) fn sync_offchain_worker(today: Timestamp) {
        let payments_possible_outcomes = Self::collect_possible_payments_outcomes(today);
        Self::submit_unsigned_payment_outcomes_transcations(today, payments_possible_outcomes);
    }
    
    fn submit_unsigned_payment_outcomes_transcations(today: Timestamp, possible_payment_outcomes: PossiblePaymentsOutcomes<T>) {
        // Split possible outcomes vector to avoid heavy transactions.
		// Unwrapped since CheckPaymentsBatchSize is u32.
		// Safe since we do not suppose compile node for 8-bit or 16-bit targets.
		let size = T::CheckPaymentsBatchSize::get()
			.try_into()
			.expect("This method does not panic.");
		for chunk in possible_payment_outcomes.chunks(size) {
			let call =
			Call::<T>::check_payments { outcomes: chunk.to_vec(), timestamp: today };
			SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
                .map_or_else(|_| log::error!("Unable to submit unsigned transaction for the following payment outcomes: {:?}.", chunk.to_vec()),|_|());
		}
		// TODO: @mikolaichuk: Move to on_init()
		// We have to remove schedule for the treated day.
		// Since only part of payment checks peformed as a separate transaction,
		// we can not be sure that all transcations will be finished within one day.
		// So one week gap is used.
		let one_week_before = Self::get_one_week_before_date_aligned_timestamp(today);
		crate::ScheduleStorage::<T>::remove(one_week_before);
	}

	fn collect_possible_payments_outcomes(today: Timestamp) -> PossiblePaymentsOutcomes<T> {
		// Retrive collection of loans(loans' accounts ids) which have to be paid now.
		// If nothing found we get empty set.
		let mut possible_payments_outcomes = vec![];
		let loans_accounts_ids = crate::ScheduleStorage::<T>::try_get(today).unwrap_or_default();
		for loan_account_id in loans_accounts_ids {
			// Treat situation when non-existent loan's id is in the global schedule.
			if !crate::LoansStorage::<T>::contains_key(&loan_account_id) {
				continue
			};
			// Collect possible payment outcomes.
			if let Some(possible_payment_outcome) =
				Self::treat_payment(&loan_account_id, today)
			{
				possible_payments_outcomes.push(possible_payment_outcome);
			}
		}
        possible_payments_outcomes
    }	

    // Try to perfom paricular payment.
	fn treat_payment(
		loan_account_id: &T::AccountId,
		today: Timestamp,
	) -> Option<PossiblePaymentOutcome<T::AccountId>> {
	    // If loan's config does not found we retun None. 	
        let loan_config = Self::get_loan_config_via_account_id(loan_account_id).ok()?;
	    // If there is no such date in the local loan's schedule we return None.	
        let payment_amount = loan_config.get_payment_for_particular_moment(&today)?;
	    // Use to check if payment transfer is possible.
        // Please note that off-chain methods do not change chain's state. 
        let outcome = match Self::pay_back_borrowed_asset(&loan_account_id, *payment_amount) {
            // We have enough money on the loans account to perform last payment.
			Ok(_) if *loan_config.last_payment_moment() == today =>
				PossiblePaymentOutcome::LastPaymentMaySucceed(loan_account_id.clone()),
            // We have enough money on the loans account to perform regular payment.
            Ok(_) => PossiblePaymentOutcome::RegularPaymentMaySucceed(loan_account_id.clone()),
			// TODO: @mikolaichuk:  we should give to borrower
			//                      several attempts.
	        // Payment is failed.	
            Err(_) => PossiblePaymentOutcome::PaymentFailed(loan_account_id.clone()),
		};
		Some(outcome)
	}
}
