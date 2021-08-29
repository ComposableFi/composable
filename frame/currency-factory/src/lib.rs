#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]
// TODO remove me!
#![allow(missing_docs)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use composable_traits::currency::CurrencyFactory;
	use frame_support::{pallet_prelude::*, PalletId};
	use sp_runtime::{traits::Convert, ArithmeticError};
	use sp_std::fmt::Debug;

	pub const PALLET_ID: PalletId = PalletId(*b"pal_curf");

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug;
		type Convert: Convert<Self::CurrencyId, u128> + Convert<u128, Self::CurrencyId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn currency_count)]
	pub type CurrencyCounter<T: Config> = StorageValue<_, u128, ValueQuery>;

	impl<T: Config> CurrencyFactory<T::CurrencyId> for Pallet<T> {
		fn create() -> Result<T::CurrencyId, DispatchError> {
			CurrencyCounter::<T>::mutate(|c| {
				let c_current = *c;
				let c_next = c_current
					.checked_add(1)
					.ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?;
				*c = c_next;
				Ok(T::Convert::convert(c_next))
			})
		}
	}
}
