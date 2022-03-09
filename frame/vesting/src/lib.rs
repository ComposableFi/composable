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
//! from the moment denoted by `start`, for every `period` amount of moments,
//! `per_period` amount of balance would unlocked, until number of periods
//! `period_count` reached. Note in vesting schedules, *time* is measured by
//! moment type parameter in the Pallet config. All `VestingSchedule`s under an account
//! could be queried in chain state.
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
//! - `claim_for` - Claim unlocked balances on behalf of another account.

#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use composable_traits::vesting::{VestedTransfer, VestingSchedule, VestingTime};
use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{EnsureOrigin, Get, LockIdentifier},
	transactional, BoundedVec,
};
use frame_support::traits::Time;
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
	use composable_traits::vesting::VestingSchedule;
	use orml_traits::{MultiCurrency, MultiLockableCurrency};
	use sp_runtime::traits::AtLeast32Bit;

	use super::*;

	pub(crate) type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub(crate) type AssetIdOf<T, I> =
		<<T as Config<I>>::Currency as MultiCurrency<AccountIdOf<T>>>::CurrencyId;
	pub(crate) type BalanceOf<T, I> =
		<<T as Config<I>>::Currency as MultiCurrency<AccountIdOf<T>>>::Balance;
	pub(crate) type VestingScheduleOf<T, I> = 
		VestingSchedule<<T as Config<I>>::Moment, BalanceOf<T, I>>;
	pub type ScheduledItem<T, I> = (
		AssetIdOf<T, I>,
		<T as frame_system::Config>::AccountId,
		<T as Config<I>>::Moment,
		<T as Config<I>>::Moment,
		u32,
		BalanceOf<T, I>,
	);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: MultiLockableCurrency<Self::AccountId, Moment = Self::Moment>;

		#[pallet::constant]
		/// The minimum amount transferred to call `vested_transfer`.
		type MinVestedTransfer: Get<BalanceOf<Self, I>>;

		/// Required origin for vested transfer.
		type VestedTransferOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

		/// Weight information for extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// The maximum vesting schedules
		type MaxVestingSchedules: Get<u32>;

		/// Measurement of time for vesting schedules.
		type Moment: AtLeast32Bit + Copy;

		/// Trait to inject Moment based logic
		type VestingTime: VestingTime<Self, Moment = Self::Moment>;
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
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
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Added new vesting schedule. \[from, to, schedule\]
		VestingScheduleAdded {
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
			asset: AssetIdOf<T, I>,
			schedule: VestingScheduleOf<T, I>,
		},
		/// Claimed vesting. \[who, locked_amount\]
		Claimed { who: AccountIdOf<T>, asset: AssetIdOf<T, I>, locked_amount: BalanceOf<T, I> },
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
	#[allow(clippy::disallowed_type)]
	pub type VestingSchedules<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		AssetIdOf<T, I>,
		BoundedVec<VestingScheduleOf<T, I>, T::MaxVestingSchedules>,
		ValueQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub vesting: Vec<ScheduledItem<T, I>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			GenesisConfig { vesting: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {
			self.vesting.iter().for_each(
				|(asset, who, start, period, period_count, per_period)| {
					let mut bounded_schedules = VestingSchedules::<T>::get(who, asset);
					bounded_schedules
						.try_push(VestingSchedule {
							start: *start,
							period: *period,
							period_count: *period_count,
							per_period: *per_period,
						})
						.expect("Max vesting schedules exceeded");
					let total_amount = bounded_schedules
						.iter()
						.try_fold::<_, _, Result<BalanceOf<T, I>, DispatchError>>(
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
					VestingSchedules::<T, I>::insert(who, asset, bounded_schedules);
				},
			);
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T, I  = ()>(_);

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberOf<T>> for Pallet<T, I> {}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::weight(T::WeightInfo::claim((<T as Config>::MaxVestingSchedules::get() / 2) as u32))]
		pub fn claim(origin: OriginFor<T>, asset: AssetIdOf<T, I>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let locked_amount = Self::do_claim(&who, asset)?;

			Self::deposit_event(Event::Claimed { who, asset, locked_amount });
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::vested_transfer())]
		pub fn vested_transfer(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T, I>,
			schedule: VestingScheduleOf<T, I>,
		) -> DispatchResult {
			let from = T::VestedTransferOrigin::ensure_origin(origin)?;
			let to = T::Lookup::lookup(dest)?;
			<Self as VestedTransfer>::vested_transfer(asset, &from, &to, schedule.clone())?;

			Self::deposit_event(Event::VestingScheduleAdded { from, to, asset, schedule });
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_vesting_schedules(vesting_schedules.len() as u32))]
		pub fn update_vesting_schedules(
			origin: OriginFor<T>,
			who: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T, I>,
			vesting_schedules: Vec<VestingScheduleOf<T, I>>,
		) -> DispatchResult {
			ensure_root(origin)?;

			let account = T::Lookup::lookup(who)?;
			Self::do_update_vesting_schedules(&account, asset, vesting_schedules)?;

			Self::deposit_event(Event::VestingSchedulesUpdated { who: account });
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::claim((<T as Config>::MaxVestingSchedules::get() / 2) as u32))]
		pub fn claim_for(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T, I>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let who = T::Lookup::lookup(dest)?;
			let locked_amount = Self::do_claim(&who, asset)?;

			Self::deposit_event(Event::Claimed { who, asset, locked_amount });
			Ok(())
		}
	}
}

