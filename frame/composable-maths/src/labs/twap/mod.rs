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

// #[cfg(test)]
// mod tests;

use crate::labs::numbers::{FixedPointMath, SignedMath};
use sp_runtime::{
	traits::AtLeast32Bit,
	ArithmeticError::{self, Overflow},
};

/// The [`Twap`] value itself, storing both the underlying time weighted average
/// price and its most recent timestamp.
///
/// The [`Twap`] type implement several convinience methods to facilitate when
/// working with time weighted average prices. Here is a list of all possible
/// functions that could be used with this type and also some examples:
///
/// # List of functions
///
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
#[derive(Debug, Clone, Copy)]
pub struct Twap<P, T>
where
	P: FixedPointMath + TryFrom<T, Error = ArithmeticError>,
	T: SignedMath + AtLeast32Bit + Into<i32> + Copy,
{
	/// The "time weighted average price", represented by a decimal number.
	pub twap: P,
	/// The last time when the [`twap`](Self::twap) value was updated.
	pub ts: T,
}

impl<'ts, P, T> Twap<P, T>
where
	P: FixedPointMath + TryFrom<T, Error = ArithmeticError> + TryFrom<&'ts T>,
	T: SignedMath + AtLeast32Bit + Into<i32> + 'ts + Copy,
{
	/// Creates a new instance of [`Twap`], returning it.
	pub const fn new(twap: P, ts: T) -> Self {
		Self { twap, ts }
	}

	/// This function *simulates* the [`twap`](Twap::twap) update, returning the
	/// value that would be used as the new [`twap`](Twap::twap), but **not**
	/// modifying the current value.
	///
	/// # Errors
	///
	/// * [`ArithmeticError::Overflow`]
	pub fn update(&self, price: &P, ts: &'ts T) -> Result<P, ArithmeticError> {
		// TODO(Cardosaum): Ensure time has passed before updating?
		let denominator = self.ts.try_add(ts)?;
		let weighted_twap_t0 = self.twap.try_mul(&self.ts.try_into()?)?;
		let weighted_twap_t1 = price.try_mul(&ts.try_into().map_err(|_| Overflow)?)?;

		weighted_twap_t0.try_add(&weighted_twap_t1)?.try_div(&denominator.try_into()?)
	}

	/// This function is similar to [`update`](Self::update), but it **does**
	/// change the current [`twap`](Twap::twap) value, and does not return
	/// anything in case of a successfull call.
	///
	/// # Errors
	///
	/// * [`ArithmeticError::Overflow`]
	pub fn update_mut(&mut self, price: &P, ts: &'ts T) -> Result<(), ArithmeticError> {
		self.twap = self.update(price, ts)?;
		self.ts = *ts;
		Ok(())
	}

	// TODO(Cardosaum): Add internal function trying to update twap using U256
	// value to prevent overflows. Maybe doing the `U256` try we could recover
	// from an overflow? (check if that approach actually helps in something)

	// TODO(Cardosaum): Creates an accumulate function?
}
