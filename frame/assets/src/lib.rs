#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::FullCodec;
	use frame_support::{
		dispatch::Codec,
		pallet_prelude::*,
		sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub},
		sp_std::fmt::Debug,
	};
	use sp_std::ops::AddAssign;

	pub trait AssetId: FullCodec + Copy + Eq + PartialEq + Debug {}
	impl<T: FullCodec + Copy + Eq + PartialEq + Debug> AssetId for T {}
	pub trait Balance:
		AtLeast32BitUnsigned
		+ FullCodec
		+ Copy
		+ Default
		+ Debug
		+ MaybeSerializeDeserialize
		+ MaxEncodedLen
	{
	}
	impl<
			T: AtLeast32BitUnsigned
				+ FullCodec
				+ Copy
				+ Default
				+ Debug
				+ MaybeSerializeDeserialize
				+ MaxEncodedLen,
		> Balance for T
	{
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after a vault has been successfully created.
		VaultCreated {
			/// The (incremented) ID of the created vault.
			id: u32,
		},
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: AssetId;
		type Balance: Balance;

		type NativeAssetId: Get<Self::AssetId>;
		type Currency;
		type MultiCurrency;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		Unknown,
	}

	mod currency {
		use super::*;
		use frame_support::traits::{Currency, ExistenceRequirement, SignedImbalance, WithdrawReasons, ReservableCurrency, BalanceStatus};

		impl<T: Config> ReservableCurrency<T::AccountId> for Pallet<T>
			where
				<T as Config>::Currency: Currency<T::AccountId, Balance = T::Balance>,
				<T as Config>::Currency: ReservableCurrency<T::AccountId, Balance = T::Balance>,
		{
			fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
				<<T as Config>::Currency>::can_reserve(who, value)
			}

			fn slash_reserved(who: &T::AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
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

			fn repatriate_reserved(slashed: &T::AccountId, beneficiary: &T::AccountId, value: Self::Balance, status: BalanceStatus) -> Result<Self::Balance, DispatchError> {
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

		use frame_support::traits::{
			fungible::{CreditOf, DebtOf},
			tokens::{
				fungible::{
					BalancedHold, Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced,
				},
				DepositConsequence, WithdrawConsequence,
			},
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
