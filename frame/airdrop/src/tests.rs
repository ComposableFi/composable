use crate::{
	mocks::{
		ethereum_address, generate_accounts, AccountId, Airdrop, AirdropId, Balance, Balances,
		ClaimKey, EthKey, ExtBuilder, MockRuntime, Moment, Origin, Stake, System, Timestamp,
		PROOF_PREFIX, VESTING_STEP,
	},
	models::AirdropState,
	AccountIdOf, Error,
};
use composable_support::types::{EcdsaSignature, EthereumAddress};
use frame_support::{assert_err, assert_noop, assert_ok, traits::Currency};
use hex_literal::hex;
use sp_core::{ed25519, storage::StateVersion, Pair};
use sp_runtime::AccountId32;

const DEFAULT_FUNDED_CLAIM: bool = false;
const DEFAULT_NB_OF_CONTRIBUTORS: u128 = 100;
const DEFAULT_VESTING_PERIOD: Moment = 3600 * 24 * 7 * 10;
const DEFAULT_REWARD: Balance = 10_000;
const CREATOR: AccountId = AccountId32::new([0_u8; 32]);
const OTHER: AccountId = AccountId32::new([1_u8; 32]);

fn with_recipients<R>(
	count: u128,
	reward: Balance,
	funded_claim: bool,
	vesting_period: Moment,
	execute: impl FnOnce(&dyn Fn(Moment), Vec<(AccountId, ClaimKey)>) -> R,
) -> R {
	let accounts = generate_accounts(count as _);
	let recipients = accounts
		.iter()
		.map(|(_, account)| (account.as_remote_public(), reward, funded_claim))
		.collect();

	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(0xDEADC0DE);
		let creator = Origin::signed(CREATOR);
		let start_moment = 0xCAFEBABE;
		let set_moment = |x: Moment| Timestamp::set_timestamp(start_moment + x);

		Balances::make_free_balance_be(&CREATOR, 10_000 + reward * count);

		assert_ok!(Airdrop::create_airdrop(creator.clone(), Some(start_moment), vesting_period));
		assert_ok!(Airdrop::add_recipient(creator, AirdropId::from(1_u32), recipients));

		execute(&set_moment, accounts)
	})
}

fn with_default_recipients<R>(
	execute: impl FnOnce(&dyn Fn(Moment), Vec<(AccountId, ClaimKey)>) -> R,
) -> R {
	with_recipients(
		DEFAULT_NB_OF_CONTRIBUTORS,
		DEFAULT_REWARD,
		DEFAULT_FUNDED_CLAIM,
		DEFAULT_VESTING_PERIOD,
		execute,
	)
}

#[cfg(test)]
mod create_airdrop_tests {

	use frame_support::traits::fungible::Inspect;

	use super::*;

	#[test]
	fn create_valid_airdrop_no_start() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = None;
		let vesting_schedule = DEFAULT_VESTING_PERIOD;

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			assert_ok!(Airdrop::create_airdrop(creator, start, vesting_schedule));
			assert_eq!(Airdrop::airdrop_count(), 1);
			assert_eq!(Balances::balance(&Airdrop::get_airdrop_account_id(1)), 10_000);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn create_valid_airdrop_with_start() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			assert_ok!(Airdrop::create_airdrop(creator, start, vesting_schedule));
			assert_eq!(Airdrop::airdrop_count(), 1);
			assert_eq!(Balances::balance(&Airdrop::get_airdrop_account_id(1)), 10_000);
			assert_eq!(Airdrop::airdrops(1).unwrap().start, start);
		})
	}

	/// bttf - BackToTheFuture
	#[test]
	fn create_invalid_airdrop_bttf() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			Timestamp::set_timestamp(DEFAULT_VESTING_PERIOD * 3);
			assert_noop!(
				Airdrop::create_airdrop(creator, start, vesting_schedule),
				Error::<MockRuntime>::BackToTheFuture
			);
			assert_eq!(Airdrop::airdrop_count(), 0);
		})
	}
}

#[cfg(test)]
mod add_recipient_tests {

	use super::*;

