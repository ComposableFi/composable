pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::traits::CurrencyFactory;
	use composable_traits::currency::RangeId;
	use frame_support::{pallet_prelude::*, PalletId};
	use frame_system::pallet_prelude::OriginFor;
	use scale_info::TypeInfo;

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
		codec::MaxEncodedLen,
		serde::Serialize,
		serde::Deserialize,
		TypeInfo,
	)]
	#[allow(clippy::upper_case_acronyms)] // currencies should be CONSTANT_CASE
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

	impl From<u128> for MockCurrencyId {
		fn from(id: u128) -> Self {
			match id {
				0 => MockCurrencyId::A,
				1 => MockCurrencyId::B,
				2 => MockCurrencyId::C,
				3 => MockCurrencyId::D,
				x => MockCurrencyId::LpToken(x as u32),
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
	// FIXME: Temporary fix to get CI to pass, separate PRs will be made per pallet to refactor to
	// use OptionQuery instead
	#[allow(clippy::disallowed_type)]
	pub type CurrencyCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(_origin: OriginFor<T>, id: RangeId) -> DispatchResultWithPostInfo {
			let currency_id = <Self as CurrencyFactory<MockCurrencyId>>::create(id)?;
			Self::deposit_event(Event::Created(currency_id));
			Ok(().into())
		}
	}

	impl<T: Config> CurrencyFactory<MockCurrencyId> for Pallet<T> {
		fn create(_: RangeId) -> Result<MockCurrencyId, DispatchError> {
			let lp_token_id = CurrencyCounter::<T>::mutate(|c| {
				*c += 1;
				*c
			});
			Ok(MockCurrencyId::LpToken(lp_token_id))
		}
	}
}
