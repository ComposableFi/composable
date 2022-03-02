use composable_support::validation::Validate;
use frame_support::{pallet_prelude::*, traits::Get};

#[derive(Debug, Decode)]
pub struct ValidTTL<U> {
	_marker: PhantomData<U>,
}

impl<U> Copy for ValidTTL<U> {}

impl<U> Clone for ValidTTL<U> {
	fn clone(&self) -> Self {
		*self
	}
}

#[derive(Debug, Decode)]
pub struct ValidTimeLockPeriod<U> {
	_marker: PhantomData<U>,
}

impl<U> Copy for ValidTimeLockPeriod<U> {}

impl<U> Clone for ValidTimeLockPeriod<U> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<TTL: PartialOrd, MinimumTTL> Validate<TTL, ValidTTL<MinimumTTL>> for ValidTTL<MinimumTTL>
where
	MinimumTTL: Get<TTL>,
{
	fn validate(input: TTL) -> Result<TTL, &'static str> {
		if input <= MinimumTTL::get() {
			return Err("TTL_BELOW_MINIMUM")
		}
		Ok(input)
	}
}

impl<TimeLockPeriod: PartialOrd, MinimumTimeLockPeriod>
	Validate<TimeLockPeriod, ValidTimeLockPeriod<MinimumTimeLockPeriod>>
	for ValidTimeLockPeriod<MinimumTimeLockPeriod>
where
	MinimumTimeLockPeriod: Get<TimeLockPeriod>,
{
	fn validate(input: TimeLockPeriod) -> Result<TimeLockPeriod, &'static str> {
		if input <= MinimumTimeLockPeriod::get() {
			return Err("TIME_LOCK_PERIOD_BELOW_MINIMUM")
		}
		Ok(input)
	}
}
