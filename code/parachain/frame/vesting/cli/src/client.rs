use sp_core::{ConstU16, ConstU32};
use sp_runtime::BoundedBTreeMap;
use subxt::utils::AccountId32;

// raw types from subxt
use crate::prelude::*;

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, Serialize, Deserialize)]
pub struct VestingSchedule<VestingScheduleId, BlockNumber, Moment, Balance: HasCompact> {
	/// Vesting schedule id
	pub vesting_schedule_id: VestingScheduleId,
	pub window: VestingWindow<BlockNumber, Moment>,
	/// Number of vest
	pub period_count: u32,
	/// Amount of tokens to release per vest
	#[codec(compact)]
	pub per_period: Balance,
	/// Amount already claimed
	pub already_claimed: Balance,
}

#[derive(
	Clone,
	Encode,
	Decode,
	PartialEq,
	Eq,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
	Serialize,
	Deserialize,
)]
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

pub type VestingScheduleT =
	BoundedBTreeMap<u128, VestingSchedule<u128, u32, u64, u128>, ConstU32<32>>;

pub type VestingScheduleKeyT = ([u8; 32], u128 , AccountId32, [u8; 32], );

