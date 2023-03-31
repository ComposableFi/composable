//! Unit tests for the bonded-finance pallet.

#![cfg(test)]

use super::*;
use composable_tests_helpers::{prop_assert_acceptable_computation_error, prop_assert_ok};
use composable_traits::{
	bonded_finance::{BondDuration, BondOffer, BondOfferReward},
};
use pallet_vesting::VestingScheduleIdSet;
use frame_support::{
	error::BadOrigin,
	traits::{
		fungibles::{Inspect, Mutate},
		tokens::WithdrawConsequence,
	},
};
use mock::{RuntimeEvent, *};
use proptest::prelude::*;

prop_compose! {
	  // NOTE(hussein-aitlahcen): we use u32 before casting to avoid overflows
	  /// Pseudo random valid simple offer
	  fn simple_offer(min_nb_of_bonds: Balance)
			  (
					  bond_price in MIN_VESTED_TRANSFER as u128..u32::MAX as Balance,
					  nb_of_bonds in min_nb_of_bonds..u32::MAX as Balance,
					  maturity in prop_oneof![
							  Just(BondDuration::Infinite),
							  (1..BlockNumber::MAX / 2).prop_map(|return_in| BondDuration::Finite { return_in })
					  ],
					  // avoid overflowing when advancing blocks and mint_into for a couple of offers
					  reward_amount in MIN_REWARD..Balance::MAX / 2,
					  reward_maturity in 1..BlockNumber::MAX / 2
			  )
			  -> BondOffer<AccountId, MockCurrencyId, Balance, BlockNumber> {
					  BondOffer {
							  beneficiary: ALICE,
							  asset: MockCurrencyId::BTC,
								bond_price,
								nb_of_bonds,
								maturity,
							  reward: BondOfferReward {
									  asset: MockCurrencyId::ETH,
									  amount: Balance::max(MIN_REWARD.saturating_mul(nb_of_bonds), reward_amount),
									  maturity: reward_maturity,
							  }
					  }
			  }
}

