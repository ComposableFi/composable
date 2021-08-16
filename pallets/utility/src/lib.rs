#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::PostDispatchInfo, //DispatchInfo,
    traits::{IsSubType, OriginTrait, UnfilteredDispatchable},
    transactional,
    weights::{extract_actual_weight, GetDispatchInfo},
};
use sp_core::TypeId;
use sp_io::hashing::blake2_256;
use sp_runtime::traits::Dispatchable;
use sp_std::prelude::*;

pub mod weights;
pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event> + IsType<<Self as frame_system::Config>::Event>;

        type Call: Parameter
            + Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>
            + UnfilteredDispatchable<Origin = Self::Origin>
            + IsSubType<Call<Self>>
            + IsType<<Self as frame_system::Config>::Call>;

        type WeightInfo: WeightInfo;
    }

    /*
        pub fn get_dispatch_class(call: <T::Config>::Call) -> DispatchClass {
                let dispatch_info = call.get_dispatch_info();
                (
                    T::WeightInfo::as_derivative()
                        .saturating_add(dispatch_info.weight)
                        .saturating_add(T::DbWeight::get().reads_writes(1, 1)),
                    dispatch_info.class,
                )

        }

    */

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event {
        BatchInterrupted(u32, DispatchError),
        BatchCompleted,
        ItemCompleted,
    }

// Custom Error classes
	#[pallet::error]
	pub enum Error<T> {
	NotSigned,
	Unknown,
	}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        //		pub fn bulk_balance()
        #[pallet::weight({
			let dispatch_infos = calls.iter().map(|call| call.get_dispatch_info()).collect::<Vec<_>>();
			let dispatch_weight = dispatch_infos.iter()
				.map(|di| di.weight)
				.fold(0, |total: Weight, weight: Weight| total.saturating_add(weight))
				.saturating_add(T::WeightInfo::batch(calls.len() as u32));
			let dispatch_class = {
				let all_operational = dispatch_infos.iter()
					.map(|di| di.class)
					.all(|class| class == DispatchClass::Operational);
				if all_operational {
					DispatchClass::Operational
				} else {
					DispatchClass::Normal
				}
			};
			(dispatch_weight, dispatch_class)
		})]
        pub fn batch(
            origin: OriginFor<T>,
            calls: Vec<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
            let is_root = ensure_root(origin.clone()).is_ok();
            let calls_len = calls.len();
            let mut weight: Weight = 0;
            for (index, call) in calls.into_iter().enumerate() {
                let info = call.get_dispatch_info();
                let result = if is_root {
                    call.dispatch_bypass_filter(origin.clone())
                } else {
                    call.dispatch(origin.clone())
                };
                weight = weight.saturating_add(extract_actual_weight(&result, &info));
                if let Err(e) = result {
                    Self::deposit_event(Event::BatchInterrupted(index as u32, e.error));
                    let base_weight = T::WeightInfo::batch(index.saturating_add(1) as u32);
                    return Ok(Some(base_weight + weight).into());
                }
                Self::deposit_event(Event::ItemCompleted);
            }
            Self::deposit_event(Event::BatchCompleted);
            let base_weight = T::WeightInfo::batch(calls_len as u32);
            Ok(Some(base_weight + weight).into())
        }

        /*
                #[pallet::weight({
                    let dispatch_infos = calls.iter().map(|call| call.get_dispatch_info()).collect::<Vec<_>>();
                    let dispatch_weight = dispatch_infos.iter()
                        .map(|di| di.weight)
                        .fold(0, |total: Weight, weight: Weight| total.saturating_add(weight))
                        .saturating_add(T::WeightInfo::batch(calls.len() as u32));
                    let dispatch_class = {
                        let all_operational = dispatch_infos.iter()
                            .map(|di| di.class)
                            .all(|class| class == DispatchClass::Operational);
                        if all_operational {
                            DispatchClass::Operational
                        } else {
                            DispatchClass::Normal
                        }
                    };
                    (dispatch_weight, dispatch_class)
                })]
                pub fn batch_2(origin: OriginFor<T>, calls: Vec<<T as Config>::Call>) -> DispatchResultsWithPostInfo {
                    let mut weight: Weight = 0; // append weights based on tx data

        // if not okey throw a batch interupt

                    Self::deposit_event(Event::BatchCompleted); // mark Batch as done
                    let b_weight = T::WeightInfo::batch(calls_len as u32);
                    Ok(Some(weight + base_weight).into())
                    }
        */
        #[pallet::weight(10_000)]
        ///// Get Batch info(weight) without sending anything
        pub fn batch_info(
            origin: OriginFor<T>,
            calls: Vec<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
	    let checksig = ensure_signed(origin); 
	    ensure!(checksig.is_ok(), Error::<T>::NotSigned); //check if signed origin is in the request
            let call_len = calls.len();
            let mut weight: Weight = 0;
            for (_, call) in calls.into_iter().enumerate() {
                weight = weight.saturating_add(call.get_dispatch_info().weight);
            }
            let base_weight = T::WeightInfo::batch(call_len as u32);
            Ok(Some(weight + base_weight).into())
        }

        //		/// Check batch status
        //		#[pallet::weight](10_000)
        //		pub fn check_batch_status(origin: OriginFor<T>, call: Box<<T as Config>::Call>) -> {

        //}

        //		#[pallet::weight](10_000) // todo change
        //		/// Get basic batch tx info
        //		pub fn batch_info(origin: OriginFor<T>, call: Box<<T as Config>::Call>) -> DispatchInfo {
        //			let org = ensure_signed(org);// Check if signed
        //			let dispatch_info = call.get_dispatch_info();

        //}
        //
        #[pallet::weight({
