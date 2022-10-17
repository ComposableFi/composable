use crate::mocks::*;
use composable_tests_helpers::prop_assert_ok;
use composable_traits::currency::{CurrencyFactory, RangeId};
use proptest::prelude::*;
use sp_runtime::DispatchError;

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
				let res = <CurrencyRanges as CurrencyFactory>::create(RangeId::from(range), 42);
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
				let res = <CurrencyRanges as CurrencyFactory>::create(RangeId::TOKENS, 42);
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

mod protocol_asset_id_to_unique_asset_id {
	use super::*;

	#[test]
	fn should_error_when_non_preconfigured_range() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				<CurrencyRanges as CurrencyFactory>::protocol_asset_id_to_unique_asset_id(
					0,
					RangeId::from(6)
				),
				Err(DispatchError::Other("RangeId outside of preconfigured ranges!"))
			)
		});
	}

	#[test]
	fn should_provide_correct_global_asset_id() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				<CurrencyRanges as CurrencyFactory>::protocol_asset_id_to_unique_asset_id(
					1,
					RangeId::from(1)
				),
				Ok(u32::MAX as u128 * 2 + 1)
			)
		})
	}
}

mod unique_asset_id_to_protocol_asset_id {
	use super::*;

	#[test]
	fn should_provide_correct_protocol_asset_id() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				<CurrencyRanges as CurrencyFactory>::unique_asset_id_to_protocol_asset_id(
					u32::MAX as u128 * 2 + 1
				),
				1
			)
		})
	}
}
