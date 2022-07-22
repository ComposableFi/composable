//! # Vesting Module
//!
//! ## Overview
//!
//! Vesting module provides a means of scheduled balance lock on an account. It
//! uses the *graded vesting* way, which unlocks a specific amount of balance
//! every period of time, until all balance unlocked.
//!
//! ### Vesting Schedule
//!
//! The schedule of a vesting is described by data structure `VestingSchedule`:
//! from the time of `window.start`, for every `window.period` amount of time,
//! `per_period` amount of balance would unlocked, until number of periods
//! `period_count` reached. The pallet supports measuring time windows in terms of absolute
//! timestamps as well as block numbers for vesting schedules. All `VestingSchedule`s under
//! an account could be queried in chain state.
//!
//! ## Interface
//! - `VestedTransfer` - allowing a third party pallet to have this implementation as dependency to
//!   execute vested transfers.
//!
//! ### Dispatchable Functions
//!
//! - `vested_transfer` - Add a new vesting schedule for an account.
//! - `claim` - Claim unlocked balances.
//! - `update_vesting_schedules` - Update all vesting schedules under an account, `root` origin
//!   required.

#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use composable_traits::vesting::{VestedTransfer, VestingSchedule};
use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{EnsureOrigin, Get, LockIdentifier, Time},
	transactional, BoundedVec,
};
use frame_system::{ensure_root, ensure_signed, pallet_prelude::*};
use orml_traits::{MultiCurrency, MultiLockableCurrency};
use sp_runtime::{
	traits::{BlockNumberProvider, CheckedAdd, Saturating, StaticLookup, Zero},
	ArithmeticError, DispatchResult,
};
use sp_std::{convert::TryInto, vec::Vec};

mod weights;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarks;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use module::*;
pub use weights::WeightInfo;

pub const VESTING_LOCK_ID: LockIdentifier = *b"compvest";

#[frame_support::pallet]
pub mod module {
	use codec::FullCodec;
	use composable_traits::vesting::{VestingSchedule, VestingWindow};
	use frame_support::traits::Time;
	use orml_traits::{MultiCurrency, MultiLockableCurrency};
	use sp_runtime::traits::AtLeast32Bit;

	use super::*;

	pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub(crate) type MomentOf<T> = <T as Config>::Moment;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T> =
		<<T as Config>::Currency as MultiCurrency<AccountIdOf<T>>>::CurrencyId;
	pub(crate) type BalanceOf<T> =
		<<T as Config>::Currency as MultiCurrency<AccountIdOf<T>>>::Balance;
	pub(crate) type VestingScheduleOf<T> =
		VestingSchedule<BlockNumberOf<T>, MomentOf<T>, BalanceOf<T>>;
	pub type ScheduledItem<T> = (
		AssetIdOf<T>,
		<T as frame_system::Config>::AccountId,
		VestingWindow<BlockNumberOf<T>, MomentOf<T>>,
		u32,
		BalanceOf<T>,
	);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: MultiLockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

		#[pallet::constant]
		/// The minimum amount transferred to call `vested_transfer`.
		type MinVestedTransfer: Get<BalanceOf<Self>>;

		/// Required origin for vested transfer.
		type VestedTransferOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// The maximum vesting schedules
		type MaxVestingSchedules: Get<u32>;

		/// Type of time
		type Moment: AtLeast32Bit
			+ Parameter
			+ Default
			+ Copy
			+ MaxEncodedLen
			+ FullCodec
			+ MaybeSerializeDeserialize;

