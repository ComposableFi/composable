//Bribe DAO

pub use pallet::*;

pub mod sortedvec;
pub mod tests;
pub use crate::sortedvec::FastMap;

#[frame_support::pallet]
pub mod pallet {
	use crate::sortedvec::{BribesStorage, FastMap};
	use codec::Codec;
	use composable_traits::{bribe::Bribe, democracy::Democracy};
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{InspectHold, MutateHold, Transfer},
	};
	use frame_system::pallet_prelude::*;
	use num_traits::{CheckedAdd, CheckedMul, CheckedSub, SaturatingSub};
	use pallet_democracy::{Vote, VotingOf};
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, Zero},
		SaturatedConversion,
	};
	use sp_std::fmt::Debug;

	pub type BribeIndex = u32;
	pub type ReferendumIndex = pallet_democracy::ReferendumIndex;

	// User asks to buy X amount of votes for a certain amount | Briber
	pub type CreateBribeRequest<T> = composable_traits::bribe::CreateBribeRequest<
		<T as frame_system::Config>::AccountId,
		ReferendumIndex,
		<T as Config>::Balance,
		<T as Config>::Conviction,
		<T as Config>::CurrencyId,
	>;

	// Bribe'e, the user selling its vote for tokens
	pub type TakeBribeRequest<T> = composable_traits::bribe::TakeBribeRequest<
		BribeIndex,
		<T as Config>::Balance,
		<T as Config>::Conviction,
	>;

	pub type DeleteBribeRequest = composable_traits::bribe::DeleteBribeRequest<BribeIndex>;

	//	pub type VotingOf<T> = pallet_democracy::VotingOf<T>;

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

		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;

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
		DeleteBribe { id: BribeIndex, request: DeleteBribeRequest },
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
	#[pallet::getter(fn fast_vexc)]
	pub(super) type Fastvec<T: Config> = StorageValue<_, FastMap, ValueQuery>; // using value query instead of OptionQuery cuz OptionsQuery returns null if its empty

	#[pallet::storage]
	#[pallet::getter(fn bribe_requests)]
	pub(super) type BribeRequests<T: Config> =
		StorageMap<_, Blake2_128Concat, BribeIndex, CreateBribeRequest<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Buy votes request
		#[pallet::weight(10_000)]
		pub fn create_bribe(
			origin: OriginFor<T>,
			request: CreateBribeRequest<T>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			// Freeze/hold the users funds, to verify payment
			let holdreq = request.clone();
			T::Currency::hold(holdreq.asset_id, &from, holdreq.total_reward)
				.map_err(|_| Error::<T>::CantFreezeFunds)?; //Freeze assets

			let id = <Self as Bribe>::create_bribe(request.clone())?;
			Self::deposit_event(Event::BribeCreated { id, request });
			Ok(().into())
		}

		/// Sell Votes request
		#[pallet::weight(10_000)]
		pub fn take_bribe(
			origin: OriginFor<T>,
			request: TakeBribeRequest<T>,
		) -> DispatchResultWithPostInfo {
			let _from = ensure_signed(origin)?;
			let amount_of_votes = T::Democracy::count_votes(_from.into()).unwrap();
			//			let test = VotingOf::<T>::get(origin);
			let bribe_index = request.bribe_index;
			let bribe_taken = <Self as Bribe>::take_bribe(request.clone())?;
			if bribe_taken {
				Self::deposit_event(Event::BribeTaken { id: bribe_index, request });
			}
			Ok(().into())
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidBribe,
		InvalidIndex,
		NotEnoughFunds,
		NotEnoughStake,
		PriceNotRequested,
		AlreadyBribed,
		EmptySupply,
		CantFreezeFunds,
		ReleaseFailed,
		BribeDeletionFailed,
		InvalidVote,
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

		/// Register new bribe request
		fn create_bribe(request: CreateBribeRequest<T>) -> Result<Self::BribeIndex, DispatchError> {
			Self::do_create_bribe(request)
		}

		/// Register the votes a user wants to sell
		fn take_bribe(request: TakeBribeRequest<T>) -> Result<bool, DispatchError> {
			Self::do_take_bribe(request)
		}

		/// Delete a finished Bribe Request
		fn delete_bribe(request: DeleteBribeRequest) -> Result<bool, DispatchError> {
			Self::do_delete_bribe(request)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Create a new Bribe Request
		fn do_create_bribe(request: CreateBribeRequest<T>) -> Result<BribeIndex, DispatchError> {
			let id = BribeCount::<T>::mutate(|id| {
				*id += 1;
				*id
			});

			BribeRequests::<T>::insert(id, request);
			Ok(id)
		}

		/// Find votes for a bribe request
		fn do_match_votes(bribe_index: BribeIndex) -> Result<bool, DispatchError> {
			let bribe_request =
				BribeRequests::<T>::try_get(bribe_index).map_err(|_| Error::<T>::InvalidIndex)?;

			let ref_index = bribe_request.ref_index;
			// Yield all the bribe votes for sale with the same ref index

			let loot: Vec<BribesStorage> = Fastvec::<T>::get().find_all_pid(ref_index);

			if !loot.is_empty() {
				let mut spendamount: u32 = 0;

				for bribes in loot {
					// Cast Vote

					let vote = Vote { aye: bribe_request.is_aye, conviction: Default::default() }; //todo

					let ss = bribe_request.clone();
					T::Democracy::vote(ss.account_id, bribes.p_id, vote); //AccountId,

					// Remove from storage
					Fastvec::<T>::mutate(|a| {
						a.remove_bribe(bribes.amount, bribes.p_id, bribes.votes)
					});

					// append the amount to our spend amount tracking
					spendamount += bribes.amount;

					// Pay out to the seller of the vote

					let tmp_value = bribe_request.clone();
					let currencyid = tmp_value.asset_id;
					T::Currency::release(
						currencyid,
						&tmp_value.account_id,
						bribes.amount.into(),
						false,
					)
					.map_err(|_| Error::<T>::ReleaseFailed)?;
					// todo: Check if all votes are fullfilled
					let bribe_balance: u32 = bribe_request.total_reward.saturated_into::<u32>();
					//					if we have spent all the money we have for votes, we assume the order is
					// fullfilled and can not interact anymore so we remove it
					if spendamount >= bribe_balance {
						//todo also check if the correct amount of votes has been fullfilled
						// Delete The bribe if fullfilled
						let dr: DeleteBribeRequest = DeleteBribeRequest { bribe_index };
						Self::do_delete_bribe(dr).map_err(|_| Error::<T>::BribeDeletionFailed)?;
					}
				}
			}
			Ok(true)
		}

		/// Take Bribe user sell votes request   
		fn do_take_bribe(request: TakeBribeRequest<T>) -> Result<bool, DispatchError> {
			// todo: make sure the user is not selling the same vote twice
			let bribe_request = BribeRequests::<T>::try_get(request.bribe_index)
				.map_err(|_| Error::<T>::InvalidIndex)?;

			let pid = bribe_request.ref_index; // save based on the referendumIndex
			let amount_votes: u32 = 3; //change me
			let amount: u32 = bribe_request.total_reward.saturated_into::<u32>(); // amount of tokens locked in
																	  // insert into fastvec
			Fastvec::<T>::mutate(|a| a.add(amount, pid, amount_votes));
			//Check if we can sell the votes now
			Self::do_match_votes(request.bribe_index).map_err(|_| Error::<T>::InvalidVote)?;

			Ok(true)
		}

		/// Delete Bribe Request
		/// Check the bribe id, delete from BribeRequests and from FastMap
		fn do_delete_bribe(request: DeleteBribeRequest) -> Result<bool, DispatchError> {
			let bribe_id = request.bribe_index;
			// Check if the bribe request id exists
			ensure!(!BribeRequests::<T>::contains_key(bribe_id), Error::<T>::InvalidBribe);

			// Remove from BribeRequests Storage Map
			BribeRequests::<T>::remove(bribe_id);
			// Emit the event
			Self::deposit_event(Event::DeleteBribe { id: bribe_id, request });
			Ok(true)
		}
	}
}
