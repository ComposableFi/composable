pub use crate::{
	mock::runtime::{ClearingHouse, ExtBuilder, Origin, Runtime, Tokens, ALICE, PICA, USDC},
	pallet::*,
};
use frame_support::assert_err;
use orml_tokens::Error as TokenError;

#[test]
fn test_add_margin_returns_transfer_error() {
	ExtBuilder::default().build().execute_with(|| {
		let origin = Origin::signed(ALICE);
		assert_err!(
			ClearingHouse::add_margin(origin, USDC, 1_000u32.into()),
			TokenError::<Runtime>::BalanceTooLow
		);
	});
}

#[test]
fn test_deposit_unsupported_collateral_returns_error() {
	ExtBuilder::default().build().execute_with(|| {
		let origin = Origin::signed(ALICE);
		assert_err!(
			ClearingHouse::add_margin(origin, PICA, 1_000u32.into()),
			Error::<Runtime>::UnsupportedCollateralType
		)
	});
}
