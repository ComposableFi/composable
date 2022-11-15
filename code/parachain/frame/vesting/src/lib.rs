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
//! - `claim_for` - Claim unlocked balances for a `target` account.
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

use composable_support::{
	abstractions::utils::increment::Increment,
	math::safe::{SafeAdd, SafeSub},
};
use composable_traits::vesting::{
	VestedTransfer, VestingSchedule, VestingScheduleIdSet, VestingScheduleInfo,
};
use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{EnsureOrigin, Get, LockIdentifier, Time},
	transactional, BoundedBTreeMap,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use orml_traits::{MultiCurrency, MultiLockableCurrency};
use sp_runtime::{
	traits::{BlockNumberProvider, One, StaticLookup, Zero},
	ArithmeticError, DispatchResult,
};
use sp_std::{collections::btree_map::BTreeMap, convert::TryInto, vec, vec::Vec};

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
	use codec::{FullCodec, MaxEncodedLen};
	use composable_support::{
		abstractions::{
			nonce::Nonce,
			utils::{increment::SafeIncrement, start_at::ZeroInit},
		},
		math::safe::SafeAdd,
	};
	use composable_traits::vesting::{VestingSchedule, VestingScheduleInfo, VestingWindow};
	use frame_support::{traits::Time, BoundedBTreeMap};
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
	pub(crate) type VestingScheduleOf<T> = VestingSchedule<
		<T as Config>::VestingScheduleId,
		BlockNumberOf<T>,
		MomentOf<T>,
		BalanceOf<T>,
	>;
	pub(crate) type VestingScheduleInfoOf<T> =
		VestingScheduleInfo<BlockNumberOf<T>, MomentOf<T>, BalanceOf<T>>;
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

		/// Required origin for updating schedules.
		type UpdateSchedulesOrigin: EnsureOrigin<Self::Origin>;

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

		/// The ID of a vesting schedule.
		type VestingScheduleId: Member
			+ Copy
			+ Zero
			+ SafeAdd
			+ One
			+ Ord
			+ FullCodec
			+ MaxEncodedLen
			+ TypeInfo;
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
		VestingScheduleNotFound,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Added new vesting schedule.
		VestingScheduleAdded {
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
			asset: AssetIdOf<T>,
			vesting_schedule_id: T::VestingScheduleId,
			schedule: VestingScheduleOf<T>,
			schedule_amount: BalanceOf<T>,
		},
		/// Claimed vesting.
		Claimed {
			who: AccountIdOf<T>,
			asset: AssetIdOf<T>,
			vesting_schedule_ids:
				VestingScheduleIdSet<T::VestingScheduleId, T::MaxVestingSchedules>,
			locked_amount: BalanceOf<T>,
			claimed_amount_per_schedule:
				BoundedBTreeMap<T::VestingScheduleId, BalanceOf<T>, T::MaxVestingSchedules>,
		},
		/// Updated vesting schedules.
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
		BoundedBTreeMap<T::VestingScheduleId, VestingScheduleOf<T>, T::MaxVestingSchedules>,
		ValueQuery,
	>;

	/// Counter used to uniquely identify vesting schedules within this pallet.
	#[pallet::storage]
	#[pallet::getter(fn vesting_schedules_count)]
	#[allow(clippy::disallowed_types)] // nonce, ValueQuery is OK
	pub type VestingScheduleNonce<T: Config> =
		StorageValue<_, T::VestingScheduleId, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

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
				let vesting_schedule_id =
					VestingScheduleNonce::<T>::increment().expect("Max vesting schedules exceeded");

				bounded_schedules
					.try_insert(
						vesting_schedule_id,
						VestingSchedule {
							vesting_schedule_id,
							window: window.clone(),
							period_count: *period_count,
							per_period: *per_period,
							already_claimed: BalanceOf::<T>::zero(),
						},
					)
					.expect("Max vesting schedules exceeded");

				let total_amount = bounded_schedules
					.iter()
					.try_fold::<_, _, Result<BalanceOf<T>, DispatchError>>(
						Zero::zero(),
						|acc_amount, (_id, schedule)| {
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
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberOf<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Unlock any vested funds of the origin account.
		///
		/// The dispatch origin for this call must be _Signed_ and the sender must have funds still
		/// locked under this pallet.
		///
		/// - `asset`: The asset associated with the vesting schedule
		/// - `vesting_schedule_ids`: The ids of the vesting schedules to be claimed
		///
		/// Emits `Claimed`.
		#[pallet::weight(<T as Config>::WeightInfo::claim((<T as Config>::MaxVestingSchedules::get() / 2) as u32))]
		pub fn claim(
			origin: OriginFor<T>,
			asset: AssetIdOf<T>,
			vesting_schedule_ids: VestingScheduleIdSet<
				T::VestingScheduleId,
				T::MaxVestingSchedules,
			>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_claim(&who, asset, vesting_schedule_ids)?;

			Ok(())
		}

		/// Create a vested transfer.
		///
		/// The dispatch origin for this call must be _Root_ or Democracy.
		///
		/// - `from`: The account sending the vested funds.
		/// - `beneficiary`: The account receiving the vested funds.
		/// - `asset`: The asset associated with this vesting schedule.
		/// - `schedule_info`: The vesting schedule data attached to the transfer.
		///
		/// Emits `VestingScheduleAdded`.
		///
		/// NOTE: This will unlock all schedules through the current block.
		#[pallet::weight(<T as Config>::WeightInfo::vested_transfer())]
		pub fn vested_transfer(
			origin: OriginFor<T>,
			from: <T::Lookup as StaticLookup>::Source,
			beneficiary: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T>,
			schedule_info: VestingScheduleInfoOf<T>,
		) -> DispatchResult {
			T::VestedTransferOrigin::ensure_origin(origin)?;
			let from = T::Lookup::lookup(from)?;
			let to = T::Lookup::lookup(beneficiary)?;
			<Self as VestedTransfer>::vested_transfer(asset, &from, &to, schedule_info)?;

			Ok(())
		}

		/// Update vesting schedules
		///
		/// The dispatch origin for this call must be _Root_ or democracy.
		///
		/// - `who`: The account whose vested funds should be updated.
		/// - `asset`: The asset associated with the vesting schedules.
		/// - `vesting_schedules`: The updated vesting schedules.
		///
		/// Emits `VestingSchedulesUpdated`.
		#[pallet::weight(<T as Config>::WeightInfo::update_vesting_schedules(vesting_schedules.len() as u32))]
		pub fn update_vesting_schedules(
			origin: OriginFor<T>,
			who: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T>,
			vesting_schedules: Vec<VestingScheduleInfoOf<T>>,
		) -> DispatchResult {
			T::UpdateSchedulesOrigin::ensure_origin(origin)?;

			let account = T::Lookup::lookup(who)?;
			Self::do_update_vesting_schedules(&account, asset, vesting_schedules)?;

			Self::deposit_event(Event::VestingSchedulesUpdated { who: account });
			Ok(())
		}

		/// Unlock any vested funds of a `target` account.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `dest`: The account whose vested funds should be unlocked. Must have funds still
		/// locked under this pallet.
		/// - `asset`: The asset associated with the vesting schedule.
		/// - `vesting_schedule_ids`: The ids of the vesting schedules to be claimed.
		///
		/// Emits `Claimed`.
		#[pallet::weight(<T as Config>::WeightInfo::claim((<T as Config>::MaxVestingSchedules::get() / 2) as u32))]
		pub fn claim_for(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			asset: AssetIdOf<T>,
			vesting_schedule_ids: VestingScheduleIdSet<
				T::VestingScheduleId,
				T::MaxVestingSchedules,
			>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let who = T::Lookup::lookup(dest)?;
			Self::do_claim(&who, asset, vesting_schedule_ids)?;

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
	type VestingScheduleId = T::VestingScheduleId;
	type VestingScheduleNonce = VestingScheduleNonce<T>;

	#[transactional]
	fn vested_transfer(
		asset: Self::AssetId,
		from: &Self::AccountId,
		to: &Self::AccountId,
		schedule_info: VestingScheduleInfo<Self::BlockNumber, Self::Moment, Self::Balance>,
	) -> frame_support::dispatch::DispatchResult {
		ensure!(from != to, Error::<T>::TryingToSelfVest);

		let vesting_schedule_id = Self::VestingScheduleNonce::increment()?;
		let schedule = VestingSchedule::from_input(vesting_schedule_id, schedule_info);

		let schedule_amount = ensure_valid_vesting_schedule::<T>(&schedule)?;

		let locked = Self::locked_balance(to, asset, VestingScheduleIdSet::All)
			.unwrap_or_else(|_| Zero::zero());

		let total_amount = locked.safe_add(&schedule_amount)?;

		T::Currency::transfer(asset, from, to, schedule_amount)?;
		T::Currency::set_lock(VESTING_LOCK_ID, asset, to, total_amount)?;

		<VestingSchedules<T>>::mutate(to, asset, |schedules| {
			schedules
				.try_insert(vesting_schedule_id, schedule.clone())
				.map_err(|_| Error::<T>::MaxVestingSchedulesExceeded)
		})?;

		Self::deposit_event(Event::VestingScheduleAdded {
			from: from.clone(),
			to: to.clone(),
			asset,
			schedule,
			vesting_schedule_id,
			schedule_amount,
		});

		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	fn do_claim(
		who: &AccountIdOf<T>,
		asset: AssetIdOf<T>,
		vesting_schedule_ids: VestingScheduleIdSet<T::VestingScheduleId, T::MaxVestingSchedules>,
	) -> Result<(), DispatchError> {
		let current_locked_amount = Self::unclaimed_balance(who, asset, VestingScheduleIdSet::All)?;
		let (balance_to_claim, claimed_amount_per_schedule) =
			Self::unlocked_claimable_balance(who, asset, vesting_schedule_ids.clone())?;

		let new_locked_amount = current_locked_amount.safe_sub(&balance_to_claim)?;

		if new_locked_amount.is_zero() {
			// cleanup the storage and unlock the fund
			<VestingSchedules<T>>::remove(who, asset);
			T::Currency::remove_lock(VESTING_LOCK_ID, asset, who)?;
		} else {
			T::Currency::set_lock(VESTING_LOCK_ID, asset, who, new_locked_amount)?;
		}

		Self::deposit_event(Event::Claimed {
			who: who.clone(),
			asset,
			locked_amount: new_locked_amount,
			vesting_schedule_ids,
			claimed_amount_per_schedule,
		});

		Ok(())
	}

	/// Claims all available balance
	/// Returns total locked balance for a given account, asset and vesting schedules, based on
	/// current block number
	fn locked_balance(
		who: &AccountIdOf<T>,
		asset: AssetIdOf<T>,
		vesting_schedule_ids: VestingScheduleIdSet<T::VestingScheduleId, T::MaxVestingSchedules>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let maybe_schedules = <VestingSchedules<T>>::try_get(who, asset)
			.map_err(|_| Error::<T>::VestingScheduleNotFound);

		let total_locked = match maybe_schedules {
			Ok(mut schedules) => vesting_schedule_ids
				.into_all_ids(&schedules)
				.iter()
				.try_fold::<_, _, Result<BalanceOf<T>, DispatchError>>(
					Zero::zero(),
					|accumulated_amount, schedule_id| {
						let schedule = schedules
							.get(schedule_id)
							.ok_or(Error::<T>::VestingScheduleNotFound)?;

						let locked_amount = schedule.locked_amount(
							frame_system::Pallet::<T>::current_block_number(),
							T::Time::now(),
						);

						if locked_amount.is_zero() {
							schedules.remove(schedule_id);
						}

						Ok(accumulated_amount.safe_add(&locked_amount)?)
					},
				)?,
			_ => return Err(Error::<T>::VestingScheduleNotFound.into()),
		};

		Ok(total_locked)
	}

	/// Returns total unclaimed balance for a given account, asset and vesting schedules
	fn unclaimed_balance(
		who: &AccountIdOf<T>,
		asset: AssetIdOf<T>,
		vesting_schedule_ids: VestingScheduleIdSet<T::VestingScheduleId, T::MaxVestingSchedules>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let maybe_schedules = <VestingSchedules<T>>::try_get(who, asset)
			.map_err(|_| Error::<T>::VestingScheduleNotFound);

		let total_unclaimed = match maybe_schedules {
			Ok(schedules) => vesting_schedule_ids
				.into_all_ids(&schedules)
				.iter()
				.try_fold::<_, _, Result<BalanceOf<T>, DispatchError>>(
					Zero::zero(),
					|accumulated_amount, schedule_id| {
						let schedule = schedules
							.get(schedule_id)
							.ok_or(Error::<T>::VestingScheduleNotFound)?;

						schedule
							.total_amount()?
							.safe_sub(&schedule.already_claimed)?
							.safe_add(&accumulated_amount)
							.map_err(Into::into)
					},
				)?,
			_ => return Err(Error::<T>::VestingScheduleNotFound.into()),
		};

		Ok(total_unclaimed)
	}

	/// Returns balance available to claim for a given account, asset and vesting schedules, based
	/// on current block number, as well as the claimed balance per vesting schedule.
	/// It also updates the already claimed balance and removes completely vested schedules
	fn unlocked_claimable_balance(
		who: &AccountIdOf<T>,
		asset: AssetIdOf<T>,
		vesting_schedule_ids: VestingScheduleIdSet<T::VestingScheduleId, T::MaxVestingSchedules>,
	) -> Result<
		(BalanceOf<T>, BoundedBTreeMap<T::VestingScheduleId, BalanceOf<T>, T::MaxVestingSchedules>),
		DispatchError,
	> {
		<VestingSchedules<T>>::try_mutate_exists(who, asset, |maybe_schedules| {
			let schedules = maybe_schedules.as_mut().ok_or(Error::<T>::VestingScheduleNotFound)?;
			vesting_schedule_ids.into_all_ids(schedules).iter().try_fold(
				(
					BalanceOf::<T>::zero(),
					BoundedBTreeMap::<
						T::VestingScheduleId,
						BalanceOf<T>,
						T::MaxVestingSchedules,
					>::new(),
				),
				|(mut total_balance_to_claim, mut claims_per_schedule), id_to_claim| {
					let schedule = schedules
						.get_mut(id_to_claim)
						.ok_or(Error::<T>::VestingScheduleNotFound)?;

					// Total amount for vesting schedule
					let total_amount = ensure_valid_vesting_schedule::<T>(schedule)?;
					// Currently locked amount
					let locked_amount = schedule.locked_amount(
						frame_system::Pallet::<T>::current_block_number(),
						T::Time::now(),
					);
					// All balance that is not locked, including both claimed and unclaimed
					let unlocked_amount = total_amount.safe_sub(&locked_amount)?;
					// Balance that is not locked and has not been claimed yet
					let available_amount =
						unlocked_amount.safe_sub(&schedule.already_claimed)?;

					// Update claimed amount for specified schedules
					schedule.already_claimed =
						schedule.already_claimed.safe_add(&available_amount)?;

					total_balance_to_claim =
						total_balance_to_claim.safe_add(&available_amount)?;

					claims_per_schedule
						.try_insert(*id_to_claim, available_amount)
						.expect("Max vesting schedules exceeded");

					if locked_amount.is_zero() {
						// Remove fully claimed schedules
						schedules.remove(id_to_claim);
					};

					Ok((total_balance_to_claim, claims_per_schedule))
				},
			)
		})
	}

	fn do_update_vesting_schedules(
		who: &AccountIdOf<T>,
		asset: AssetIdOf<T>,
		schedules: Vec<VestingScheduleInfoOf<T>>,
	) -> DispatchResult {
		// empty vesting schedules cleanup the storage and unlock the fund
		if schedules.is_empty() {
			<VestingSchedules<T>>::remove(who, asset);
			T::Currency::remove_lock(VESTING_LOCK_ID, asset, who)?;
			return Ok(())
		}

		ensure!(
			schedules.len() as u32 <= T::MaxVestingSchedules::get(),
			Error::<T>::MaxVestingSchedulesExceeded
		);

		let bounded_schedules: BoundedBTreeMap<_, _, _> = schedules
			.into_iter()
			.map(|schedule_info| {
				VestingScheduleNonce::<T>::increment()
					.map(|id| (id, VestingSchedule::from_input(id, schedule_info)))
			})
			.collect::<Result<BTreeMap<T::VestingScheduleId, VestingScheduleOf<T>>, _>>()?
			.try_into()
			.map_err(|_| Error::<T>::MaxVestingSchedulesExceeded)?;

		let total_amount =
			bounded_schedules.iter().try_fold::<_, _, Result<BalanceOf<T>, DispatchError>>(
				Zero::zero(),
				|acc_amount, (_, schedule)| {
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

	let total_total = schedule.total_amount()?;

	ensure!(total_total >= T::MinVestedTransfer::get(), Error::<T>::AmountLow);

	Ok(total_total)
}
