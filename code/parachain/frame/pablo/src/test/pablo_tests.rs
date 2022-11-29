use crate::{
	mock::{Pablo, *},
	test::common_test_functions::*,
};
use frame_support::assert_ok;
use sp_runtime::Permill;

mod create {

	use crate::PoolInitConfigurationOf;

	use super::*;

	#[test]
	fn should_successfully_create_50_50_pool() {
		new_test_ext().execute_with(|| {
			let owner = ALICE;
			let assets_weights = dual_asset_pool_weights(USDC, Permill::from_percent(50), USDT);
			let fee = Permill::from_percent(1);
			let pool_config = PoolInitConfigurationOf::<Test>::DualAssetConstantProduct {
				owner,
				assets_weights,
				fee,
			};

			assert_ok!(Pablo::do_create_pool(pool_config));
		});
	}
}