impl<T: Config<I>, I: 'static> VestedTransfer for Pallet<T, I> {
	type AccountId = AccountIdOf<T>;
	type AssetId = AssetIdOf<T, I>;
	type BlockNumber = BlockNumberOf<T>;
	type Balance = BalanceOf<T, I>;
	type MinVestedTransfer = T::MinVestedTransfer;

	#[transactional]
	fn vested_transfer(
		asset: Self::AssetId,
		from: &Self::AccountId,
		to: &Self::AccountId,
		schedule: VestingSchedule<Self::BlockNumber, Self::Balance>,
	) -> frame_support::dispatch::DispatchResult {
		let schedule_amount = ensure_valid_vesting_schedule::<T>(&schedule)?;

		let total_amount = Self::locked_balance(to, asset)
			.checked_add(&schedule_amount)
			.ok_or(ArithmeticError::Overflow)?;

		T::Currency::transfer(asset, from, to, schedule_amount)?;
		T::Currency::set_lock(VESTING_LOCK_ID, asset, to, total_amount)?;
		<VestingSchedules<T, I>>::try_append(to, asset, schedule)
			.map_err(|_| Error::<T, I>::MaxVestingSchedulesExceeded)?;
		Ok(())
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	fn do_claim(who: &AccountIdOf<T>, asset: AssetIdOf<T, I>) -> Result<BalanceOf<T, I>, DispatchError> {
		let locked = Self::locked_balance(who, asset);
		if locked.is_zero() {
			// cleanup the storage and unlock the fund
			<VestingSchedules<T, I>>::remove(who, asset);
			T::Currency::remove_lock(VESTING_LOCK_ID, asset, who)?;
		} else {
			T::Currency::set_lock(VESTING_LOCK_ID, asset, who, locked)?;
		}
		Ok(locked)
	}

	/// Returns locked balance based on current block number.
	fn locked_balance(who: &AccountIdOf<T>, asset: AssetIdOf<T, I>) -> BalanceOf<T, I> {
		let now = T::VestingTime::now();
		<VestingSchedules<T, I>>::mutate_exists(who, asset, |maybe_schedules| {
			let total = if let Some(schedules) = maybe_schedules.as_mut() {
				let mut total: BalanceOf<T, I> = Zero::zero();
				schedules.retain(|s| {
					let amount = s.locked_amount(now);
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
		asset: AssetIdOf<T, I>,
		schedules: Vec<VestingScheduleOf<T, I>>,
	) -> DispatchResult {
		let bounded_schedules: BoundedVec<VestingScheduleOf<T, I>, T::MaxVestingSchedules> =
			schedules.try_into().map_err(|_| Error::<T, I>::MaxVestingSchedulesExceeded)?;

		// empty vesting schedules cleanup the storage and unlock the fund
		if bounded_schedules.len().is_zero() {
			<VestingSchedules<T, I>>::remove(who, asset);
			T::Currency::remove_lock(VESTING_LOCK_ID, asset, who)?;
			return Ok(())
		}

		let total_amount =
			bounded_schedules.iter().try_fold::<_, _, Result<BalanceOf<T, I>, DispatchError>>(
				Zero::zero(),
				|acc_amount, schedule| {
					let amount = ensure_valid_vesting_schedule::<T, I>(schedule)?;
					Ok(acc_amount + amount)
				},
			)?;
		ensure!(
			T::Currency::free_balance(asset, who) >= total_amount,
			Error::<T, I>::InsufficientBalanceToLock,
		);

		T::Currency::set_lock(VESTING_LOCK_ID, asset, who, total_amount)?;
		<VestingSchedules<T, I>>::insert(who, asset, bounded_schedules);

		Ok(())
	}
}

/// Returns `Ok(total_total)` if valid schedule, or error.
fn ensure_valid_vesting_schedule<T: Config<I>, I: 'static>(
	schedule: &VestingScheduleOf<T, I>,
) -> Result<BalanceOf<T, I>, DispatchError> {
	ensure!(!schedule.period.is_zero(), Error::<T, I>::ZeroVestingPeriod);
	ensure!(!schedule.period_count.is_zero(), Error::<T, I>::ZeroVestingPeriodCount);
	ensure!(schedule.end().is_some(), ArithmeticError::Overflow);

	let total_total = schedule.total_amount().ok_or(ArithmeticError::Overflow)?;

	ensure!(total_total >= T::MinVestedTransfer::get(), Error::<T, I>::AmountLow);

	Ok(total_total)
}