		/// The time provider.
		type Time: Time<Moment = Self::Moment>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Vesting period is zero
		ZeroVestingPeriod,
		/// Number of vests is zero
		ZeroVestingPeriodCount,
		/// Insufficient amount of balance to lock
		InsufficientBalanceToLock,
		/// This account have too many vesting schedules
		TooManyVestingSchedules,
		/// The vested transfer amount is too low
		AmountLow,
		/// Failed because the maximum vesting schedules was exceeded
		MaxVestingSchedulesExceeded,
		/// Trying to vest to ourselves
		TryingToSelfVest,
		/// There is no vesting schedule with a given id
		NonExistentVestingSchedule,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Added new vesting schedule. \[from, to, schedule\]
		VestingScheduleAdded {
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
			asset: AssetIdOf<T>,
			schedule: VestingScheduleOf<T>,
		},
		/// Claimed vesting. \[who, locked_amount\]
		/// TODO: add vesting_schedule_id to event
		Claimed { who: AccountIdOf<T>, asset: AssetIdOf<T>, locked_amount: BalanceOf<T> },
		/// Updated vesting schedules. \[who\]
		VestingSchedulesUpdated { who: AccountIdOf<T> },
	}

	/// Vesting schedules of an account.
	///
	/// VestingSchedules: map AccountId => Vec<VestingSchedule>
	#[pallet::storage]
	#[pallet::getter(fn vesting_schedules)]
	// FIXME: Temporary fix to get CI to pass, separate PRs will be made per pallet to refactor to
	// use OptionQuery instead
	#[allow(clippy::disallowed_types)]
	pub type VestingSchedules<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		AssetIdOf<T>,
		BoundedVec<VestingScheduleOf<T>, T::MaxVestingSchedules>,
		ValueQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub vesting: Vec<ScheduledItem<T>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig { vesting: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			self.vesting.iter().for_each(|(asset, who, window, period_count, per_period)| {
				let mut bounded_schedules = VestingSchedules::<T>::get(who, asset);
				bounded_schedules
					.try_push(VestingSchedule {
						/// vesting_schedule_id: /// TODO: generate random id
						window: window.clone(),
						period_count: *period_count,
						per_period: *per_period,
					})
					.expect("Max vesting schedules exceeded");
				let total_amount = bounded_schedules
					.iter()
					.try_fold::<_, _, Result<BalanceOf<T>, DispatchError>>(
						Zero::zero(),
						|acc_amount, schedule| {
							let amount = ensure_valid_vesting_schedule::<T>(schedule)?;
							Ok(acc_amount + amount)
						},
					)
					.expect("Invalid vesting schedule");

				assert!(
					T::Currency::free_balance(*asset, who) >= total_amount,
					"Account do not have enough balance"
				);

				T::Currency::set_lock(VESTING_LOCK_ID, *asset, who, total_amount)
					.expect("impossible; qed;");
				VestingSchedules::<T>::insert(who, asset, bounded_schedules);
			});
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberOf<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::claim((<T as Config>::MaxVestingSchedules::get() / 2) as u32))]
		/// TODO: add vesting_schedule_id parameter
		pub fn claim(origin: OriginFor<T>, asset: AssetIdOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let locked_amount = Self::do_claim(&who, asset)?;

			Self::deposit_event(Event::Claimed { who, asset, locked_amount });
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::vested_transfer())]
		pub fn vested_transfer(
			origin: OriginFor<T>,
			from: <T::Lookup as StaticLookup>::Source,
			beneficiary: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T>,
			schedule: VestingScheduleOf<T>,
		) -> DispatchResult {
			T::VestedTransferOrigin::ensure_origin(origin)?;
			let from = T::Lookup::lookup(from)?;
			let to = T::Lookup::lookup(beneficiary)?;
			<Self as VestedTransfer>::vested_transfer(asset, &from, &to, schedule)?;

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::update_vesting_schedules(vesting_schedules.len() as u32))]
		pub fn update_vesting_schedules(
			origin: OriginFor<T>,
			who: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T>,
			vesting_schedules: Vec<VestingScheduleOf<T>>,
		) -> DispatchResult {
			ensure_root(origin)?;

			let account = T::Lookup::lookup(who)?;
			Self::do_update_vesting_schedules(&account, asset, vesting_schedules)?;

			Self::deposit_event(Event::VestingSchedulesUpdated { who: account });
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::claim((<T as Config>::MaxVestingSchedules::get() / 2) as u32))]
		pub fn claim_for(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T>,
			// TODO: add vesting_schedule_id parameter
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let who = T::Lookup::lookup(dest)?;
			let locked_amount = Self::do_claim(&who, asset)?;

			/// TODO: add vesting_schedule_id to event
			Self::deposit_event(Event::Claimed { who, asset, locked_amount });
			Ok(())
		}
	}
}

impl<T: Config> VestedTransfer for Pallet<T> {
	type AccountId = AccountIdOf<T>;
	type AssetId = AssetIdOf<T>;
	type BlockNumber = BlockNumberOf<T>;
	type Moment = MomentOf<T>;
	type Balance = BalanceOf<T>;
	type MinVestedTransfer = T::MinVestedTransfer;

