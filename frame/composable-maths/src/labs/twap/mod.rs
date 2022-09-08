//! The `TWAP` module provides a helpful type and associated methods that are
//! generic over the underlying `twap` and `timestamp` types used to operate on
//! time weighted average price values.
//!
//! TODO(Cardosaum): Create beautifull gif showing mark price and twap.
//!
//! TODO(Cardosaum): Add list of available methods and link them here. (same as in Vamm Pallet)
//!
//! # Examples
//!
//! ```
//! // Initialize some twap values.
//! let ts = Timestamp::now();
//! let base_twap = Twap::new(1337.0, ts);
//! let quote_twap = Twap::new(42.0, ts)
//!
//! // Some time passes...
//!
//! // Update twap values according to a exponencial moving average function:
//! let new_ts = Timestamp::now();
//! base_twap.update(25.0, new_ts)
//! quote_twap.update(1337.0, new_ts)
//!
//! // Check values
//! // TODO(Cardosaum): Check which would be the expected twap for both
//! // variables.
//! assert_eq!(base_twap.twap, twap)
//! assert_eq!(quote_twap.twap, twap)
//! ```
// TODO(Cardosaum): Expand this discription

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// Specify linters to EMA module.
// TODO(Cardosaum): Enable all these linters
// #![cfg_attr(
// 	not(test),
// 	deny(
// 		clippy::all,
// 		clippy::cargo,
// 		clippy::complexity,
// 		clippy::correctness,
// 		clippy::nursery,
// 		// clippy::pedantic,
// 		clippy::perf,
// 		// clippy::restriction,
// 		clippy::style,
// 		clippy::suspicious,
// 		missing_docs,
// 		rustdoc::missing_crate_level_docs,
// 		rustdoc::missing_doc_code_examples,
// 		warnings,
// 	)
// )]

// TODO(Cardosaum): Make tests, all passing
#[cfg(test)]
mod tests;

use crate::labs::numbers::{FixedPointMath, UnsignedMath};
use num_traits::CheckedMul;
use sp_runtime::{
	ArithmeticError::{self, Overflow},
	FixedPointNumber,
};
use sp_std::cmp::Ord;

/// The [`Twap`] value itself, storing both the underlying time weighted average
/// price and its most recent timestamp.
///
/// The [`Twap`] type implement several convinience methods to facilitate when
/// working with time weighted average prices. Here is a list of all possible
/// functions that could be used with this type and also some examples:
///
/// # List of functions
///
/// // TODO(Cardosaum): Update list of functions
/// * [`update`](Self::update): Computes new twap, returning it; Does *not*
/// modifies storage.
/// * [`update_mut`](Self::update_mut): Computes new twap, changing the old;
/// *Does* modify storage.
///
/// # Examples
///
/// ```
/// // Initialize twap value.
/// let ts = Timestamp::now();
/// let base_twap = Twap::new(42.0, ts);
///
/// // Some time passes...
///
/// // Simulate twap update.
/// let new_ts = Timestamp::now();
/// let simulated_new_twap = base_twap.update(25.0, new_ts);
///
/// // Actually update twap.
/// let mut base_twap_mut = base_twap.clone();
/// base_twap_mut.update_mut(25.0, new_ts);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Twap<FixedPoint, Moment>
where
	FixedPoint: FixedPointMath + FixedPointNumber,
	Moment: Copy + From<u64> + Into<FixedPoint::Inner> + Ord + UnsignedMath,
{
	/// The "time weighted average price", represented by a decimal number.
	twap: FixedPoint,
	/// The last time when the [`twap`](Self::twap) value was updated.
	ts: Moment,
	period: Moment,
	// TODO(Cardosaum): Assess if it's better to have this values hard coded or
	// dinamic
	//
	// TODO(Cardosaum): Make tests with different values for these variables and
	// assess which would be the best value for them.
	// since_last_min: M,
	// from_start_min: M,
}

