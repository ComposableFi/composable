//! The `TWAP` module provides a helpful type and associated methods that are
//! generic over the underlying `twap` and `timestamp` types used to operate on
//! time weighted average price values.
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
// #![cfg_attr(
// 	not(test),
// 	deny(
// 		clippy::all,
// 		clippy::cargo,
// 		clippy::complexity,
// 		clippy::correctness,
// 		clippy::nursery,
// 		clippy::pedantic,
// 		clippy::perf,
// 		// clippy::restriction,
// 		clippy::style,
// 		clippy::suspicious,
// 		missing_docs,
// 		// rustdoc::missing_crate_level_docs,
// 		// rustdoc::missing_doc_code_examples,
// 		warnings,
// 	)
// )]

// TODO(Cardosaum): Make tests, all passing
#[cfg(test)]
mod tests;

use crate::labs::numbers::{FixedPointMath, UnsignedMath};
use num_traits::CheckedMul;
use sp_runtime::{
	traits::One,
	ArithmeticError::{self, Overflow},
	FixedPointNumber, FixedU128,
};
use std::cmp::Ordering::Greater;

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
pub struct Twap {
	/// The "time weighted average price", represented by a decimal number.
	twap: FixedU128,
	/// The last time when the [`twap`](Self::twap) value was updated.
	ts: u64,
	period: u64,

	// TODO(Cardosaum): Assess if it's better to have this values hard coded or
	// dinamic
	//
	// TODO(Cardosaum): Make tests with different values for these variables and
	// assess which would be the best value for them.
	since_last_min: u64,
	from_start_min: u64,
}

impl Twap {
	// TODO(Cardosaum): Update function documentation.
	/// Creates a new instance of [`Twap`], returning it.
	///
	/// # Examples
	/// TODO(Cardosaum)
	pub const fn new(twap: FixedU128, ts: u64, period: u64) -> Self {
		//  TODO(Cardosaum): Maybe remove this default value?
		let default_time = 1000; // 1 second
		Self { twap, ts, period, since_last_min: default_time, from_start_min: default_time }
	}

	// TODO(Cardosaum): Update function documentation.
	pub const fn get_twap(&self) -> FixedU128 {
		self.twap
	}

	// TODO(Cardosaum): Update function documentation.
	/// This function updates the [`twap`](Twap::twap) value using the default
	/// EMA function.
	///
	/// # Examples
	/// TODO(Cardosaum)
	///
	/// # Errors
	/// TODO(Cardosaum)
	pub fn accumulate(
		&mut self,
		price: &FixedU128,
		now: &u64,
	) -> Result<FixedU128, ArithmeticError> {
		// TODO(Cardosaum): Ensure time has passed before updating?
		// TODO(Cardosaum): If time passes more than period the call will always fail,
		// how to fix it?
		let since_last_tmp = now.try_sub(&self.ts)?.max(self.since_last_min);
		let (since_last, from_start, time) = match self.period.try_sub(&since_last_tmp) {
			Ok(from_start) => (since_last_tmp, from_start, self.ts),
			_ => (self.period.try_sub(&self.from_start_min)?, self.from_start_min, *now),
		};

		self.update_mut(price, from_start, since_last, &time)?;
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
	/// * [`ArithmeticError::Overflow`]
	fn update(
		&self,
		price: &FixedU128,
		from_start: u64,
		since_last: u64,
	) -> Result<FixedU128, ArithmeticError> {
		// TODO(Cardosaum): Create function that convert u64 to FixedU128
		let unit = FixedU128::DIV;
		let denominator = FixedU128::from_inner(
			unit.checked_mul(since_last.try_add(&from_start)?.into()).ok_or(Overflow)?,
		);
		let twap_t0 = self.twap.try_mul(&FixedU128::from_inner(
			unit.checked_mul(from_start.into()).ok_or(Overflow)?,
		))?;
		let twap_t1 = price.try_mul(&FixedU128::from_inner(
			unit.checked_mul(since_last.into()).ok_or(Overflow)?,
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
	/// * [`ArithmeticError::Overflow`]
	fn update_mut(
		&mut self,
		price: &FixedU128,
		from_start: u64,
		since_last: u64,
		ts: &u64,
	) -> Result<(), ArithmeticError> {
		self.twap = self.update(price, from_start, since_last)?;
		self.ts = *ts;
		Ok(())
	}

	// TODO(Cardosaum): Add internal function trying to update twap using U256
	// value to prevent overflows. Maybe doing the `U256` try we could recover
	// from an overflow? (check if that approach actually helps in something)
}
