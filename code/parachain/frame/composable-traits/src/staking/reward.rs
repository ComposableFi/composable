use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_runtime::traits::Zero;

use self::rate_based::{RateBasedConfig, RateBasedReward};

pub mod rate_based;

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
