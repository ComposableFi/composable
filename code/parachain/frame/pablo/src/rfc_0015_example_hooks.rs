use crate::{
	mock::{new_test_ext, Test},
	Config,
};
use frame_support::{
	pallet_prelude::DispatchResult,
	traits::fungibles::{Inspect, Mutate},
};

fn double_balance<AssetId: Clone, AccountId, Balance>(
	asset: AssetId,
	who: &AccountId,
	balance: impl Fn(AssetId, &AccountId) -> Balance,
	mint_into: impl Fn(AssetId, &AccountId, Balance) -> DispatchResult,
) {
	mint_into(asset.clone(), who, balance(asset, who));
}

#[test]
fn test_do_double_balance() {
	new_test_ext().execute_with(|| {
		let who = 1;
		let asset = 1000;
		let initial_balance = 10;

		<Test as Config>::Assets::mint_into(asset, &who, initial_balance).unwrap();

		double_balance(
			asset,
			&who,
			<<Test as Config>::Assets as Inspect<_>>::balance,
			<<Test as Config>::Assets as Mutate<_>>::mint_into,
		);
		assert_eq!(
			<<Test as Config>::Assets as Inspect<_>>::balance(asset, &who),
			2 * initial_balance
		);

		double_balance(
			asset,
			&who,
			<<Test as Config>::Assets as Inspect<_>>::balance,
			<<Test as Config>::Assets as Mutate<_>>::mint_into,
		);
		assert_eq!(
			<<Test as Config>::Assets as Inspect<_>>::balance(asset, &who),
			4 * initial_balance
		);
	})
}

#[test]
fn test_do_double_balance_mocked() {
	use core::cell::RefCell;
	use sp_std::collections::btree_map::BTreeMap;

	let who: u128 = 1;
	let asset: u128 = 1000;
	let initial_balance: u128 = 10;

	// refcell is required here due to mutably borrowing `balances` in both closures
	// abstracted this into a helper would be trivial
	let balances = RefCell::new(BTreeMap::<_, BTreeMap<_, _>>::new());

	let mint_into = |asset, who: &_, amount| {
		balances
			.borrow_mut()
			.entry(*who)
			.or_insert_with(BTreeMap::new)
			.entry(asset)
			.and_modify(|b| *b += amount)
			.or_insert(amount);

		Ok(())
	};

	let balance = |asset, who: &_| {
		balances
			.borrow_mut()
			.entry(*who)
			.or_insert_with(BTreeMap::new)
			.get(&asset)
			.copied()
			.unwrap_or_default()
	};

	mint_into(asset, &who, initial_balance).unwrap();

	double_balance::<u128, u128, u128>(asset, &who, balance, mint_into);
	assert_eq!(balance(asset, &who), 2 * initial_balance);

	double_balance::<u128, u128, u128>(asset, &who, balance, mint_into);
	assert_eq!(balance(asset, &who), 4 * initial_balance);
}
