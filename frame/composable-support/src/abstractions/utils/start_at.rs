use sp_runtime::traits::{One, Zero};

use crate::abstractions::utils::start_at::sealed::Sealed;

pub trait StartAtValue<T: 'static>: Sealed + 'static {
	fn value() -> T;
}

/// Marker for nonces that should start at `<T as Zero>::zero`.
pub struct ZeroInit;

impl<T: Zero + 'static> StartAtValue<T> for ZeroInit {
	fn value() -> T {
		T::zero()
	}
}

/// Marker for nonces that should start at `<T as One>::one`.
pub struct OneInit;

impl<T: One + 'static> StartAtValue<T> for OneInit {
	fn value() -> T {
		T::one()
	}
}

/// Marker for nonces that should start at `<T as Default>::default`.
pub struct DefaultInit;

impl<T: Default + 'static> StartAtValue<T> for DefaultInit {
	fn value() -> T {
		T::default()
	}
}

mod sealed {
	use super::*;

	/// Sealing trait for [`StartAtValue`][super::StartAtValue]. If you want to add a new
	/// implementor, be sure to add it here and ensure it's tested.
	pub trait Sealed {}

	impl Sealed for ZeroInit {}
	impl Sealed for OneInit {}
	impl Sealed for DefaultInit {}
}
