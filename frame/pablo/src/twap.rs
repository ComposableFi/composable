use crate::{
	types::{PriceCumulative, TimeWeightedAveragePrice},
	Config, MomentOf, PriceCumulativeStateOf, PriceRatio, TWAPStateOf,
};
use composable_maths::dex::price::{compute_next_price_cumulative, compute_twap};
use composable_traits::defi::Rate;
use frame_support::{pallet_prelude::*, traits::Time};
use sp_runtime::{
	traits::{Saturating, Zero},
	DispatchError,
};

pub(crate) fn get_next_price_cumulative<T: Config>(
	pool_id: T::PoolId,
	previous_price_cumulative: &PriceCumulativeStateOf<T>,
) -> Result<(T::Balance, T::Balance), DispatchError> {
	let current_timestamp = T::Time::now();
	let rate_base = crate::Pallet::<T>::do_get_exchange_rate(pool_id, PriceRatio::NotSwapped)?;
	let rate_quote = crate::Pallet::<T>::do_get_exchange_rate(pool_id, PriceRatio::Swapped)?;
	let ((_, base_price_cumulative), (_, quote_price_cumulative)) = (
		compute_next_price_cumulative::<T::Convert, _, _>(
			previous_price_cumulative.timestamp,
			previous_price_cumulative.base_price_cumulative,
			current_timestamp,
			rate_base,
		)?,
		compute_next_price_cumulative::<T::Convert, _, _>(
			previous_price_cumulative.timestamp,
			previous_price_cumulative.quote_price_cumulative,
			current_timestamp,
			rate_quote,
		)?,
	);
	Ok((base_price_cumulative, quote_price_cumulative))
}

pub(crate) fn get_twap_price<T: Config>(
	current_base_price_cumulative: T::Balance,
	previous_base_price_cumulative: T::Balance,
	current_quote_price_cumulative: T::Balance,
	previous_quote_price_cumulative: T::Balance,
	elapsed: MomentOf<T>,
) -> Result<(Rate, Rate), DispatchError> {
	let average_price_base = compute_twap::<T::Convert, _, _>(
		current_base_price_cumulative,
		previous_base_price_cumulative,
		elapsed,
	)?;
	let average_price_quote = compute_twap::<T::Convert, _, _>(
		current_quote_price_cumulative,
		previous_quote_price_cumulative,
		elapsed,
	)?;
	Ok((average_price_base, average_price_quote))
}

pub(crate) fn update_price_cumulative_state<T: Config>(
	pool_id: T::PoolId,
	prev_price_cumulative: &mut Option<PriceCumulativeStateOf<T>>,
) -> Result<(T::Balance, T::Balance), DispatchError> {
	if let Some(previous_price_cumulative) = prev_price_cumulative {
		let current_timestamp = T::Time::now();
		let (base_price_cumulative, quote_price_cumulative) =
			get_next_price_cumulative::<T>(pool_id, previous_price_cumulative)?;
		*prev_price_cumulative = Some(PriceCumulative {
			timestamp: current_timestamp,
			base_price_cumulative,
			quote_price_cumulative,
		});
		Ok((base_price_cumulative, quote_price_cumulative))
	} else {
		Ok((T::Balance::zero(), T::Balance::zero()))
	}
}

pub(crate) fn update_twap_state<T: Config>(
	base_price_cumulative: T::Balance,
	quote_price_cumulative: T::Balance,
	prev_twap_state: &mut Option<TWAPStateOf<T>>,
) -> Result<(), DispatchError> {
	if let Some(previous_twap_state) = prev_twap_state {
		let current_timestamp = T::Time::now();
		ensure!(
			current_timestamp > previous_twap_state.timestamp,
			DispatchError::Other("Stale TWAP Request Found")
		);
		let elapsed = current_timestamp.saturating_sub(previous_twap_state.timestamp);
		if elapsed >= T::TWAPInterval::get() {
			let (average_price_base, average_price_quote) = get_twap_price::<T>(
				base_price_cumulative,
				previous_twap_state.base_price_cumulative,
				quote_price_cumulative,
				previous_twap_state.quote_price_cumulative,
				elapsed,
			)?;
			*prev_twap_state = Some(TimeWeightedAveragePrice {
				base_price_cumulative,
				quote_price_cumulative,
				timestamp: current_timestamp,
				base_twap: average_price_base,
				quote_twap: average_price_quote,
			});
		} else {
			return DispatchError::Other("Elapsed time < TWAPInterval").into()
		}
	}
	Ok(())
}
