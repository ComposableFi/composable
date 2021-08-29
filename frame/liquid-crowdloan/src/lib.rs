#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, PalletId, transactional,
		traits::{
			fungibles::{Inspect, Transfer, Mutate},
			tokens::{fungibles::MutateHold},
		},
	};
	pub use composable_traits::{
		currency::CurrencyFactory,
	};
	use codec::{Codec, FullCodec};
	use frame_system::{ensure_root, ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use sp_std::fmt::Debug;
	use num_traits::SaturatingSub;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub,
			Zero,
		},
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type LiquidRewardId: Get<PalletId>;
		type CurrencyFactory: CurrencyFactory<Self::CurrencyId>;
		type CurrencyId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default;

		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::CurrencyId>;

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
			+ Zero
			+ From<u64>;

	}

	pub type CurrencyIdOf<T> =
	<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::AssetId;
	pub type BalanceOf<T> = <T as Config>::Balance;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn is_claimable)]
	pub type IsClaimable<T> = StorageValue<_, bool>;

	#[pallet::storage]
	#[pallet::getter(fn token_id)]
	pub type TokenId<T> = StorageValue<_, CurrencyIdOf<T>>;

	#[pallet::error]
	pub enum Error<T> {
		CannotCreateAsset,
		AlreadyInitiated,
		FailedMint,
		NotClaimable
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[transactional]
		#[pallet::weight(10_000)]
		pub fn initiate(origin: OriginFor<T>, amount: T::Balance, manager: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!<TokenId<T>>::exists(), Error::<T>::AlreadyInitiated);
			let lp_token_id = {
				T::CurrencyFactory::create().map_err(|e| {
					log::debug!("failed to create asset: {:?}", e);
					Error::<T>::CannotCreateAsset
				})?
			};
			T::Currency::mint_into(lp_token_id, &manager, amount)?;
			<TokenId<T>>::put(lp_token_id);
			//TODO emit event with token id
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn make_claimable(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			<IsClaimable<T>>::put(true);
			Ok(().into())

		}

		#[pallet::weight(10_000)]
		pub fn claim(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			ensure!(Self::is_claimable().unwrap_or(false), Error::<T>::NotClaimable);
			// TODO finish this function by burning LP token and applying proper formula to withdraw
			Ok(().into())
		}


	}

	impl<T: Config> Pallet<T> {
		/// Get a unique, inaccessible account id from the `LiquidRewardId`.
		pub fn account_id() -> T::AccountId {
			T::LiquidRewardId::get().into_account()
		}
	}
}