//			get_dispatch_class(call)
			let dispatch_info = call.get_dispatch_info();
			(
				T::WeightInfo::as_derivative()
					.saturating_add(dispatch_info.weight)
					.saturating_add(T::DbWeight::get().reads_writes(1, 1)),
				dispatch_info.class,
			)
		})]
        pub fn as_derivative(
            origin: OriginFor<T>,
            index: u16,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
            let mut origin = origin;
            let who = ensure_signed(origin.clone())?;
            let pseudonym = Self::derivative_account_id(who, index);
            origin.set_caller_from(frame_system::RawOrigin::Signed(pseudonym));
            let info = call.get_dispatch_info();
            let result = call.dispatch(origin);
            let mut weight = T::WeightInfo::as_derivative()
                .saturating_add(T::DbWeight::get().reads_writes(1, 1));
            weight = weight.saturating_add(extract_actual_weight(&result, &info));
            result
                .map_err(|mut err| {
                    err.post_info = Some(weight).into();
                    err
                })
                .map(|_| Some(weight).into())
        }

        #[pallet::weight({
			let dispatch_infos = calls.iter().map(|call| call.get_dispatch_info()).collect::<Vec<_>>();
			let dispatch_weight = dispatch_infos.iter()
				.map(|di| di.weight)
				.fold(0, |total: Weight, weight: Weight| total.saturating_add(weight))
				.saturating_add(T::WeightInfo::batch_all(calls.len() as u32));
			let dispatch_class = {
				let all_operational = dispatch_infos.iter()
					.map(|di| di.class)
					.all(|class| class == DispatchClass::Operational);
				if all_operational {
					DispatchClass::Operational
				} else {
					DispatchClass::Normal
				}
			};
			(dispatch_weight, dispatch_class)
		})]
        #[transactional]
        pub fn batch_all(
            origin: OriginFor<T>,
            calls: Vec<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
            let is_root = ensure_root(origin.clone()).is_ok();
            let calls_len = calls.len();
            // Track the actual weight of each of the batch calls.
            let mut weight: Weight = 0;
            for (index, call) in calls.into_iter().enumerate() {
                let info = call.get_dispatch_info();
                // If origin is root, bypass any dispatch filter; root can call anything.
                let result = if is_root {
                    call.dispatch_bypass_filter(origin.clone())
                } else {
                    let mut filtered_origin = origin.clone();
                    // Don't allow users to nest `batch_all` calls.
                    filtered_origin.add_filter(move |c: &<T as frame_system::Config>::Call| {
                        let c = <T as Config>::Call::from_ref(c);
                        !matches!(c.is_sub_type(), Some(Call::batch_all(_)))
                    });
                    call.dispatch(filtered_origin)
                };
                weight = weight.saturating_add(extract_actual_weight(&result, &info));
                result.map_err(|mut err| {
                    let base_weight = T::WeightInfo::batch_all(index.saturating_add(1) as u32);
                    err.post_info = Some(base_weight + weight).into();
                    err
                })?;
                Self::deposit_event(Event::ItemCompleted);
            }
            Self::deposit_event(Event::BatchCompleted);
            let base_weight = T::WeightInfo::batch_all(calls_len as u32);
            Ok(Some(base_weight + weight).into())
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode)]
struct IndexedUtilityPalletId(u16);

impl TypeId for IndexedUtilityPalletId {
    const TYPE_ID: [u8; 4] = *b"suba";
}

impl<T: Config> Pallet<T> {
    pub fn derivative_account_id(who: T::AccountId, index: u16) -> T::AccountId {
        let entropy = (b"modlpy/utilisuba", who, index).using_encoded(blake2_256);
        T::AccountId::decode(&mut &entropy[..]).unwrap_or_default()
    }

    /*
    // Check if multiaccount
        pub fn is_multiaccount(origin: OriginFor<T>) -> T::AccountId {
            let who = ensure_signed(origin);
            let multi_account = <MultiAccounts<T>>::get(&who).ok_or(Error::<T>::MultiAccountNotFound)?;
            Ok(())
            }
    */
}
