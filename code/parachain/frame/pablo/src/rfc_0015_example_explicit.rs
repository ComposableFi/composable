use crate::{
	mock::{new_test_ext, Test},
	Config,
};
use frame_support::traits::fungibles::{Inspect, Mutate};
use sp_runtime::traits::CheckedAdd;

fn triple_balance<Balance: CheckedAdd>(
	balance: Balance,
) -> Result<TripleBalanceResult<Balance>, TripleBalanceErr> {
	match balance.checked_add(&balance).and_then(|b| b.checked_add(&balance)) {
		Some(new_balance) => Ok(TripleBalanceResult { new_balance }),
		None => Err(TripleBalanceErr::Overflow),
	}
}

#[derive(Debug, PartialEq)]
enum TripleBalanceErr {
	Overflow,
}

#[derive(Debug, PartialEq)]
struct TripleBalanceResult<Balance> {
	new_balance: Balance,
}

#[test]
fn test_triple_balance() {
	new_test_ext().execute_with(|| {
		let who = 1;
		let asset = 1000;
		let initial_balance = 10;

		<Test as Config>::Assets::mint_into(asset, &who, initial_balance).unwrap();

		let old_balance = <<Test as Config>::Assets as Inspect<_>>::balance(asset, &who);
		let new_balance = triple_balance(old_balance);

		<<Test as Config>::Assets as Mutate<_>>::mint_into(
			asset,
			&who,
			new_balance.unwrap().new_balance - old_balance,
		)
		.unwrap();
		assert_eq!(
			<<Test as Config>::Assets as Inspect<_>>::balance(asset, &who),
			3 * initial_balance
		);
	})
}

#[test]
fn test_triple_balance_mocked() {
	let initial_balance: u128 = 10;

	assert_eq!(
		triple_balance(initial_balance),
		Ok(TripleBalanceResult { new_balance: initial_balance * 3 })
	);

	assert_eq!(triple_balance(0), Ok(TripleBalanceResult { new_balance: 0 }));

	assert_eq!(triple_balance(u128::MAX / 2), Err(TripleBalanceErr::Overflow));
}
