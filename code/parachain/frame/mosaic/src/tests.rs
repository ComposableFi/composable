/// TODO
///
/// 1. Test each extrinsic
/// 2. Make sure unlocks etc are respected (timing)
/// 3. Add tests for linear decay.
///
///
/// grouping tests
///
/// test every failure case
/// every error that an extrinsic can return
///
/// all the happy path cases
///
///
/// interaction logic between extrinsics
/// such as:
///  transfer_to -> waiting for a block (til lock_time expires) -> claiming
///         check if the funds are correctly moved to the user's account
///
///  transfer_to -> waiting for a block -> relayer accepts transfer -> (til lock_time expires)
/// -> we should no longer be able to claim
///
///  incoming -> waiting til lock_time expires -> claiming
///
///  incoming -> waiting for a block -> relayer cancels transfer (finality issue) -> we should
/// no longer be able to claim
///
///
///  For every test, make sure that you check wether the funds moved to the correct (sub)
/// accounts.
use crate::{decay::*, mock::*, *};
use composable_support::{types::EthereumAddress, validation::Validated};
use composable_tests_helpers::{prop_assert_noop, prop_assert_ok};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use proptest::prelude::*;
use sp_runtime::{DispatchError, TokenError};

pub trait OriginExt {
	fn relayer() -> RuntimeOrigin {
		RuntimeOrigin::signed(RELAYER)
	}

	fn alice() -> RuntimeOrigin {
		RuntimeOrigin::signed(ALICE)
	}

	fn bob() -> RuntimeOrigin {
		RuntimeOrigin::signed(BOB)
	}
}

const BUDGET: Balance = 10000;
const NETWORK_ID: NetworkId = 1;
const ASSET_ID: AssetId = 1;
const REMOTE_ASSET_ID: RemoteAssetId = [1u8; 20];

impl OriginExt for RuntimeOrigin {}

prop_compose! {
	fn account_id()
		(x in 1..AccountId::MAX) -> AccountId {
			x
		}
}

prop_compose! {
	fn amount_within_budget()
		(x in 1..BUDGET) -> Balance {
			x
		}
}

prop_compose! {
	fn lock_time_gen()
		(x in 1..10000u64) -> u64 {
			x
		}
}

prop_compose! {
	fn wait_after_lock_gen()
		(x in 1..1000u64) -> u64 {
			x
		}
}

prop_compose! {
	fn budget_with_split()
		 (budget in 1..10_000_000u128, split in 1..100u128) -> (Balance, Balance, Balance) {
			let first_part = (budget * split) / 100u128;
			let second_part = budget - first_part;

			(budget, first_part, second_part)
	}
}

mod ensure_relayer {
	use super::*;

	#[test]
	fn ensure_relayer_is_set() {
		new_test_ext().execute_with(|| {
			assert_err!(
				Mosaic::ensure_relayer(RuntimeOrigin::signed(ALICE)),
				Error::<Test>::RelayerNotSet
			);
		})
	}

	#[test]
	fn ensure_relayer_origin_checked() {
		new_test_ext().execute_with(|| {
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
			assert_err!(
				Mosaic::ensure_relayer(RuntimeOrigin::signed(ALICE)),
				DispatchError::BadOrigin
			);
		})
	}
}

mod set_relayer {
	use super::*;

	#[test]
	fn set_relayer() {
		new_test_ext().execute_with(|| {
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
			assert_eq!(Mosaic::relayer_account_id(), Ok(RELAYER));
		})
	}

	#[test]
	fn relayer_cannot_set_relayer() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Mosaic::set_relayer(RuntimeOrigin::relayer(), ALICE),
				DispatchError::BadOrigin
			);
		})
	}

	#[test]
	fn none_cannot_set_relayer() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Mosaic::set_relayer(RuntimeOrigin::none(), ALICE),
				DispatchError::BadOrigin
			);
		})
	}
	#[test]
	fn alice_cannot_set_relayer() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Mosaic::set_relayer(RuntimeOrigin::signed(ALICE), ALICE),
				DispatchError::BadOrigin
			);
		})
	}
}

mod rotate_relayer {
	use super::*;

	#[test]
	fn relayer_can_rotate_relayer() {
		new_test_ext().execute_with(|| {
			let ttl = 500;
			let current_block = System::block_number();
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

			//first rotation
			let validated_ttl = Validated::new(ttl).unwrap();
			assert_ok!(Mosaic::rotate_relayer(RuntimeOrigin::relayer(), BOB, validated_ttl));
			System::set_block_number(current_block + ttl);
			assert_eq!(Mosaic::relayer_account_id(), Ok(BOB));

			// second rotation
			assert_ok!(Mosaic::rotate_relayer(RuntimeOrigin::signed(BOB), CHARLIE, validated_ttl));
			System::set_block_number(current_block + 2 * ttl);
			assert_eq!(Mosaic::relayer_account_id(), Ok(CHARLIE));
		})
	}

	#[test]
	fn relayer_must_not_rotate_early() {
		new_test_ext().execute_with(|| {
			let ttl = 500;
			let current_block = System::block_number();
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
			let validated_ttl = Validated::new(ttl).unwrap();
			assert_ok!(Mosaic::rotate_relayer(RuntimeOrigin::relayer(), BOB, validated_ttl));
			System::set_block_number(current_block + ttl - 1); // just before the ttl
			assert_eq!(Mosaic::relayer_account_id(), Ok(RELAYER)); // not BOB
		})
	}

	#[test]
	fn arbitrary_account_cannot_rotate_relayer() {
		new_test_ext().execute_with(|| {
			let ttl = Validated::new(500).unwrap();
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
			assert_noop!(
				Mosaic::rotate_relayer(RuntimeOrigin::signed(ALICE), BOB, ttl),
				DispatchError::BadOrigin
			);
		})
	}

