use crate::{integrations::mock::ExtBuilder, mock::assets::USDC};
use sp_arithmetic::FixedI128;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_type: Some(USDC),
			oracle_asset_support: Some(true),
			oracle_price: Some(10_000),
			oracle_twap: Some(10_000),
			max_price_divergence: FixedI128::from_inner(i128::MAX),
		}
	}
}

#[test]
fn externalities_builder_works() {
	ExtBuilder::default().build().execute_with(|| {});
}

// ----------------------------------------------------------------------------------------------------
//                                         Open position
// ----------------------------------------------------------------------------------------------------
