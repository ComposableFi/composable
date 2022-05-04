pub mod decrement;
pub mod increment;
pub mod start_at;

/// Helper macro to create a type that can be used in [`IncrementToMax`][increment::IncrementToMax].
///
/// # Usage
///
/// ```rust,ignore
/// #[pallet::config]
/// pub trait Config: frame_system::Config {
///     type SomeType: Copy + Zero + SafeAdd + One + TypeInfo + Member + FullCodec;
///
///     #[pallet::constant]
///     type SomeTypeMaxValue: Get<Self::SomeType>;
/// }
///
/// #[pallet::error]
/// #[derive(PartialEqNoBound)]
/// pub enum Error<T> {
///     SomeTypeTooLarge
/// }
///
/// #[pallet::storage]
/// #[allow(clippy::disallowed_type)] // counter
/// pub type Counter_ZeroInit_ToMax<T: Config> = StorageValue<
///     _,
///     T::SomeType,
///     ValueQuery,
///     Counter<
///         ZeroInit,
///         IncrementToMax<
///             T::SomeTypeMaxValue,
///             SomeTypeTooLarge,
///             Error<T>,
///         >,
///         SafeDecrement,
///     >,
/// >;
///
/// error_to_pallet_error!(
///      SomeTypeTooLarge,
/// );
/// ```
///
/// Note that this assumes that the pallet's `Error` and `Config` types are in scope and not
/// renamed.
#[macro_export]
macro_rules! error_to_pallet_error {
    ($($name:ident,)+) => {
        $(
            #[derive(::core::fmt::Debug, ::core::default::Default, ::core::cmp::PartialEq)]
            pub struct $name;

            impl<T: Config> From<$name> for Error<T> {
                fn from(_: $name) -> Error<T> {
                    Error::<T>::$name
                }
            }
        )+
    };
}

/// `#[allow(clippy::disallowed_types)]` on an import currently errors:
///
/// ```rust,ignore
/// #[allow(clippy::disallowed_types)]
/// use frame_support::pallet_prelude::ValueQuery;
/// ```
///
/// Output:
///
/// ```plaintext
/// error: useless lint attribute
///   --> frame/composable-support/src/abstractions/nonce/mod.rs:14:1
///    |
/// 14 | #[allow(clippy::disallowed_types)]
///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(clippy::disallowed_types)]`
///    |
///    = note: `#[deny(clippy::useless_attribute)]` on by default
///    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#useless_attribute
/// ```
///
/// This type is a re-export to allow for importing it (as opposed to using a fully qualified
/// path) when using a nonce in a pallet that isn't importing `frame_support::pallet_prelude::*`.
#[allow(clippy::disallowed_types)]
pub type ValueQuery = frame_support::pallet_prelude::ValueQuery;
