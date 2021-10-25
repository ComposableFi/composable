//Bribe DAO

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	use composable_traits::{bribe::Bribe, democracy::Democracy};
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, InspectHold, MutateHold, Transfer},
	};
	use frame_system::pallet_prelude::*;
	use num_traits::{CheckedAdd, CheckedMul, CheckedSub, SaturatingSub};
	use pallet_democracy::Vote;
	use primitives::currency::CurrencyId;
	use sp_runtime::{
		scale_info::TypeInfo,
		traits::{AtLeast32BitUnsigned, Zero},
	};
	use sp_std::fmt::Debug;

	pub type BribeIndex = u32;

	pub type ReferendumIndex = pallet_democracy::ReferendumIndex;
	pub type CreateBribeRequest<T> = composable_traits::bribe::CreateBribeRequest<
		<T as frame_system::Config>::AccountId,
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

	// Status of Bribe request
	#[derive(Copy, Clone, Encode, Decode, PartialEq, RuntimeDebug)]
	pub enum BribeStatuses {
		Created,
		Started,
		OnHold,
		Failed,
		Finished,
		InvalidId,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
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

		// Currency config supporting transfer, freezing and inspect
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>
			+ InspectHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>;

		type Conviction: Parameter;

		type Democracy: Democracy<
			AccountId = Self::AccountId,
			ReferendumIndex = pallet_democracy::ReferendumIndex,
			Vote = pallet_democracy::Vote,
		>;

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

		//		#[transactional]
		//		#[pallet::weight(10_000)]
		//		pub fn deposit_funds(
		//			origin: OriginFor<T>,
		//			bribe: BribeIndex,
		//			amount: u128,
		//		) -> DispatchResult {
		//			transfer(account_id, origin, amount);
		//			todo!("deposit_tokens into vault ");

		//			todo!("transfer funds");
		//			todo!("Update token funds status");

		//			Ok(())
		//		}

		#[transactional]
		#[pallet::weight(10_000)]
		pub fn release_funds(
			origin: OriginFor<T>,
			bribe: BribeIndex,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// todo more validate logic
			T::Currency::release(CurrencyId::PICA, &who, amount, false);

			todo!("Check token supply, if supply is less or same as asked for: release funds");
			//			Error::<T>::EmptySupply;
			todo!("update capital status");
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn take_bribe(
			origin: OriginFor<T>,
			request: TakeBribeRequest<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let bribe_index = request.bribe_index;
			let bribe_taken = <Self as Bribe>::take_bribe(request.clone())?;
			let og_request = BribeRequests::<T>::get(request.bribe_index).unwrap(); // should be saved in the create bribe request, if its not then there is a logic error
																		// somewhere, so unwrap should be okey to use
			let amount = og_request.total_reward; // amount of tokens locked in
			let currencyid = og_request.asset_id;
			T::Currency::hold(currencyid, &who, amount); //Freeze assets
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
		EmptySupply,
	}

	// offchain indexing
	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn offchain_worker(_b: T::BlockNumber) {
			log::info!("Indexing request offchain");
		}
	}

	impl<T: Config> Bribe for Pallet<T> {
		type BribeIndex = BribeIndex;
		type AccountId = T::AccountId;
		type ReferendumIndex = ReferendumIndex;
		type Balance = T::Balance;
		type Conviction = T::Conviction;
		type CurrencyId = T::CurrencyId;

		//		fn lockup_funds(origin: Origin<T>, request: CreateBribeRequest<T>) -> Result<bool,
		// DispatchError>{ 			todo!("lock up users funds until vote is finished");
		//		}

		//		fn payout_funds()

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

			ensure!(!BribeRequests::<T>::contains_key(id), Error::<T>::AlreadyBribed); //dont duplicate briberequest if we already have it

			BribeRequests::<T>::insert(id, request);
			Ok(id)
		}

		fn do_take_bribe(request: TakeBribeRequest<T>) -> Result<bool, DispatchError> {
			ensure!(
				BribeRequests::<T>::contains_key(request.bribe_index),
				Error::<T>::InvalidIndex
			);
			let bribe_request = BribeRequests::<T>::get(request.bribe_index).unwrap();

			let vote = Vote { aye: bribe_request.is_aye, conviction: Default::default() }; //todo get conviction
			T::Democracy::vote(bribe_request.account_id, bribe_request.ref_index, vote); //AccountId, Referendum Index, Vote
			Ok(true)
			//			todo!("enact vote through pallet_democracy");
		}
	}
}
