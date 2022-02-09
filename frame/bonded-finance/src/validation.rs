use composable_support::validation::{Valid, Validate};
use composable_traits::{
	bonded_finance::{BondDuration, BondOffer},
	math::SafeArithmetic,
};
use core::marker::PhantomData;
use sp_runtime::traits::Zero;

pub type CheckValidBondOfferTag<T> = (ValidBondOffer<T>, Valid);

#[derive(Debug, Eq, PartialEq)]
pub struct ValidBondOffer<T> {
	phantom: PhantomData<T>,
}

pub trait BondOfferComparer<T> {
	fn min_transfer() -> T;
	fn min_reward() -> T;
}

impl< AccountId,
		AssetId,
		Balance: PartialOrd + Zero + SafeArithmetic + From<u64>,
		BlockNumber: Zero,
	> Validate<BondOffer<AccountId, AssetId, Balance, BlockNumber>, ValidBondOffer<Balance>>
	for ValidBondOffer<Balance>
where
	ValidBondOffer<Balance>: BondOfferComparer<Balance>,
{
	fn validate(
		input: BondOffer<AccountId, AssetId, Balance, BlockNumber>,
	) -> Result<BondOffer<AccountId, AssetId, Balance, BlockNumber>, &'static str> {
		let nonzero_maturity = match &input.maturity {
			BondDuration::Finite { return_in } => !return_in.is_zero(),
			BondDuration::Infinite => true,
		};

		if nonzero_maturity == false {
			return Err("invalid maturity")
		}

		if input.bond_price < ValidBondOffer::<Balance>::min_transfer() {
			return Err("invalid bond_price")
		}

		if input.nb_of_bonds.is_zero() {
			return Err("invalid nb_of_bonds")
		}

		let valid_reward = input.reward.amount >= ValidBondOffer::<Balance>::min_reward() &&
			input
				.reward
				.amount
				.safe_div(&input.nb_of_bonds)
				.unwrap_or_else(|_| Balance::zero()) >=
				ValidBondOffer::min_transfer();

		if !valid_reward {
			return Err("invalid reward")
		}

		if input.reward.maturity.is_zero() {
			return Err("invalid reward_maturity")
		}

		if !input.total_price().is_ok() {
			return Err("invalid total_price")
		}

		Ok(input)
	}
}