	#[test]
	fn add_recipient_invalid_creator() {
		let creator = Origin::signed(CREATOR);
		let other = Origin::signed(OTHER);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;
		let accounts = generate_accounts(128);
		let recipients = accounts
			.iter()
			.map(|(_, account)| (account.as_remote_public(), DEFAULT_REWARD, DEFAULT_FUNDED_CLAIM))
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			assert_ok!(Airdrop::create_airdrop(creator, start, vesting_schedule));
			assert_noop!(
				Airdrop::add_recipient(other, 1, recipients),
				Error::<MockRuntime>::NotAirdropCreator
			);
		})
	}

	#[test]
	fn add_recipient_insufficient_funds() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;
		let accounts = generate_accounts(128);
		let recipients = accounts
			.iter()
			.map(|(_, account)| (account.as_remote_public(), DEFAULT_REWARD, DEFAULT_FUNDED_CLAIM))
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			assert_ok!(Airdrop::create_airdrop(creator.clone(), start, vesting_schedule));
			assert_noop!(
				Airdrop::add_recipient(creator, 1, recipients),
				pallet_balances::Error::<MockRuntime>::InsufficientBalance
			);
		})
	}

	#[test]
	fn add_recipient_airdrop_does_not_exist() {
		let creator = Origin::signed(CREATOR);
		let accounts = generate_accounts(128);
		let recipients = accounts
			.iter()
			.map(|(_, account)| (account.as_remote_public(), DEFAULT_REWARD, DEFAULT_FUNDED_CLAIM))
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			assert_noop!(
				Airdrop::add_recipient(creator, 1, recipients),
				Error::<MockRuntime>::AirdropDoesNotExist
			);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn add_recipient_valid() {
		with_default_recipients(|_, accounts| {
			assert_eq!(Airdrop::total_airdrop_recipients(1), DEFAULT_NB_OF_CONTRIBUTORS as u32);

			for (_, remote_account) in accounts {
				let recipient_fund =
					Airdrop::recipient_funds(1, remote_account.as_remote_public()).unwrap();
				assert_eq!(recipient_fund.total, DEFAULT_REWARD);
				assert_eq!(recipient_fund.funded_claim, DEFAULT_FUNDED_CLAIM);
			}
		});
	}
}

#[cfg(test)]
mod remove_recipient_tests {

	use super::*;

	#[test]
	fn remove_recipient_invalid_creator() {
		let other = Origin::signed(OTHER);

		with_default_recipients(|_, accounts| {
			assert_noop!(
				Airdrop::remove_recipient(other, 1, accounts[0].1.as_remote_public()),
				Error::<MockRuntime>::NotAirdropCreator
			);
		})
	}

	#[test]
	fn remove_recipient_already_claimed() {
		let creator = Origin::signed(CREATOR);

		with_default_recipients(|set_moment, accounts| {
			set_moment(DEFAULT_VESTING_PERIOD);
			assert_ok!(Airdrop::claim(
				Origin::none(),
				1,
				accounts[0].clone().0,
				accounts[0].clone().1.proof(accounts[0].clone().0)
			));
			assert_noop!(
				Airdrop::remove_recipient(creator, 1, accounts[0].1.as_remote_public()),
				Error::<MockRuntime>::RecipientAlreadyClaimed
			);
		})
	}

	#[test]
	fn remove_recipient_trigger_prune() {
		let creator = Origin::signed(CREATOR);

		with_default_recipients(|_, accounts| {
			assert!(Airdrop::airdrops(1).is_some());
			for (_, remote_account) in accounts {
				assert_ok!(Airdrop::remove_recipient(
					creator.clone(),
					1,
					remote_account.as_remote_public()
				));
			}
			assert!(Airdrop::airdrops(1).is_none());
		})
	}

	#[test]
	fn remove_recipient_valid_no_prune() {
		let creator = Origin::signed(CREATOR);

		with_default_recipients(|_, accounts| {
			assert_ok!(Airdrop::remove_recipient(
				creator.clone(),
				1,
				accounts[0].1.as_remote_public()
			));
			assert!(Airdrop::recipient_funds(1, accounts[0].1.as_remote_public()).is_none());
		})
	}
}

mod enable_airdrop_tests {

	use super::*;

	#[test]
	fn enable_airdrop_invalid_creator() {
		let other = Origin::signed(OTHER);

		with_default_recipients(|_, accounts| {
			assert_noop!(
				Airdrop::enable_airdrop(other, 1),
				Error::<MockRuntime>::NotAirdropCreator
			);
		})
	}

	#[test]
	fn enable_airdrop_already_started() {
		let creator = Origin::signed(CREATOR);
		let start_at = Some(DEFAULT_VESTING_PERIOD * 2);

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			assert_ok!(Airdrop::create_airdrop(creator.clone(), start_at, DEFAULT_VESTING_PERIOD));
			assert_noop!(
				Airdrop::enable_airdrop(creator, 1),
				Error::<MockRuntime>::AirdropAlreadyStarted
			);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn enable_airdrop_valid() {
		let creator = Origin::signed(CREATOR);

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, 10_000);
			assert_ok!(Airdrop::create_airdrop(creator.clone(), None, DEFAULT_VESTING_PERIOD));
			assert_eq!(Airdrop::get_airdrop_state(1).unwrap(), AirdropState::Created);
			assert_ok!(Airdrop::enable_airdrop(creator, 1));
			assert_eq!(Airdrop::get_airdrop_state(1).unwrap(), AirdropState::Enabled);
		})
	}
}
