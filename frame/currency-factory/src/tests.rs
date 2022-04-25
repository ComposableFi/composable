use crate::mocks::*;
use composable_tests_helpers::prop_assert_ok;
use composable_traits::{
	currency::{CurrencyFactory, RangeId},
	xcm::Balance,
};
use proptest::prelude::*;

prop_compose! {
	fn valid_ranges()
		(x in 0..u64::MAX) -> u64 {
		x
	}
}

prop_compose! {
  fn valid_range_ids()
		(x in 0..3_u32) -> u32 {
		x
	}
}

prop_compose! {
  fn increments()
		(x in 0..256_u32) -> u32 {
		x
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10000))]

	#[test]
	fn add_ranges(
		range in valid_ranges(),
	) {
		new_test_ext().execute_with(|| {
			prop_assert_ok!(CurrencyRanges::add_range(Origin::root(), range));
			Ok(())
		})?;
	}

	#[test]
	fn create(
		range in valid_range_ids(),
	) {
		new_test_ext().execute_with(|| {
			for _ in 0..30 {
				let res = <CurrencyRanges as CurrencyFactory<AssetId, Balance>>::create(RangeId::from(range), 42);
				prop_assert_ok!(res);

			}
			Ok(())
		})?;
	}

	#[test]
	fn create_monotonic_increment(
		i in increments(),
	) {
		new_test_ext().execute_with(|| {
			let mut prev = None;
			for _ in 0..i {
				let res = <CurrencyRanges as CurrencyFactory<AssetId, Balance>>::create(RangeId::TOKENS, 42);
				prop_assert_ok!(res);
				if let Some(prev) = prev {
					prop_assert_eq!(prev + 1, res.unwrap())
				}
				prev = Some(res.unwrap())
			}
			Ok(())
		})?;
	}
}