proptest! {
	  #![proptest_config(ProptestConfig::with_cases(10_000))]

	  #[test]
	  fn can_create_valid_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
					  let mut offer = offer.clone();
					  offer.beneficiary = BOB;
					  System::set_block_number(1);
					  prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
					  prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
					  let offer_id = BondedFinance::do_offer(&ALICE, offer, false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  System::assert_last_event(RuntimeEvent::BondedFinance(crate::Event::NewOffer{ offer_id, beneficiary: BOB }));
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn stake_taken(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());
						prop_assert_ok!(BondedFinance::do_offer(&ALICE, offer, false));
						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), 0);
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn reward_taken(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), offer.reward.amount);
						prop_assert_ok!(BondedFinance::do_offer(&ALICE, offer.clone(), false));
						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), 0);
					  Ok(())
			  })?;
	  }

	  #[test]
	  fn cancel_refund_reward(offer in simple_offer(2)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), offer.reward.amount);
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(), false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  // Bob bond and take half of the reward
					  let half_nb_of_bonds = offer.nb_of_bonds / 2;
					  let half_reward = offer.reward.amount / 2;
					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, half_nb_of_bonds * offer.bond_price));
					  prop_assert_ok!(BondedFinance::do_bond(offer_id, &BOB, half_nb_of_bonds, false));

					  // Alice cancel the offer
					  prop_assert_ok!(BondedFinance::cancel(RuntimeOrigin::signed(ALICE), offer_id));

					// The remaining half is refunded to alice
		  let precision = 100;
				let epsilon = 5;
				  prop_assert_acceptable_computation_error!(Tokens::balance(offer.reward.asset, &ALICE), half_reward, precision, epsilon);

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn cancel_refund_stake(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), offer.reward.amount);
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(), false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  // Alice cancel the offer
					  prop_assert_ok!(BondedFinance::cancel(RuntimeOrigin::signed(ALICE), offer_id));

					  // The stake is refunded
						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn expected_final_owner(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(), false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_ok!(BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds, false));
					  prop_assert_eq!(
							  BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds, false),
							  Err(Error::<Runtime>::OfferCompleted.into())
					  );


					  match offer.maturity {
							  BondDuration::Infinite => {
									  prop_assert_eq!(
											  Tokens::balance(offer.asset, &offer.beneficiary),
											  offer.total_price().expect("impossible; qed;")
									  );
							  }
							  BondDuration::Finite { return_in } => {
									  prop_assert_eq!(
											  Tokens::balance(offer.asset, &offer.beneficiary),
											  0
									  );
									  System::set_block_number(return_in);
									  prop_assert_ok!(Vesting::claim(RuntimeOrigin::signed(BOB), offer.asset, VestingScheduleIdSet::All));
									  prop_assert_eq!(
											  Tokens::balance(offer.asset, &BOB),
											  offer.total_price().expect("impossible; qed;")
									  );
							  }
					  }

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn isolated_accounts(offer_a in simple_offer(1), offer_b in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer_a.reward.asset, &ALICE, offer_a.reward.amount));
						let offer_a_id = BondedFinance::do_offer(&ALICE, offer_a.clone(), false);
					  prop_assert_ok!(offer_a_id);
					  let offer_a_id = offer_a_id.expect("impossible; qed");

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &BOB, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer_b.reward.asset, &BOB, offer_b.reward.amount));
						let offer_b_id = BondedFinance::do_offer(&BOB, offer_b.clone(), false);
					  prop_assert_ok!(offer_b_id);
					  let offer_b_id = offer_b_id.expect("impossible; qed");

					  prop_assert_ne!(BondedFinance::account_id(offer_a_id), BondedFinance::account_id(offer_b_id));
					  prop_assert_eq!(
							  Tokens::balance(offer_a.reward.asset, &BondedFinance::account_id(offer_a_id)),
							  offer_a.reward.amount
					  );
					  prop_assert_eq!(
							  Tokens::balance(offer_b.reward.asset, &BondedFinance::account_id(offer_b_id)),
							  offer_b.reward.amount
					  );
					  Ok(())
			  })?;
	  }

	  // A user bond for the full offer
	  #[test]
	  fn single_bond(offer in simple_offer(2)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(), false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_ok!(BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds - 1, false));

					  System::assert_last_event(RuntimeEvent::BondedFinance(crate::Event::NewBond {
							  offer_id,
							  who: BOB,
							  nb_of_bonds: offer.nb_of_bonds - 1
					  }));

					  prop_assert_ok!(BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, 1, false));

					  System::assert_has_event(RuntimeEvent::BondedFinance(crate::Event::NewBond {
							  offer_id,
							  who: BOB,
							  nb_of_bonds: 1
					  }));

					  System::assert_last_event(RuntimeEvent::BondedFinance(crate::Event::OfferCompleted { offer_id }));

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn multiple_bonds(offer in simple_offer(2)) {
			  ExtBuilder::build().execute_with(|| {
					  prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
					  prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));

					  let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(),false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  let half_nb_of_bonds = offer.nb_of_bonds / 2;
					  let half_reward = offer.reward.amount / 2;

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, half_nb_of_bonds * offer.bond_price));
					  let bob_reward = BondedFinance::do_bond(offer_id, &BOB, half_nb_of_bonds,false);
					  prop_assert_ok!(bob_reward);
					  let bob_reward = bob_reward.expect("impossible; qed;");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &CHARLIE, half_nb_of_bonds * offer.bond_price));
					  let charlie_reward = BondedFinance::do_bond(offer_id, &CHARLIE, half_nb_of_bonds,false);
					  prop_assert_ok!(charlie_reward);
					  let charlie_reward = charlie_reward.expect("impossible; qed;");

			let precision = 100;
			let epsilon = 5;
					  prop_assert_acceptable_computation_error!(bob_reward, half_reward, precision, epsilon);
					  prop_assert_acceptable_computation_error!(charlie_reward, half_reward, precision, epsilon);

					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &BOB, bob_reward) == WithdrawConsequence::Frozen);
					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &CHARLIE, charlie_reward) == WithdrawConsequence::Frozen);

					  System::set_block_number(offer.reward.maturity);

					  prop_assert_ok!(Vesting::claim(RuntimeOrigin::signed(BOB), offer.reward.asset, VestingScheduleIdSet::All));
					  prop_assert_ok!(Vesting::claim(RuntimeOrigin::signed(CHARLIE), offer.reward.asset, VestingScheduleIdSet::All));

					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &BOB, bob_reward) == WithdrawConsequence::Success);
					  prop_assert!(Tokens::can_withdraw(offer.reward.asset, &CHARLIE, charlie_reward) == WithdrawConsequence::Success);

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn non_existing_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(),false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_eq!(
							  BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id + 1, offer.nb_of_bonds,false),
							  Err(Error::<Runtime>::BondOfferNotFound.into())
					  );

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn invalid_nb_of_bonds(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(),false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_eq!(
							  BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds + 1,false),
							  Err(Error::<Runtime>::InvalidNumberOfBonds.into())
					  );
					  prop_assert_eq!(
							  BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, 0,false),
							  Err(Error::<Runtime>::InvalidNumberOfBonds.into())
					  );

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn offer_completed(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(),false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_ok!(Tokens::mint_into(offer.asset, &BOB, offer.total_price().expect("impossible; qed;")));
					  prop_assert_ok!(BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds,false));
					  prop_assert_eq!(
							  BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds,false),
							  Err(Error::<Runtime>::OfferCompleted.into())
					  );

						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn issuer_cancel_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(),false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_eq!(
							  BondedFinance::cancel(RuntimeOrigin::signed(BOB), offer_id),
							  Err(BadOrigin.into())
					  );

					  prop_assert_ok!(BondedFinance::cancel(RuntimeOrigin::signed(ALICE), offer_id));
						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());
						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), offer.reward.amount);

					  prop_assert_eq!(
							  BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds,false),
							  Err(Error::<Runtime>::BondOfferNotFound.into())
					  );

					  System::assert_last_event(RuntimeEvent::BondedFinance(crate::Event::OfferCancelled { offer_id }));

					  Ok(())
			  })?;
	  }

	  #[test]
	  fn admin_cancel_offer(offer in simple_offer(1)) {
			  ExtBuilder::build().execute_with(|| {
					  System::set_block_number(1);

						prop_assert_ok!(Tokens::mint_into(NATIVE_CURRENCY_ID, &ALICE, Stake::get()));
						prop_assert_ok!(Tokens::mint_into(offer.reward.asset, &ALICE, offer.reward.amount));
						let offer_id = BondedFinance::do_offer(&ALICE, offer.clone(),false);
					  prop_assert_ok!(offer_id);
					  let offer_id = offer_id.expect("impossible; qed");

					  prop_assert_eq!(
							  BondedFinance::cancel(RuntimeOrigin::signed(BOB), offer_id),
							  Err(BadOrigin.into())
					  );

					  prop_assert_ok!(BondedFinance::cancel(RuntimeOrigin::root(), offer_id));
						prop_assert_eq!(Tokens::balance(NATIVE_CURRENCY_ID, &ALICE), Stake::get());
						prop_assert_eq!(Tokens::balance(offer.reward.asset, &ALICE), offer.reward.amount);

					  prop_assert_eq!(
							  BondedFinance::bond(RuntimeOrigin::signed(BOB), offer_id, offer.nb_of_bonds,false),
							  Err(Error::<Runtime>::BondOfferNotFound.into())
					  );

					  System::assert_last_event(RuntimeEvent::BondedFinance(crate::Event::OfferCancelled { offer_id }));

					  Ok(())
			  })?;
	  }
}

