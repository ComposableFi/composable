//Bribe DAO

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use composable_traits::bribe::{Bribe, CreateBribeRequest, TakeBribeRequest};
	use frame_support::pallet_prelude::*;
	use num_traits::{CheckedAdd, CheckedMul, CheckedSub, SaturatingSub};
	use sp_runtime::traits::{AtLeast32BitUnsigned, Zero};

	pub type BribeIndex = u32;
	pub type ReferendumIndex = pallet_democracy::ReferendumIndex;

	#[pallet::config]
	pub trait Config: /* pallet_balances::Config + */ frame_system::Config {
		// TODO: Balance traits
		// following are copied from pallet-vault
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ Zero;

		// TODO: Conviction traits
		type Conviction: Codec;
		// TODO: CurrencyId traits
		type CurrencyId: Codec;
		// type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// /// Type representing the weight of this pallet
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// #[pallet::hooks]
	// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	// 	fn lorem(_n: T::BlockNumber) -> Weight {
	// 		0
	// 	}

	// 	fn on_finalize(_n: T::BlockNumber) {}
	// }

	// #[pallet::call]
	// impl<T: Config> Pallet<T> {
	// 	#[pallet::weight(
	// 		<T as pallet::Config>::WeightInfo::accumulate_dummy((*increase_by).saturated_into())
	// 	)]
	// 	pub fn accumulate_dummy(origin: OriginFor<T>, increase_by: T::Balance) -> DispatchResult {
	// 		let _sender = ensure_signed(origin)?;
	// 		<Dummy<T>>::mutate(|dummy| {
	// 			let new_dummy = dummy.map_or(increase_by, |d| d.saturating_add(increase_by));
	// 			*dummy = Some(new_dummy);
	// 		});

	// 		Self::deposit_event(Event::AccumulateDummy(increase_by));
	// 		Ok(())
	// 	}
	// }

	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	// pub enum Event<T: Config> {
	// 	AccumulateDummy(BalanceOf<T>),
	// 	SetDummy(BalanceOf<T>),
	// }

	#[pallet::storage]
	#[pallet::getter(fn bribe_requests)]
	pub(super) type BribeRequests<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BribeIndex,
		CreateBribeRequest<ReferendumIndex, T::Conviction, T::Balance, T::CurrencyId>,
	>;

	// // The genesis config type.
	// #[pallet::genesis_config]
	// pub struct GenesisConfig<T: Config> {
	// 	pub dummy: T::Balance,
	// 	pub bar: Vec<(T::AccountId, T::Balance)>,
	// 	pub foo: T::Balance,
	// }

	// #[cfg(feature = "std")]
	// impl<T: Config> Default for GenesisConfig<T> {
	// 	fn default() -> Self {
	// 		Self { dummy: Default::default(), bar: Default::default(), foo: Default::default() }
	// 	}
	// }

	// #[pallet::genesis_build]
	// impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
	// 	fn build(&self) {
	// 		<Dummy<T>>::put(&self.dummy);
	// 		for (a, b) in &self.bar {
	// 			<Bar<T>>::insert(a, b);
	// 		}
	// 		<Foo<T>>::put(&self.foo);
	// 	}
	// }

	impl<T: Config> Bribe for Pallet<T> {
		type Balance = T::Balance;
		type BribeIndex = BribeIndex;
		type Conviction = T::Conviction;
		// TODO: CurrencyId type
		type CurrencyId = T::CurrencyId;
		type ReferendumIndex = ReferendumIndex;

		fn create_bribe(
			_request: CreateBribeRequest<
				Self::ReferendumIndex,
				Self::Conviction,
				Self::Balance,
				Self::CurrencyId,
			>,
		) -> Result<Self::BribeIndex, DispatchError> {
			todo!()
		}

		fn take_bribe(
			_request: TakeBribeRequest<Self::BribeIndex, Self::Balance, Self::Conviction>,
		) -> Result<bool, DispatchError> {
			todo!()
		}
	}
}

// impl<T: Config> Pallet<T> {
// 	#[allow(dead_code)]
// 	fn accumulate_foo(origin: T::Origin, increase_by: T::Balance) -> DispatchResult {
// 		let _sender = ensure_signed(origin)?;

// 		let prev = <Foo<T>>::get();
// 		let result = <Foo<T>>::mutate(|foo| {
// 			*foo = foo.saturating_add(increase_by);
// 			*foo
// 		});
// 		assert!(prev + increase_by == result);
// 		Ok(())
// 	}
// }

// pub struct WatchDummy<T: Config + Send + Sync>(PhantomData<T>);

// impl<T: Config + Send + Sync> sp_std::fmt::Debug for WatchDummy<T> {
// 	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
// 		write!(f, "WatchDummy")
// 	}
// }
