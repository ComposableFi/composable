use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, ArithmeticError};

use crate::math::SafeArithmetic;

pub trait BondedFinance {
	type AccountId;
	type AssetId;
	type Balance;
	type BlockNumber;
	type BondOfferId;

	/// Create a new offer.
	fn offer(
		from: &Self::AccountId,
		offer: BondOffer<Self::AssetId, Self::Balance, Self::BlockNumber>,
	) -> Result<Self::BondOfferId, DispatchError>;

	/// Bond for an offer.
	fn bond(
		offer: Self::BondOfferId,
		from: &Self::AccountId,
		parts: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;
}

/// The Bond duration.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum BondDuration<BlockNumber> {
	/// Finite duration, liquidity is returned after a number of `blocks`.
	Finite { blocks: BlockNumber },
	/// Infinite duration, the protocol is now owning the liquidity
	Infinite,
}

/// The Bond offer.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct BondOffer<AssetId, Balance, BlockNumber> {
	/// Asset to be locked. Unlockable after `duration`.
	pub asset: AssetId,
	/// Price of a bond.
	pub price: Balance,
	/// Number of bonds. We use the Balance type for the sake of simplicity.
	pub parts: Balance,
	/// Duration for which the asset has to be locked.
	pub duration: BondDuration<BlockNumber>,
	/// Asset given as reward.
	pub reward_asset: AssetId,
	/// Total reward.
	pub reward_amount: Balance,
	/// Duration after which the reward can be claimed.
	pub reward_duration: BlockNumber,
}

impl<AssetId, Balance: Zero + PartialOrd + SafeArithmetic, BlockNumber: Zero>
	BondOffer<AssetId, Balance, BlockNumber>
{
	pub fn total_price(&self) -> Result<Balance, ArithmeticError> {
		self.parts.safe_mul(&self.price)
	}
	pub fn valid(&self, min_price: Balance, min_reward: Balance) -> bool {
		let valid_duration = match &self.duration {
			BondDuration::Finite { blocks } => !blocks.is_zero(),
			BondDuration::Infinite => true,
		};
		let valid_price = self.price >= min_price;
		let positive_parts = !self.parts.is_zero();
		let valid_reward = self.reward_amount >= min_reward;
		let positive_reward_duration = !self.reward_duration.is_zero();
		let valid_total = self.total_price().is_ok();
		valid_duration &&
			positive_parts &&
			valid_price && positive_reward_duration &&
			valid_reward && valid_total
	}
}
