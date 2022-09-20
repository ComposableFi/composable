use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_runtime::traits::Zero;
use sp_std::num::NonZeroU64;

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub enum Reward<Balance> {
	ProtocolDistribution(),
	RateBased(RateBasedReward<Balance>),
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub enum RewardConfig<Balance> {
	ProtocolDistribution(),
	RateBased(RateBasedConfig<Balance>),
}

impl<Balance: Zero> Reward<Balance> {
	pub fn from_config(reward_config: RewardConfig<Balance>, now_seconds: u64) -> Self {
		match reward_config {
			RewardConfig::ProtocolDistribution() => Reward::ProtocolDistribution(),
			RewardConfig::RateBased(rate_based_config) =>
				Reward::RateBased(RateBasedReward::from_config(rate_based_config, now_seconds)),
		}
	}
}

/// Defines staking duration, rewards and early unstake penalty for a given asset type.
/// TODO refer to the relevant section in the design doc.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
pub struct RateBasedReward<Balance> {
	/// Total rewards including inflation for adjusting for new stakers joining the pool. All
	/// stakers in a pool are eligible to receive a part of this value based on their share of
	/// the pool.
	pub total_rewards: Balance,

	/// Already claimed rewards by stakers by unstaking.
	pub claimed_rewards: Balance,

	/// A book keeping field to track the actual total reward without the reward dilution
	/// adjustment caused by new stakers joining the pool.
	///
	/// This field is the same as the sum of all of the reductions of all of the stakes in the
	/// pool.
	pub total_dilution_adjustment: Balance,

	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,

	/// The last time the reward was updated, in seconds.
	pub last_updated_timestamp: u64,
}

/// Reward configurations for a given asset type.
#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RateBasedConfig<Balance> {
	/// Upper bound on the `total_rewards - total_dilution_adjustment`.
	pub max_rewards: Balance,

	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub struct RewardRate<Balance> {
	/// The period that the rewards are handed out in.
	pub period: RewardRatePeriod,
	/// The amount that is rewarded each period.
	pub amount: Balance,
}

impl<Balance> RewardRate<Balance> {
	pub fn per_second<B: Into<Balance>>(amount: B) -> Self {
		Self { period: RewardRatePeriod::PerSecond, amount: amount.into() }
	}
}

#[derive(RuntimeDebug, PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo)]
pub enum RewardRatePeriod {
	PerSecond,
}

impl RewardRatePeriod {
	/// Returns the length of the period in seconds.
	pub fn as_secs(&self) -> NonZeroU64 {
		match self {
			RewardRatePeriod::PerSecond => NonZeroU64::new(1).expect("1 is non-zero; qed;"),
		}
	}
}

/// A reward update states the new reward and reward_rate for a given asset
// REVIEW(benluelo): Make this an enum with variants per reward type?
#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
pub struct RewardUpdate<Balance> {
	/// The rewarding rate that increases the pool `total_reward`
	/// at a given time.
	pub reward_rate: RewardRate<Balance>,
}

impl<Balance: Zero> RateBasedReward<Balance> {
	pub fn from_config(
		reward_config: RateBasedConfig<Balance>,
		now_seconds: u64,
	) -> RateBasedReward<Balance> {
		RateBasedReward {
			total_rewards: Zero::zero(),
			claimed_rewards: Zero::zero(),
			total_dilution_adjustment: Zero::zero(),
			max_rewards: reward_config.max_rewards,
			reward_rate: reward_config.reward_rate,
			last_updated_timestamp: now_seconds,
		}
	}
}
