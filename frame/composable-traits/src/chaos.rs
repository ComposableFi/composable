use crate::{
	financial_nft::{NFTClass, NFTVersion},
	math::{SafeAdd, SafeSub},
	time::Timestamp,
};
use codec::{Decode, Encode};
use core::fmt::Debug;
use frame_support::{dispatch::DispatchResult, traits::Get};
use scale_info::TypeInfo;
use sp_runtime::{traits::AtLeast32BitUnsigned, DispatchError, Permill};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct ChaosStakingNFT<Balance, Indexes> {
	/// The stake this NFT was minted for.
	pub stake: Balance,
	/// The indexes at which this NFT was minted, used to compute the rewards.
	pub reward_indexes: Indexes,
	/// The date at which this NFT was minted.
	pub lock_date: Timestamp,
	/// The duration for which this NFT stake was locked.
	pub lock_duration: Timestamp,
}

impl<Balance: AtLeast32BitUnsigned + Copy, Indexes> ChaosStakingNFT<Balance, Indexes> {
	pub fn penalize_early_unstake_amount(
		&self,
		now: Timestamp,
		penalty: Permill,
	) -> Result<(Balance, Balance), DispatchError> {
		if self.lock_date.safe_add(&self.lock_duration)? <= now {
			Ok((self.stake, Balance::zero()))
		} else {
			let penalty_amount = penalty.mul_floor(self.stake);
			let penalized_amount = self.stake.safe_sub(&penalty_amount)?;
			Ok((penalized_amount, penalty_amount))
		}
	}
}

impl<Balance, Indexes> Get<NFTClass> for ChaosStakingNFT<Balance, Indexes> {
	fn get() -> NFTClass {
		NFTClass::CHAOS_STAKING
	}
}

impl<Balance, Indexes> Get<NFTVersion> for ChaosStakingNFT<Balance, Indexes> {
	fn get() -> NFTVersion {
		NFTVersion::VERSION_1
	}
}

/// Interface for the Chaos protocol.
pub trait ChaosProtocol {
	type AccountId;
	type AssetId;
	type Balance;
	type InstanceId;

	/// Stake an amount of Chaos in the protocol. A new NFT representing the user position will be
	/// minted.
	///
	/// Arguments
	///
	/// * `from` the account to transfer the Chaos stake from.
	/// * `amount` the amount of Chaos to stake.
	/// * `duration` the staking duration (must be one of the predefined presets). Unstaking before
	///   the end trigger the unstake penalty.
	/// * `keep_alive` whether to keep the `from` account alive or not while transferring the Chaos
	///   stake.
	fn stake(
		from: &Self::AccountId,
		amount: Self::Balance,
		duration: Timestamp,
		keep_alive: bool,
	) -> Result<Self::InstanceId, DispatchError>;

	/// Unstake an actual staked position, represented by a NFT.
	///
	/// Arguments
	///
	/// * `to` the account to transfer the final claimed rewards to.
	/// * `instance_id` the ID uniquely identifiying the NFT from which we will compute the
	///   available rewards.
	fn unstake(to: &Self::AccountId, instance_id: &Self::InstanceId) -> DispatchResult;

	/// Claim the current rewards.
	///
	/// Arguments
	///
	/// * `to` the account to transfer the rewards to.
	/// * `instance_id` the ID uniquely identifiying the NFT from which we will compute the
	///   available rewards.
	fn claim(to: &Self::AccountId, instance_id: &Self::InstanceId) -> DispatchResult;

	/// Transfer a reward to the Chaos protocol.
	///
	/// Arguments
	///
	/// * `from` the account to transfer the reward from.
	/// * `asset` the reward asset to transfer.
	/// * `amount` the amount of reward to transfer.
	/// * `keep_alive` whether to keep alive or not the `from` account while transferring the
	///   reward.
	fn transfer_reward(
		from: &Self::AccountId,
		asset: Self::AssetId,
		amount: Self::Balance,
		keep_alive: bool,
	) -> DispatchResult;
}
