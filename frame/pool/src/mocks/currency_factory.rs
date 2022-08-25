pub use pallet::*;
pub use proptest::{prelude::*, strategy::Strategy};

#[frame_support::pallet]
pub mod pallet {
	use composable_traits::currency::CurrencyFactory;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;
	use scale_info::TypeInfo;

	use proptest::prelude::*;

	// pub const PALLET_ID: PalletId = PalletId(*b"mck_curf");

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
		E,
		F,
		G,
		H,
		I,
		J,
		K,
		L,
		M,
		N,
		O,
		P,
		Q,
		R,
		S,
		T,
		U,
		V,
		W,
		X,
		Y,
		Z,
		LpToken(u32),
	}

	impl Default for MockCurrencyId {
		fn default() -> Self {
			MockCurrencyId::A
		}
	}

	pub fn strategy_pick_random_mock_currency() -> impl Strategy<Value = MockCurrencyId> {
		prop_oneof![
			Just(MockCurrencyId::A),
			Just(MockCurrencyId::B),
			Just(MockCurrencyId::C),
			Just(MockCurrencyId::D),
			Just(MockCurrencyId::E),
			Just(MockCurrencyId::F),
			Just(MockCurrencyId::G),
			Just(MockCurrencyId::H),
			Just(MockCurrencyId::I),
			Just(MockCurrencyId::J),
			Just(MockCurrencyId::K),
			Just(MockCurrencyId::L),
			Just(MockCurrencyId::N),
			Just(MockCurrencyId::O),
			Just(MockCurrencyId::P),
			Just(MockCurrencyId::Q),
			Just(MockCurrencyId::R),
			Just(MockCurrencyId::S),
			Just(MockCurrencyId::T),
			Just(MockCurrencyId::U),
			Just(MockCurrencyId::V),
			Just(MockCurrencyId::W),
			Just(MockCurrencyId::X),
			Just(MockCurrencyId::Y),
			Just(MockCurrencyId::Z),
			any::<u32>().prop_map(MockCurrencyId::LpToken),
		]
	}

	// impl Strategy for MockCurrencyId {
	// 	fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {

	// 	}
	// }

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