	#[test]
	fn none_cannot_rotate_relayer() {
		new_test_ext().execute_with(|| {
			let ttl = Validated::new(500).unwrap();
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
			assert_noop!(
				Mosaic::rotate_relayer(RuntimeOrigin::none(), BOB, ttl),
				DispatchError::BadOrigin
			);
		})
	}
}

mod set_network {
	use super::*;

	#[test]
	fn relayer_can_set_network() {
		let network_id = 3;
		let network_info =
			NetworkInfo { enabled: false, min_transfer_size: 1, max_transfer_size: 100000 };
		new_test_ext().execute_with(|| {
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

			assert_ok!(Mosaic::set_network(
				RuntimeOrigin::relayer(),
				network_id,
				network_info.clone()
			));
			assert_eq!(Mosaic::network_infos(network_id), Some(network_info));
		})
	}

	#[test]
	fn root_cannot_set_network() {
		let network_id = 3;
		let network_info =
			NetworkInfo { enabled: false, min_transfer_size: 1, max_transfer_size: 100000 };
		new_test_ext().execute_with(|| {
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

			assert_noop!(
				Mosaic::set_network(RuntimeOrigin::root(), network_id, network_info.clone()),
				DispatchError::BadOrigin
			);
		})
	}

	#[test]
	fn none_cannot_set_network() {
		let network_id = 3;
		let network_info =
			NetworkInfo { enabled: false, min_transfer_size: 1, max_transfer_size: 100000 };
		new_test_ext().execute_with(|| {
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

			assert_noop!(
				Mosaic::set_network(RuntimeOrigin::none(), network_id, network_info.clone()),
				DispatchError::BadOrigin
			);
		})
	}
}

mod budget {
	use super::*;

	mod set_budget {
		use super::*;

		#[test]
		fn root_can_set_budget() {
			new_test_ext().execute_with(|| {
				assert_ok!(Mosaic::set_budget(
					RuntimeOrigin::root(),
					1,
					1,
					BudgetPenaltyDecayer::linear(5)
				));
			})
		}

		#[test]
		fn arbitrary_user_cannot_set_budget() {
			new_test_ext().execute_with(|| {
				assert_noop!(
					Mosaic::set_budget(
						RuntimeOrigin::signed(ALICE),
						1,
						1,
						BudgetPenaltyDecayer::linear(5)
					),
					DispatchError::BadOrigin
				);
			})
		}

		#[test]
		fn none_cannot_set_budget() {
			new_test_ext().execute_with(|| {
				assert_noop!(
					Mosaic::set_budget(
						RuntimeOrigin::none(),
						1,
						1,
						BudgetPenaltyDecayer::linear(5)
					),
					DispatchError::BadOrigin
				);
			})
		}
	}

	#[test]
	fn budget_are_isolated() {
		new_test_ext().execute_with(|| {
			assert_ok!(Mosaic::set_budget(
				RuntimeOrigin::root(),
				1,
				0xCAFEBABE,
				BudgetPenaltyDecayer::linear(10)
			));
			assert_ok!(Mosaic::set_budget(
				RuntimeOrigin::root(),
				2,
				0xDEADC0DE,
				BudgetPenaltyDecayer::linear(5)
			));
			assert_eq!(Mosaic::asset_infos(1).expect("budget must exists").budget, 0xCAFEBABE);
			assert_eq!(Mosaic::asset_infos(2).expect("budget must exists").budget, 0xDEADC0DE);
		})
	}

	#[test]
	fn last_deposit_does_not_change_after_updating_budget() {
		new_test_ext().execute_with(|| {
			let initial_block = System::block_number();
			assert_ok!(Mosaic::set_budget(
				RuntimeOrigin::root(),
				1,
				0xCAFEBABE,
				BudgetPenaltyDecayer::linear(10)
			));
			assert_eq!(
				Mosaic::asset_infos(1).expect("budget must exists").last_mint_block,
				initial_block
			);

			System::set_block_number(initial_block + 1);
			assert_ok!(Mosaic::set_budget(
				RuntimeOrigin::root(),
				1,
				0xDEADC0DE,
				BudgetPenaltyDecayer::linear(10)
			));
			assert_eq!(
				Mosaic::asset_infos(1).expect("budget must exists").last_mint_block,
				initial_block
			);
		})
	}
}

#[test]
fn incoming_outgoing_accounts_are_isolated() {
	ExtBuilder { balances: Default::default() }.build().execute_with(|| {
		initialize();

		let amount = 100;
		let ethereum_address = EthereumAddress([0; 20]);
		let network_id = 1;
		let asset_id: u128 = 1u128;

		assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
		let account_balance = || Tokens::balance(asset_id, &ALICE);
		let balance_of = |t| Tokens::balance(asset_id, &Mosaic::sub_account_id(t));
		assert_eq!(account_balance(), amount);
		assert_eq!(balance_of(SubAccount::new_outgoing(ALICE)), 0);
		assert_eq!(balance_of(SubAccount::new_incoming(ALICE)), 0);
		assert_ok!(Mosaic::transfer_to(
			RuntimeOrigin::signed(ALICE),
			network_id,
			asset_id,
			ethereum_address,
			amount,
			amount,
			false,
			ALICE,
			None,
			true,
		));
		assert_eq!(account_balance(), 0);
		assert_eq!(balance_of(SubAccount::new_outgoing(ALICE)), amount);
		assert_eq!(balance_of(SubAccount::new_incoming(ALICE)), 0);
	})
}

fn initialize() {
	System::set_block_number(1);

	assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
	assert_ok!(Mosaic::set_network(
		RuntimeOrigin::relayer(),
		1,
		NetworkInfo { enabled: true, min_transfer_size: 1, max_transfer_size: 100000 },
	));
	assert_ok!(Mosaic::set_budget(
		RuntimeOrigin::root(),
		1,
		BUDGET,
		BudgetPenaltyDecayer::linear(10)
	));
	assert_ok!(Mosaic::update_asset_mapping(
		RuntimeOrigin::root(),
		ASSET_ID,
		NETWORK_ID,
		Some(REMOTE_ASSET_ID)
	));
}

