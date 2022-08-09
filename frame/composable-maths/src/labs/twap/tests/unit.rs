use crate::labs::twap::Twap;
use rstest::rstest;
use sp_runtime::{ArithmeticError, FixedU128};

#[rstest]
#[case(FixedU128::Inner::MIN, i64::MIN)]
fn should_create_twap_struct_successfully(#[case] twap: FixedU128, #[case] ts: i64) {
	let t = Twap::new(&twap, &ts);
	assert_eq!(t.twap, twap);
	assert_eq!(t.ts, ts);
	assert_eq!(t, Twap { twap, ts })
}
