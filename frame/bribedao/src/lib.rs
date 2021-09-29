//Bribe DAO

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn lorem(_n: T::BlockNumber) -> Weight {
			0
		}

		fn on_finalize(_n: T::BlockNumber) {}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::accumulate_dummy((*increase_by).saturated_into())
		)]
		pub fn accumulate_dummy(origin: OriginFor<T>, increase_by: T::Balance) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			<Dummy<T>>::mutate(|dummy| {
				let new_dummy = dummy.map_or(increase_by, |d| d.saturating_add(increase_by));
				*dummy = Some(new_dummy);
			});

			Self::deposit_event(Event::AccumulateDummy(increase_by));
			Ok(())
		}
	}

	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AccumulateDummy(BalanceOf<T>),
		SetDummy(BalanceOf<T>),
	}

	#[pallet::storage]
	#[pallet::getter(fn dummy)]
	pub(super) type Dummy<T: Config> = StorageValue<_, T::Balance>;

	#[pallet::storage]
	#[pallet::getter(fn bar)]
	pub(super) type Bar<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance>;

	#[pallet::storage]
	#[pallet::getter(fn foo)]
	pub(super) type Foo<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type CountedMap<T> = CountedStorageMap<_, Blake2_128Concat, u8, u16>;

	// The genesis config type.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub dummy: T::Balance,
		pub bar: Vec<(T::AccountId, T::Balance)>,
		pub foo: T::Balance,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { dummy: Default::default(), bar: Default::default(), foo: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<Dummy<T>>::put(&self.dummy);
			for (a, b) in &self.bar {
				<Bar<T>>::insert(a, b);
			}
			<Foo<T>>::put(&self.foo);
		}
	}
}

impl<T: Config> Pallet<T> {
	#[allow(dead_code)]
	fn accumulate_foo(origin: T::Origin, increase_by: T::Balance) -> DispatchResult {
		let _sender = ensure_signed(origin)?;

		let prev = <Foo<T>>::get();
		let result = <Foo<T>>::mutate(|foo| {
			*foo = foo.saturating_add(increase_by);
			*foo
		});
		assert!(prev + increase_by == result);
		Ok(())
	}
}

pub struct WatchDummy<T: Config + Send + Sync>(PhantomData<T>);

impl<T: Config + Send + Sync> sp_std::fmt::Debug for WatchDummy<T> {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "WatchDummy")
	}
}