#[cfg(test)]
mod test_bond_offer {
	use super::*;
	use crate::BondOfferOf;
	use composable_support::validation::Validate;
	use composable_traits::bonded_finance::{BondDuration, ValidBondOffer};
	use frame_support::assert_ok;
	use mock::Runtime;

	#[test]
	fn test_valid_offer() {
		let valid_bond_offer = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
			nb_of_bonds: 100_000_u128,
			maturity: BondDuration::Infinite,
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::PICA,
				amount: 1_000_000_u128 * 100_000_u128,
				maturity: 96_u64,
			},
		};

		assert_ok!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(valid_bond_offer));

		let valid_bond_offer2 = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
			nb_of_bonds: 1_u128,
			maturity: BondDuration::Finite { return_in: 1 },
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::BTC,
				amount: 1_000_000_u128,
				maturity: 96_u64,
			},
		};

		assert_ok!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(valid_bond_offer2));

		let valid_bond_offer3 = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
			nb_of_bonds: 100_000_u128,
			maturity: BondDuration::Finite { return_in: 1_000_000 },
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::BTC,
				amount: 1_000_000_u128 * 100_000_u128,
				maturity: 96_u64,
			},
		};

		assert_ok!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(valid_bond_offer3));
	}

	#[test]
	fn invalid_bond_price() {
		let invalid = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::PICA,
			bond_price: MIN_VESTED_TRANSFER as u128 - 1,
			nb_of_bonds: 100_000_u128,
			maturity: BondDuration::Infinite,
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::PICA,
				amount: 1_000_000_u128,
				maturity: 96_u64,
			},
		};

		assert!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(invalid)
		.is_err());
	}

	#[test]
	fn test_invalid_nb_of_bonds() {
		let invalid = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: MIN_VESTED_TRANSFER as _,
			nb_of_bonds: 0,
			maturity: BondDuration::Finite { return_in: 1 },
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::BTC,
				amount: 1_000_000_u128,
				maturity: 96_u64,
			},
		};

		assert!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(invalid)
		.is_err());
	}

	#[test]
	fn test_invalid_maturity() {
		let invalid = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
			nb_of_bonds: 100_000_u128,
			maturity: BondDuration::Finite { return_in: 0 },
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::BTC,
				amount: 1_000_000_u128,
				maturity: 96_u64,
			},
		};

		assert!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(invalid)
		.is_err());
	}

	#[test]
	fn test_invalid_reward() {
		let invalid = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
			nb_of_bonds: 100_000_u128,
			maturity: BondDuration::Finite { return_in: 1_000_000 },
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::BTC,
				amount: 0,
				maturity: 96_u64,
			},
		};

		assert!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(invalid)
		.is_err());
	}

	#[test]
	fn test_invalid_reward_less_than_minvested() {
		let invalid = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
			nb_of_bonds: 100_000_u128,
			maturity: BondDuration::Finite { return_in: 1_000_000 },
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::BTC,
				amount: MIN_VESTED_TRANSFER * 1_000_u128 - 1,
				maturity: 96_u64,
			},
		};

		assert!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(invalid)
		.is_err());
	}

	#[test]
	fn test_invalid_reward_maturity() {
		let invalid = BondOfferOf::<Runtime> {
			beneficiary: ALICE,
			asset: mock::MockCurrencyId::BTC,
			bond_price: 1_000_000 + MIN_VESTED_TRANSFER as u128,
			nb_of_bonds: 100_000_u128,
			maturity: BondDuration::Finite { return_in: 1_000_000 },
			reward: BondOfferReward {
				asset: mock::MockCurrencyId::BTC,
				amount: MIN_VESTED_TRANSFER * 1_000_u128 - 1,
				maturity: 0_u64,
			},
		};

		assert!(<ValidBondOffer<MinReward, MinVestedTransfer> as Validate<
			BondOfferOf<Runtime>,
			ValidBondOffer<MinReward, MinVestedTransfer>,
		>>::validate(invalid)
		.is_err());
	}
}
