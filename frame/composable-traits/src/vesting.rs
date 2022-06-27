use codec::{HasCompact, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::AtLeast32Bit;

/// An object allowing us to transfer funds from one account to another in a vested fashion.
pub trait VestedTransfer {
	type AccountId;
	type AssetId;
	type BlockNumber;
	type Moment;
	type Balance: HasCompact;
	type MinVestedTransfer: Get<Self::Balance>;

	/// Transfer `asset` from `from` to `to` vested based on `schedule`.
	fn vested_transfer(
		asset: Self::AssetId,
		from: &Self::AccountId,
		to: &Self::AccountId,
		schedule: VestingSchedule<Self::BlockNumber, Self::Moment, Self::Balance>,
	) -> DispatchResult;
}

/// Vesting window type for the vesting schedules.
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum VestingWindow<BlockNumber, Moment> {
	MomentBased {
		/// Vesting start
		start: Moment,
		/// Number of moments between vest
		period: Moment,
	},
	BlockNumberBased {
		/// Vesting start
		start: BlockNumber,
		/// Number of blocks between vest
		period: BlockNumber,
	},
}

/// The vesting schedule.
///
/// Benefits would be granted gradually, `per_period` amount every `window.period`
/// of blocks after `window.start`.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct VestingSchedule<BlockNumber, Moment, Balance: HasCompact> {
	pub window: VestingWindow<BlockNumber, Moment>,
	/// Number of vest
	pub period_count: u32,
	/// Amount of tokens to release per vest
	#[codec(compact)]
	pub per_period: Balance,
}

pub enum VestingWindowResult<BlockNumber, Moment> {
	MomentResult(Moment),
	BlockNumberResult(BlockNumber),
}

impl<
		BlockNumber: AtLeast32Bit + Copy,
		Moment: AtLeast32Bit + Copy,
		Balance: AtLeast32Bit + Copy,
	> VestingSchedule<BlockNumber, Moment, Balance>
{
	/// Check if the period is zero
	pub fn is_zero_period(&self) -> bool {
		match self.window {
			VestingWindow::BlockNumberBased { start: _, period } => period.is_zero(),
			VestingWindow::MomentBased { start: _, period } => period.is_zero(),
		}
	}

	/// Returns the end of all periods, `None` if calculation overflows.
	pub fn end(&self) -> Option<VestingWindowResult<BlockNumber, Moment>> {
		// period * period_count + start
		match self.window {
			VestingWindow::BlockNumberBased { start, period } => period
				.checked_mul(&self.period_count.into())?
				.checked_add(&start)
				.map(|val| VestingWindowResult::<BlockNumber, Moment>::BlockNumberResult(val)),
			VestingWindow::MomentBased { start, period } => period
				.checked_mul(&self.period_count.into())?
				.checked_add(&start)
				.map(|val| VestingWindowResult::<BlockNumber, Moment>::MomentResult(val)),
		}
	}

	/// Returns all locked amount, `None` if calculation overflows.
	pub fn total_amount(&self) -> Option<Balance> {
		self.per_period.checked_mul(&self.period_count.into())
	}

	/// Returns locked amount for a given schedule of VestingWindow.
	///
	/// Note this func assumes schedule is a valid one(non-zero period and
	/// non-overflow total amount), and it should be guaranteed by callers.
	pub fn locked_amount(&self, block_number: BlockNumber, moment: Moment) -> Balance {
		// full = (time - start) / period
		// unrealized = period_count - full
		// per_period * unrealized
		let unrealized = match self.window {
			VestingWindow::BlockNumberBased { start, period } => {
				let full = block_number
					.saturating_sub(start)
					.checked_div(&period)
					.expect("ensured non-zero period; qed");
				self.period_count.saturating_sub(full.unique_saturated_into())
			},
			VestingWindow::MomentBased { start, period } => {
				let full = moment
					.saturating_sub(start)
					.checked_div(&period)
					.expect("ensured non-zero period; qed");
				self.period_count.saturating_sub(full.unique_saturated_into())
			},
		};
		self.per_period
			.checked_mul(&unrealized.into())
			.expect("ensured non-overflow total amount; qed")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::vesting::VestingWindow::{BlockNumberBased, MomentBased};

	#[test]
	fn test_is_zero_period() {
		let mut vesting_schedule_time_based = VestingSchedule::<u32, u64, u64> {
			window: MomentBased { start: 1u64, period: 10u64 },
			period_count: 100,
			per_period: 1u64,
		};
		assert!(!vesting_schedule_time_based.is_zero_period());
		vesting_schedule_time_based.window = MomentBased { start: 1, period: 0 };
		assert!(vesting_schedule_time_based.is_zero_period());

		let mut vesting_schedule_block_number_based = VestingSchedule::<u64, u32, u64> {
			window: BlockNumberBased { start: 1u64, period: 10u64 },
			period_count: 100,
			per_period: 1u64,
		};
		assert!(!vesting_schedule_block_number_based.is_zero_period());
		vesting_schedule_block_number_based.window = BlockNumberBased { start: 1, period: 0 };
		assert!(vesting_schedule_block_number_based.is_zero_period());
	}

	#[test]
	fn test_end() {
		let vesting_schedule_time_based = VestingSchedule::<u32, u64, u64> {
			window: MomentBased { start: 1u64, period: 10u64 },
			period_count: 100,
			per_period: 1u64,
		};
		match vesting_schedule_time_based.end() {
			None => {},
			Some(result) => match result {
				VestingWindowResult::MomentResult(val) => assert_eq!(val, 1001),
				VestingWindowResult::BlockNumberResult(_) => panic!("Unexpected BlockNumberResult"),
			},
		}
		let vesting_schedule_block_number_based = VestingSchedule::<u64, u32, u64> {
			window: BlockNumberBased { start: 1u64, period: 10u64 },
			period_count: 100,
			per_period: 1u64,
		};
		match vesting_schedule_block_number_based.end() {
			None => {},
			Some(result) => match result {
				VestingWindowResult::MomentResult(_) => panic!("Unexpected MomentResult"),
				VestingWindowResult::BlockNumberResult(val) => assert_eq!(val, 1001),
			},
		}
	}

	#[test]
	fn test_total_amount() {
		let vesting_schedule = VestingSchedule::<u64, u64, u64> {
			window: BlockNumberBased { start: 1u64, period: 10u64 },
			period_count: 100,
			per_period: 1u64,
		};
		assert_eq!(vesting_schedule.total_amount().unwrap(), 100)
	}

	/// TODO proptest for exhastive tests
	#[test]
	fn test_locked_amount() {
		let vesting_schedule_time_based = VestingSchedule::<u32, u64, u64> {
			window: MomentBased { start: 1u64, period: 10u64 },
			period_count: 100,
			per_period: 1u64,
		};
		assert_eq!(vesting_schedule_time_based.locked_amount(1, 1), 100);
		assert_eq!(vesting_schedule_time_based.locked_amount(1, 11), 99);
		assert_eq!(vesting_schedule_time_based.locked_amount(1, 1001), 0);

		let vesting_schedule_block_number_based = VestingSchedule::<u64, u32, u64> {
			window: BlockNumberBased { start: 1u64, period: 10u64 },
			period_count: 100,
			per_period: 1u64,
		};
		assert_eq!(vesting_schedule_block_number_based.locked_amount(1, 1), 100);
		assert_eq!(vesting_schedule_block_number_based.locked_amount(11, 1), 99);
		assert_eq!(vesting_schedule_block_number_based.locked_amount(1001, 1), 0);
	}
}
