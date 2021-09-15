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

	fn update_config(_config: &LiquidationConfig) {
		todo!()
	}

	fn liquidate(_pair: Self::PairId, _borrower: &Self::AccountId) -> Result<(), Self::Error> {
		todo!()
	}

	fn calculate_liquidation_fee(
		_amount: Self::Balance,
		_config: &LiquidationConfig,
	) -> Self::Balance {
		todo!()
	}

	fn get_liquidation_risk(_pair: Self::PairId) -> Self::Balance {
		todo!()
	}
}