fn do_timelocked_mint(to: AccountId, amount: Balance, lock_time: u64) {
	let initial_block = System::block_number();

	assert_ok!(Mosaic::timelocked_mint(
		RuntimeOrigin::relayer(),
		NETWORK_ID,
		REMOTE_ASSET_ID,
		to,
		amount,
		lock_time,
		Default::default()
	));

	assert_eq!(
		Mosaic::incoming_transactions(to, ASSET_ID),
		Some((amount, initial_block + lock_time))
	);
}

mod timelocked_mint {
	use super::*;

	#[test]
	fn timelocked_mint() {
		new_test_ext().execute_with(|| {
			initialize();
			do_timelocked_mint(ALICE, 50, 10);
		})
	}

	#[test]
	fn cannot_mint_unsupported_assets() {
		new_test_ext().execute_with(|| {
			initialize();
			let unsupported_remote_asset_id: RemoteAssetId = [0xFFu8; 20];
			assert_ok!(Mosaic::update_asset_mapping(
				RuntimeOrigin::root(),
				0xCAFEBABE,
				NETWORK_ID,
				Some(unsupported_remote_asset_id)
			));
			assert_noop!(
				Mosaic::timelocked_mint(
					RuntimeOrigin::relayer(),
					NETWORK_ID,
					unsupported_remote_asset_id,
					ALICE,
					50,
					10,
					Default::default()
				),
				Error::<Test>::UnsupportedAsset
			);
		})
	}

	#[test]
	fn cannot_mint_more_than_budget() {
		new_test_ext().execute_with(|| {
			initialize();
			assert_noop!(
				Mosaic::timelocked_mint(
					RuntimeOrigin::relayer(),
					NETWORK_ID,
					REMOTE_ASSET_ID,
					ALICE,
					10001,
					10,
					Default::default()
				),
				Error::<Test>::InsufficientBudget
			);
		})
	}

	#[test]
	fn only_relayer_can_timelocked_mint() {
		new_test_ext().execute_with(|| {
			initialize();
			assert_noop!(
				Mosaic::timelocked_mint(
					RuntimeOrigin::signed(ALICE),
					NETWORK_ID,
					REMOTE_ASSET_ID,
					ALICE,
					50,
					10,
					Default::default()
				),
				DispatchError::BadOrigin
			);
		})
	}

	#[test]
	fn none_cannot_timelocked_mint() {
		new_test_ext().execute_with(|| {
			initialize();
			assert_noop!(
				Mosaic::timelocked_mint(
					RuntimeOrigin::none(),
					NETWORK_ID,
					REMOTE_ASSET_ID,
					ALICE,
					50,
					10,
					Default::default()
				),
				DispatchError::BadOrigin
			);
		})
	}

