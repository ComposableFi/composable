use composable_support::{
	math::safe::{SafeDiv, SafeMul},
	validation::{Validate, Validated},
};
use frame_support::{pallet_prelude::*, traits::Get};
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

	/// Create a new offer.
	fn offer(
		from: &Self::AccountId,
		offer: Validated<
			BondOffer<Self::AccountId, Self::AssetId, Self::Balance, Self::BlockNumber>,
			ValidBondOffer<Self::MinReward, Self::MinVestedTransfer>,
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

#[derive(Debug, Decode)]
pub struct ValidBondOffer<U, V> {
	_marker: PhantomData<(U, V)>,
}

impl<U, V> Copy for ValidBondOffer<U, V> {}

impl<U, V> Clone for ValidBondOffer<U, V> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<
		MinTransfer,
		MinReward,
		AccountId,
		AssetId,
		Balance: Zero + PartialOrd + SafeDiv + SafeMul,
		BlockNumber: Zero,
	>
	Validate<
		BondOffer<AccountId, AssetId, Balance, BlockNumber>,
		ValidBondOffer<MinTransfer, MinReward>,
	> for ValidBondOffer<MinTransfer, MinReward>
where
	ValidBondOffer<MinTransfer, MinReward>: Decode,
	MinTransfer: Get<Balance>,
	MinReward: Get<Balance>,
{
	fn validate(
		input: BondOffer<AccountId, AssetId, Balance, BlockNumber>,
	) -> Result<BondOffer<AccountId, AssetId, Balance, BlockNumber>, &'static str> {
		let nonzero_maturity = match &input.maturity {
			BondDuration::Finite { return_in } => !return_in.is_zero(),
			BondDuration::Infinite => true,
		};

		if !nonzero_maturity {
			return Err("MATURITY_CANNOT_BE_ZERO")
		}

		if input.bond_price < MinTransfer::get() {
			return Err("BOND_PRICE_BELOW_MIN_TRANSFER")
		}

		if input.nb_of_bonds.is_zero() {
			return Err("NUMBER_OF_BOND_CANNOT_BE_ZERO")
		}

		let valid_reward = input.reward.amount >= MinReward::get() &&
			input
				.reward
				.amount
				.safe_div(&input.nb_of_bonds)
				.unwrap_or_else(|_| Balance::zero()) >=
				MinTransfer::get();

		if !valid_reward {
			return Err("INVALID_REWARD")
		}

		if input.reward.maturity.is_zero() {
			return Err("ZERO_REWARD_MATURITY")
		}

		if input.total_price().is_err() {
			return Err("INVALID_TOTAL_PRICE")
		}

		Ok(input)
	}
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
