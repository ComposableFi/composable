#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	pub use composable_traits::currency::CurrencyFactory;
	use core::ops::{Div, Mul};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::fungibles::MutateHold,
			Currency as NativeCurrency, EnsureOrigin,
			ExistenceRequirement::AllowDeath,
		},
		transactional, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use num_traits::SaturatingSub;
	use sp_runtime::traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub,
		SaturatedConversion, Zero,
	};
	use sp_std::fmt::Debug;
	pub use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type LiquidRewardId: Get<PalletId>;
		/// The currency mechanism.
		type NativeCurrency: NativeCurrency<Self::AccountId>;
		type JumpStart: EnsureOrigin<Self::Origin>;
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

		/// The weight information of this pallet.
		type WeightInfo: WeightInfo;
	}

	pub type CurrencyIdOf<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::AssetId;
	pub type BalanceOf<T> = <T as Config>::Balance;
	pub type NativeBalanceOf<T> =
		<<T as Config>::NativeCurrency as NativeCurrency<<T as SystemConfig>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn is_claimable)]
	pub type IsClaimable<T> = StorageValue<_, bool>;

	#[pallet::storage]
	#[pallet::getter(fn token_id)]
	pub type TokenId<T> = StorageValue<_, CurrencyIdOf<T>>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId", CurrencyIdOf<T> = "CurrencyId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Initiated(CurrencyIdOf<T>),
		Claimed(T::AccountId, u128),
	}

	#[pallet::error]
	pub enum Error<T> {
		CannotCreateAsset,
		AlreadyInitiated,
		FailedMint,
		NotClaimable,
		ConversionError,
		InsufficientTokens,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::weight(T::WeightInfo::initiate())]
		pub fn initiate(
			origin: OriginFor<T>,
			manager: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			T::JumpStart::ensure_origin(origin)?;
			ensure!(!<TokenId<T>>::exists(), Error::<T>::AlreadyInitiated);
			let lp_token_id = {
				T::CurrencyFactory::create().map_err(|e| {
					log::debug!("failed to create asset: {:?}", e);
					Error::<T>::CannotCreateAsset
				})?
			};
			T::Currency::mint_into(lp_token_id, &manager, amount)?;
			<TokenId<T>>::put(lp_token_id);
			Self::deposit_event(Event::Initiated(lp_token_id));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::make_claimable())]
		pub fn make_claimable(origin: OriginFor<T>) -> DispatchResult {
			T::JumpStart::ensure_origin(origin)?;
			<IsClaimable<T>>::put(true);
			Ok(().into())
		}

		#[transactional]
		#[pallet::weight(T::WeightInfo::claim())]
		pub fn claim(origin: OriginFor<T>, amount: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_claimable().unwrap_or(false), Error::<T>::NotClaimable);

			let token_id = <TokenId<T>>::get().ok_or(Error::<T>::NotClaimable)?;
			let token_supply = T::Currency::total_issuance(token_id);
			let pot_balance = T::NativeCurrency::free_balance(&Self::account_id());
			let token_supply_value: u128 = token_supply.saturated_into::<u128>();
			let pot_balance_value: u128 = pot_balance.saturated_into::<u128>();

			ensure!(pot_balance_value > 0, Error::<T>::ConversionError);
			ensure!(token_supply_value > 0, Error::<T>::ConversionError);

			let to_payout = pot_balance_value.mul(amount).div(token_supply_value);
			let amount_value: T::Balance = amount.saturated_into();
			let converted_payout: NativeBalanceOf<T> = to_payout.saturated_into();

			ensure!(converted_payout > 0u32.into(), Error::<T>::ConversionError);
			ensure!(amount_value > 0u32.into(), Error::<T>::ConversionError);

			T::Currency::burn_from(token_id, &who, amount_value)
				.map_err(|_| Error::<T>::InsufficientTokens)?;

			T::NativeCurrency::transfer(&Self::account_id(), &who, converted_payout, AllowDeath)?;
			Self::deposit_event(Event::Claimed(who, amount));
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
