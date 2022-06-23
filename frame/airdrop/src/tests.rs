use crate::{
	mocks::{
		ethereum_address, generate_accounts, AccountId, Airdrop, AirdropId, Balance, Balances,
		EthereumKey, ExtBuilder, Identity, MockRuntime, Moment, Origin, System, Timestamp,
		PROOF_PREFIX, STAKE,
	},
	models::AirdropState,
	Error,
};
use codec::Encode;
use composable_support::{
	signature_verification,
	types::{CosmosEcdsaSignature, CosmosPublicKey, EcdsaSignature, EthereumAddress},
};
use composable_tests_helpers::prop_assert_ok;
use frame_support::{
	assert_noop, assert_ok,
	traits::{fungible::Inspect, Currency},
};
use hex_literal::hex;
use p256::ecdsa::{signature::Signer, SigningKey, VerifyingKey};
use proptest::prelude::*;
use rand_core::OsRng;
use sp_io::hashing::sha2_256;
use sp_runtime::AccountId32;

const DEFAULT_FUNDED_CLAIM: bool = false;
const DEFAULT_NB_OF_CONTRIBUTORS: u128 = 100;
const DEFAULT_VESTING_SCHEDULE: Moment = 3600 * 24 * 7;
const DEFAULT_VESTING_PERIOD: Moment = 3600 * 24 * 7 * 10;
const DEFAULT_REWARD: Balance = 10_000;
const CREATOR: AccountId = AccountId32::new([0_u8; 32]);
const OTHER: AccountId = AccountId32::new([1_u8; 32]);

prop_compose! {
	fn vesting_point()
	(x in 1..(DEFAULT_VESTING_PERIOD / DEFAULT_VESTING_SCHEDULE)) -> Moment {
		x * DEFAULT_VESTING_SCHEDULE
	}
}

fn with_recipients<R>(
	count: u128,
	reward: Balance,
	funded_claim: bool,
	vesting_schedule: Moment,
	vesting_period: Moment,
	execute: impl FnOnce(&dyn Fn(Moment), Vec<(AccountId, Identity)>) -> R,
) -> R {
	let accounts = generate_accounts(count as _);
	let recipients = accounts
		.iter()
		.map(|(_, account)| (account.as_remote_public(), reward, vesting_period, funded_claim))
		.collect();

	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(0xDEADC0DE);
		let creator = Origin::signed(CREATOR);
		let start_moment = 0xCAFEBABE;
		let set_moment = |x: Moment| Timestamp::set_timestamp(start_moment + x);

		Balances::make_free_balance_be(&CREATOR, STAKE + reward * count);

		assert_ok!(Airdrop::create_airdrop(creator.clone(), Some(start_moment), vesting_schedule));
		assert_ok!(Airdrop::add_recipient(creator, AirdropId::from(1_u32), recipients));

		execute(&set_moment, accounts)
	})
}

fn with_default_recipients<R>(
	execute: impl FnOnce(&dyn Fn(Moment), Vec<(AccountId, Identity)>) -> R,
) -> R {
	with_recipients(
		DEFAULT_NB_OF_CONTRIBUTORS,
		DEFAULT_REWARD,
		DEFAULT_FUNDED_CLAIM,
		DEFAULT_VESTING_SCHEDULE,
		DEFAULT_VESTING_PERIOD,
		execute,
	)
}

#[cfg(test)]
mod create_airdrop {

	use super::*;

	#[test]
	fn should_create_airdrop_without_start_successfully() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = None;
		let vesting_schedule = DEFAULT_VESTING_PERIOD;

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, STAKE);

			assert_ok!(Airdrop::create_airdrop(creator, start, vesting_schedule));
			assert_eq!(Airdrop::airdrop_count(), 1);
			assert_eq!(Balances::balance(&Airdrop::get_airdrop_account_id(1)), STAKE);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn should_create_airdrop_with_start_successfully() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, STAKE);

			assert_ok!(Airdrop::create_airdrop(creator, start, vesting_schedule));
			assert_eq!(Airdrop::airdrop_count(), 1);
			assert_eq!(Balances::balance(&Airdrop::get_airdrop_account_id(1)), STAKE);
			assert_eq!(Airdrop::airdrops(1).unwrap().start, start);
		})
	}

	#[test]
	fn should_fail_to_create_an_airdrop_in_the_past() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, STAKE);
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
mod add_recipient {

	use super::*;

	#[test]
	fn should_fail_to_add_recipients_if_origin_is_not_creator() {
		let creator = Origin::signed(CREATOR);
		let other = Origin::signed(OTHER);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;
		let accounts = generate_accounts(128);
		let recipients = accounts
			.iter()
			.map(|(_, account)| {
				(
					account.as_remote_public(),
					DEFAULT_REWARD,
					DEFAULT_VESTING_PERIOD,
					DEFAULT_FUNDED_CLAIM,
				)
			})
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, STAKE);

