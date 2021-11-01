use crate::*;
use mocks::{new_test_ext, GovernanceRegistry, Origin, Test};

#[test]
fn set_only_by_root() {
	new_test_ext().execute_with(|| {
		GovernanceRegistry::set(Origin::root(), 1, 1).unwrap();
		ensure_root_or_governance::<Test>(Origin::root(), &2).unwrap();
		ensure_root_or_governance::<Test>(Origin::signed(1), &2).unwrap_err();
		ensure_root_or_governance::<Test>(Origin::signed(2), &1).unwrap_err();
		ensure_root_or_governance::<Test>(Origin::signed(1), &1).unwrap();
		ensure_root_or_governance::<Test>(Origin::none(), &1).unwrap_err();
		ensure_root_or_governance::<Test>(Origin::none(), &2).unwrap_err();
	});
}
