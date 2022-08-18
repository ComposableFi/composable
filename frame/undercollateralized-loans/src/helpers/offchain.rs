use crate::{
	types::{Timestamp, PaymentsOutcomes},
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
    
    fn submit_unsigned_payment_outcomes_transcations(today: Timestamp, possible_payment_outcomes: PaymentsOutcomes<T>) {
        // Split possible outcomes vector to avoid heavy transactions.
		// Unwrapped since CheckPaymentsBatchSize is u32.
		// Safe since we do not suppose compile node for 8-bit or 16-bit targets.
		let size = T::CheckPaymentsBatchSize::get()
			.try_into()
			.expect("This method does not panic.");
		for chunk in possible_payment_outcomes.chunks(size) {
			let call =
			Call::<T>::process_checked_payments { outcomes: chunk.to_vec(), timestamp: today };
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

	fn collect_possible_payments_outcomes(today: Timestamp) -> PaymentsOutcomes<T> {
		// Retrive collection of loans(loans' accounts ids) which have to be paid today.
		// If nothing found we get empty set.
		let mut possible_payments_outcomes = vec![];
		let loans_accounts_ids = crate::ScheduleStorage::<T>::try_get(today).unwrap_or_default();
		for loan_account_id in loans_accounts_ids {
	        let loan_config = match Self::get_loan_config_via_account_id(&loan_account_id) {
                Ok(loan_config) => loan_config,
                Err(error) => { log::error!("Error: {:?}", error);
                    continue;
                }
            };
			// Collect possible payment outcomes.
			if let Some(possible_payment_outcome) =
				Self::pre_process_payment(&loan_config, today)
			{
				possible_payments_outcomes.push(possible_payment_outcome);
			}
		}
        possible_payments_outcomes
    }	

    }
