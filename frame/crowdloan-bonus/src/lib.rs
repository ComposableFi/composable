#![cfg_attr(not(test), warn(clippy::disallowed_method, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[allow(clippy::unnecessary_cast)]
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::Codec;
	use composable_traits::math::SafeArithmetic;
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
	use primitives::currency::CurrencyId;
	use sp_runtime::traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub,
		SaturatedConversion, Zero,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config + sudo::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Pallet ID, used for the pot of funds
		type LiquidRewardId: Get<PalletId>;
		/// The currency mechanism.
		type NativeCurrency: NativeCurrency<Self::AccountId>;
		/// Origin that controls this pallet
		type JumpStart: EnsureOrigin<Self::Origin>;
		/// Currency Id for this pallet
		type CurrencyId: Get<CurrencyId>;
		/// Total number of tokens to mint initially.
		type TokenTotal: Get<Self::Balance>;
		/// Multicurrency implementation
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = CurrencyId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = CurrencyId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = CurrencyId>;
		/// Balance type
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

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Initiated(CurrencyIdOf<T>),
		Claimed(T::AccountId, u128),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Pallet has already been initiated.
		AlreadyInitiated,
		/// Claiming has not yet been enabled.
		NotClaimable,
		/// Crowdloan Bonus pot is empty.
		EmptyPot,
		/// User has insufficent tokens to claim crowdloan bonus.
		InsufficientTokens,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			if let Err(err) = Self::initialize() {
				log::error!("failed to initialize: {:?}", err)
			}
			0
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::make_claimable())]
		pub fn make_claimable(origin: OriginFor<T>) -> DispatchResult {
			T::JumpStart::ensure_origin(origin)?;
			<IsClaimable<T>>::put(true);
			Ok(())
		}

		/// Attempts to claim some crowdloan bonus from the crowdloan pot.
		/// No-op if amount is zero.
		#[transactional]
		#[pallet::weight(T::WeightInfo::claim())]
		pub fn claim(origin: OriginFor<T>, amount: u128) -> DispatchResult {
			if amount.is_zero() {
				return Ok(())
			}
			let who = ensure_signed(origin)?;
			ensure!(Self::is_claimable().unwrap_or(false), Error::<T>::NotClaimable);

			let token_id = T::CurrencyId::get();
			let token_supply = T::Currency::total_issuance(token_id);
			let pot_balance = T::NativeCurrency::free_balance(&Self::account_id());
			let token_supply_value: u128 = token_supply.saturated_into();
			let pot_balance_value: u128 = pot_balance.saturated_into();

			ensure!(pot_balance_value > 0 && token_supply_value > 0, Error::<T>::EmptyPot);

			let to_payout = pot_balance_value.safe_mul(&amount)?.safe_div(&token_supply_value)?;

			let amount_value: T::Balance = amount.saturated_into();
			let converted_payout: NativeBalanceOf<T> = to_payout.saturated_into();

			T::Currency::burn_from(token_id, &who, amount_value)
				.map_err(|_| Error::<T>::InsufficientTokens)?;

			T::NativeCurrency::transfer(&Self::account_id(), &who, converted_payout, AllowDeath)?;
			Self::deposit_event(Event::Claimed(who, amount));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// initialize the pallet
		pub fn initialize() -> Result<(), sp_runtime::DispatchError> {
			// at genesis `IsClaimable` is `None`
			if Self::is_claimable().is_none() {
				let token_id = T::CurrencyId::get();
				let manager = <sudo::Pallet<T>>::key();
				// not really sure why this would fail, but keep trying to mint?
				T::Currency::mint_into(token_id, &manager, T::TokenTotal::get())?;
				<IsClaimable<T>>::put(false);
				Self::deposit_event(Event::Initiated(token_id));
			}

			Ok(())
		}
		/// Get a unique, inaccessible account id from the `LiquidRewardId`.
		pub fn account_id() -> T::AccountId {
			T::LiquidRewardId::get().into_account()
		}
	}
}