	#[test]
	fn timelocked_mint_adds_to_incoming_transactions() {
		new_test_ext().execute_with(|| {
			initialize();
			let amount = 50;
			let lock_time = 10;
			assert_ok!(Mosaic::timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				amount,
				lock_time,
				Default::default(),
			));
			assert_eq!(
				Mosaic::incoming_transactions(ALICE, 1),
				Some((amount, lock_time + System::block_number()))
			);
		})
	}

	#[test]
	fn timelocked_mint_updates_incoming_transactions() {
		new_test_ext().execute_with(|| {
			initialize();
			let amount = 50;
			let lock_time = 10;

			assert_ok!(Mosaic::timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				amount,
				lock_time,
				Default::default(),
			));
			assert_eq!(
				Mosaic::incoming_transactions(ALICE, 1),
				Some((amount, lock_time + System::block_number()))
			);

			let amount_2 = 100;
			let new_lock_time = 20;

			assert_ok!(Mosaic::timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				amount_2,
				new_lock_time,
				Default::default(),
			));

			assert_eq!(
				Mosaic::incoming_transactions(ALICE, 1),
				Some((amount + amount_2, new_lock_time + System::block_number()))
			);
		})
	}

	#[test]
	fn rescind_timelocked_mint() {
		new_test_ext().execute_with(|| {
			initialize();
			let lock_time = 10;
			do_timelocked_mint(ALICE, 50, lock_time);

			let initial_block = System::block_number();

			assert_ok!(Mosaic::rescind_timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				40
			));
			assert_eq!(
				Mosaic::incoming_transactions(ALICE, 1),
				Some((10, initial_block + lock_time))
			);
			let transfer_amount = 9;
			assert_ok!(Mosaic::rescind_timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				transfer_amount
			));
			assert_eq!(Mosaic::incoming_transactions(ALICE, 1), Some((1, 11)));
		})
	}

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn can_mint_up_to_the_penalised_budget(
			account_a in account_id(),
			decay in 1..100u128, // todo,
	  min_transfer_size in 1..10_000_000u128,
			max_transfer_size in 10_000_000u128..100_000_000u128,
			asset_id in 1..100u128,
			network_id in 1..100u32,
		  remote_asset_id in any::<RemoteAssetId>(),
			start_block in 1..10_000u64,
			(budget, first_part, second_part) in budget_with_split(),
		) {
			new_test_ext().execute_with(|| {
				// initialize
				System::set_block_number(start_block);

				prop_assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
				prop_assert_ok!(Mosaic::set_network(
					RuntimeOrigin::relayer(),
					network_id,
					NetworkInfo { enabled: true, min_transfer_size, max_transfer_size },
				), "relayer may set network info");
				prop_assert_ok!(Mosaic::set_budget(RuntimeOrigin::root(), asset_id, budget, BudgetPenaltyDecayer::linear(decay)), "root may set budget");
			prop_assert_ok!(Mosaic::update_asset_mapping(RuntimeOrigin::root(), asset_id, network_id, Some(remote_asset_id)));


				// We've split the budget in two parts. Both within the budget
				prop_assert_eq!(budget, first_part + second_part);
				// When mint the first part of the budget, it should be fine.
				prop_assert_ok!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, first_part, 0, Default::default()));
				// The new penalised_budget should be budget - first_part.
				// When we mint the second part of the budget, it should be fine because it matches the penalised_budget.
				prop_assert_ok!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, second_part, 0, Default::default()));

				Ok(())
			})?;
		}

		#[test]
		fn cannot_mint_more_than_the_penalised_budget(
			account_a in account_id(),
			decay in 1..100u128, // todo,
	  min_transfer_size in 1..10_000_000u128,
			max_transfer_size in 10_000_000u128..100_000_000u128,
			asset_id in 1..100u128,
			network_id in 1..100u32,
		  remote_asset_id in any::<RemoteAssetId>(),
			start_block in 1..10_000u64,
			(budget, first_part, second_part) in budget_with_split(),
		) {
			new_test_ext().execute_with(|| {
				// initialize
				System::set_block_number(start_block);

				prop_assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
				prop_assert_ok!(Mosaic::set_network(
					RuntimeOrigin::relayer(),
					network_id,
					NetworkInfo { enabled: true, min_transfer_size, max_transfer_size },
				), "relayer may set network info");
				prop_assert_ok!(Mosaic::set_budget(RuntimeOrigin::root(), asset_id, budget, BudgetPenaltyDecayer::linear(decay)), "root may set budget");
			prop_assert_ok!(Mosaic::update_asset_mapping(RuntimeOrigin::root(), asset_id, network_id, Some(remote_asset_id)));


				// We've split the budget in two parts. Both within the budget
				prop_assert_eq!(budget, first_part + second_part);
				// When mint the first part of the budget, it should be fine.
				prop_assert_ok!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, first_part, 0, Default::default()));
				// The new penalised_budget should be budget - first_part.
				// When we mint the second part of the budget, it should be fine because it matches the penalised_budget.
				prop_assert_ok!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, second_part, 0, Default::default()));
				// When we mint more than the penalised budget, it should fail.
				prop_assert_noop!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, 1, 0, Default::default()), Error::<Test>::InsufficientBudget);
				Ok(())
			})?;
		}

		#[test]
		fn should_be_able_to_mint_again_after_waiting_for_penalty_to_decay(
			account_a in account_id(),
			decay_factor in 1..100u128, // todo,
	  min_transfer_size in 1..10_000_000u128,
			max_transfer_size in 10_000_000u128..100_000_000u128,
			asset_id in 1..100u128,
			network_id in 1..100u32,
		  remote_asset_id in any::<RemoteAssetId>(),
			start_block in 1..10_000u64,
			(budget, first_part, second_part) in budget_with_split(),
			iteration_count in 2..10u64,
		) {
			prop_assume!(budget > decay_factor);

			new_test_ext().execute_with(|| {

				// initialize
				System::set_block_number(start_block);

				let budget_penalty_decayer = BudgetPenaltyDecayer::linear(decay_factor);

				prop_assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
				prop_assert_ok!(Mosaic::set_network(
					RuntimeOrigin::relayer(),
					network_id,
					NetworkInfo { enabled: true, min_transfer_size, max_transfer_size },
				), "relayer may set network info");
				prop_assert_ok!(Mosaic::set_budget(RuntimeOrigin::root(), asset_id, budget, budget_penalty_decayer.clone()), "root may set budget");
			prop_assert_ok!(Mosaic::update_asset_mapping(RuntimeOrigin::root(), asset_id, network_id, Some(remote_asset_id)));


				// We've split the budget in two parts. Both within the budget
				prop_assert_eq!(budget, first_part + second_part);


				let budget_recovery_period: BlockNumber = budget_penalty_decayer.full_recovery_period(budget)
						.expect("impossible as per the prop_assume! above, qed");


				for _ in 0..iteration_count {

					// When mint the first part of the budget, it should be fine.
					prop_assert_ok!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, first_part, 0, Default::default()));
					// The new penalised_budget should be budget - first_part.
					// When we mint the second part of the budget, it should be fine because it matches the penalised_budget.
					prop_assert_ok!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, second_part, 0, Default::default()));


					// When we mint more than the penalised budget, it should fail.
					prop_assert_noop!(Mosaic::timelocked_mint(RuntimeOrigin::relayer(), network_id, remote_asset_id, account_a, 1, 0, Default::default()), Error::<Test>::InsufficientBudget);


					// We wait until the budget has recovered
					System::set_block_number(System::block_number() + budget_recovery_period);
				}

				Ok(())
			})?;
		}
	}
}

mod rescind_timelocked_mint {
	use super::*;

	#[test]
	fn cannot_rescind_timelocked_mint_if_no_transaction() {
		new_test_ext().execute_with(|| {
			initialize();
			assert_noop!(
				Mosaic::rescind_timelocked_mint(
					RuntimeOrigin::relayer(),
					NETWORK_ID,
					REMOTE_ASSET_ID,
					ALICE,
					50
				),
				Error::<Test>::NoClaimableTx
			);
		})
	}

	#[test]
	fn cannot_rescind_timelocked_mint_if_wrong_asset_id() {
		new_test_ext().execute_with(|| {
			initialize();
			let lock_time = 10;
			do_timelocked_mint(ALICE, 50, lock_time);
			let another_remote_asset_id = [0xFFu8; 20];
			assert_ok!(Mosaic::update_asset_mapping(
				RuntimeOrigin::root(),
				0xCAFEBABE,
				NETWORK_ID,
				Some(another_remote_asset_id)
			));
			assert_noop!(
				Mosaic::rescind_timelocked_mint(
					RuntimeOrigin::relayer(),
					NETWORK_ID,
					another_remote_asset_id,
					ALICE,
					50
				),
				Error::<Test>::NoClaimableTx
			);
		})
	}

