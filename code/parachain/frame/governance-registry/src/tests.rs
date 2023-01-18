use composable_traits::governance::SignedRawOrigin;

use crate::mock::{new_test_ext, GovRegistry, RuntimeOrigin};

#[test]
fn set_only_by_root() {
	new_test_ext().execute_with(|| {
		GovRegistry::set(RuntimeOrigin::none(), 1, 1).unwrap_err();
		GovRegistry::set(RuntimeOrigin::signed(0), 1, 1).unwrap_err();
		GovRegistry::set(RuntimeOrigin::root(), 1, 1).unwrap();
		assert_eq!(GovRegistry::get(&1).unwrap(), SignedRawOrigin::Signed(1))
	});
}

#[test]
fn grant_root_only_by_root() {
	new_test_ext().execute_with(|| {
		GovRegistry::grant_root(RuntimeOrigin::none(), 1).unwrap_err();
		GovRegistry::grant_root(RuntimeOrigin::signed(0), 1).unwrap_err();
		GovRegistry::grant_root(RuntimeOrigin::root(), 1).unwrap();
		assert_eq!(GovRegistry::get(&1).unwrap(), SignedRawOrigin::Root)
	});
}