impl<FixedPoint, Moment> Twap<FixedPoint, Moment>
where
	FixedPoint: FixedPointMath,
	Moment: Copy + From<u64> + Into<FixedPoint::Inner> + Ord + UnsignedMath,
{
	/// Creates a new [`Twap`] instance, returning it.
	///
	/// # Examples
	/// ```
	/// # use sp_runtime::FixedU128;
	/// # use composable_maths::labs::twap::Twap;
	/// // Set the initial twap value to `42.0`, with an initial timestamp
	/// // representing the date `Sun Sep 13 12:26:40 AM 2020`, and a period of
	/// // one hour.
	/// let price = FixedU128::from_float(42.0);
	/// let timestamp: u64 = 1600000000;
	/// let period: u64 = 3600;
	///
	/// let twap = Twap::new(price, timestamp, period);
	/// dbg!(twap);
	/// // Twap {
	/// //    twap: FixedU128(42.000000000000000000),
	/// //    ts: 1600000000,
	/// //    period: 3600,
	/// // };
	/// ```
	pub const fn new(twap: FixedPoint, ts: Moment, period: Moment) -> Self {
		//  TODO(Cardosaum): Maybe remove this default value?
		// let default_time = 1000; // 1 second
		// Self { twap, ts, period, since_last_min: default_time, from_start_min: default_time }
		Self { twap, ts, period }
	}

	/// Returns the Twap's value.
	///
	/// # Examples
	/// ```
	/// # use sp_runtime::FixedU128;
	/// # use composable_maths::labs::twap::Twap;
	/// let price = FixedU128::from_float(42.0);
	/// # let timestamp: u64 = 1600000000;
	/// # let period: u64 = 3600;
	///
	/// let twap = Twap::new(price, timestamp, period);
	///
	/// assert_eq!(twap.get_twap(), price);
	/// ```
	pub const fn get_twap(&self) -> FixedPoint {
		self.twap
	}

	/// Returns the Twap's timestamp.
	///
	/// # Examples
	/// ```
	/// # use sp_runtime::FixedU128;
	/// # use composable_maths::labs::twap::Twap;
	/// # let price = FixedU128::from_float(42.0);
	/// let timestamp: u64 = 1600000000;
	/// # let period: u64 = 3600;
	///
	/// let twap = Twap::new(price, timestamp, period);
	///
	/// assert_eq!(twap.get_timestamp(), timestamp);
	/// ```
	pub const fn get_timestamp(&self) -> Moment {
		self.ts
	}

	// TODO(Cardosaum): Update function documentation.
	/// Returns the Twap's period.
	///
	/// # Examples
	/// ```
	/// # use sp_runtime::FixedU128;
	/// # use composable_maths::labs::twap::Twap;
	/// # let price = FixedU128::from_float(42.0);
	/// # let timestamp: u64 = 1600000000;
	/// let period: u64 = 3600;
	///
	/// let twap = Twap::new(price, timestamp, period);
	///
	/// assert_eq!(twap.get_period(), period);
	/// ```
	pub const fn get_period(&self) -> Moment {
		self.period
	}

	/// Updates the Twap's value using the default exponential moving average
	/// function.
	///
	/// # Examples
	/// ```
	/// # use sp_runtime::FixedU128;
	/// # use composable_maths::labs::twap::Twap;
	/// # use frame_support::assert_ok;
	/// let mut price = FixedU128::from_float(42.0);
	/// let mut timestamp: u64 = 1600000000;
	/// let period: u64 = 3600;
	/// let mut twap = Twap::new(price, timestamp, period);
	///
	/// // Assumes one hour has passed.
	/// timestamp += period;
	///
	/// // Cut price by half.
	/// price = price / FixedU128::from_float(2.0);
	///
	/// // Update twap value with a new price.
	/// let result = twap.accumulate(&price, timestamp);
	/// assert_ok!(result, price);
	/// ```
	///
	/// # Errors
	///
	/// * [`ArithmeticError`]
	pub fn accumulate(
		&mut self,
		price: &FixedPoint,
		now: Moment,
	) -> Result<FixedPoint, ArithmeticError> {
		// TODO(Cardosaum): Ensure time has passed before updating?
		// maybe creating a new enum of erros would be good in this situation?
		let since_last = now.try_sub(&self.ts)?.max(1.into());
		match self.period.try_sub(&since_last) {
			Ok(from_start) => self.update_mut(price, from_start, since_last, now)?,
			_ => self.update_mut(price, 1.into(), self.period.try_sub(&1.into())?, now)?,
		};

		Ok(self.twap)
	}

	// TODO(Cardosaum): Update function documentation.
	// TODO(Cardosaum): Change function name (update implies the value will be mutated)
	/// This function *simulates* the [`twap`](Twap::twap) update, returning the
	/// value that would be used as the new [`twap`](Twap::twap), but **not**
	/// modifying the current value.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`]
	fn update(
		&self,
		price: &FixedPoint,
		from_start: Moment,
		since_last: Moment,
	) -> Result<FixedPoint, ArithmeticError> {
		let unit = FixedPoint::DIV;
		let denominator = FixedPoint::from_inner(
			unit.checked_mul(&since_last.try_add(&from_start)?.into()).ok_or(Overflow)?,
		);
		let twap_t0 = self.twap.try_mul(&FixedPoint::from_inner(
			unit.checked_mul(&from_start.into()).ok_or(Overflow)?,
		))?;
		let twap_t1 = price.try_mul(&FixedPoint::from_inner(
			unit.checked_mul(&since_last.into()).ok_or(Overflow)?,
		))?;

		twap_t0.try_add(&twap_t1)?.try_div(&denominator)
	}

	// TODO(Cardosaum): Update function documentation.
	// TODO(Cardosaum): `update_mut` and `update` are not good names...
	/// This function is similar to [`update`](Self::update), but it **does**
	/// change the current [`twap`](Twap::twap) value, and does not return
	/// anything in case of a successfull call.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`]
	fn update_mut(
		&mut self,
		price: &FixedPoint,
		from_start: Moment,
		since_last: Moment,
		ts: Moment,
	) -> Result<(), ArithmeticError> {
		self.twap = self.update(price, from_start, since_last)?;
		self.ts = ts;
		Ok(())
	}
}