	#[test]
	fn cannot_rescind_timelocked_mint_if_wrong_account() {
		new_test_ext().execute_with(|| {
			initialize();
			let lock_time = 10;
			do_timelocked_mint(ALICE, 50, lock_time);
			assert_noop!(
				Mosaic::rescind_timelocked_mint(
					RuntimeOrigin::relayer(),
					NETWORK_ID,
					REMOTE_ASSET_ID,
					BOB,
					50
				),
				Error::<Test>::NoClaimableTx
			);
		})
	}

	#[test]
	fn cannot_rescind_timelocked_mint_if_wrong_amount() {
		new_test_ext().execute_with(|| {
			initialize();
			let lock_time = 10;
			let amount = 50;
			do_timelocked_mint(ALICE, amount, lock_time);
			assert_noop!(
				Mosaic::rescind_timelocked_mint(
					RuntimeOrigin::relayer(),
					NETWORK_ID,
					REMOTE_ASSET_ID,
					ALICE,
					amount + 1
				),
				TokenError::NoFunds
			);
		})
	}

	#[test]
	fn rescind_timelocked_mint_in_two_steps() {
		new_test_ext().execute_with(|| {
			initialize();
			let lock_time = 10;
			let start_amount = 50;
			do_timelocked_mint(ALICE, start_amount, lock_time);
			assert_eq!(
				Mosaic::incoming_transactions(ALICE, 1),
				Some((start_amount, lock_time + System::block_number()))
			);

			let rescind_amount = 9;
			assert_ok!(Mosaic::rescind_timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				rescind_amount
			));
			assert_eq!(
				Mosaic::incoming_transactions(ALICE, 1),
				Some((start_amount - rescind_amount, lock_time + System::block_number()))
			);

			assert_ok!(Mosaic::rescind_timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				start_amount - rescind_amount,
			));

			assert_eq!(Mosaic::incoming_transactions(ALICE, 1), None);
		})
	}
}

mod set_timelock_duration {
	use super::*;

	#[test]
	fn set_timelock_duration() {
		new_test_ext().execute_with(|| {
			assert_ok!(Mosaic::set_timelock_duration(
				RuntimeOrigin::root(),
				Validated::new(MinimumTimeLockPeriod::get() + 1).unwrap()
			));
		})
	}

	#[test]
	fn set_timelock_duration_with_non_root() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Mosaic::set_timelock_duration(
					RuntimeOrigin::signed(ALICE),
					Validated::new(MinimumTimeLockPeriod::get() + 1).unwrap()
				),
				DispatchError::BadOrigin
			);
		})
	}

	#[test]
	fn set_timelock_duration_with_origin_none() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Mosaic::set_timelock_duration(
					RuntimeOrigin::none(),
					Validated::new(MinimumTimeLockPeriod::get() + 1).unwrap()
				),
				DispatchError::BadOrigin
			);
		})
	}
}

mod transfer_to {
	use super::*;

