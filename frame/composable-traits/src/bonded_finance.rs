use composable_support::{math::safe::SafeMul, validation2::Validated};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, ArithmeticError};

pub trait BondedFinance {
	type AccountId;
	type AssetId;
	type Balance;
	type BlockNumber;
	type BondOfferId;
	type MinReward;
	type MinVestedTransfer;
	type ValidateBondOffer;

	/// Create a new offer.
	fn offer(
		from: &Self::AccountId,
		offer: Validated<
			BondOffer<Self::AccountId, Self::AssetId, Self::Balance, Self::BlockNumber>,
			Self::ValidateBondOffer,
			&'static str,
		>,
		keep_alive: bool,
	) -> Result<Self::BondOfferId, DispatchError>;

	/// Bond for an offer.
	fn bond(
		offer: Self::BondOfferId,
		from: &Self::AccountId,
		nb_of_bonds: Self::Balance,
		keep_alive: bool,
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
	/// Asset to be locked. Unlockable after `maturity`.
	/// Asset which `beneficiary` wants to get for their offer.
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

impl<AccountId, AssetId, Balance: Zero + PartialOrd + SafeMul, BlockNumber: Zero>
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
}
