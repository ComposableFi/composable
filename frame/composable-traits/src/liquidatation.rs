use frame_support::{pallet_prelude::*, sp_runtime::Permill};

#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct LiquidationConfig {
	pub liquidation_fee: Permill,
}

pub trait Liquidation {
	type PairId: core::cmp::Ord;
	type AccountId: core::cmp::Ord;
	type Error;
	type Balance;

	fn update_config(config: &LiquidationConfig) {
		todo!()
	}

	fn liquidate(pair: Self::PairId, borrower: &Self::AccountId) -> Result<(), Self::Error> {
		todo!()
	}

	fn calculate_liquidation_fee(
		amount: Self::Balance,
		config: &LiquidationConfig,
	) -> Self::Balance {
		todo!()
	}

	fn get_liquidation_risk(pair: Self::PairId) -> Self::Balance {
		todo!()
	}
}