	#[test]
	fn transfer_to() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
		})
	}

	#[test]
	fn accept_transfer() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
			assert_ok!(Mosaic::accept_transfer(
				RuntimeOrigin::relayer(),
				ALICE,
				NETWORK_ID,
				REMOTE_ASSET_ID,
				100
			));
		})
	}

	#[test]
	fn cannot_accept_transfer_larger_than_balance() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
			assert_noop!(
				Mosaic::accept_transfer(
					RuntimeOrigin::relayer(),
					ALICE,
					NETWORK_ID,
					REMOTE_ASSET_ID,
					101
				),
				Error::<Test>::AmountMismatch
			);
		})
	}

	#[test]
	fn claim_stale_to() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
			let current_block = System::block_number();
			System::set_block_number(current_block + Mosaic::timelock_period() + 1);
			assert_ok!(Mosaic::claim_stale_to(RuntimeOrigin::signed(ALICE), 1, ALICE));
		})
	}

	#[test]
	fn cannot_claim_stale_to_early() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
			let current_block = System::block_number();
			System::set_block_number(current_block + Mosaic::timelock_period() - 1);
			assert_noop!(
				Mosaic::claim_stale_to(RuntimeOrigin::signed(ALICE), 1, ALICE),
				Error::<Test>::TxStillLocked
			);
		})
	}

	#[test]
	fn cannot_claim_after_relayer_accepts_transfer() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
			assert_ok!(Mosaic::accept_transfer(
				RuntimeOrigin::relayer(),
				ALICE,
				NETWORK_ID,
				REMOTE_ASSET_ID,
				100
			));
			let current_block = System::block_number();
			System::set_block_number(current_block + Mosaic::timelock_period() + 1);
			assert_noop!(
				Mosaic::claim_stale_to(RuntimeOrigin::signed(ALICE), 1, ALICE),
				Error::<Test>::NoStaleTransactions
			);
		})
	}

	#[test]
	fn relayer_cannot_accept_transfer_after_claim() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
			let current_block = System::block_number();
			System::set_block_number(current_block + Mosaic::timelock_period() + 1);
			assert_ok!(Mosaic::claim_stale_to(RuntimeOrigin::signed(ALICE), 1, ALICE));
			assert_noop!(
				Mosaic::accept_transfer(
					RuntimeOrigin::relayer(),
					ALICE,
					NETWORK_ID,
					REMOTE_ASSET_ID,
					100
				),
				Error::<Test>::NoOutgoingTx
			);
		})
	}

	#[test]
	fn can_claim_stale_after_partial_accept_transfer() {
		new_test_ext().execute_with(|| {
			initialize();
			do_transfer_to();
			let current_block = System::block_number();
			System::set_block_number(current_block + Mosaic::timelock_period() + 1);
			assert_ok!(Mosaic::accept_transfer(
				RuntimeOrigin::relayer(),
				ALICE,
				NETWORK_ID,
				REMOTE_ASSET_ID,
				20
			));
			// System::set_block_number(current_block + Mosaic::timelock_period() + 1);
			assert_ok!(Mosaic::claim_stale_to(RuntimeOrigin::signed(ALICE), 1, ALICE));
		})
	}

	#[test]
	fn transfer_to_below_min_transfer_size() {
		ExtBuilder { balances: Default::default() }.build().execute_with(|| {
			let min_transfer_size = 1000;
			let max_transfer_size = 100000;

			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

			let network_id = 1;
			assert_ok!(Mosaic::set_network(
				RuntimeOrigin::relayer(),
				network_id,
				NetworkInfo { enabled: true, min_transfer_size, max_transfer_size },
			));

			let asset_id: u128 = 1;
			assert_ok!(Mosaic::set_budget(
				RuntimeOrigin::root(),
				asset_id,
				10000,
				BudgetPenaltyDecayer::linear(10)
			));

			let remote_asset_id = [0xFFu8; 20];
			assert_ok!(Mosaic::update_asset_mapping(
				RuntimeOrigin::root(),
				asset_id,
				network_id,
				Some(remote_asset_id)
			));

			let amount = min_transfer_size - 1;
			let ethereum_address = EthereumAddress([0; 20]);
			assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
			assert_noop!(
				Mosaic::transfer_to(
					RuntimeOrigin::signed(ALICE),
					network_id,
					asset_id,
					ethereum_address,
					amount,
					amount,
					false,
					ALICE,
					None,
					true
				),
				Error::<Test>::BelowMinTransferSize
			);
		})
	}

	#[test]
	fn transfer_to_exceeds_max_transfer_size() {
		ExtBuilder { balances: Default::default() }.build().execute_with(|| {
			let min_transfer_size = 1;
			let max_transfer_size = 100000;

			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

			let network_id = 1;
			assert_ok!(Mosaic::set_network(
				RuntimeOrigin::relayer(),
				network_id,
				NetworkInfo { enabled: true, min_transfer_size, max_transfer_size },
			));

			let asset_id: u128 = 1;
			assert_ok!(Mosaic::set_budget(
				RuntimeOrigin::root(),
				asset_id,
				10000,
				BudgetPenaltyDecayer::linear(10)
			));

			let remote_asset_id = [0xFFu8; 20];
			assert_ok!(Mosaic::update_asset_mapping(
				RuntimeOrigin::root(),
				asset_id,
				network_id,
				Some(remote_asset_id)
			));

			// We exceed the max transfer size
			let amount = max_transfer_size + 1;
			let ethereum_address = EthereumAddress([0; 20]);
			assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
			assert_noop!(
				Mosaic::transfer_to(
					RuntimeOrigin::signed(ALICE),
					network_id,
					asset_id,
					ethereum_address,
					amount,
					amount,
					false,
					ALICE,
					None,
					true
				),
				Error::<Test>::ExceedsMaxTransferSize
			);
		})
	}

	#[test]
	fn transfer_to_move_funds_to_outgoing() {
		ExtBuilder { balances: Default::default() }.build().execute_with(|| {
			initialize();

			let amount = 100;
			let ethereum_address = EthereumAddress([0; 20]);
			let network_id = 1;
			let asset_id: u128 = 1;

			assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
			let account_balance = || Tokens::balance(asset_id, &ALICE);
			let outgoing_balance = || {
				Tokens::balance(asset_id, &Mosaic::sub_account_id(SubAccount::new_outgoing(ALICE)))
			};
			assert_eq!(account_balance(), amount);
			assert_eq!(outgoing_balance(), 0);
			assert_ok!(Mosaic::transfer_to(
				RuntimeOrigin::signed(ALICE),
				network_id,
				asset_id,
				ethereum_address,
				amount,
				amount,
				false,
				ALICE,
				None,
				true
			));
			assert_eq!(account_balance(), 0);
			assert_eq!(outgoing_balance(), amount);
		})
	}

	#[test]
	fn transfer_to_unsupported_asset() {
		ExtBuilder { balances: Default::default() }.build().execute_with(|| {
			assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));
			assert_ok!(Mosaic::set_network(
				RuntimeOrigin::relayer(),
				1,
				NetworkInfo { enabled: true, min_transfer_size: 1, max_transfer_size: 100000 },
			));

			// We don't register the asset

			let ethereum_address = EthereumAddress([0; 20]);
			let amount = 100;
			let network_id = 1;
			let asset_id: u128 = 1;

			assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
			assert_noop!(
				Mosaic::transfer_to(
					RuntimeOrigin::signed(ALICE),
					network_id,
					asset_id,
					ethereum_address,
					amount,
					amount,
					true,
					ALICE,
					None,
					false
				),
				Error::<Test>::UnsupportedAsset
			);
		})
	}

	fn do_transfer_to() {
		let ethereum_address = EthereumAddress([0; 20]);
		let amount = 100;
		let swap_to_native = false;

		assert_ok!(Mosaic::transfer_to(
			RuntimeOrigin::signed(ALICE),
			NETWORK_ID,
			ASSET_ID,
			ethereum_address,
			amount,
			amount,
			swap_to_native,
			ALICE,
			None,
			true,
		));
		assert_eq!(
			Mosaic::outgoing_transactions(&ALICE, ASSET_ID),
			Some((100, MinimumTimeLockPeriod::get() + System::block_number()))
		);

		// normally we don't unit test events being emitted, but in this case it is very crucial for
		// the relayer to observe the events.

		// When a transfer is made, the nonce is incremented. However, nonce is one of the
		// dependencies for `generate_id`, we want to check if the events match, so we decrement the
		// nonce and increment it back when we're done
		// TODO: this is a hack, cfr: CU-1ubrf2y
		Nonce::<Test>::mutate(|nonce| {
			*nonce = nonce.wrapping_sub(1);
			*nonce
		});

		let id = generate_id::<Test>(
			&ALICE,
			&NETWORK_ID,
			&ASSET_ID,
			&ethereum_address,
			&amount,
			&System::block_number(),
		);
		Nonce::<Test>::mutate(|nonce| {
			*nonce = nonce.wrapping_add(1);
			*nonce
		});

		System::assert_last_event(mock::RuntimeEvent::Mosaic(crate::Event::TransferOut {
			id,
			to: ethereum_address,
			asset_id: ASSET_ID,
			network_id: NETWORK_ID,
			remote_asset_id: REMOTE_ASSET_ID,
			amount,
			minimum_amount_out: amount,
			swap_to_native,
			source_user_account: ALICE,
			amm_swap_info: None,
		}));
	}
}

