use composable_support::validation::{Validate};
use composable_traits::{
	bonded_finance::{BondDuration},
	math::SafeArithmetic,
};
use codec::{Decode};
use scale_info::{TypeInfo};
use scale_info;
use core::marker::PhantomData;
use sp_runtime::traits::Zero;
use crate::{pallet::BalanceOf, Config, BondOfferOf};

#[derive(Debug,  Clone, Copy, Decode, TypeInfo)]
pub struct ValidBondOffer<T> {
	phantom: PhantomData<T>,
}

pub trait BondOfferComparer<T> {
	fn min_transfer() -> T;
	fn min_reward() -> T;
}

impl<T: Config > Validate<BondOfferOf<T>, ValidBondOffer<T>> for ValidBondOffer<T> where
	ValidBondOffer<T>: BondOfferComparer<BalanceOf<T>> + Decode{

	fn validate(input: BondOfferOf<T>) -> Result<BondOfferOf<T>, &'static str> {
		
        let nonzero_maturity = match &input.maturity {
			BondDuration::Finite { return_in } => !return_in.is_zero(),
			BondDuration::Infinite => true,
		};

		if nonzero_maturity == false {
			return Err("invalid maturity")
		}

		if input.bond_price < ValidBondOffer::<T>::min_transfer() {
			return Err("invalid bond_price")
		}

		if input.nb_of_bonds.is_zero() {
			return Err("invalid nb_of_bonds")
		}

		let valid_reward = input.reward.amount >= ValidBondOffer::<T>::min_reward() &&
			input
				.reward
				.amount
				.safe_div(&input.nb_of_bonds)
				.unwrap_or_else(|_| BalanceOf::<T>::zero()) >=
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