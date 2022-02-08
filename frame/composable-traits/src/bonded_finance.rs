use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, ArithmeticError};
use composable_support::{validation::{Validate, Valid} };

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
		offer: BondOffer<Self::AccountId, Self::AssetId, Self::Balance, Self::BlockNumber>,
	) -> Result<Self::BondOfferId, DispatchError>;

	/// Bond for an offer.
	fn bond(
		offer: Self::BondOfferId,
		from: &Self::AccountId,
		nb_of_bonds: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;
}

/// The Bond duration.
#[derive(Clone, Encode, Decode, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub enum BondDuration<BlockNumber> {
	/// Finite duration, liquidity is returned after a number of `blocks`.
	Finite { return_in: BlockNumber },
	/// Infinite duration, the protocol is now owning the liquidity
	Infinite,
}

/// The Bond offer.
#[derive(Clone, Encode, Decode, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct BondOffer<AccountId, AssetId, Balance, BlockNumber> {
	/// The account that will receive the locked assets.
	pub beneficiary: AccountId,
	/// Asset to be locked. Unlockable after `duration`.
	/// Asset which `beneficiary` wants to get for his offer.
	pub asset: AssetId,
	/// Price of a bond unit in `asset`.
	pub bond_price: Balance,
	/// Number of bonds. We use the Balance type for the sake of simplicity.
	pub nb_of_bonds: Balance,
	/// Duration for which the asset has to be locked.
	pub maturity: BondDuration<BlockNumber>,
	/// Total reward for this offer.
	pub reward: BondOfferReward<AssetId, Balance, BlockNumber>,
}

/// The Bond reward. Asset and rules reward will be given.
#[derive(Clone, Encode, Decode, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct BondOfferReward<AssetId, Balance, BlockNumber> {
	/// The actual reward asset.
	pub asset: AssetId,
	/// Total reward.
	pub amount: Balance,
	/// Duration after which the reward can be claimed.
	pub maturity: BlockNumber,
}

  
// #[cfg(test)]
// mod test {
// 	use super::*;
// 	use codec::{Decode, Encode};
// 	use frame_support::assert_ok;

// 	impl<Balance: From<u64>> ValidBondOfferTrait<Balance> for ValidBondOffer<u64> {
// 		fn min_transfer() -> Balance {
// 			Balance::from(10_u64)
// 		}
// 		fn min_reward() -> Balance {
// 			Balance::from(20_u64)
// 		}
// 	}

// 	#[test]
// 	fn test_validate_bond_offer() {
// 		let invalid = BondOffer::<u64, u64, u64, u64> { 
// 			beneficiary : 23_u64,
// 			asset: 10_u64,
// 			bond_price: 12_u64,
// 			nb_of_bonds: 30_u64,
// 			maturity: BondDuration::Finite { return_in: 1 },
// 			reward: BondOfferReward {
// 				 asset: 2_u64,
// 				 amount: 20,
// 				 maturity: 45_u64
// 			}

// 		};

// 		assert!(<CheckValidBondOfferTag as Validate<
// 			BondOffer<u64, u64, u64, u64>,
// 			CheckValidBondOfferTag,
// 		>>::validate(invalid)
// 		.is_err());

// 		let valid = BondOffer::<u64, u64, u64, u64> { 
// 			beneficiary : 23_u64,
// 			asset: 10_u64,
// 			bond_price: 12_u64,
// 			nb_of_bonds: 30_u64,
// 			maturity: BondDuration::Finite { return_in: 1 },
// 			reward: BondOfferReward {
// 				 asset: 2_u64,
// 				 amount: 2000_u64,
// 				 maturity: 45_u64
// 			}

// 		};
// 		assert_ok!(
// 			<CheckValidBondOfferTag as Validate<BondOffer<u64, u64, u64, u64>, CheckValidBondOfferTag>>::validate(
// 				valid
// 			)
// 		);
// 	}

// }

impl<AccountId, AssetId, Balance: Zero + PartialOrd + SafeArithmetic, BlockNumber: Zero>
	BondOffer<AccountId, AssetId, Balance, BlockNumber>
{
	/// An offer is completed once all it's nb_of_bonds has been sold.
	pub fn completed(&self) -> bool {
		self.nb_of_bonds.is_zero()
	}
	/// The total price of the offer, which is the number of nb_of_bonds * the bond_price.
	pub fn total_price(&self) -> Result<Balance, ArithmeticError> {
		self.nb_of_bonds.safe_mul(&self.bond_price)
	}
	/// Check whether an offer is valid and can be submitted.
	pub fn valid(&self, min_transfer: Balance, min_reward: Balance) -> bool {
		let nonzero_maturity = match &self.maturity {
			BondDuration::Finite { return_in } => !return_in.is_zero(),
			BondDuration::Infinite => true,
		};
		let valid_price = self.bond_price >= min_transfer;
		let nonzero_nb_of_bonds = !self.nb_of_bonds.is_zero();
		let valid_reward = self.reward.amount >= min_reward &&
			self.reward
				.amount
				.safe_div(&self.nb_of_bonds)
				.unwrap_or_else(|_| Balance::zero()) >=
				min_transfer;
		let nonzero_reward_maturity = !self.reward.maturity.is_zero();
		let valid_total = self.total_price().is_ok();
		nonzero_maturity &&
			nonzero_nb_of_bonds &&
			valid_price && nonzero_reward_maturity &&
			valid_reward && valid_total
	}
}