mod accept_transfer {
	use super::*;

	#[test]
	fn cannot_mint_more_than_budget() {
		new_test_ext().execute_with(|| {
			initialize();
			assert_noop!(
				Mosaic::timelocked_mint(
					RuntimeOrigin::relayer(),
					NETWORK_ID,
					REMOTE_ASSET_ID,
					ALICE,
					10001,
					10,
					Default::default()
				),
				Error::<Test>::InsufficientBudget
			);
		})
	}

	#[test]
	fn rescind_timelocked_mint() {
		new_test_ext().execute_with(|| {
			initialize();
			let lock_time = 10;
			do_timelocked_mint(ALICE, 50, lock_time);

			let initial_block = System::block_number();

			assert_ok!(Mosaic::rescind_timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				40
			));
			assert_eq!(
				Mosaic::incoming_transactions(ALICE, 1),
				Some((10, initial_block + lock_time))
			);
			let transfer_amount = 9;
			assert_ok!(Mosaic::rescind_timelocked_mint(
				RuntimeOrigin::relayer(),
				NETWORK_ID,
				REMOTE_ASSET_ID,
				ALICE,
				transfer_amount
			));
			assert_eq!(Mosaic::incoming_transactions(ALICE, 1), Some((1, 11)));
		})
	}
}

mod claim_to {
	use super::*;

	#[test]
	fn claim_to() {
		new_test_ext().execute_with(|| {
			initialize();
			let lock_time = 10;
			do_timelocked_mint(ALICE, 50, lock_time);
			let current_block = System::block_number();
			assert_noop!(
				Mosaic::claim_to(RuntimeOrigin::alice(), 1, ALICE),
				Error::<Test>::TxStillLocked
			);
			System::set_block_number(current_block + lock_time + 1);
			assert_ok!(Mosaic::claim_to(RuntimeOrigin::alice(), 1, ALICE));
		})
	}
}

#[cfg(test)]
mod test_validation {
	use super::*;
	use composable_support::validation::Validate;
	use frame_support::assert_ok;
	use validation::{ValidTTL, ValidTimeLockPeriod};

	#[test]
	fn set_ttl_with_invalid_period() {
		assert!(<ValidTTL<MinimumTTL> as Validate<BlockNumber, ValidTTL<MinimumTTL>>>::validate(
			0_u64
		)
		.is_err());
	}

	#[test]
	fn set_ttl_with_invalid_period_3() {
		assert!(<ValidTTL<MinimumTTL> as Validate<BlockNumber, ValidTTL<MinimumTTL>>>::validate(
			MinimumTTL::get() - 1
		)
		.is_err());
	}

	#[test]
	fn set_ttl_period_3() {
		assert_ok!(
			<ValidTTL<MinimumTTL> as Validate<BlockNumber, ValidTTL<MinimumTTL>>>::validate(
				MinimumTTL::get() + 1
			)
		);
	}

	#[test]
	fn set_timelock_duration_with_invalid_period() {
		assert!(<ValidTimeLockPeriod<MinimumTimeLockPeriod> as Validate<
			BlockNumber,
			ValidTimeLockPeriod<MinimumTimeLockPeriod>,
		>>::validate(0_u64)
		.is_err());
	}

	#[test]
	fn set_timelock_duration_with_invalid_period_2() {
		assert!(<ValidTimeLockPeriod<MinimumTimeLockPeriod> as Validate<
			BlockNumber,
			ValidTimeLockPeriod<MinimumTimeLockPeriod>,
		>>::validate(MinimumTimeLockPeriod::get() - 1)
		.is_err());
	}

	#[test]
	fn set_timelock_duration_with_invalid_period_3() {
		assert_ok!(<ValidTimeLockPeriod<MinimumTimeLockPeriod> as Validate<
			BlockNumber,
			ValidTimeLockPeriod<MinimumTimeLockPeriod>,
		>>::validate(MinimumTimeLockPeriod::get() + 1));
	}
}

mod add_remote_amm_id {
	use super::*;

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn should_be_able_to_add_remote_amm_id(
			network_id in 1..u32::max_value(),
			amm_id in 0..u128::max_value(),
			start_block in 1..10_000u64,
		) {
			new_test_ext().execute_with(|| {

				System::set_block_number(start_block);
				prop_assert_ok!(Mosaic::add_remote_amm_id(
					RuntimeOrigin::root(),
					network_id,
					amm_id
				));

				Ok(())
			})?;
		}

