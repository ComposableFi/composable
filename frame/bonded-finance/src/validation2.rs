use composable_support::{
	math::safe::{SafeDiv, SafeMul},
	validation2::Validate,
};
use composable_traits::bonded_finance::{BondDuration, BondOffer};
use frame_support::pallet_prelude::*;
use sp_runtime::traits::Zero;

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
		&'static str,
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
