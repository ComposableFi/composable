pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::traits::{FundsAvailability, ReportableStrategicVault, StrategicVault};
	use composable_traits::vault::Vault;
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		PalletId,
	};
	use frame_system::{ensure_root, pallet_prelude::OriginFor, Config as SystemConfig};
	use sp_runtime::traits::AccountIdConversion;

	pub const PALLET_ID: PalletId = PalletId(*b"mck_strt");

	type BalanceOf<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::Balance;

	type CurrencyIdFor<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::AssetId;

	type VaultIdOf<T> = <<T as Config>::Vault as Vault>::VaultId;
	type ReportOf<T> = <<T as Config>::Vault as ReportableStrategicVault>::Report;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Reported(ReportOf<T>),
		RevenueGenerated(BalanceOf<T>),
		Rebalanced(FundsAvailability<BalanceOf<T>>, BalanceOf<T>),
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Vault: ReportableStrategicVault<
			AccountId = Self::AccountId,
			AssetId = <<Self as Config>::Currency as Inspect<Self::AccountId>>::AssetId,
			Report = <<Self as Config>::Currency as Inspect<Self::AccountId>>::Balance,
			Balance = <<Self as Config>::Currency as Inspect<Self::AccountId>>::Balance,
		>;
		type Currency: Transfer<Self::AccountId> + Mutate<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		fn account_id() -> T::AccountId {
			PALLET_ID.into_account()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mints new tokens and sends them to self, mocking the generating of revenue through DeFi
		#[pallet::weight(10_000)]
		pub fn generate_revenue(
			origin: OriginFor<T>,
			vault: VaultIdOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			let currency_id = T::Vault::asset_id(&vault)?;
			T::Currency::mint_into(currency_id, &Self::account_id(), amount)?;
			Self::deposit_event(Event::RevenueGenerated(amount));
			Ok(().into())
		}

		/// Reports the current balance to the vault.
		#[pallet::weight(10_000)]
		pub fn report(origin: OriginFor<T>, vault: VaultIdOf<T>) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			let currency_id = T::Vault::asset_id(&vault)?;
			let balance = T::Currency::balance(currency_id, &Self::account_id());
			T::Vault::update_strategy_report(&vault, &Self::account_id(), &balance)?;
			Self::deposit_event(Event::Reported(balance));
			Ok(().into())
		}

		/// Queries the vault for the current rebalance strategy and executes it.
		#[pallet::weight(10_000)]
		pub fn rebalance(origin: OriginFor<T>, vault: VaultIdOf<T>) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			let asset_id = T::Vault::asset_id(&vault)?;
			let task = T::Vault::available_funds(&vault, &Self::account_id())?;
			let action = match task {
				FundsAvailability::MustLiquidate => {
					let balance = T::Currency::balance(asset_id, &Self::account_id());
					T::Currency::transfer(
						asset_id,
						&Self::account_id(),
						&T::Vault::account_id(&vault),
						balance,
						true,
					)?;
					balance
				},
				FundsAvailability::Withdrawable(balance) => {
					T::Currency::transfer(
						asset_id,
						&T::Vault::account_id(&vault),
						&Self::account_id(),
						balance,
						true,
					)?;
					balance
				},
				FundsAvailability::Depositable(balance) => {
					T::Currency::transfer(
						asset_id,
						&Self::account_id(),
						&T::Vault::account_id(&vault),
						balance,
						true,
					)?;
					balance
				},
			};
			Self::deposit_event(Event::Rebalanced(task, action));
			Ok(().into())
		}
	}
}