		#[test]
		fn should_be_impossible_to_add_amm_id_twice(
			network_id in 1..u32::max_value(),
			amm_id in 0..u128::max_value(),
			start_block in 1..10_000u64,
		) {
			new_test_ext().execute_with(|| {

				System::set_block_number(start_block);
				prop_assert_ok!(Mosaic::add_remote_amm_id(
					RuntimeOrigin::root(),
					network_id,
					amm_id
				));

				prop_assert_noop!(
					Mosaic::add_remote_amm_id(
						RuntimeOrigin::root(),
						network_id,
						amm_id
					),
					Error::<Test>::RemoteAmmIdAlreadyExists
				);

				Ok(())
			})?;
		}
	}
}

mod remove_remote_amm_id {
	use super::*;

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn should_be_able_remove_amm_id_after_adding(
			network_id in 1..u32::max_value(),
			amm_id in 0..u128::max_value(),
			start_block in 1..10_000u64,
		) {
			new_test_ext().execute_with(|| {

				System::set_block_number(start_block);
				prop_assert_ok!(Mosaic::add_remote_amm_id(
					RuntimeOrigin::root(),
					network_id,
					amm_id
				));

				prop_assert_ok!(Mosaic::remove_remote_amm_id(
					RuntimeOrigin::root(),
					network_id,
					amm_id
				));

				Ok(())
			})?;
		}

		#[test]
		fn should_not_be_able_to_remove_nonexistent_remote_amm_ids(
			network_id in 1..u32::max_value(),
			amm_id in 0..u128::max_value(),
			start_block in 1..10_000u64,
		) {
			new_test_ext().execute_with(|| {

				System::set_block_number(start_block);
				prop_assert_noop!(
					Mosaic::remove_remote_amm_id(
						RuntimeOrigin::root(),
						network_id,
						amm_id
					),
					Error::<Test>::RemoteAmmIdNotFound
				);

				Ok(())
			})?;
		}
	}
}

mod do_transfer_with_remote_amm_swap {
	use super::*;

	proptest! {
		#![proptest_config(ProptestConfig::with_cases(10000))]

		#[test]
		fn should_work_with_whitelisted_amm_id(
			account_a in account_id(),
			network_id in 1..u32::max_value(),
			asset_id in 1..100u128,
			amm_id in 0..u128::max_value(),
			amount in 100..u128::max_value(),
			start_block in 1..10_000u64,
			minimum_amount_out in 0..u128::max_value(),
			// ethereum_address in ethereum_address()
		) {
			new_test_ext().execute_with(|| {

				assert_ok!(Tokens::mint_into(asset_id, &account_a, amount));

				assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

				assert_ok!(Mosaic::set_network(
					RuntimeOrigin::relayer(),
					network_id,
					NetworkInfo { enabled: true,  min_transfer_size: 1, max_transfer_size: amount },
				));

				assert_ok!(Mosaic::set_budget(
					RuntimeOrigin::root(),
					asset_id,
					amount,
					BudgetPenaltyDecayer::linear(10)
				));

				let remote_asset_id = [0xFFu8; 20];
				assert_ok!(Mosaic::update_asset_mapping(
					RuntimeOrigin::root(),
					asset_id,
					network_id,
					Some(remote_asset_id)
				));

				let ethereum_address = EthereumAddress([0; 20]);

				System::set_block_number(start_block);
				prop_assert_ok!(Mosaic::add_remote_amm_id(
					RuntimeOrigin::root(),
					network_id,
					amm_id
				));

				let amm_swap_info = AmmSwapInfo {
					destination_token_out_address: ethereum_address,
					destination_amm: RemoteAmm {
						network_id,
						amm_id,
					},
					minimum_amount_out,
				};

				prop_assert_ok!(Mosaic::transfer_to(
					RuntimeOrigin::signed(account_a),
					network_id,
					asset_id,
					ethereum_address,
					amount,
					amount,
					true,
					account_a,
					Some(amm_swap_info),
					true
				));

				Ok(())
			})?;
		}

		#[test]
		fn should_not_work_with_not_whitelisted_amm_id(
			account_a in account_id(),
			network_id in 1..u32::max_value(),
			asset_id in 1..100u128,
			amm_id in 0..u128::max_value(),
			amount in 100..u128::max_value(),
			start_block in 1..10_000u64,
			minimum_amount_out in 0..u128::max_value(),
			// ethereum_address in ethereum_address()
		) {
			new_test_ext().execute_with(|| {

				assert_ok!(Tokens::mint_into(asset_id, &account_a, amount));

				assert_ok!(Mosaic::set_relayer(RuntimeOrigin::root(), RELAYER));

				assert_ok!(Mosaic::set_network(
					RuntimeOrigin::relayer(),
					network_id,
					NetworkInfo { enabled: true,  min_transfer_size: 1, max_transfer_size: amount },
				));

				assert_ok!(Mosaic::set_budget(
					RuntimeOrigin::root(),
					asset_id,
					amount,
					BudgetPenaltyDecayer::linear(10)
				));

				let remote_asset_id = [0xFFu8; 20];
				assert_ok!(Mosaic::update_asset_mapping(
					RuntimeOrigin::root(),
					asset_id,
					network_id,
					Some(remote_asset_id)
				));

				let ethereum_address = EthereumAddress([0; 20]);


				System::set_block_number(start_block);


				// Note how we are NOT calling Mosaic::add_remote_amm_id here.
				// Therefore, any (network_id, amm_id) is not-whitelisted.

				let amm_swap_info = AmmSwapInfo {
					destination_token_out_address: ethereum_address,
					destination_amm: RemoteAmm {
						network_id,
						amm_id,
					},
					minimum_amount_out,
				};

				prop_assert_noop!(Mosaic::transfer_to(
					RuntimeOrigin::signed(account_a),
					network_id,
					asset_id,
					ethereum_address,
					amount,
					amount,
					true,
					account_a,
					Some(amm_swap_info),
					true
				), Error::<Test>::DestinationAmmIdNotWhitelisted);


				Ok(())
			})?;
		}
	}
}
