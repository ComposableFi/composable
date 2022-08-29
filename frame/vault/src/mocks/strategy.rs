//! An example pallet showing how a `strategy` could be implemented as a secondary pallet. The
//! extrinsics show how to interact with the `vault` pallet.

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

	type BalanceOf<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::Balance;
	type VaultIdOf<T> = <<T as Config>::Vault as Vault>::VaultId;
	type ReportOf<T> = <<T as Config>::Vault as ReportableStrategicVault>::Report;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after the pallet reports it's current balance to the vault.
		Reported(ReportOf<T>),
		/// Emitted after the pallet mints new funds to mimic the generation of revenue.
		RevenueGenerated(BalanceOf<T>),
		/// Emitted after the pallet re-balances it's funds in accordance with the vault.
		Rebalanced(FundsAvailability<BalanceOf<T>>, BalanceOf<T>),
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Vault used to obtain funds from, report balances and return funds to.
		type Vault: ReportableStrategicVault<
			AccountId = Self::AccountId,
			AssetId = <<Self as Config>::Currency as Inspect<Self::AccountId>>::AssetId,
			Report = <<Self as Config>::Currency as Inspect<Self::AccountId>>::Balance,
			Balance = <<Self as Config>::Currency as Inspect<Self::AccountId>>::Balance,
		>;

		/// Currency implementation used by the pallet. Should be the same pallet as used by the
		/// vault.
		type Currency: Transfer<Self::AccountId> + Mutate<Self::AccountId>;

		/// The id used as the `AccountId` of the pallet. This should be unique across all pallets
		/// to avoid name collisions with other strategies.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mints new tokens and sends them to self, mocking the generating of revenue through DeFi.
		#[pallet::weight(10_000)]
		pub fn generate_revenue(
			origin: OriginFor<T>,
			vault: VaultIdOf<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let currency_id = T::Vault::asset_id(&vault)?;
			T::Currency::mint_into(currency_id, &Self::account_id(), amount)?;
			Self::deposit_event(Event::RevenueGenerated(amount));
			Ok(().into())
		}

		/// Reports the current balance to the vault.
		#[pallet::weight(10_000)]
		pub fn report(origin: OriginFor<T>, vault: VaultIdOf<T>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let currency_id = T::Vault::asset_id(&vault)?;
			let balance = T::Currency::balance(currency_id, &Self::account_id());
			T::Vault::update_strategy_report(&vault, &Self::account_id(), &balance)?;
			Self::deposit_event(Event::Reported(balance));
			Ok(().into())
		}

		/// Queries the vault for the current re-balance strategy and executes it.
		#[pallet::weight(10_000)]
		pub fn rebalance(origin: OriginFor<T>, vault: VaultIdOf<T>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let asset_id = T::Vault::asset_id(&vault)?;
			let task = T::Vault::available_funds(&vault, &Self::account_id())?;
			let action = match task {
				FundsAvailability::None => T::Currency::balance(asset_id, &Self::account_id()),
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
