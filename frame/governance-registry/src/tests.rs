use composable_traits::governance::SignedRawOrigin;

use crate::mock::{new_test_ext, GovRegistry, Origin};

#[test]
fn set_only_by_root() {
	new_test_ext().execute_with(|| {
		GovRegistry::set(Origin::none(), 1, 1).unwrap_err();
		GovRegistry::set(Origin::signed(0), 1, 1).unwrap_err();
		GovRegistry::set(Origin::root(), 1, 1).unwrap();
		assert_eq!(GovRegistry::get(&1).unwrap(), SignedRawOrigin::Signed(1))
	});
}

#[test]
fn grant_root_only_by_root() {
	new_test_ext().execute_with(|| {
		GovRegistry::grant_root(Origin::none(), 1).unwrap_err();
		GovRegistry::grant_root(Origin::signed(0), 1).unwrap_err();
		GovRegistry::grant_root(Origin::root(), 1).unwrap();
		assert_eq!(GovRegistry::get(&1).unwrap(), SignedRawOrigin::Root)
	});
}
