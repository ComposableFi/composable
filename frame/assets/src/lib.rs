#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod orml;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use composable_traits::currency::{AssetId, Balance, CurrencyFactory};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		sp_runtime::traits::StaticLookup,
		traits::{
			fungible::{
				Inspect as NativeInspect, Mutate as NativeMutate, Transfer as NativeTransfer,
			},
			fungibles::{Inspect, Mutate, Transfer},
		},
	};
	use frame_system::{ensure_root, ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::DispatchError;

	use crate::weights::WeightInfo;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type AssetId: AssetId;
		type Balance: Balance;

		type NativeAssetId: Get<Self::AssetId>;
		type GenerateCurrencyId: CurrencyFactory<Self::AssetId>;
		type Currency;
		type MultiCurrency;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn markets)]
	pub type PalletAccounts<T: Config> =
		StorageMap<_, Twox64Concat, T::AssetId, T::AccountId, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		BadOrigin,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		<T as Config>::Currency: NativeTransfer<T::AccountId, Balance = T::Balance>
			+ NativeInspect<T::AccountId, Balance = T::Balance>
			+ NativeMutate<T::AccountId, Balance = T::Balance>,
		<T as Config>::MultiCurrency: Inspect<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>
			+ Transfer<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>
			+ Mutate<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
	{
		/// Transfer `amount` of `asset` from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			asset: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let src = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as Transfer<T::AccountId>>::transfer(asset, &src, &dest, amount, keep_alive)?;
			Ok(().into())
		}

		/// Transfer `amount` of the native asset from `origin` to `dest`. This is slightly
		/// cheaper to call, as it avoids an asset lookup.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_native())]
		pub fn transfer_native(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			let src = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as NativeTransfer<T::AccountId>>::transfer(&src, &dest, value, keep_alive)?;
			Ok(().into())
		}

		/// Transfer `amount` of the `asset` from `origin` to `dest`. This requires root.
		///
		/// # Errors
		///  - When `origin` is not root.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::force_transfer())]
		pub fn force_transfer(
			origin: OriginFor<T>,
			asset: T::AssetId,
			source: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let source = T::Lookup::lookup(source)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as Transfer<_>>::transfer(asset, &source, &dest, value, keep_alive)?;
			Ok(().into())
		}

		/// Transfer `amount` of the the native asset from `origin` to `dest`. This requires root.
		///
		/// # Errors
		///  - When `origin` is not root.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::force_transfer_native())]
		pub fn force_transfer_native(
			origin: OriginFor<T>,
			source: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let source = T::Lookup::lookup(source)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as NativeTransfer<_>>::transfer(&source, &dest, value, keep_alive)?;
			Ok(().into())
		}

		/// Transfer all free balance of the `asset` from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_all())]
		pub fn transfer_all(
			origin: OriginFor<T>,
			asset: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			keep_alive: bool,
		) -> DispatchResult {
			let transactor = ensure_signed(origin)?;
			let reducible_balance =
				<Self as Inspect<T::AccountId>>::reducible_balance(asset, &transactor, keep_alive);
			let dest = T::Lookup::lookup(dest)?;
			<Self as Transfer<T::AccountId>>::transfer(
				asset,
				&transactor,
				&dest,
				reducible_balance,
				keep_alive,
			)?;
			Ok(())
		}

		/// Transfer all free balance of the native asset from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_all_native())]
		pub fn transfer_all_native(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			keep_alive: bool,
		) -> DispatchResult {
			let transactor = ensure_signed(origin)?;
			let reducible_balance =
				<Self as NativeInspect<T::AccountId>>::reducible_balance(&transactor, keep_alive);
			let dest = T::Lookup::lookup(dest)?;
			<Self as NativeTransfer<T::AccountId>>::transfer(
				&transactor,
				&dest,
				reducible_balance,
				keep_alive,
			)?;
			Ok(())
		}

		/// Creates a new asset, minting `amount` of funds into the `dest` account. Intented to be
		/// used for creating wrapped assets, not associated with any project.
		#[pallet::weight(T::WeightInfo::mint_initialize())]
		pub fn mint_initialize(
			origin: OriginFor<T>,
			amount: T::Balance,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let id = T::GenerateCurrencyId::create()?;
			<Self as Mutate<T::AccountId>>::mint_into(id, &dest, amount)?;
			Ok(().into())
		}

		/// Creates a new asset, minting `amount` of funds into the `dest` account. The `dest`
		/// account can use the democracy pallet to mint further assets, or if the governance_origin
		/// is set to an owned account, using signed transactions. In general the
		/// `governance_origin` should be generated from the pallet id.
		#[pallet::weight(T::WeightInfo::mint_initialize())]
		pub fn mint_initialize_with_governance(
			origin: OriginFor<T>,
			amount: T::Balance,
			governance_origin: T::AccountId,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			let id = T::GenerateCurrencyId::create()?;
			PalletAccounts::<T>::insert(id, governance_origin);
			<Self as Mutate<T::AccountId>>::mint_into(id, &dest, amount)?;
			Ok(().into())
		}

		/// Mints `amount` of `asset_id` into the `dest` account.
		#[pallet::weight(T::WeightInfo::mint_into())]
		pub fn mint_into(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: T::AccountId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			ensure_root_or_governance::<T>(origin, asset_id)?;
			<Self as Mutate<T::AccountId>>::mint_into(asset_id, &dest, amount)?;
			Ok(().into())
		}

		/// Mints `amount` of `asset_id` into the `dest` account.
		#[pallet::weight(T::WeightInfo::burn_from())]
		pub fn burn_from(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: T::AccountId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			ensure_root_or_governance::<T>(origin, asset_id)?;
			<Self as Mutate<T::AccountId>>::burn_from(asset_id, &dest, amount)?;
			Ok(().into())
		}
	}

	fn ensure_root_or_governance<T: Config>(
		origin: OriginFor<T>,
		asset_id: T::AssetId,
	) -> Result<Option<T::AccountId>, DispatchError> {
		let account = match origin.into() {
			Ok(frame_system::RawOrigin::Signed(account)) => {
				if PalletAccounts::<T>::try_get(asset_id) == Ok(account) {
					Some(account)
				} else {
					return Err(Error::<T>::BadOrigin.into())
				}
			},
			Ok(frame_system::RawOrigin::Root) => None,
			_ => return Err(Error::<T>::BadOrigin.into()),
		};
		Ok(account)
	}

	mod currency {
		use super::*;
		use frame_support::traits::{
			BalanceStatus, Currency, ExistenceRequirement, ReservableCurrency, SignedImbalance,
			WithdrawReasons,
		};

		impl<T: Config> ReservableCurrency<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: Currency<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: ReservableCurrency<T::AccountId, Balance = T::Balance>,
		{
			fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
				<<T as Config>::Currency>::can_reserve(who, value)
			}

			fn slash_reserved(
				who: &T::AccountId,
				value: Self::Balance,
			) -> (Self::NegativeImbalance, Self::Balance) {
				<<T as Config>::Currency>::slash_reserved(who, value)
			}

			fn reserved_balance(who: &T::AccountId) -> Self::Balance {
				<<T as Config>::Currency>::reserved_balance(who)
			}

			fn reserve(who: &T::AccountId, value: Self::Balance) -> DispatchResult {
				<<T as Config>::Currency>::reserve(who, value)
			}

			fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
				<<T as Config>::Currency>::unreserve(who, value)
			}

			fn repatriate_reserved(
				slashed: &T::AccountId,
				beneficiary: &T::AccountId,
				value: Self::Balance,
				status: BalanceStatus,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::repatriate_reserved(slashed, beneficiary, value, status)
			}
		}

		impl<T: Config> Currency<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: Currency<T::AccountId, Balance = T::Balance>,
		{
			type Balance = <<T as Config>::Currency as Currency<T::AccountId>>::Balance;
			type PositiveImbalance =
				<<T as Config>::Currency as Currency<T::AccountId>>::PositiveImbalance;
			type NegativeImbalance =
				<<T as Config>::Currency as Currency<T::AccountId>>::NegativeImbalance;

			fn total_balance(who: &T::AccountId) -> Self::Balance {
				<<T as Config>::Currency>::total_balance(who)
			}

			fn can_slash(who: &T::AccountId, value: Self::Balance) -> bool {
				<<T as Config>::Currency>::can_slash(who, value)
			}

			fn total_issuance() -> Self::Balance {
				<<T as Config>::Currency>::total_issuance()
			}

			fn minimum_balance() -> Self::Balance {
				<<T as Config>::Currency>::minimum_balance()
			}

			fn burn(amount: Self::Balance) -> Self::PositiveImbalance {
				<<T as Config>::Currency>::burn(amount)
			}

			fn issue(amount: Self::Balance) -> Self::NegativeImbalance {
				<<T as Config>::Currency>::issue(amount)
			}

			fn pair(amount: Self::Balance) -> (Self::PositiveImbalance, Self::NegativeImbalance) {
				<<T as Config>::Currency>::pair(amount)
			}

			fn free_balance(who: &T::AccountId) -> Self::Balance {
				<<T as Config>::Currency>::free_balance(who)
			}

			fn ensure_can_withdraw(
				who: &T::AccountId,
				amount: Self::Balance,
				reasons: WithdrawReasons,
				new_balance: Self::Balance,
			) -> DispatchResult {
				<<T as Config>::Currency>::ensure_can_withdraw(who, amount, reasons, new_balance)
			}

			fn transfer(
				source: &T::AccountId,
				dest: &T::AccountId,
				value: Self::Balance,
				existence_requirement: ExistenceRequirement,
			) -> DispatchResult {
				<<T as Config>::Currency>::transfer(source, dest, value, existence_requirement)
			}

			fn slash(
				who: &T::AccountId,
				value: Self::Balance,
			) -> (Self::NegativeImbalance, Self::Balance) {
				<<T as Config>::Currency>::slash(who, value)
			}

			fn deposit_into_existing(
				who: &T::AccountId,
				value: Self::Balance,
			) -> Result<Self::PositiveImbalance, DispatchError> {
				<<T as Config>::Currency>::deposit_into_existing(who, value)
			}

			fn resolve_into_existing(
				who: &T::AccountId,
				value: Self::NegativeImbalance,
			) -> Result<(), Self::NegativeImbalance> {
				<<T as Config>::Currency>::resolve_into_existing(who, value)
			}

			fn deposit_creating(
				who: &T::AccountId,
				value: Self::Balance,
			) -> Self::PositiveImbalance {
				<<T as Config>::Currency>::deposit_creating(who, value)
			}

			fn resolve_creating(who: &T::AccountId, value: Self::NegativeImbalance) {
				<<T as Config>::Currency>::resolve_creating(who, value)
			}

			fn withdraw(
				who: &T::AccountId,
				value: Self::Balance,
				reasons: WithdrawReasons,
				liveness: ExistenceRequirement,
			) -> Result<Self::NegativeImbalance, DispatchError> {
				<<T as Config>::Currency>::withdraw(who, value, reasons, liveness)
			}

			fn settle(
				who: &T::AccountId,
				value: Self::PositiveImbalance,
				reasons: WithdrawReasons,
				liveness: ExistenceRequirement,
			) -> Result<(), Self::PositiveImbalance> {
				<<T as Config>::Currency>::settle(who, value, reasons, liveness)
			}

			fn make_free_balance_be(
				who: &T::AccountId,
				balance: Self::Balance,
			) -> SignedImbalance<Self::Balance, Self::PositiveImbalance> {
				<<T as Config>::Currency>::make_free_balance_be(who, balance)
			}
		}
	}

	mod fungible {
		use super::*;

		use frame_support::traits::tokens::{
			fungible::{Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
			DepositConsequence, WithdrawConsequence,
		};

		impl<T: Config> MutateHold<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: InspectHold<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: Transfer<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: MutateHold<T::AccountId, Balance = T::Balance>,
		{
			fn hold(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
				<<T as Config>::Currency>::hold(who, amount)
			}

			fn release(
				who: &T::AccountId,
				amount: Self::Balance,
				best_effort: bool,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::release(who, amount, best_effort)
			}

			fn transfer_held(
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
				best_effort: bool,
				on_held: bool,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::transfer_held(source, dest, amount, best_effort, on_held)
			}
		}

		impl<T: Config> Mutate<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: Inspect<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: Mutate<T::AccountId, Balance = T::Balance>,
		{
			fn mint_into(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
				<<T as Config>::Currency>::mint_into(who, amount)
			}
			fn burn_from(
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::burn_from(who, amount)
			}

			fn slash(
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::slash(who, amount)
			}
			fn teleport(
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::teleport(source, dest, amount)
			}
		}

		impl<T: Config> Unbalanced<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: Unbalanced<T::AccountId, Balance = T::Balance>,
		{
			fn set_balance(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
				<<T as Config>::Currency>::set_balance(who, amount)
			}

			fn set_total_issuance(amount: Self::Balance) {
				<<T as Config>::Currency>::set_total_issuance(amount)
			}

			fn decrease_balance(
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::decrease_balance(who, amount)
			}

			fn decrease_balance_at_most(
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Self::Balance {
				<<T as Config>::Currency>::decrease_balance_at_most(who, amount)
			}

			fn increase_balance(
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::increase_balance(who, amount)
			}

			fn increase_balance_at_most(
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Self::Balance {
				<<T as Config>::Currency>::increase_balance_at_most(who, amount)
			}
		}

		impl<T: Config> Transfer<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: Transfer<T::AccountId, Balance = T::Balance>,
		{
			fn transfer(
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
				keep_alive: bool,
			) -> Result<Self::Balance, DispatchError> {
				<<T as Config>::Currency>::transfer(source, dest, amount, keep_alive)
			}
		}

		impl<T: Config> Inspect<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: Inspect<T::AccountId, Balance = T::Balance>,
		{
			type Balance = T::Balance;

			fn total_issuance() -> Self::Balance {
				<<T as Config>::Currency>::total_issuance()
			}

			fn minimum_balance() -> Self::Balance {
				<<T as Config>::Currency>::minimum_balance()
			}

			fn balance(who: &T::AccountId) -> Self::Balance {
				<<T as Config>::Currency>::balance(who)
			}

			fn reducible_balance(who: &T::AccountId, keep_alive: bool) -> Self::Balance {
				<<T as Config>::Currency>::reducible_balance(who, keep_alive)
			}

			fn can_deposit(who: &T::AccountId, amount: Self::Balance) -> DepositConsequence {
				<<T as Config>::Currency>::can_deposit(who, amount)
			}

			fn can_withdraw(
				who: &T::AccountId,
				amount: Self::Balance,
			) -> WithdrawConsequence<Self::Balance> {
				<<T as Config>::Currency>::can_withdraw(who, amount)
			}
		}

		impl<T: Config> InspectHold<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency:
				Inspect<T::AccountId, Balance = T::Balance> + InspectHold<T::AccountId>,
		{
			fn balance_on_hold(who: &T::AccountId) -> Self::Balance {
				<<T as Config>::Currency>::balance_on_hold(who)
			}

			fn can_hold(who: &T::AccountId, amount: Self::Balance) -> bool {
				<<T as Config>::Currency>::can_hold(who, amount)
			}
		}
	}

	mod fungibles {
		use super::*;

		use frame_support::traits::tokens::{
			fungible::{
				Inspect as NativeInspect, InspectHold as NativeInspectHold, Mutate as NativeMutate,
				MutateHold as NativeMutateHold, Transfer as NativeTransfer,
				Unbalanced as NativeUnbalanced,
			},
			fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
			DepositConsequence, WithdrawConsequence,
		};

		impl<T: Config> Unbalanced<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: NativeUnbalanced<T::AccountId, Balance = T::Balance>,
			<T as Config>::MultiCurrency:
				Unbalanced<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
		{
			fn set_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> DispatchResult {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::set_balance(who, amount)
				}
				<<T as Config>::MultiCurrency>::set_balance(asset, who, amount)
			}

			fn set_total_issuance(asset: Self::AssetId, amount: Self::Balance) {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::set_total_issuance(amount)
				}
				<<T as Config>::MultiCurrency>::set_total_issuance(asset, amount)
			}

			fn decrease_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::decrease_balance(who, amount)
				}
				<<T as Config>::MultiCurrency>::decrease_balance(asset, who, amount)
			}

			fn decrease_balance_at_most(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Self::Balance {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::decrease_balance_at_most(who, amount)
				}
				<<T as Config>::MultiCurrency>::decrease_balance_at_most(asset, who, amount)
			}

			fn increase_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::increase_balance(who, amount)
				}
				<<T as Config>::MultiCurrency>::increase_balance(asset, who, amount)
			}

			fn increase_balance_at_most(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Self::Balance {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::increase_balance_at_most(who, amount)
				}
				<<T as Config>::MultiCurrency>::increase_balance_at_most(asset, who, amount)
			}
		}

		impl<T: Config> Transfer<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: NativeTransfer<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: NativeInspect<T::AccountId, Balance = T::Balance>,
			<T as Config>::MultiCurrency:
				Transfer<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
		{
			fn transfer(
				asset: Self::AssetId,
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
				keep_alive: bool,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::transfer(source, dest, amount, keep_alive)
				}
				<<T as Config>::MultiCurrency>::transfer(asset, source, dest, amount, keep_alive)
			}
		}

		impl<T: Config> MutateHold<T::AccountId> for Pallet<T>
		where
			<T as Config>::Currency: NativeInspectHold<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: NativeTransfer<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: NativeMutateHold<T::AccountId, Balance = T::Balance>,

			<T as Config>::MultiCurrency:
				InspectHold<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
			<T as Config>::MultiCurrency:
				Transfer<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
			<T as Config>::MultiCurrency:
				MutateHold<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
		{
			fn hold(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> DispatchResult {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::hold(who, amount)
				}
				<<T as Config>::MultiCurrency>::hold(asset, who, amount)
			}

			fn release(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
				best_effort: bool,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::release(who, amount, best_effort)
				}
				<<T as Config>::MultiCurrency>::release(asset, who, amount, best_effort)
			}

			fn transfer_held(
				asset: Self::AssetId,
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
				best_effort: bool,
				on_hold: bool,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::transfer_held(
						source,
						dest,
						amount,
						best_effort,
						on_hold,
					)
				}
				<<T as Config>::MultiCurrency>::transfer_held(
					asset,
					source,
					dest,
					amount,
					best_effort,
					on_hold,
				)
			}
		}

		impl<T: Config> Mutate<T::AccountId> for Pallet<T>
		where
			<T as Config>::MultiCurrency:
				Inspect<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
			<T as Config>::MultiCurrency:
				Mutate<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,
			<T as Config>::Currency: NativeInspect<T::AccountId, Balance = T::Balance>,
			<T as Config>::Currency: NativeMutate<T::AccountId, Balance = T::Balance>,
		{
			fn mint_into(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> DispatchResult {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::mint_into(who, amount)
				}
				<<T as Config>::MultiCurrency>::mint_into(asset, who, amount)
			}
			fn burn_from(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::burn_from(who, amount)
				}
				<<T as Config>::MultiCurrency>::burn_from(asset, who, amount)
			}

			fn slash(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::slash(who, amount)
				}
				<<T as Config>::MultiCurrency>::slash(asset, who, amount)
			}
			fn teleport(
				asset: Self::AssetId,
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
			) -> Result<Self::Balance, DispatchError> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::teleport(source, dest, amount)
				}
				<<T as Config>::MultiCurrency>::teleport(asset, source, dest, amount)
			}
		}

		impl<T: Config> Inspect<T::AccountId> for Pallet<T>
		where
			<T as Config>::MultiCurrency:
				Inspect<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>,

			<T as Config>::Currency: NativeInspect<T::AccountId, Balance = T::Balance>,
		{
			type AssetId = T::AssetId;
			type Balance = T::Balance;

			fn total_issuance(asset: Self::AssetId) -> Self::Balance {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::total_issuance()
				}
				<<T as Config>::MultiCurrency>::total_issuance(asset)
			}

			fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::minimum_balance()
				}
				<<T as Config>::MultiCurrency>::minimum_balance(asset)
			}

			fn balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::balance(who)
				}
				<<T as Config>::MultiCurrency>::balance(asset, who)
			}

			fn reducible_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				keep_alive: bool,
			) -> Self::Balance {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::reducible_balance(who, keep_alive)
				}
				<<T as Config>::MultiCurrency>::reducible_balance(asset, who, keep_alive)
			}

			fn can_deposit(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> DepositConsequence {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::can_deposit(who, amount)
				}
				<<T as Config>::MultiCurrency>::can_deposit(asset, who, amount)
			}

			fn can_withdraw(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
			) -> WithdrawConsequence<Self::Balance> {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::can_withdraw(who, amount)
				}
				<<T as Config>::MultiCurrency>::can_withdraw(asset, who, amount)
			}
		}

		impl<T: Config> InspectHold<T::AccountId> for Pallet<T>
		where
			<T as Config>::MultiCurrency: Inspect<T::AccountId, Balance = T::Balance, AssetId = T::AssetId>
				+ InspectHold<T::AccountId>,
			<T as Config>::Currency:
				NativeInspect<T::AccountId, Balance = T::Balance> + NativeInspectHold<T::AccountId>,
		{
			fn balance_on_hold(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::balance_on_hold(who)
				}
				<<T as Config>::MultiCurrency>::balance_on_hold(asset, who)
			}

			fn can_hold(asset: Self::AssetId, who: &T::AccountId, amount: Self::Balance) -> bool {
				if asset == T::NativeAssetId::get() {
					return <<T as Config>::Currency>::can_hold(who, amount)
				}
				<<T as Config>::MultiCurrency>::can_hold(asset, who, amount)
			}
		}
	}
}