			assert_ok!(Airdrop::create_airdrop(creator, start, vesting_schedule));
			assert_noop!(
				Airdrop::add_recipient(other, 1, recipients),
				Error::<MockRuntime>::NotAirdropCreator
			);
		})
	}

	#[test]
	fn should_fail_to_add_recipients_if_origin_has_insufficient_funds() {
		let creator = Origin::signed(CREATOR);
		let start: Option<Moment> = Some(DEFAULT_VESTING_PERIOD * 2);
		let vesting_schedule = DEFAULT_VESTING_PERIOD;
		let accounts = generate_accounts(128);
		let recipients = accounts
			.iter()
			.map(|(_, account)| {
				(
					account.as_remote_public(),
					DEFAULT_REWARD,
					DEFAULT_VESTING_PERIOD,
					DEFAULT_FUNDED_CLAIM,
				)
			})
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, STAKE);

			assert_ok!(Airdrop::create_airdrop(creator.clone(), start, vesting_schedule));
			assert_noop!(
				Airdrop::add_recipient(creator, 1, recipients),
				pallet_balances::Error::<MockRuntime>::InsufficientBalance
			);
		})
	}

	#[test]
	fn should_fail_to_add_recipients_if_airdrop_does_not_exist() {
		let creator = Origin::signed(CREATOR);
		let accounts = generate_accounts(128);
		let recipients = accounts
			.iter()
			.map(|(_, account)| {
				(
					account.as_remote_public(),
					DEFAULT_REWARD,
					DEFAULT_VESTING_PERIOD,
					DEFAULT_FUNDED_CLAIM,
				)
			})
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, STAKE);

			assert_noop!(
				Airdrop::add_recipient(creator, 1, recipients),
				Error::<MockRuntime>::AirdropDoesNotExist
			);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn should_add_recipients_successfully() {
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
mod remove_recipient {

	use super::*;

	#[test]
	fn should_fail_to_remove_recipient_if_origin_is_not_creator() {
		let other = Origin::signed(OTHER);

		with_default_recipients(|_, accounts| {
			assert_noop!(
				Airdrop::remove_recipient(other, 1, accounts[0].1.as_remote_public()),
				Error::<MockRuntime>::NotAirdropCreator
			);
		})
	}

	#[test]
	fn should_fail_to_remove_recipient_if_recipient_started_claiming() {
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
	fn should_prune_airdrop_if_last_recipient_is_removed() {
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
	fn should_remove_recipient_successfully() {
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

#[cfg(test)]
mod enable_airdrop {

	use super::*;

	#[test]
	fn should_fail_to_enable_airdrop_if_origin_is_not_creator() {
		let other = Origin::signed(OTHER);

		with_default_recipients(|_, _| {
			assert_noop!(
				Airdrop::enable_airdrop(other, 1),
				Error::<MockRuntime>::NotAirdropCreator
			);
		})
	}

	#[test]
	fn should_fail_to_enable_airdrop_if_airdrop_has_already_been_scheduled() {
		let creator = Origin::signed(CREATOR);
		let start_at = Some(DEFAULT_VESTING_PERIOD * 2);

		ExtBuilder::default().build().execute_with(|| {
			Balances::make_free_balance_be(&CREATOR, STAKE);

			assert_ok!(Airdrop::create_airdrop(creator.clone(), start_at, DEFAULT_VESTING_PERIOD));
			assert_noop!(
				Airdrop::enable_airdrop(creator, 1),
				Error::<MockRuntime>::AirdropAlreadyStarted
			);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn should_enable_airdrop_successfully() {
		with_default_recipients(|set_moment, _| {
			set_moment(DEFAULT_VESTING_PERIOD * 2);

			assert_eq!(Airdrop::get_airdrop_state(1).unwrap(), AirdropState::Enabled);
		})
	}
}

#[cfg(test)]
mod disable_airdrop {
	use super::*;

	#[test]
	fn should_fail_to_disable_airdrop_if_origin_is_not_creator() {
		let other = Origin::signed(OTHER);

		with_default_recipients(|_, _| {
			assert_noop!(
				Airdrop::disable_airdrop(other, 1),
				Error::<MockRuntime>::NotAirdropCreator
			);
		})
	}

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn should_disable_airdrop_successfully() {
		let creator = Origin::signed(CREATOR);

		with_default_recipients(|set_moment, _| {
			set_moment(DEFAULT_VESTING_PERIOD * 2);

			assert_eq!(Airdrop::get_airdrop_state(1).unwrap(), AirdropState::Enabled);
			assert_ok!(Airdrop::disable_airdrop(creator, 1));
			assert!(Airdrop::airdrops(1).is_none());
		})
	}
}

#[cfg(test)]
mod claim {
	use super::*;

	#[test]
	fn should_give_full_fund_to_recipients_at_end_of_vesting_period() {
		with_default_recipients(|set_moment, accounts| {
			set_moment(DEFAULT_VESTING_PERIOD);

			for (local_account, remote_account) in accounts {
				assert_ok!(remote_account.claim(1, local_account.clone()));
				assert_eq!(Balances::balance(&local_account), DEFAULT_REWARD);
			}
		})
	}

	#[test]
	fn should_prune_airdrop_if_all_funds_are_claimed() {
		with_default_recipients(|set_moment, accounts| {
			set_moment(DEFAULT_VESTING_PERIOD);

			for (local_account, remote_account) in accounts {
				assert_ok!(remote_account.claim(1, local_account.clone()));
			}

			assert!(Airdrop::airdrops(1).is_none());
		})
	}

	#[test]
	fn should_fail_when_nothing_to_claim() {
		with_default_recipients(|set_moment, accounts| {
			set_moment(1);

			for (local_account, remote_account) in accounts {
				assert_noop!(
					remote_account.claim(1, local_account.clone()),
					Error::<MockRuntime>::NothingToClaim
				);
			}
		})
	}

	proptest! {
		#![proptest_config(ProptestConfig::with_cases((DEFAULT_VESTING_PERIOD / DEFAULT_VESTING_SCHEDULE) as u32))]

		#[test]
		fn should_give_fund_proportional_to_the_vesting_point(vesting_point in vesting_point()) {
			with_default_recipients(|set_moment, accounts| {
				let vesting_window = vesting_point.saturating_sub(vesting_point % DEFAULT_VESTING_SCHEDULE);
				let should_have_claimed = DEFAULT_REWARD.saturating_mul(vesting_window as u128) / (DEFAULT_VESTING_PERIOD as u128);
				set_moment(vesting_point);

				for (local_account, remote_account) in accounts {
					prop_assert_ok!(remote_account.claim(1, local_account.clone()));
					prop_assert_eq!(Balances::balance(&local_account), should_have_claimed );
				}
				Ok(())
			})?;
		}
	}
}

#[cfg(test)]
mod ethereum_recover {
	use super::*;

	#[test]
	#[allow(clippy::disallowed_methods)] // Allow unwrap
	fn should_recover_hard_coded_eth_address() {
		let eth_address = EthereumAddress(hex!("176FD6F90730E02D2AF55681c65a115C174bA2C7"));
		let eth_account = EthereumKey::parse(&hex!(
			"29134835563739bae90483ee3d80945edf2c87a9b55c9193a694291cfdf23a05"
		))
		.unwrap();

		assert_eq!(ethereum_address(&eth_account), eth_address);

		// sign(concat("picasso-"), CREATOR) = sign(concat("picasso-", [0u8; 32]))
		let eth_proof = EcdsaSignature(hex!("42f2fa6a3db41e6654891e4408ce56ba31fc2b4dea18e82db1c78e33a3f65a55119a23fa7b3fe7a5088197a74a0102266836bb721461b9eaef128bec120db0401c"));

		// Make sure we are able to recover the address
		let recovered_address = signature_verification::ethereum_recover(
			PROOF_PREFIX,
			&CREATOR.using_encoded(|x| hex::encode(x).as_bytes().to_vec()),
			&eth_proof,
		);

		assert_eq!(Some(eth_address), recovered_address);
	}
}

#[cfg(test)]
mod cosmos_recover {
	use super::*;

	#[test]
	fn should_verify_r1_sig_and_pub_key() {
		// TODO: Use constant values for testing instead of random
		let sign_key = SigningKey::random(&mut OsRng);
		let verify_key = VerifyingKey::from(&sign_key);
		let message =
			sha2_256(format!("picasso-{}", &CREATOR.using_encoded(|x| hex::encode(x))).as_bytes());
		let mut sig: [u8; 64] = [0; 64];
		sig.copy_from_slice(sign_key.sign(&message).to_vec().as_slice());

		let mut pub_key: [u8; 33] = [0; 33];
		pub_key.copy_from_slice(verify_key.to_encoded_point(true).as_bytes());

		let verified = signature_verification::cosmos_recover(
			PROOF_PREFIX,
			&CREATOR.using_encoded(|x| hex::encode(x).as_bytes().to_vec()),
			CosmosPublicKey::Secp256r1(pub_key),
			&CosmosEcdsaSignature(sig),
		);

		assert_eq!(verified, Some(CosmosPublicKey::Secp256r1(pub_key)));
	}
}
