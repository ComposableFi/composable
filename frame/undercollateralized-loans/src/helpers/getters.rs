use crate::{
	types::{LoanConfigOf, LoanInfoOf, MarketConfigOf, MarketInfoOf, Timestamp},
	Config, Error, Pallet,
};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use frame_support::{ensure, traits::Get};
use sp_std::ops::Add;

impl<T: Config> Pallet<T> {
	pub(crate) fn get_market_info_via_account_id(
		market_account_id: &T::AccountId,
	) -> Result<MarketInfoOf<T>, Error<T>> {
		crate::MarketsStorage::<T>::try_get(market_account_id)
			.map_err(|_| Error::<T>::MarketDoesNotExist)
	}

	pub(crate) fn get_market_config_via_account_id(
		market_account_id: &T::AccountId,
	) -> Result<MarketConfigOf<T>, Error<T>> {
		let market_info = Self::get_market_info_via_account_id(market_account_id)?;
		Ok(market_info.config().clone())
	}

	pub(crate) fn get_loan_info_via_account_id(
		loan_account_id: &T::AccountId,
	) -> Result<LoanInfoOf<T>, Error<T>> {
		crate::LoansStorage::<T>::try_get(loan_account_id).map_err(|_| Error::<T>::LoanNotFound)
	}

	pub(crate) fn get_loan_config_via_account_id(
		loan_account_id: &T::AccountId,
	) -> Result<LoanConfigOf<T>, Error<T>> {
		let loan_info = Self::get_loan_info_via_account_id(loan_account_id)?;
		Ok(loan_info.config().clone())
	}

	pub(crate) fn get_payment_for_particular_moment(
		timestamp: Timestamp,
		loan_account_id: &T::AccountId,
	) -> Option<T::Balance> {
		return crate::ScheduleStorage::<T>::get(timestamp).get(loan_account_id).cloned()
	}

	pub(crate) fn get_current_date_timestamp() -> Timestamp {
		crate::CurrentDateStorage::<T>::get()
	}

	// Get current date from the storage.
	pub(crate) fn get_current_date() -> NaiveDate {
		Self::get_date_from_timestamp(Self::get_current_date_timestamp())
	}

	// Get naive date from a timestamp
	pub(crate) fn get_date_from_timestamp(timestamp: Timestamp) -> NaiveDate {
		NaiveDateTime::from_timestamp(timestamp, 0).date()
	}

	// Align a timestamp to the beginign of the day.
	// 24.08.1991 08:45:03 -> 24.08.1991 00:00:00
	// (in terms of seconds from the beginning of Unix epoche)
	pub(crate) fn get_date_aligned_timestamp(timestamp: Timestamp) -> Timestamp {
		Self::get_date_from_timestamp(timestamp)
			.and_time(NaiveTime::default())
			.timestamp()
	}

	// Returns shifted date aligned timestamp.
	// 24.08.1991 08:45:03 -> 28.08.1991 00:00:00
	// (in terms of seconds from the beginning of Unix epoche)
	pub(crate) fn get_shifted_date_aligned_timestamp(
		timestamp: Timestamp,
		days: i64,
	) -> Result<Timestamp, Error<T>> {
		ensure!(days <= T::MaxDateShiftingInDays::get(), Error::<T>::DateShiftingExceeded);
		Ok(Self::get_date_from_timestamp(timestamp)
			.add(Duration::days(days))
			.and_time(NaiveTime::default())
			.timestamp())
	}

	// Returns next date aligned timestamp.
	// 24.08.1991 08:45:03 -> 25.08.1991 00:00:00
	// (in terms of seconds from the beginning of Unix epoche)
	pub(crate) fn get_next_date_aligned_timestamp(timestamp: Timestamp) -> Timestamp {
		// Unwrapped since we use only one day shift
		Self::get_shifted_date_aligned_timestamp(timestamp, 1).expect("This mehtod never panics.")
	}
}
