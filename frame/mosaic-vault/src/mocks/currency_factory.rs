pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::traits::CurrencyFactory;
	use frame_support::{pallet_prelude::*, PalletId};
	use frame_system::pallet_prelude::OriginFor;
	use composable_traits::{
       currency::DynamicCurrencyId,
	};
use scale_info::TypeInfo;
use sp_runtime::{
	ArithmeticError,
	DispatchError
};

	pub const PALLET_ID: PalletId = PalletId(*b"mck_curf");

	#[derive(
		PartialOrd,
		Ord,
		PartialEq,
		Eq,
		Debug,
		Copy,
		Clone,
		codec::Encode,
		codec::Decode,
		serde::Serialize,
		serde::Deserialize,
		TypeInfo,
	)]
	pub enum MockCurrencyId {
		A,
		B,
		C,
		D,
		LpToken(u32),
	}

	impl Default for MockCurrencyId {
		fn default() -> Self {
			MockCurrencyId::A
		}
	}

	impl DynamicCurrencyId for MockCurrencyId {
		fn next(self) -> Result<Self, DispatchError> {
			match self {
				MockCurrencyId::LpToken(x) => Ok(MockCurrencyId::LpToken(
					x.checked_add(1).ok_or(DispatchError::Arithmetic(ArithmeticError::Overflow))?,
				)),
				_=>unreachable!(),
			}
		}
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created(MockCurrencyId),
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn vault_count)]
	pub type CurrencyCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let currency_id = <Self as CurrencyFactory<MockCurrencyId>>::create()?;
			Self::deposit_event(Event::Created(currency_id));
			Ok(().into())
		}
	}

	impl<T: Config> CurrencyFactory<MockCurrencyId> for Pallet<T> {
		fn create() -> Result<MockCurrencyId, DispatchError> {
			let lp_token_id = CurrencyCounter::<T>::mutate(|c| {
				*c += 1;
				*c
			});
			Ok(MockCurrencyId::LpToken(lp_token_id))
		}
	}
}