	#[transactional]
	fn vested_transfer(
		asset: Self::AssetId,
		from: &Self::AccountId,
		to: &Self::AccountId,
		schedule: VestingSchedule<Self::BlockNumber, Self::Moment, Self::Balance>,
	) -> frame_support::dispatch::DispatchResult {
		ensure!(from != to, Error::<T>::TryingToSelfVest);

		let schedule_amount = ensure_valid_vesting_schedule::<T>(&schedule)?;

		// TODO: pass schedule.vesting_schedule_id to this function
		let total_amount = Self::locked_balance(to, asset)
			.checked_add(&schedule_amount)
			.ok_or(ArithmeticError::Overflow)?;

		T::Currency::transfer(asset, from, to, schedule_amount)?;
		T::Currency::set_lock(VESTING_LOCK_ID, asset, to, total_amount)?;
		<VestingSchedules<T>>::try_append(to, asset, &schedule)
			.map_err(|_| Error::<T>::MaxVestingSchedulesExceeded)?;

		Self::deposit_event(Event::VestingScheduleAdded {
			from: from.clone(),
			to: to.clone(),
			asset,
			schedule,
		});

		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	fn do_claim(who: &AccountIdOf<T>, asset: AssetIdOf<T>) -> Result<BalanceOf<T>, DispatchError> {
		let locked = Self::locked_balance(who, asset);
		if locked.is_zero() {
			// cleanup the storage and unlock the fund
			<VestingSchedules<T>>::remove(who, asset);
			T::Currency::remove_lock(VESTING_LOCK_ID, asset, who)?;
		} else {
			T::Currency::set_lock(VESTING_LOCK_ID, asset, who, locked)?;
		}
		Ok(locked)
	}

	/// Returns locked balance based on current block number.
	/// TODO: add vesting_schedule_id parameter
	fn locked_balance(who: &AccountIdOf<T>, asset: AssetIdOf<T>) -> BalanceOf<T> {
		<VestingSchedules<T>>::mutate_exists(who, asset, |maybe_schedules| {
			let total = if let Some(schedules) = maybe_schedules.as_mut() {
				let mut total: BalanceOf<T> = Zero::zero();
				schedules.retain(|s| {
					/// TODO: only use schedule with given vesting_schedule_id, or throw
					/// NonExistentVestingSchedule error if not found
					let amount = s.locked_amount(
						frame_system::Pallet::<T>::current_block_number(),
						T::Time::now(),
					);
					total = total.saturating_add(amount);
					!amount.is_zero()
				});
				total
			} else {
				Zero::zero()
			};
			if total.is_zero() {
				*maybe_schedules = None;
			}
			total
		})
	}

	fn do_update_vesting_schedules(
		who: &AccountIdOf<T>,
		asset: AssetIdOf<T>,
		schedules: Vec<VestingScheduleOf<T>>,
	) -> DispatchResult {
		let bounded_schedules: BoundedVec<VestingScheduleOf<T>, T::MaxVestingSchedules> =
			schedules.try_into().map_err(|_| Error::<T>::MaxVestingSchedulesExceeded)?;

		// empty vesting schedules cleanup the storage and unlock the fund
		if bounded_schedules.len().is_zero() {
			<VestingSchedules<T>>::remove(who, asset);
			T::Currency::remove_lock(VESTING_LOCK_ID, asset, who)?;
			return Ok(())
		}

		let total_amount =
			bounded_schedules.iter().try_fold::<_, _, Result<BalanceOf<T>, DispatchError>>(
				Zero::zero(),
				|acc_amount, schedule| {
					let amount = ensure_valid_vesting_schedule::<T>(schedule)?;
					Ok(acc_amount + amount)
				},
			)?;
		ensure!(
			T::Currency::free_balance(asset, who) >= total_amount,
			Error::<T>::InsufficientBalanceToLock,
		);

		T::Currency::set_lock(VESTING_LOCK_ID, asset, who, total_amount)?;
		<VestingSchedules<T>>::insert(who, asset, bounded_schedules);

		Ok(())
	}
}

/// Returns `Ok(total_total)` if valid schedule, or error.
fn ensure_valid_vesting_schedule<T: Config>(
	schedule: &VestingScheduleOf<T>,
) -> Result<BalanceOf<T>, DispatchError> {
	ensure!(!schedule.is_zero_period(), Error::<T>::ZeroVestingPeriod);
	ensure!(schedule.end().is_some(), ArithmeticError::Overflow);
	ensure!(!schedule.period_count.is_zero(), Error::<T>::ZeroVestingPeriodCount);

	let total_total = schedule.total_amount().ok_or(ArithmeticError::Overflow)?;

	ensure!(total_total >= T::MinVestedTransfer::get(), Error::<T>::AmountLow);

	Ok(total_total)
}
