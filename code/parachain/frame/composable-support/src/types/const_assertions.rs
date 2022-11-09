//! Workarounds for `generic_const_exprs` on stable rust.
//!
//! See https://github.com/rust-lang/rust/issues/76560 for more information.

/// Asserts that the parameter `X` is non-zero.
///
/// Since generic_const_exprs isn't stable yet, this check can't be done in the type system -
/// instead, the check is pushed to the evaluation of an associated const (AssertNonZero::OK).
///
/// # Examples
///
/// ```rust
/// # use composable_support::types::const_assertions::AssertNonZero;
///
/// // a custom implementation of a non-zero u64
/// pub struct NonZeroU64(u64);
///
/// impl NonZeroU64 {
///     pub const fn new<const N: u64>() -> Self {
///         // this will raise a const_err if N is 0:
///         //
///         // error[E0080]: evaluation of `composable_support::types::rational::AssertNonZero::<0>::OK` failed
///         // attempt to compute `0_u8 - 1_u8`, which would overflow
///         let _ = AssertNonZero::<N>::OK;
///         
///         Self(N)
///     }
/// }
/// ```
pub struct AssertNonZero<const X: u64>;

impl<const X: u64> AssertNonZero<X> {
	pub const OK: u8 = 0 - !(X > 0) as u8;
}
