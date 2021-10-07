//Bribe DAO

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::Codec;
	use composable_traits::bribe::Bribe;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use num_traits::{CheckedAdd, CheckedMul, CheckedSub, SaturatingSub};
	use sp_runtime::traits::{AtLeast32BitUnsigned, Zero};

	pub type BribeIndex = u32;
	pub type ReferendumIndex = pallet_democracy::ReferendumIndex;
	pub type CreateBribeRequest<T> = composable_traits::bribe::CreateBribeRequest<
		ReferendumIndex,
		<T as Config>::Balance,
		<T as Config>::Conviction,
		<T as Config>::CurrencyId,
	>;
	pub type TakeBribeRequest<T> = composable_traits::bribe::TakeBribeRequest<
		BribeIndex,
		<T as Config>::Balance,
		<T as Config>::Conviction,
	>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// REVIEW(oleksii): Balance traits; following are copied from pallet-vault
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

		type Conviction: Parameter;

		// TODO(oleksii): CurrencyId traits
		type CurrencyId: Parameter;

		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// TODO(oleksii): WeightInfo type
		// type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		BribeCreated { id: BribeIndex, request: CreateBribeRequest<T> },
		BribeTaken { id: BribeIndex, request: TakeBribeRequest<T> },
	}

	/// The number of bribes, also used to generate the next bribe identifier.
	///
	/// # Note
	///
	/// Cleaned up bribes do not decrement the counter.
	#[pallet::storage]
	#[pallet::getter(fn bribe_count)]
	pub(super) type BribeCount<T: Config> = StorageValue<_, BribeIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn bribe_requests)]
	pub(super) type BribeRequests<T: Config> =
		StorageMap<_, Blake2_128Concat, BribeIndex, CreateBribeRequest<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_bribe(
			origin: OriginFor<T>,
			request: CreateBribeRequest<T>,
		) -> DispatchResultWithPostInfo {
			let _from = ensure_signed(origin)?;
			let id = <Self as Bribe>::create_bribe(request.clone())?;
			Self::deposit_event(Event::BribeCreated { id, request });
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn take_bribe(
			origin: OriginFor<T>,
			request: TakeBribeRequest<T>,
		) -> DispatchResultWithPostInfo {
			let _from = ensure_signed(origin)?;
			let bribe_index = request.bribe_index;
			let bribe_taken = <Self as Bribe>::take_bribe(request.clone())?;
			if bribe_taken {
				Self::deposit_event(Event::BribeTaken { id: bribe_index, request });
			}
			Ok(().into())
		}
	}

	// TODO(oleksii): Errors (#[pallet::error])
	#[pallet::error]
	pub enum Error<T> {
		InvalidBribe,
		InvalidIndex,
		NotEnoughFunds,
		NotEnoughStake,
		PriceNotRequested,
		AlreadyBribed,
	}

	impl<T: Config> Bribe for Pallet<T> {
		type BribeIndex = BribeIndex;
		type ReferendumIndex = ReferendumIndex;

		type Balance = T::Balance;
		type Conviction = T::Conviction;
		type CurrencyId = T::CurrencyId;

		fn create_bribe(request: CreateBribeRequest<T>) -> Result<Self::BribeIndex, DispatchError> {
			Self::do_create_bribe(request)
		}

		fn take_bribe(request: TakeBribeRequest<T>) -> Result<bool, DispatchError> {
			Self::do_take_bribe(request)
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_create_bribe(request: CreateBribeRequest<T>) -> Result<BribeIndex, DispatchError> {
			let id = BribeCount::<T>::mutate(|id| {
				*id += 1;
				*id
			});

			ensure!(BribeRequests::<T>::contains_key(id), Error::<T>::AlreadyBribed); //dont duplicate briberequest if we already have it
			BribeRequests::<T>::insert(id, request);
			Ok(id)
		}

		fn do_take_bribe(request: TakeBribeRequest<T>) -> Result<bool, DispatchError> {
			ensure!(
				BribeRequests::<T>::contains_key(request.bribe_index),
				Error::<T>::InvalidIndex
			);
			let bribe_request = BribeRequests::<T>::get(request.bribe_index).unwrap();

			todo!("account for bribe progress");
			todo!("enact vote through pallet_democracy");
		}
	}
}
