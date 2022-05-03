pub mod decrement;
pub mod increment;
pub mod start_at;

/// Helper macro to create a type that can be used in [`DecrementToMax`].
///
/// # Usage
///
/// ```rust,ignore
/// error_to_pallet_error!(
///     SomeErrorType -> PalletErrorVariant;
/// );
/// ```
///
/// Note that this assumes that the pallet's `Error` and `Config` types are in scope and not
/// renamed.
#[macro_export]
macro_rules! error_to_pallet_error {
	($($name:ident -> $to:ident;)+) => {
		$(
			#[derive(::core::fmt::Debug, ::core::default::Default, ::core::cmp::PartialEq)]
			pub struct $name;

			impl<T: Config> From<$name> for Error<T> {
				fn from(_: $name) -> Error<T> {
					Error::<T>::$to
				}
			}

			// impl<T: From<$name>> From<$name> for DispatchError {
			// 	fn from(_: $name) -> DispatchError {
			// 		T::from($to).into()
			// 	}
			// }
		)+
	};
}
