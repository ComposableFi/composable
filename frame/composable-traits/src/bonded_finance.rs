use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::{CheckedAdd, Zero};

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
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;
}

/// The Bond duration.
/// Can either be finite or infinite, infinite representing the protocol owned liquidity case.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "proptest-support", derive(proptest_derive::Arbitrary))]
pub enum BondDuration<BlockNumber> {
	Finite { blocks: BlockNumber },
	Infinite,
}

/// The Bond offer.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "proptest-support", derive(proptest_derive::Arbitrary))]
pub struct BondOffer<AssetId, Balance, BlockNumber> {
	/// Asset to be locked. Unlockable after `duration`.
	pub asset: AssetId,
	/// Total amount of the asset to be locked.
	pub amount: Balance,
	/// Duration for which the asset has to be locked.
	pub duration: BondDuration<BlockNumber>,
	/// Asset given as reward.
	pub reward_asset: AssetId,
	/// Total reward.
	pub reward_amount: Balance,
	/// Duration after which the reward can be claimed.
	pub reward_duration: BlockNumber,
}

impl<AssetId: PartialEq, Balance: Zero + CheckedAdd + PartialOrd, BlockNumber: Zero>
	BondOffer<AssetId, Balance, BlockNumber>
{
	pub fn valid(&self, min_offer: Balance, min_reward: Balance) -> bool {
		let valid_duration = match &self.duration {
			BondDuration::Finite { blocks } => !blocks.is_zero(),
			BondDuration::Infinite => true,
		};
		let valid_reward_duration = !self.reward_duration.is_zero();
		valid_duration &&
			valid_reward_duration &&
			self.reward_amount >= min_reward &&
			self.amount >= min_offer
	}
}
